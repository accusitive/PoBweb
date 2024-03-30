use mlua::{ffi::lua, prelude::*};

fn main() {
    println!("Hello emscripten");
    let lua = Lua::new();

    let get_time = lua.create_function(|_, ()| -> LuaResult<i32> {
        Ok(0)
    }).unwrap();
    lua.globals().set("GetTime", get_time).unwrap();

    let set_window_title = lua.create_function(|_, ()| -> LuaResult<()> {
        Ok(())
    }).unwrap();
    lua.globals().set("SetWindowTitle", set_window_title).unwrap();

    let con_execute = lua.create_function(|_, ()| -> LuaResult<()> {
        Ok(())
    }).unwrap();
    lua.globals().set("ConExecute", con_execute).unwrap();

    let set_main_object = lua.create_function(|state, obj: mlua::Table| -> LuaResult<()> {
        // state.inspect_stack(0).unwrap().stack().num_ups
            // state.inspect_stack(0).unwrap().
            
            dbg!("It passed in a table", &obj);
        Ok(())
    }).unwrap();
    lua.globals().set("SetMainObject", set_main_object).unwrap();

    let c = lua.load(include_str!("../PathOfBuilding/src/Launch.lua"));
    c.exec().unwrap();
    let launch: LuaTable = lua.globals().get("launch").unwrap();
    let on_init: LuaFunction = launch.get("OnInit").unwrap();

    let this = lua.create_table().unwrap();
    on_init.call::<_, ()>(this).expect("failed to call");
        // dbg!(&lua.globals());
    // let f: LuaFunction = lua.globals().get("launch:OnInit").unwrap();
    // f.call::<_, ()>(()).unwrap();
    
    // let _: () = chunk.call("OnInit").unwrap();
    // chunk.exec().unwrap();

    
}
