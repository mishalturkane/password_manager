mod db;
mod models;
mod repository;
mod cli;

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    println!("🔐 Starting Password Manager...");

    // Auto create DB + table if not exists → then connect
    let pool = db::init_db().await;

    // Run CLI
    cli::run(&pool).await;
}