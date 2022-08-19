#!/bin/bash

export LUA_INC=./lua/include
export LUA_LIB=./lua/lua5.1
export LUA_LIB_NAME=./lua/lua5.1

cargo build --lib --release --target=i686-pc-windows-msvc
