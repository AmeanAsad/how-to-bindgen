#![allow(unused_imports)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MyStruct {
    pub foo: i32,
    pub bar: String,
}
