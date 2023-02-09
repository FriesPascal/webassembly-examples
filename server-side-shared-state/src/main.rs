use axum::{
    extract::State,
    Router,
    routing::get,
};
use rand::Rng;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use wasmtime::*;

type AppState = Arc::<RwLock::<Store<i32>>>;

#[tokio::main]
async fn main() {
    let store = AppState::default();
    let app = Router::new()
        .route("/", get(handler))
        .with_state(Arc::clone(&store));
    
    let addr = SocketAddr::from(([0,0,0,0], 8000));
    println!("Listening on {addr:?}");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(store): State<AppState>) -> String {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen();
    
    let mut store = store.write().expect("State should never be poisoned.");
    let engine = store.engine();
    let module = Module::from_file(engine, "../client-side-bind-js/target/wasm32-unknown-unknown/release/client_side_bind_js.wasm")
        .expect("Error loading module");

    let mut linker = Linker::new(engine);
    linker.func_wrap("env", "alert", |mut caller: Caller<'_, i32>, x: i32| *caller.data_mut() = x)
        .expect("Error compiling host function.");
    
    linker.instantiate(&mut *store, &module)
        .expect("Error instantiating")
        .get_typed_func::<(i32, i32), ()>(&mut *store, "alert_sum")
        .expect("Error getting function")
        .call(&mut *store, (random_number,1))
        .expect("Error calling");

    format!("State is now: {}", store.data())
}
