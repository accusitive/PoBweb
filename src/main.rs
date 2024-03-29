
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use mlua::prelude::*;

// In order to work with the memory we expose (de)allocation methods
#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

fn main() {
    // let lua = Lua::new();
    // println!("Hello ems");
    (|| -> Option<()> {
        let lua = Lua::new();
        // let map_table = lua.create_table().ok()?;
        // map_table.set(1, "one").ok()?;
        // map_table.set("two", 2).ok()?;
    
        // lua.globals().set("map_table", map_table).ok()?;
    
        // lua.load("for k,v in pairs(map_table) do print(k,v) end").exec().ok()?;
        Some(())
    })().unwrap();
    println!("Hello emscripten");

}