use std::sync::Arc;

use components_store::ComponentStore;
use wasmtime::Store;

use crate::base_pkg::{
    exports::pkg::base_pkg::description::PingRequest, BasePkgWasiView, WasmBasePkg,
};

use super::pkg::super_pkg::base_pkg::PingRequest as SuperPingRequest;

#[derive(Clone)]
pub struct BasePkgProxy {
    base_pkg: Arc<WasmBasePkg>,
    component_store: Arc<ComponentStore>,
}

impl From<SuperPingRequest> for PingRequest {
    fn from(request: SuperPingRequest) -> Self {
        Self {
            component_name: request.component_name,
            payload: request.payload,
        }
    }
}

impl BasePkgProxy {
    pub fn new(base_pkg: Arc<WasmBasePkg>, component_store: Arc<ComponentStore>) -> Self {
        Self {
            base_pkg,
            component_store,
        }
    }

    pub fn ping(&self, request: SuperPingRequest, store: &mut Store<BasePkgWasiView>) -> String {
        self.base_pkg
            .ping(request.into(), &self.component_store, store)
            .unwrap()
    }
}
