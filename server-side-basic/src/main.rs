use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    // first, we need a wasmtime engine, representing a AOT compilation context
    let engine = Engine::default();

    // let us now load and compile our WebAssembly module
    let module = Module::from_file(&engine, "../client-side-bind-js/pkg/client_side_bind_js_bg.wasm")?;
    
    // stores are based on an engine and represent runtime contexts with individual states
    // this store has state (), i.e., no state at all
    let mut store = Store::new(&engine, ());
    
    // next, we define a linker for the engine
    // this is optional -- we could instantiate modules and functions directly into
    // the sore, however that makes working with namespaces tricky
    let mut linker = Linker::new(&engine);

    // the linker takes care of host-functions -- the equivalents of the JS importObject
    linker.func_wrap("env", "alert", |x: i32| println!("{x}"))?;

    // we can now use the linker to instantiate the module in our store 
    let instance = linker.instantiate(&mut store, &module)?;

    // finally, let us get and call exported function
    let alert_sum = instance.get_typed_func::<(i32, i32), ()>(&mut store, "alert_sum")?;
    alert_sum.call(&mut store, (2, 40))?;

    Ok(())
}
