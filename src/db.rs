use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub async fn init_db() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    create_database_if_not_exists(&database_url).await;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    run_migrations(&pool).await;

    pool
}

async fn create_database_if_not_exists(database_url: &str) {
    let db_name = database_url.split('/').last()
        .expect("Invalid DATABASE_URL");

    let base_url = database_url.rsplitn(2, '/').last()
        .expect("Invalid DATABASE_URL");

    let admin_url = format!("{}/postgres", base_url);

    let admin_pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_url)
        .await
        .expect("Failed to connect to PostgreSQL. Is it running?");

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

async fn run_migrations(pool: &PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS passwords (
            id             VARCHAR(100) PRIMARY KEY,
            encrypted_pass TEXT NOT NULL,
            created_at     TIMESTAMP DEFAULT NOW()
        )"
    )
    .execute(pool)
    .await
    .expect("Failed to create passwords table");

    println!("✅ Table ready!");
}