#[allow(warnings)]
mod bindings;

use bindings::exports::pkg::base_pkg::description::{Guest, PingRequest};

struct Component;

impl Guest for Component {
    fn ping(request: PingRequest) -> String {
        format!("pong from base-component-1: {}", request.payload)
    }
}

bindings::export!(Component with_types_in bindings);
