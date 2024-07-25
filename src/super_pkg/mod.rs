use wasmtime::component::bindgen;

bindgen!({
    with: {
        "pkg:base-pkg/types": pkg::base_pkg::types,
    },
    trappable_imports: true,
});
