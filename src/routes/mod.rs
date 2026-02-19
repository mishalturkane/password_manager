use axum::{
    Router,
    routing::get,
};
use sqlx::PgPool;
use tower_http::{cors::{CorsLayer, Any}, services::ServeDir};

use crate::handlers::password_handler;

pub fn create_routes(pool: PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes
    let api = Router::new()
        .route("/passwords",        get(password_handler::get_all)
                                   .post(password_handler::add))
        .route("/passwords/search", get(password_handler::search))
        .route("/passwords/:id",    get(password_handler::get_one)
                                   .put(password_handler::update)
                                   .delete(password_handler::delete));

    Router::new()
        // API
        .nest("/api", api)
        // Serve static HTML frontend
        .nest_service("/", ServeDir::new("static"))
        .layer(cors)
        .with_state(pool)
}