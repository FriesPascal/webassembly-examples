use anyhow::Result;
use axum::{
    extract::Path,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/:name", get(greet));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Listening on {addr:?}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn greet(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}
