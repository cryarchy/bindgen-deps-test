use std::{fs, sync::Arc};

use components_store::ComponentStore;
use runner::{
    base_pkg::{BasePkgWasiView, WasmBasePkg},
    super_pkg::{BasePkgProxy, SuperPkgEvaluator, SuperPkgWasiView},
};
use wasmtime::Store;

fn main() {
    // Create a component store
    let component_store = Arc::new(ComponentStore::new().unwrap());

    // Register a component with the name "base-component-1"
    let component_name = "base-component-1";
    let wasm_bin = fs::read("target/wasm32-wasip1/release/base_component_1.wasm").unwrap();
    component_store
        .register_component(component_name.to_owned(), wasm_bin)
        .unwrap();

    // Register a component with the name "base-component-2"
    let component_name = "base-component-2";
    let wasm_bin = fs::read("target/wasm32-wasip1/release/base_component_2.wasm").unwrap();
    component_store
        .register_component(component_name.to_owned(), wasm_bin)
        .unwrap();

    // Register a component with the name "super-pkg"
    let component_name = "super-pkg";
    let wasm_bin = fs::read("target/wasm32-wasip1/release/super_component.wasm").unwrap();
    component_store
        .register_component(component_name.to_owned(), wasm_bin)
        .unwrap();

    // Create an evaluator for the super package
    let evaluator = SuperPkgEvaluator::new(component_store.engine()).unwrap();

    // Create a proxy for the base package (required in WIT)
    let wasm_node_info = WasmBasePkg::new().unwrap();
    let node_info_proxy = BasePkgProxy::new(Arc::new(wasm_node_info), component_store.clone());
    let node_info_wasi_view = BasePkgWasiView::new().unwrap();

    // Create a WASI view for the super package
    // Add the base package proxy to it's resource table
    let evaluator_wasi_view = SuperPkgWasiView::new(
        node_info_proxy,
        node_info_wasi_view,
        component_store.engine(),
    );

    // Create a store for the super package
    let store = Store::new(component_store.engine(), evaluator_wasi_view);

    // Create a message to be processed by the super package
    let message = "Hello, world!";

    // Process the message
    let evaluation_result = evaluator
        .pingify_message(&component_store, store, component_name, message)
        .unwrap();

    // Print the result
    println!("Evaluation result: {}", evaluation_result.processed_ping);
}
