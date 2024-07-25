use anyhow::Result;
use wasmtime::component::ResourceTable;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

pub struct BasePkgWasiView {
    table: ResourceTable,
    ctx: WasiCtx,
}

impl BasePkgWasiView {
    pub fn new() -> Result<Self> {
        let ctx = WasiCtxBuilder::new().inherit_stdio().build();
        let table = ResourceTable::new();
        Ok(Self { table, ctx })
    }
}

impl WasiView for BasePkgWasiView {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }

    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
