## How to build
Compile:
```
cargo build --target wasm32-unknown-unknown 
```

Convert module to component:
```
wasm-tools component new target/wasm32-unknown-unknown/release/greeter.wasm --adapt target/wasi_snapshot_preview1.wasm -o target/greeter-component.wasm
```
