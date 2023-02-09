use axum::{
    routing::get,
    http::StatusCode,
    Router,
};
use rand::Rng;
use std::net::SocketAddr;
use wasmtime::*;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let addr = SocketAddr::from(([0,0,0,0], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Result<String, StatusCode> {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen();
    
    match add(random_number, 1) {
        Ok(sum) => {
            let response = format!("Random number was: {}, Add 1 makes: {}", random_number, sum);
            Ok(response)
        },
        Err(e) => {
            eprintln!("Error during function call: {e}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn add(a: i32, b: i32) -> anyhow::Result<i32> {
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);
    let module = Module::from_file(&engine, "../client-side-basic/target/wasm32-unknown-unknown/release/client_side_basic.wasm")?;
    
    linker.instantiate(&mut store, &module)?
        .get_typed_func::<(i32, i32), i32>(&mut store, "add")?
        .call(&mut store, (a, b))
}
