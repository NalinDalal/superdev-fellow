use axum::{Router, routing::post, serve};
use handlers::{
    keypair::generate_keypair,
    message::{sign_message, verify_message},
    send::{send_sol, send_token},
    token::{create_token, mint_token},
};
use std::net::SocketAddr;

use tower_http::cors::CorsLayer;

mod handlers;
mod utils;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
        .layer(CorsLayer::permissive());

    // Listen on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    println!("Server running at http://{addr}");
    serve(listener, app).await.unwrap();
}
