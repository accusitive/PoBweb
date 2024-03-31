use std::time::{SystemTime, UNIX_EPOCH};

use mlua::{ffi::lua, prelude::*};
use quick_xml::{events::Event, Reader};

fn run() -> Result<(), LuaError> {
    let lua = unsafe { Lua::unsafe_new_with(LuaStdLib::ALL, LuaOptions::new()) };

    let get_time = lua
        .create_function(|_, ()| -> LuaResult<i64> {
            let now = SystemTime::now();
            let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
            let milliseconds_since_epoch = duration_since_epoch.as_millis();
            Ok(milliseconds_since_epoch.try_into().unwrap())
        })
        .unwrap();
    lua.globals().set("GetTime", get_time)?;

    let set_window_title = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("SetWindowTitle", set_window_title)?;

    let require = lua.create_function(|lua, module: LuaString| -> LuaResult<LuaTable> {
        let output = lua.create_table()?;
        match module.to_str()? {
            "xml" => {
                let load_xml_file =
                    lua.create_function(|lua, file: LuaString| -> LuaResult<LuaTable> {
                        // let source = match file.to_str()? {
                        //     "manifest.xml" | "../manifest.xml" => include_str!("../PathOfBuilding/manifest.xml"),
                        //     _ => unimplemented!()
                        // };

                        let source = r#"<test> <test2><test3/></test2> </test>"#;
                        // dbg!(&source);
                        let table = lua.create_table()?;
                        let mut reader = Reader::from_str(source);
                        reader.trim_text(false);

                        // lua indexes from 1 :/
                        let mut count = 1;
                        // let mut txt = Vec::new();
                        let mut buf = Vec::new();
                        loop {
                            // dbg!(&reader.buffer_position(), &reader.get_ref().len());
                            match reader.read_event_into(&mut buf) {
                                Err(e) => panic!(
                                    "Error at position {}: {:?}",
                                    reader.buffer_position(),
                                    e
                                ),
                                Ok(Event::Empty(e)) => {
                                    println!("Empty !?");
                                }
                                Ok(Event::Start(e)) => {
                                    println!(
                                        "Started element {:?}",
                                        std::str::from_utf8(e.name().as_ref()).unwrap()
                                    );
                                }
                                Ok(Event::Eof) => {
                                    dbg!(&"eof");
                                    break;
                                }

                                _ => (),
                            }
                            buf.clear();
                        }
                        println!("Finished gracefully");
                        dbg!(&table);
                        Ok(table)
                    })?;
                output.set("LoadXMLFile", load_xml_file)?;
            }
            _ => unimplemented!(),
        }
        Ok(output)
    })?;
    lua.globals().set("require", require)?;

    let con_execute = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("ConExecute", con_execute)?;

    let set_main_object = lua
        .create_function(|state, obj: mlua::Table| -> LuaResult<()> {
            // state.inspect_stack(0).unwrap().stack().num_ups
            // state.inspect_stack(0).unwrap().

            dbg!("It passed in a table", &obj);
            Ok(())
        })
        .unwrap();
    lua.globals().set("SetMainObject", set_main_object)?;

    let c = lua.load(
        include_str!("../PathOfBuilding/src/Launch.lua")
            .replace("#@ SimpleGraphic", "-- #@ SimpleGraphic"),
    );
    c.exec()?;

    lua.globals()
        .get::<_, LuaTable>("package")?
        .get::<_, LuaTable>("preload")?
        .set("xml", lua.create_table().unwrap())?;

    let launch: LuaTable = lua.globals().get("launch")?;
    let on_init: LuaFunction = launch.get("OnInit")?;

    let this = lua.create_table()?;
    on_init.call::<_, ()>(this)?;
    Ok(())
}
fn main() {
    println!("Hello emscripten");
    match run() {
        Ok(_) => println!("Ran and exited successfully"),
        Err(LuaError::RuntimeError(e)) => println!("Runtime Error: {}", e.replace("\\\\n", "\n")),
        Err(e) => println!("Error: {:#?}", e),
    }
    // dbg!(&lua.globals());
    // let f: LuaFunction = lua.globals().get("launch:OnInit").unwrap();
    // f.call::<_, ()>(()).unwrap();

    // let _: () = chunk.call("OnInit").unwrap();
    // chunk.exec().unwrap();
}
