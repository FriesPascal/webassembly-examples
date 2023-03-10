use anyhow::Result;
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use wasmtime::{component::*, Config, Engine, Store};

// Declare and implement the host functionality globally this time
bindgen!({path: "../component-model-basic/wit/example.wit"});
struct ExampleImports {}

impl host::Host for ExampleImports {
    fn log(&mut self, _msg: String) -> Result<()> {
        // Logging would be a bottleneck, so we disable it. To make logging efficient enough,
        // we would need to implement concurrent log processing. This can be done by having
        // `ExampleImports` contain the "producer" end of an mpsc channel, which
        // in turn sends all logs to the "consumer" end in a different thread, where
        // they are processed (in order). 
        Ok(())
    }
}

// Struct to hold compiler options ("Engine"), together with precompiled component.
struct Cache {
    engine: Engine,
    component: Vec<u8>,
}

impl Cache {
    fn init() -> Result<Self> {
        // Create new Engine
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;

        // Compile and serialise Component
        let component = Component::from_file(
            &engine,
            "../component-model-basic/wasm/target/greeter-component.wasm",
        )?
        .serialize()?;

        // return them
        Ok(Self { engine, component })
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/:name", get(greet))
        .with_state(Arc::new(Cache::init()?));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn greet(Path(name): Path<String>, State(cache): State<Arc<Cache>>) -> String {
    greet_inner(&name, &cache).unwrap_or_else(|e| {
        eprintln!("Error in calling greet: {e}");
        "Error, see logs for more info!".to_owned()
    })
}

fn greet_inner(name: &str, cache: &Cache) -> Result<String> {
    // Let us still use new Linker and Store per call
    let mut linker = Linker::new(&cache.engine);
    host::add_to_linker(&mut linker, |i: &mut ExampleImports| i)?;
    let mut store = Store::new(&cache.engine, ExampleImports {});

    // Load precompiled component and instantiate it
    let example = unsafe { Component::deserialize(&cache.engine, &cache.component) }?;
    let (example, _) = Example::instantiate(&mut store, &example, &linker)?;

    // Call the `greet` function of the compontent
    let result = example.greeter().call_greet(&mut store, name)?;

    Ok(result)
}
