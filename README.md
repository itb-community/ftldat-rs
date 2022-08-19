# FTLDat-rs

Rust implementation of FTLDat - a simple library for unpacking and repacking of .dat files, which are used
by the games [Faster than Light](https://subsetgames.com/ftl.html) and [Into the Breach](https://subsetgames.com/itb.html).

This library is intended to be loaded and interfaced with via Lua scripts.


# Building

Building assumes you have Rust set up. If not, see here: https://www.rust-lang.org/learn/get-started.

1. Open a terminal in the project's root directory.
2. Add `i686-pc-windows-msvc` target with the command `rustup target add i686-pc-windows-msvc`.
3. After that, you can build the library anytime by running the `build.sh` script.
    - the script just has to set a couple vars and call cargo, so you can easily modify it to suit your preferences.

After that, the compiled .dll will be available in `./target/i686-pc-windows-msvc/release/ftldat.dll`.

# Usage

Load the library in your lua script:

```lua
package.loadlib("ftldat.dll", "luaopen_ftldat")()

pack = ftldat.read_package("path/to/resource.dat")
pack.add_text_entry("img/some/file.txt", "the file's content")
pack.to_file("path/to/resource.dat")
```

# Troubleshooting

The build process is a little finicky.

Into the Breach runs as a 32-bit application, so the library has to be compiled with 32-bit target.

Also, the library has to be built in `mlua`'s [module mode](https://github.com/khvzak/mlua#module-mode), otherwise the
game crashes during exit. The crash doesn't *actually* cause any issues, as far as I could tell, but it does leave sort
of a sour aftertaste after getting everything else to work. Building in module mode under Windows requires linking to
a Lua dll (as mentioned in the link). This is what the `lua` directory and `build.sh` script are for - if you don't want
to run the script file, you'll need to set the variables from the script in your desired environment.

# Areas to Improve

Considering that this project served me as a way to familiarize myself with Rust, there's bound to be a lot of room for
improvement. In no particular order:
- Error handling. Tried to use `thiserror`, a popular crate for error-handling, but I don't feel particularly confident about it.
- Ownership of strings, I just used heap-allocated Strings and copied them left and right
- Naming of functions, following proper Rust conventions (`from`, `into`, etc.)
- Serialization of structs to bytes can probably be handled better (though I like keeping in-memory and on-disk representations separate)

# Attributions

Project setup, as well as linking the compiled .dll file and loading it in Lua was based off of https://github.com/voidshine/renoise_tools. 
