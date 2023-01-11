use crate::types::*;

#[fp_bindgen_support::fp_export_signature]
pub fn custom_func(data: MyStruct) -> MyStruct;
