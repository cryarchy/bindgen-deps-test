# bindgen-deps-test

## Project Overview

This project demonstrates the use of WebAssembly Interface Types (WIT) in a Rust workspace to create a system of interconnected WebAssembly components. It showcases how to structure, build, and use WIT packages in a Rust-based WebAssembly component system, with a focus on component interaction and resource proxying.

## Project Structure

The project is a Rust workspace consisting of the following packages:

1. **base-component-1**: Implements the base package interface.
2. **base-component-2**: Another implementation of the base package interface.
3. **super-component**: Implements the super package interface, which depends on the base package.
4. **runner**: The main application that orchestrates the components.
5. **components-store**: A utility for managing WebAssembly components.

### Key Files and Directories

-   `wit/`: Contains WIT files defining interfaces and worlds.
    -   `super-pkg.wit`: Defines the super package interfaces and world.
    -   `deps/base-pkg/`: Contains WIT files for the base package.
-   `src/`: Source code for each package.
-   `Cargo.toml`: Workspace definition and dependencies.

## Project Goals and Functionality

1. **Component Interaction**: Demonstrate how WebAssembly components can interact with each other through well-defined interfaces.

2. **Resource Proxying**: Show how resources defined in one package can be used to proxy calls to another package.

3. **Host Implementation**: Illustrate how the host (Rust in this case) can implement and manage WebAssembly components.

4. **Dynamic Component Loading**: Showcase dynamic loading and execution of WebAssembly components.

## Sequence of Operations

1. The host (runner) loads and registers WebAssembly components (base-component-1, base-component-2, super-component).
2. The host creates proxies and views for the components.
3. The host calls the `pingify_message` function on the super-component.
4. The super-component processes the message, calling the `ping` function on both base components through the proxy.
5. The results from both base components are combined and returned to the host.

## Use Case Example

This project demonstrates a system where:

1. Base components provide a simple `ping` functionality.
2. A super component can process messages by utilizing the `ping` functionality of base components.
3. The host application orchestrates these components, showing how WebAssembly components can be composed and used together.

## Getting Started

To run the project:

1. Clone the repository:
    - `git clone https://github.com/yourusername/bindgen-deps-test.git`
    - `cd bindgen-deps-test`
2. Build the components:
    - `cargo component build --release --package base-component-1`
    - `cargo component build --release --package base-component-2`
    - `cargo component build --release --package super-component`
3. Run the application:
    - `cargo run --bin runner`

## Expected Output

```
Evaluation result: processed message:
        pong from base-component-1: Hello, world!
        pong from base-component-2: Hello, world!
```

This output demonstrates that the super-component successfully processed the input message by calling both base components and combining their responses.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
