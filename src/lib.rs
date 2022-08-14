use mlua::Lua;
use mlua::prelude::{LuaResult, LuaTable};

#[no_mangle]
pub extern "C" fn luaopen_ftldat(lua_state: *mut mlua::lua_State) -> i32 {
    // Leak the Lua purposefully because it's supposed to live for the duration of the program.
    // It should be owned by the game, so as a client DLL, we can assume it's truly 'static.
    let mut lua = unsafe { Lua::init_from_ptr(lua_state) }.into_static();

    let ftldat = init(lua).expect("Failed to initialize ftldat module export table");
    lua.globals().set("ftldat", ftldat).unwrap();

    0
}

fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("add", lua.create_function(add)?)?;

    Ok(exports)
}

/// First parameter is the lua environment, the second is a tuple containing actual arguments
/// the function takes.
fn add(_: &Lua, (a1, a2): (i32, i32)) -> LuaResult<i32> {
    Ok(a1 + a2)
}
