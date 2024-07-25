pub(super) mod bindings {
    //! This module will house the generated bindings for the WIT files
    wasmtime::component::bindgen!({
        path: "wit/deps/base-pkg/base-pkg.wit",
        trappable_imports: true,
    });
}

// use bindings::pkg::base_pkg::types::PingRequest;

// /// Implementation of the "description" interface in base-pkg at the host level
// pub struct HostImpl;

// impl bindings::pkg::base_pkg::description::Host for HostImpl {
//     fn ping(
//         &mut self,
//         _request: PingRequest,
//     ) -> wasmtime::Result<wasmtime::component::__internal::String> {
//         todo!()
//     }
// }
