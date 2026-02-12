bind:
	cargo component bindings
build:
	cargo component build
run:
	wasmtime run --invoke 'hello-world()' ./target/wasm32-wasip1/debug/add.wasm
