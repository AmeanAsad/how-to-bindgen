# how-to-bindgen

This repo of how to create wasm bindings for complex data structures in Rust using [fp-bindgen](https://github.com/fiberplane/fp-bindgen)

## Why is this useful

WASM functions only accept primitive [data types](https://webassembly.github.io/spec/core/syntax/types.html) as inputs or output params.
The data types are basically floats, ints, and vectors. 
Out of the box, runtimes like Wasmer and Wasmtime only support these datatypes when running WASM bundles. If you want to interact with WASM
functions and use more complex data structures like a HashMaps, arrays, etc. , you need what is called "glue" code to have a mechanism 
to communicate those datatypes in way WASM can understand. The most common library that does that is [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)
but it is only useful if you are exchanding those values between Javascript and Rust. 

This is where [fp-bindgen](https://github.com/fiberplane/fp-bindgen) becomes helpful, it is a tool you can use to be able to interact
with WASM functions with your desired complex data structures. 

This repo provides a simple example of how to write a protocol that defines the functions imported or exported in your wasm bundle and 
the input/ouput params. It then shows how to use that protocol to generate bindings (the "glue" code) that act as a bridge between your
application and the wasm bundle. 


