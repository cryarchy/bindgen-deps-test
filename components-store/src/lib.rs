mod error;

use std::{collections::BTreeMap, sync::RwLock};

use anyhow::Result;
use wasmtime::component::Component;
use wasmtime::{Config, Engine};

pub use error::Error;

pub struct ComponentStore {
    engine: Engine,
    components: RwLock<BTreeMap<String, Component>>,
}

impl ComponentStore {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;

        Ok(Self {
            engine,
            components: RwLock::new(BTreeMap::new()),
        })
    }

    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    pub fn has_component(&self, component_name: &str) -> Result<bool, Error> {
        self.components
            .read()
            .map(|compiled_scripts| compiled_scripts.contains_key(component_name))
            .map_err(|_| Error::ModulesReadLock)
    }

    pub fn register_component(
        &self,
        component_name: String,
        wasm_bin: impl AsRef<[u8]>,
    ) -> Result<()> {
        if self.has_component(&component_name)? {
            return Ok(());
        }
        let module = Component::from_binary(&self.engine, wasm_bin.as_ref())?;
        self.components
            .write()
            .map(|mut modules| {
                modules.insert(component_name, module);
            })
            .map_err(|_| Error::ModulesWriteLock.into())
    }

    pub fn get_component(&self, module_name: &str) -> Result<Component, Error> {
        let modules_store = self.components.read().map_err(|_| Error::ModulesReadLock)?;

        modules_store
            .get(module_name)
            .cloned()
            .ok_or(Error::ModuleNotFound(module_name.to_string()))
    }
}
