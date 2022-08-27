use std::error::Error;
use std::sync::Arc;

use mlua::{Lua, UserDataMethods};
use mlua::prelude::{LuaError, LuaResult, LuaTable, LuaUserData};

use crate::prelude::{AddFtlDatEntry, FtlDatContentByPath, FtlDatPackage, PutFtlDatEntry};

/// Build the module's exports table, governing what is exposed to Lua.
pub fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("read_package", lua.create_function(read)?)?;
    exports.set("new_package", lua.create_function(new)?)?;

    Ok(exports)
}

fn external_lua_error<T: Error + Send + Sync + 'static>(error: T) -> LuaError {
    LuaError::ExternalError(Arc::new(error))
}

//region <Exported adapter functions>
fn new(_: &Lua, (): ()) -> LuaResult<FtlDatPackage> {
    Ok(FtlDatPackage::new())
}

fn read(_: &Lua, (path, ): (String, )) -> LuaResult<FtlDatPackage> {
    FtlDatPackage::from_file(&path)
        .map_err(external_lua_error)
}
//endregion

impl LuaUserData for FtlDatPackage {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("to_file", |_, this, (path, ): (String, )| {
            this.to_file(&path)
                .map_err(external_lua_error)
        });

        methods.add_method_mut("add_entry_from_string", |_, this, (path, content): (String, String)| {
            this.add_entry(path, content)
                .map_err(external_lua_error)
        });

        methods.add_method_mut("add_entry_from_byte_array", |_, this, (path, content): (String, Vec<u8>)| {
            this.add_entry(path, content)
                .map_err(external_lua_error)
        });

        methods.add_method_mut("put_entry_from_string", |_, this, (path, content): (String, String)| {
            Ok(this.put_entry(path, content))
        });

        methods.add_method_mut("put_entry_from_byte_array", |_, this, (path, content): (String, Vec<u8>)| {
            Ok(this.put_entry(path, content))
        });

        methods.add_method("read_content_as_string", |_, this, (path, ): (String, )| {
            let result: Option<String> = this.content_by_path(&path);
            Ok(result)
        });

        methods.add_method("read_content_as_byte_array", |_, this, (path, ): (String, )| {
            let result: Option<Vec<u8>> = this.content_by_path(&path);
            Ok(result)
        });

        methods.add_method_mut("remove", |_, this, (path, ): (String, )| {
            Ok(this.remove_entry(&path))
        });

        methods.add_method("exists", |_, this, (path, ): (String, )| {
            Ok(this.entry_exists(&path))
        });

        methods.add_method_mut("clear", |_, this, ()| {
            Ok(this.clear())
        });

        methods.add_method("inner_paths", |_lua, this, ()| {
            Ok(this.inner_paths())
        });

        methods.add_method("len", |_, this, ()| {
            Ok(this.len())
        });

        methods.add_method("entry_count", |_, this, ()| {
            Ok(this.entry_count())
        })
    }
}
