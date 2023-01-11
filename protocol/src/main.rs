// use fp_bindgen::prelude::Serializable;
use fp_bindgen::{prelude::*, types::CargoDependency, BindingConfig, BindingsType};
use once_cell::sync::Lazy;

use std::collections::{BTreeMap, BTreeSet};
const VERSION: &str = "1.0.0";
const AUTHORS: &str = r#"["Amean Asad <amean.asad@protocol.ai>"]"#;
const NAME: &str = "bindings";

static PLUGIN_DEPENDENCIES: Lazy<BTreeMap<&str, CargoDependency>> = Lazy::new(|| {
    BTreeMap::from([(
        "fp-bindgen-support",
        CargoDependency {
            git: Some("https://github.com/fiberplane/fp-bindgen"),
            features: BTreeSet::from(["async", "guest"]),
            ..CargoDependency::default()
        },
    )])
});

#[derive(Serializable)]
pub struct MyStruct {
    pub foo: i32,
    pub bar: String,
}

fp_export! {
    fn custom_func(data: MyStruct) -> bool;
}

fp_import! {}

fn gen_binding(bindings_type: BindingsType) {
    let output_path = format!("../bindings/{}", bindings_type);

    fp_bindgen!(BindingConfig {
        bindings_type,
        path: &output_path
    });
}

fn main() {
    let plugin_binding = BindingsType::RustPlugin(RustPluginConfig {
        name: NAME,
        authors: AUTHORS,
        version: VERSION,
        dependencies: PLUGIN_DEPENDENCIES.clone(),
    });

    let runtime_binding = BindingsType::RustWasmerWasiRuntime;

    gen_binding(plugin_binding);
    gen_binding(runtime_binding);
}
