use bindings::fp_export_impl;

#[fp_export_impl(bindings)]
fn custom_func(params: bindings::MyStruct) -> bindings::MyStruct {
    params
}
