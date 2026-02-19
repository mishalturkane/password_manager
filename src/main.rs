mod db;
mod crypto;
mod models;
mod repository;
mod handlers;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("🔐 Starting Password Manager V2...");

    // Auto create DB + table
    let pool = db::init_db().await;

    // Build app with routes
    let app = routes::create_routes(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Server running at http://localhost:8080");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}