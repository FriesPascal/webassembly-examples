# Minimal client side app with only basic data types
This creates a WebAssembly module small enough for inspection with `wasm2wat`.
Visit the browser console to see what the app is doing.
JS glue code is included manually.

## How to build
To build:
```
cargo build --release --target wasm32-unknown-unknown
```
To serve:
```
live-server -h 0.0.0.0
```
