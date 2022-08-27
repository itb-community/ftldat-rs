mod error;
mod entry;
mod package;
mod dat_reader;
mod dat_writer;
mod lua_exports;

pub mod prelude;

#[no_mangle]
pub extern "C" fn luaopen_ftldat(lua_state: *mut mlua::lua_State) -> i32 {
    // Leak the Lua purposefully because it's supposed to live for the duration of the program.
    // It should be owned by the game, so as a client DLL, we can assume it's truly 'static.
    let lua = unsafe { mlua::Lua::init_from_ptr(lua_state) }.into_static();

    let ftldat = lua_exports::init(&lua).expect("Failed to initialize ftldat module export table");
    lua.globals().set("ftldat", ftldat).unwrap();

    0
}
