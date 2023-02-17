use anyhow::{Context, Result};
use axum::{extract::Path, routing::get, Router};
use std::net::SocketAddr;
use wasmtime::{component::*, Config, Engine, Store};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/:name", get(greet)) 
        .route("/bindgen/:name", get(greet_bindgen)); 
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn greet(Path(name): Path<String>) -> String {
    greet_inner(&name)
        .unwrap_or_else(|e| {
            eprintln!("Error in calling greet: {e}");
            "Error, see logs for more info!".to_owned()
        })
}

fn greet_inner(name: &str) -> Result<String> {
    // Create new component-model enabled engine.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    
    // Create a component-model enabled linker.
    let mut linker = Linker::new(&engine);
    linker
        // Create the "host" instance in this linker.
        .instance("host")?
        // Compile the "log" function into this linker.
        // The closure generally has signature:
        // |Context, (param_1, ..., param_N)| -> Result<(return_1, ..., return_N)>
        .func_wrap("log", |_, (msg,): (String,)| -> Result<()> {
            println!("Log: {msg}");
            Ok(())
        })?;

    // Create a component-model enabled store.
    let mut store = Store::new(&engine, ());

    // Compile "example" component and instatiate it into the store.
    let example = Component::from_file(&engine, "./wasm/target/greeter-component.wasm")?;
    let example = linker.instantiate(&mut store, &example)?;
    
    // Example is now a component instance so let us explore :)
    let greet = example
        // We can get its exports,
        .exports(&mut store)
        // look for an instance of the "greeter" interface,
        .instance("greeter")
        .context("Couldn't find greeter component :/")?
        // check for the "greet" function.
        .typed_func::<(String,),(String,)>("greet")?;

    // finally, we call the "greet" function
    let (result,) = greet.call(&mut store, (name.to_owned(),))?;
    
    Ok(result)
}

async fn greet_bindgen(Path(name): Path<String>) -> String {
    greet_bindgen_inner(&name)
        .unwrap_or_else(|e| {
            eprintln!("Error in calling greet_bindgen: {e}");
            "Error, see logs for more info!".to_owned()
        })
}

fn greet_bindgen_inner(name: &str) -> Result<String> {
    // Generate bindings for ./wit/example.wit
    bindgen!({path: "./wit", world: "example"});

    // Define a struct to implement the "host" interface for.
    // If this struct has fields, we can even use it to store state :D
    struct ExampleImports {}

    // The host::Host trait was defined by the bindgen! macro and corresponds to
    // the "host" interface defined in our wit file.
    impl host::Host for ExampleImports {
        fn log(&mut self, msg: String) -> Result<()> {
            println!("Log (bindgen): {msg}");
            Ok(())
        }
    }

    // Create Config, Engine, and Linker.
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let mut linker = Linker::new(&engine);

    // Register our "host" implementation with the Linker.
    host::add_to_linker(&mut linker, |i: &mut ExampleImports| i)?;

    // Create a new store that uses our empty "host" implementation as a state.
    let mut store = Store::new(&engine, ExampleImports {});

    // Compile Component from file and instantiate it into the store using the linker.
    // Note the appearance of the Example struct, corresponding to the "example" world
    // definition in the wit file.
    let example = Component::from_file(&engine, "./wasm/target/greeter-component.wasm")?;
    let (example, _) = Example::instantiate(&mut store, &example, &linker)?;

    // Invoke the "greet" function in the "greeter" interface.
    let result = example.greeter().call_greet(&mut store, name)?;

    Ok(result)
}
