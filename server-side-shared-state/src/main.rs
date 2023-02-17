use axum::{extract::State, routing::get, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use wasmtime::*;

type AppState = Arc<Mutex<i32>>;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .with_state(AppState::default());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(state): State<AppState>) -> String {
    // create new engine and compile module bytecode
    let engine = Engine::default();
    let module = Module::from_file(
        &engine,
        "./wasm/target/wasm32-unknown-unknown/release/handle_request.wasm",
    )
    .expect("Error loading module");

    // create a store, passing in a cloned Arc<Mutex> of the state
    let mut store = Store::new(&engine, Arc::clone(&state));

    // define a state writer function
    // this function increases state by 1 and returns the result
    let mut linker = Linker::new(&engine);
    linker
        .func_wrap("counter", "inc", |caller: Caller<'_, AppState>| {
            let mut data = caller.data().lock().expect("Error acquiring state lock.");
            *data += 1;
            *data
        })
        .expect("Error compiling host function.");

    // do stuff
    linker
        .instantiate(&mut store, &module)
        .expect("Error instantiating")
        .get_typed_func::<i32, i32>(&mut store, "handle_request")
        .expect("Error getting function")
        .call(&mut store, 1)
        .expect("Error calling")
        .to_string()
}
