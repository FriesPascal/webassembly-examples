# Minimal client side app
This demonstrates importing JS functions (here, `alert`) to Rust via the `#![wasm_bindgen]` macro and an `extern` block.
It also shows how to export a rust function (`greet`) to JS, again via `#![wasm_bindgen]`.
The JS-glue code is created by `wasm-bindgen` as part of running `wasm-pack` and will end up in `pkg/minimal_client_side.js`.
The resulting WebAssembly is in `pkg/minimal_client_side_bg.wasm`.

To see how to include the resulting files, have a look at [index.html](index.html). 
For distribution purposes, you will only need the three files mentioned above.

## How to build
To build:
```
wasm-pack build --taget web
```
To serve:
```
live-server -h 0.0.0.0
```
