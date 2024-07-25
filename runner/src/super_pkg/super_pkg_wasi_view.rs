use anyhow::Result;
use wasmtime::{
    component::{Resource, ResourceTable},
    Engine, Store,
};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::{
    base_pkg::BasePkgWasiView,
    super_pkg::pkg::super_pkg::{
        base_pkg::{self, BasePkgProxy, HostBasePkgProxy},
        types,
    },
};

use super::pkg::super_pkg::base_pkg::PingRequest;

pub struct SuperPkgWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
    base_pkg_proxy_id: Resource<BasePkgProxy>,
    base_pkg_store_id: Resource<Store<BasePkgWasiView>>,
}

impl SuperPkgWasiView {
    pub fn new(base_pkg_proxy: BasePkgProxy, store: BasePkgWasiView, engine: &Engine) -> Self {
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let mut table = ResourceTable::new();

        let base_pkg_proxy_id = table.push(base_pkg_proxy).unwrap();

        let store = Store::new(engine, store);
        let base_pkg_store_id = table.push(store).unwrap();

        Self {
            table,
            ctx,
            base_pkg_proxy_id,
            base_pkg_store_id,
        }
    }
}

impl base_pkg::Host for SuperPkgWasiView {}
impl types::Host for SuperPkgWasiView {}

impl HostBasePkgProxy for SuperPkgWasiView {
    fn new(&mut self) -> Result<Resource<BasePkgProxy>> {
        let nmg_proxy = Resource::new_own(self.base_pkg_proxy_id.rep());
        Ok(nmg_proxy)
    }

    fn ping(
        &mut self,
        base_pkg_proxy: Resource<BasePkgProxy>,
        request: PingRequest,
    ) -> Result<String> {
        let base_pkg_proxy = self.table.get(&base_pkg_proxy)?.clone();
        let base_pkg_store = self.table.get_mut(&self.base_pkg_store_id)?;
        Ok(base_pkg_proxy.ping(request, base_pkg_store))
    }

    fn drop(&mut self, base_pkg_proxy: Resource<BasePkgProxy>) -> Result<()> {
        let _base_pkg_proxy: BasePkgProxy = self.table.delete(base_pkg_proxy)?;
        Ok(())
    }
}

impl WasiView for SuperPkgWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
