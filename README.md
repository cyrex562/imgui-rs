# imgui-rs

## Intro

This is a port of the awesome C/C++ Dear Imgui [https://github.com/cyrex562/imgui-rs] library to the Rust programming language. All code is Rust-native, save bindings to the graphics environments themselves.

### Search Patterns

`^\s*(\*mut|\*const)?\s*([\w+\*\<\>]+)\s*(\w+);` => `pub $3: $1 $2,`
