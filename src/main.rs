use std::{
    ops::{BitAnd, BitOr},
    time::{SystemTime, UNIX_EPOCH},
};

use mlua::prelude::*;
mod lua_sources;
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

    // let show_error_message = lua.create_function(|_, val: LuaValue| -> LuaResult<()> {
    //     eprintln!("show_error_message: {:#?}", val);
    //     Ok(())
    // })?;
    // lua.globals().get::<_, LuaTable>("launch").unwrap().set("ShowErrMsg", show_error_message)?;

    let bit = lua.create_table()?;
    bit.set(
        "rshift",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x >> y) })?,
    )?;
    bit.set(
        "band",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x.bitand(y)) })?,
    )?;
    bit.set(
        "bor",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x.bitor(y)) })?,
    )?;
    bit.set(
        "bxor",
        lua.create_function(|_, (x, y): (i32, i32)| -> LuaResult<i32> { Ok(x ^ y) })?,
    )?;
    bit.set(
        "bnot",
        lua.create_function(|_, x: i32| -> LuaResult<i32> { Ok(!x) })?,
    )?;
    lua.globals().set("bit", bit)?;

    let require = lua.create_function(|lua, module: LuaString| -> LuaResult<Option<LuaTable>> {
        let source = match module.to_str()? {
            "xml" => include_str!("../PathOfBuilding/runtime/lua/xml.lua"),
            "base64" => include_str!("../PathOfBuilding/runtime/lua/base64.lua"),
            "sha1" => include_str!("../PathOfBuilding/runtime/lua/sha1/init.lua"),
            "sha1.common" => include_str!("../PathOfBuilding/runtime/lua/sha1/common.lua"),

            "bit" => "return bit",
            "sha1.bit_ops" => include_str!("../PathOfBuilding/runtime/lua/sha1/bit_ops.lua"),

            "lua-profiler" => "return nil",

            // "bit32" => "return { }",
            _ => unimplemented!("Require: {} is not yet implemented", module.to_str()?),
        };

        let chunk = lua.load(source);
        let func = chunk.into_function().unwrap();
        let table: Option<LuaTable> = func.call(()).unwrap();
        Ok(table)
    })?;
    lua.globals().set("require", require)?;

    let pload_module =
        lua.create_function(|lua, module: LuaString| -> LuaResult<LuaMultiValue> {
            let source = match module.to_str()? {
                "Modules/Main" => include_str!("../PathOfBuilding/src/Modules/Main.lua"),
                _ => unimplemented!("{} is not yet implemented", module.to_str()?),
            };

            let chunk = lua.load(source);
            let module_function = chunk.into_function().unwrap();
            let module_table: LuaTable = module_function.call(()).unwrap();
            // nil means no error
            Ok(lua.pack_multi((LuaNil, module_table)).unwrap())
        })?;
    lua.globals().set("PLoadModule", pload_module)?;

    let pcall = lua.create_function(
        |_, (func, todo_name_this): (LuaFunction, LuaTable)| -> LuaResult<()> {
            func.call::<_, Option<LuaValue>>(todo_name_this).unwrap();
            Ok(())
        },
    )?;
    lua.globals().set("PCall", pcall)?;

    lua.globals().set("arg", lua.create_table()?)?;
    let load_module = lua.create_function(
        |lua, (module, args): (LuaString, LuaMultiValue)| -> LuaResult<LuaMultiValue> {
            let source = lua_sources::get_lua_source(module.to_str().unwrap());
            dbg!(&module.to_str().unwrap(), &args.len());
            let chunk = lua.load(source);
            let chunk = chunk.set_name(module.to_str().unwrap());
            let module_function = chunk.into_function().unwrap();

            match module_function.call(args.clone()) {
                Ok(module) => return Ok(module),
                Err(e) => panic!(
                    "Failed to include module {}, error: {}",
                    module.to_str().unwrap(),
                    e
                ),
            };
        },
    )?;
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

    let set_main_object = lua
        .create_function(|lua, obj: mlua::Table| -> LuaResult<()> {
            lua.globals().set("MainObject", obj).unwrap();
            Ok(())
        })
        .unwrap();
    lua.globals().set("SetMainObject", set_main_object)?;

    lua.globals()
        .set(
            "GetScriptPath",
            lua.create_function(|lua, ()| -> LuaResult<LuaString> { Ok(lua.create_string("")?) })
                .unwrap(),
        )
        .unwrap();
    
    lua.globals()
        .set(
            "GetRuntimePath",
            lua.create_function(|lua, ()| -> LuaResult<LuaString> { Ok(lua.create_string("")?) })
                .unwrap(),
        )
        .unwrap();
    lua.globals()
    .set(
        "MakeDir",
        lua.create_function(|lua, path: LuaString| -> LuaResult<()> { 
            drop(std::fs::create_dir(path.to_str().unwrap()));
            Ok(())
        })
            .unwrap(),
    )
    .unwrap();
    // launch: very ambitious

    let c = lua.load(
        include_str!("../PathOfBuilding/src/Launch.lua")
            .replace("#@ SimpleGraphic", "-- #@ SimpleGraphic"),
    );

    c.set_name("launch.lua").exec()?;

    let launch: LuaTable = lua.globals().get("launch")?;
    let on_init: LuaFunction = launch.get("OnInit")?;

    let this = lua.create_table()?;
    on_init.call::<_, ()>(this)?;

    // let headless = lua
    //     .load(lua_sources::get_lua_source("HeadlessWrapper"))
    //     .set_name("HeadlessWrapper.lua");
    // headless.exec().unwrap();

    Ok(())
}

fn main() {
    println!("Hello emscripten");

    match run() {
        Ok(_) => println!("Ran and exited successfully"),
        Err(LuaError::RuntimeError(e)) => {
            println!("Runtime Error: {}", e.replace("\\\\n", "\n"))
        }
        Err(e) => println!("Error: {:#?}", e),
    }
}
