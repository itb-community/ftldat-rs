# FTLDat-rs

Rust implementation of FTLDat - a simple library for unpacking and repacking of .dat files, which are used
by the games Faster than Light and Into the Breach.

The library is intended to be loaded and interfaced with via Lua scripts.


# Building

Building assumes you have Rust set up. If not, see here: https://www.rust-lang.org/learn/get-started.

Into the Breach runs as a 32-bit application, so the library has to be compiled with 32-bit target.

1. Open a terminal in the project's root directory.
2. Add `i686-pc-windows-msvc` target with the command `rustup target add i686-pc-windows-msvc`.
3. After that, you can rebuild the library by running the command `cargo build --lib --release --target=i686-pc-windows-msvc`.

After that, the compiled .dll will be available in `./target/i686-pc-windows-msvc/release/ftldat.dll`.


# Usage

Load the library in your lua script:

```lua
package.loadlib("ftldat.dll", "luaopen_ftldat")()

result = ftldat.add(1, 2) -- will return 3
```

# Attributions

Project setup, as well as linking the compiled .dll file and loading it in Lua was based off of https://github.com/voidshine/renoise_tools. 
