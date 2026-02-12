Example of using WIT files to declare data via worlds/interfaces.
- Component Model: https://component-model.bytecodealliance.org/composing-and-distributing/composing.html
- WIT reference: https://component-model.bytecodealliance.org/design/wit.html

Process:
- Use wit-bindgen crate for Ryst APIs
- Use Cargo Component to generate bindings (CLI command or macro)
- Use Cargo Component to build
- Use Wasmtime to run the resulting WASM files

```
bind:
    cargo component bindings
build:
    cargo component build
run:                                                                                                                                                    
    wasmtime run --invoke 'hello-world()' ./target/wasm32-wasip1/debug/add.wasm
```
Note:
- Cargo Component uses 'wasm32-wasip1'
- cargo build --target=wasm32-wasip2
