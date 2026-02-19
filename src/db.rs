use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

/// Connects to PostgreSQL
/// Auto creates database and table if not present
pub async fn init_db() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Step 1 — Auto create database if not exists
    create_database_if_not_exists(&database_url).await;

    // Step 2 — Connect to our database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Step 3 — Auto create table if not exists
    run_migrations(&pool).await;

    pool
}

/// Connects to default 'postgres' db first
/// Then creates our database if it doesn't exist
async fn create_database_if_not_exists(database_url: &str) {
    // Extract db name from URL
    // URL format: postgres://user:pass@host:port/dbname
    let db_name = database_url
        .split('/')
        .last()
        .expect("Invalid DATABASE_URL format");

    // Connect to default postgres db
    let base_url = database_url
        .rsplitn(2, '/')
        .last()
        .expect("Invalid DATABASE_URL format");

    let admin_url = format!("{}/postgres", base_url);

    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_url)
        .await
        .expect("Failed to connect to PostgreSQL server. Is PostgreSQL running?");

    // Check if database exists
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM pg_database WHERE datname = $1)"
    )
    .bind(db_name)
    .fetch_one(&admin_pool)
    .await
    .unwrap_or(false);

    if !exists {
        sqlx::query(&format!("CREATE DATABASE {}", db_name))
            .execute(&admin_pool)
            .await
            .expect("Failed to create database");

        println!("✅ Database '{}' created!", db_name);
    } else {
        println!("✅ Database '{}' found!", db_name);
    }
}

/// Creates tables if they don't exist
async fn run_migrations(pool: &PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS passwords (
            id       VARCHAR(100) PRIMARY KEY,
            password TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await
    .expect("Failed to create passwords table");

    println!("✅ Table ready!");
}