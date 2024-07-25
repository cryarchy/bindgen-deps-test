# bindgen-deps-test

## Project Overview

This project demonstrates and tests the generation of Rust bindings from WebAssembly Interface Type (WIT) files that have dependencies on each other. It showcases how to structure and use interconnected WIT packages in a Rust-based WebAssembly component system.

## Project Structure

The project consists of two main WIT packages:

1. **pkg:super-pkg** (defined in `wit/super-pkg.wit`)
2. **pkg:base-pkg** (defined in `wit/deps/base-pkg/base-pkg.wit`)

These packages are designed to show how one package can depend on and use types from another.

### Key Files

-   `src/base_pkg/mod.rs`: Generates bindings for the base package
-   `src/super_pkg/mod.rs`: Generates bindings for the super package, with dependencies on the base package
-   `wit/super-pkg.wit`: Defines the super package interfaces and world
-   `wit/deps/base-pkg/types.wit`: Defines types used in the base package
-   `wit/deps/base-pkg/base-pkg.wit`: Defines the base package interfaces and world

## Project Goals

1. **Demonstrate WIT Dependencies**: Show how one WIT package (super-pkg) can depend on and use types from another (base-pkg).

2. **Bindgen Usage**: Illustrate the use of `wasmtime::component::bindgen` macro for generating Rust bindings from WIT files, including handling of dependencies.

3. **Resource Proxying**: Showcase how resources defined in one package can be used to proxy calls to another package.

4. **Interface Design**: Provide examples of well-structured WIT interfaces and worlds that can be composed and reused.

## Use Case Example

A primary use case demonstrated in this project is the implementation of a proxy pattern:

1. A type defined in the `super_pkg` Rust module would implement the `pkg::super_pkg::base_pkg::Host` trait.
2. This type would then be used to forward calls from the `base-pkg-proxy.ping` function in `pkg:super-pkg/base-pkg` to the `description.ping` function in `pkg:base-pkg`.

This pattern allows for flexible composition of WebAssembly components and interfaces.

## Getting Started

To run the test:

-   Clone the repository:
    `git clone https://github.com/cryarchy/bindgen-deps-test.git && cd bindgen-deps-test`
-   Build the project:
    `cargo build`

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file in the project root for the full license text.

The MIT License is a permissive open-source license that allows for reuse of the code with minimal restrictions. It permits use, modification, and distribution of the code, both for private and commercial purposes, as long as the original copyright and license notices are included.
