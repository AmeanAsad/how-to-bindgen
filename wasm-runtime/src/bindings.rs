use super::types::*;
use fp_bindgen_support::{
    common::{abi::WasmAbi, mem::FatPtr},
    host::{
        errors::{InvocationError, RuntimeError},
        mem::{
            deserialize_from_slice, export_to_guest_raw, import_from_guest_raw, serialize_to_vec,
        },
        r#async::resolve_async_value,
        runtime::RuntimeInstanceData,
    },
};
use std::sync::Arc;
use wasmer::{Function, FunctionEnv, FunctionEnvMut, Instance, Module, Singlepass, Store};

pub struct Runtime {
    store: Store,
    instance: Instance,
    env: FunctionEnv<Arc<RuntimeInstanceData>>,
}

impl Runtime {
    pub fn new(wasm_module: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let mut store = Self::default_store();
        let module = Module::new(&store, wasm_module)?;
        let env = FunctionEnv::new(&mut store, Arc::new(RuntimeInstanceData::default()));
        let mut wasi_env = wasmer_wasi::WasiState::new("fp")
            .finalize(&mut store)
            .unwrap();
        let mut import_object = wasi_env.import_object(&mut store, &module).unwrap();
        import_object.register_namespace("fp", create_imports(&mut store, &env));
        let instance = Instance::new(&mut store, &module, &import_object).unwrap();
        wasi_env.initialize(&mut store, &instance).unwrap();
        let env_from_instance = RuntimeInstanceData::from_instance(&mut store, &instance);
        Arc::get_mut(env.as_mut(&mut store))
            .unwrap()
            .copy_from(env_from_instance);
        Ok(Self {
            store,
            instance,
            env,
        })
    }

    fn default_store() -> wasmer::Store {
        Store::new(Singlepass::default())
    }

    fn function_env_mut(&mut self) -> FunctionEnvMut<Arc<RuntimeInstanceData>> {
        self.env.clone().into_mut(&mut self.store)
    }

    pub fn custom_func(&mut self, data: MyStruct) -> Result<MyStruct, InvocationError> {
        let data = serialize_to_vec(&data);
        let result = self.custom_func_raw(data);
        let result = result.map(|ref data| deserialize_from_slice(data));
        result
    }
    pub fn custom_func_raw(&mut self, data: Vec<u8>) -> Result<Vec<u8>, InvocationError> {
        let data = export_to_guest_raw(&mut self.function_env_mut(), data);
        let function = self
            .instance
            .exports
            .get_typed_function::<FatPtr, FatPtr>(&mut self.store, "__fp_gen_custom_func")
            .map_err(|_| InvocationError::FunctionNotExported("__fp_gen_custom_func".to_owned()))?;
        let result = function.call(&mut self.store, data.to_abi())?;
        let result = import_from_guest_raw(&mut self.function_env_mut(), result);
        Ok(result)
    }
}

fn create_imports(
    store: &mut Store,
    env: &FunctionEnv<Arc<RuntimeInstanceData>>,
) -> wasmer::Exports {
    let mut namespace = wasmer::Exports::new();
    namespace.insert(
        "__fp_host_resolve_async_value",
        Function::new_typed_with_env(store, env, resolve_async_value),
    );

    namespace
}
