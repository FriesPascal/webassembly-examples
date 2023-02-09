# WebAssembly examples

## Prerequisites
You will need `rustup` for managing the Rust toolchain.
Install the stable WebAssembly toolchain via:
```
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
rustup target add wasm32-wasi
rustup show
```

You will see something like this:
```
Default host: x86_64-unknown-linux-gnu
rustup home:  $HOME/.rustup

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
wasm32-wasi
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.67.0 (fc594f156 2023-01-24)
```

Finally, install `wasm-pack` (for generating WebAssembly-JS bindings), `live-server` (a generic live-reloading server), and `trunk` (development server for WebAssembly based apps wirtten in Rust) with Cargo:
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install live-server
cargo install trunk
```

## Examples
In didactical order:
1. [Minimal bindings with only WASM native types](client-side-basic/)
2. [Calling JS functions fro WebAssemby](client-side-bind-js/)
3. [Using `wasm-pack` / `wasm-bindgen` for generating the JS glue code](client-side-wasm-pack/)
4. [Complex types, using shared memory](client-side-complex-types/)
