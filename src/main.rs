use std::{
    fs::File,
    io::{self, Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

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
        let source = match module.to_str()? {
            "xml" => include_str!("../PathOfBuilding/runtime/lua/xml.lua"),
            "base64" => include_str!("../PathOfBuilding/runtime/lua/base64.lua"),
            "sha1" => include_str!("../PathOfBuilding/runtime/lua/sha1/init.lua"),
            "sha1.common" => include_str!("../PathOfBuilding/runtime/lua/sha1/common.lua"),

            _ => unimplemented!("{} is not yet implemented", module.to_str()?),
        };

        let chunk = lua.load(source);
        let func = chunk.into_function().unwrap();
        let table: LuaTable = func.call(()).unwrap();
        Ok(table)
    })?;
    lua.globals().set("require", require)?;

    let pload_module = lua.create_function(|lua, module: LuaString| -> LuaResult<LuaTable> {
        let source = match module.to_str()? {
            "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
            _ => unimplemented!("{} is not yet implemented", module.to_str()?),
        };

        let chunk = lua.load(source);
        let module_function = chunk.into_function().unwrap();
        let module_table: LuaTable = module_function.call(()).unwrap();
        Ok(module_table)
    })?;
    lua.globals().set("PLoadModule", pload_module)?;

    let load_module =
        lua.create_function(|lua, module: LuaString| -> LuaResult<Option<LuaTable>> {
            let source = match module.to_str()? {
                "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
                /*
                            LoadModule("GameVersions")
                LoadModule("Modules/Common")
                LoadModule("Modules/Data")
                LoadModule("Modules/ModTools")
                LoadModule("Modules/ItemTools")
                LoadModule("Modules/CalcTools")
                LoadModule("Modules/PantheonTools")
                LoadModule("Modules/BuildSiteTools")
                 */
                "GameVersions" => {
                    include_str!("../PathOfBuilding/src/GameVersions.lua")
                }
                "Modules/Common" => include_str!("../PathOfBuilding/src/Modules/Common.lua"),
                "Modules/Data" => include_str!("../PathOfBuilding/src/Modules/Data.lua"),
                "Modules/ModTools" => include_str!("../PathOfBuilding/src/Modules/ModTools.lua"),
                "Modules/ItemTools" => include_str!("../PathOfBuilding/src/Modules/ItemTools.lua"),
                "Modules/CalcTools" => include_str!("../PathOfBuilding/src/Modules/CalcTools.lua"),
                "Modules/PantheonTools" => {
                    include_str!("../PathOfBuilding/src/Modules/PantheonTools.lua")
                }
                "Modules/BuildSiteTools" => {
                    include_str!("../PathOfBuilding/src/Modules/BuildSiteTools.lua")
                }
                _ => unimplemented!("{} is not yet implemented", module.to_str()?),
            };

            let chunk = lua.load(source);
            let chunk = chunk.set_name(module.to_str().unwrap());
            let module_function = chunk.into_function().unwrap();
            let module_table: Option<LuaTable> = module_function
                .call(())
                .expect(&format!("Failed to load module {}", module.to_str()?));
            Ok(module_table)
        })?;
    lua.globals().set("LoadModule", load_module)?;

    let con_execute = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("ConExecute", con_execute)?;

    let render_init = lua.create_function(|_, ()| -> LuaResult<()> { Ok(()) })?;
    lua.globals().set("RenderInit", render_init)?;

    let con_printf = lua.create_function(|_, val: LuaValue| -> LuaResult<()> {
        println!("con_printf: {:#?}", val);
        Ok(())
    })?;
    lua.globals().set("ConPrintf", con_printf)?;

    let bit = lua.create_table()?;
    bit.set(
        "rshift",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x >> y) })?,
    )?;
    bit.set(
        "band",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x & y) })?,
    )?;
    bit.set(
        "bxor",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x ^ y) })?,
    )?;
    lua.globals().set("bit", bit)?;

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

    // lua.globals()
    //     .get::<_, LuaTable>("package")?
    //     .get::<_, LuaTable>("preload")?
    //     .set("xml", lua.create_table().unwrap())?;

    let launch: LuaTable = lua.globals().get("launch")?;
    let on_init: LuaFunction = launch.get("OnInit")?;

    let this = lua.create_table()?;
    on_init.call::<_, ()>(this)?;
    Ok(())
}
fn test_fs() -> io::Result<()> {
    let mut file = File::create("hello.txt")?;
    file.write_all(b"hello world")?;
    file.flush()?;
    let mut file = File::open("hello.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    assert_eq!(content.trim(), "hello world");
    println!("File content: {}", content);
    Ok(())
}
fn main() {
    println!("Hello emscripten");

    drop(test_fs());
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
