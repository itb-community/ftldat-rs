# FTLDat-rs

Rust implementation of FTLDat - a simple library for unpacking and repacking of .dat files, which are used
by the games [Faster than Light](https://subsetgames.com/ftl.html) and [Into the Breach](https://subsetgames.com/itb.html).

This library is intended to be loaded and interfaced with via Lua scripts.


# Building

This section assumes you have Rust set up with MSVC. If not, see here: https://www.rust-lang.org/learn/get-started.

Building for release mode with MINGW should also be possible, and potentially a bit simpler, but I didn't want to try
getting MINGW set up yet.

### Development

For development, the build process is very simple: 

1. Open a terminal in the project's root directory.
2. Run `cargo build`.

### Release

For release (as in, getting a .dll that Lua can interface with), the build process is quite a bit more involved.

1. Change configuration to build the library in module mode.
   1. Go to `Cargo.toml`.
   2. Find the `[dependencies]` section.
   3. Find the entry for `mlua` and replace `"vendored"` with `"module"`.
      - Or just comment/uncomment the prepared entries.
   - For explanation why this is needed, see the [Troubleshooting](#troubleshooting) section below.
2. Open a terminal in the project's root directory.
3. (First time only) Add `i686-pc-windows-msvc` target with the command `rustup target add i686-pc-windows-msvc`.
4. Specify environment variables:
   - `LUA_INC=lua/include` - path to Lua headers
   - `LUA_LIB=lua/lua5.1` - path to Lua .lib file
   - `LUA_LIB_NAME=lua/lua5.1` - same path as in `LUA_LIB`
5. Run `cargo build --lib --release --target=i686-pc-windows-msvc`

Steps 4 and 5 are automated in the form of `build.sh` script.

Compiled .dll will be available in `./target/i686-pc-windows-msvc/release/ftldat.dll`.

# Usage

Load the library in your Lua script:

```lua
-- load the dll - this exposes `ftldat` global variable,
-- with functions `new_package` and `read_package`
package.loadlib("ftldat.dll", "luaopen_ftldat")()

-- read a .dat file into memory
pack = ftldat.read_package("path/to/resource.dat")

-- add an entry - this can fail if the entry already exists
pack:add_text_entry("img/some/file.txt", "the file's content")

-- alternatively, put an entry, which can overwrite an existing entry
pack:put_text_entry("img/some/file.txt", "another content")

-- can also read an entry's content:
content = pack:content_text_by_path("img/some/file.txt")

-- write out the package from memory to a file
pack:to_file("path/to/resource.dat")

-- binary content can be read/written, too. This is done in the form of byte arrays,
-- which in Lua take the shape of tables of numbers.
pack:put_binary_entry("img/image.png", { 1, 2, 3, 4, 5 })
```

# Troubleshooting

The build process for getting a .dll that can interface with Lua is a little finicky.

Into the Breach runs as a 32-bit application, so the library has to be compiled with 32-bit target.

Also, the library has to be built in `mlua`'s [module mode](https://github.com/khvzak/mlua#module-mode), otherwise the
game crashes during exit. The crash doesn't *actually* cause any issues, as far as I could tell, but it does leave sort
of a sour aftertaste after getting everything else to work. It also has the advantage of producing a smaller binary.

Building in module mode under Windows requires linking to a Lua dll (as mentioned in the link). 
This is what the `lua` directory and `build.sh` script are for - if you don't want to run the script file, you'll need
to set the variables from the script in your desired environment.

# Areas to Improve

Considering that this project served me as a way to familiarize myself with Rust, there's bound to be a lot of room for
improvement. In no particular order:
- Error handling. Tried to use `thiserror`, a popular crate for error-handling, but I don't feel particularly confident about it.
- Ownership of strings, I just used heap-allocated Strings and copied them left and right
- Naming of functions, following proper Rust conventions (`from`, `into`, etc.)
- Serialization of structs to bytes can probably be handled better (though I like keeping in-memory and on-disk representations separate)

# Attributions

Project setup, as well as linking the compiled .dll file and loading it in Lua was based off of https://github.com/voidshine/renoise_tools. 
