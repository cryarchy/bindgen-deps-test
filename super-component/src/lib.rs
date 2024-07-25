#[allow(warnings)]
mod bindings;

use bindings::exports::pkg::super_pkg::ping_processor::{Guest, ProcessPingResult};
use bindings::pkg::super_pkg::base_pkg::{BasePkgProxy, PingRequest};

struct Component;

impl Guest for Component {
    fn process_ping(input: String) -> ProcessPingResult {
        let base_pkg_proxy = BasePkgProxy::new();

        // call the ping function in base-component-1
        let request = PingRequest {
            component_name: "base-component-1".to_string(),
            payload: input.to_owned(),
        };
        let ping_response_1 = base_pkg_proxy.ping(&request);

        // call the ping function in base-component-2
        let request = PingRequest {
            component_name: "base-component-2".to_string(),
            payload: input,
        };
        let ping_response_2 = base_pkg_proxy.ping(&request);

        ProcessPingResult {
            processed_ping: format!(
                "processed message:\n\t{}\n\t{}",
                ping_response_1, ping_response_2
            ),
        }
    }
}

bindings::export!(Component with_types_in bindings);
