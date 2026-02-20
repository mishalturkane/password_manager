mod db;
mod crypto;
mod models;
mod repository;
mod handlers;
mod routes;

use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("🔐 Starting Password Manager V2...");

    // Auto create DB + table
    let pool = db::init_db().await;

    // Build app with routes
    let app = routes::create_routes(pool);

    // Dynamic PORT — Render inject karta hai
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Server running at http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}