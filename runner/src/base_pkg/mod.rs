mod base_pkg_wasi_view;

use anyhow::Result;
pub use base_pkg_wasi_view::BasePkgWasiView;
use components_store::ComponentStore;
use exports::pkg::base_pkg::description::PingRequest;
use wasmtime::{
    component::{bindgen, Linker},
    Config, Engine, Store,
};

bindgen!({
    path: "../wit/deps/base-pkg/",
    trappable_imports: true,
});

#[derive(Clone)]
pub struct WasmBasePkg {
    linker: Linker<BasePkgWasiView>,
}

impl WasmBasePkg {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;

        let mut linker = Linker::<BasePkgWasiView>::new(&engine);

        wasmtime_wasi::add_to_linker_sync(&mut linker)?;

        Ok(Self { linker })
    }

    pub fn ping(
        &self,
        request: PingRequest,
        component_store: &ComponentStore,
        store: &mut Store<BasePkgWasiView>,
    ) -> Result<String> {
        let base_pkg_component = component_store.get_component(&request.component_name)?;
        let base_pkg_bindings =
            BasePkgWorld::instantiate(&mut *store, &base_pkg_component, &self.linker)?;
        let result = base_pkg_bindings
            .pkg_base_pkg_description()
            .call_ping(store, &request)?;

        Ok(result)
    }
}
