use mlua::Lua;
use mlua::prelude::{LuaResult, LuaTable};

mod ftldat;
mod error;
mod lua_exports;

#[no_mangle]
pub extern "C" fn luaopen_ftldat(lua_state: *mut mlua::lua_State) -> i32 {
    // Leak the Lua purposefully because it's supposed to live for the duration of the program.
    // It should be owned by the game, so as a client DLL, we can assume it's truly 'static.
    let mut lua = unsafe { Lua::init_from_ptr(lua_state) }.into_static();

    let ftldat = lua_exports::init(lua).expect("Failed to initialize ftldat module export table");
    lua.globals().set("ftldat", ftldat).unwrap();

    0
}
