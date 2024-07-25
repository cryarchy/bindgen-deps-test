mod base_pkg_proxy;
mod super_pkg_wasi_view;

use anyhow::Result;
use components_store::ComponentStore;
use exports::pkg::super_pkg::ping_processor::ProcessPingResult;
pub use super_pkg_wasi_view::SuperPkgWasiView;
use wasmtime::{
    component::{bindgen, Linker},
    Engine, Store,
};

bindgen!({
    path: "../wit/",
    with: {
        // "pkg:base-pkg/types": pkg::base_pkg::types,
        "pkg:super-pkg/base-pkg/base-pkg-proxy": BasePkgProxy,
    },
    trappable_imports: true,
});

pub use base_pkg_proxy::BasePkgProxy;

pub struct SuperPkgEvaluator {
    linker: Linker<SuperPkgWasiView>,
}

impl SuperPkgEvaluator {
    pub fn new(engine: &Engine) -> Result<Self> {
        let mut linker = Linker::<SuperPkgWasiView>::new(engine);

        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        SuperPkgWorld::add_to_linker(&mut linker, |state: &mut SuperPkgWasiView| state)?;

        Ok(Self { linker })
    }

    pub fn pingify_message(
        &self,
        component_store: &ComponentStore,
        mut store: Store<SuperPkgWasiView>,
        component_name: &str,
        message: &str,
    ) -> Result<ProcessPingResult> {
        let cp_component = component_store.get_component(component_name)?;
        let cp_bindings = SuperPkgWorld::instantiate(&mut store, &cp_component, &self.linker)?;
        let result = cp_bindings
            .pkg_super_pkg_ping_processor()
            .call_process_ping(store, message)?;

        Ok(result)
    }
}
