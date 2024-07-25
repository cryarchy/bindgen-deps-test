use wasmtime::component::bindgen;

bindgen!({
    path: "wit/deps/base-pkg/",
    trappable_imports: true,
});
