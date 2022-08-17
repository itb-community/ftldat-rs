use mlua::Lua;
use mlua::prelude::{LuaResult, LuaTable};

/// Build the module's exports table, governing what is exposed to Lua.
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    //exports.set("open_read", lua.create_function(open_read)?)?;

    Ok(exports)
}

///// First parameter is the lua environment, the second is a tuple containing actual arguments
///// the function takes.
// fn open_read(_: &Lua, (path): (String)) -> LuaResult<FtlDat> {
//     Ok(FtlDat::open_existing(&path).unwrap())
// }
