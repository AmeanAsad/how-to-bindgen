mod spec;

use spec::{bindings::Runtime, types::MyStruct};
use std::{fs::read, path::Path};

fn test_module() {
    let wasm_file = Path::new("./wasm-files/wasm_plugin.wasm");
    let bytes = read(wasm_file).unwrap();
    let mut runtime = Runtime::new(&bytes).unwrap();

    let test_struct = MyStruct {
        foo: 32,
        bar: "Test String".to_string(),
    };

    let res = runtime.custom_func(test_struct).unwrap();
    println!("{:?}", res)
}

fn main() {
    test_module();
}
