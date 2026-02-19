use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    crypto,
    models::password::{PasswordRequest, PasswordResponse},
    repository::password_repo,
};

/// GET /api/passwords — get all
pub async fn get_all(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<PasswordResponse>>, StatusCode> {
    let records = password_repo::get_all(&pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = records.into_iter().map(|r| PasswordResponse {
        id: r.id,
        password: crypto::decrypt(&r.encrypted_pass),
    }).collect();

    Ok(Json(response))
}

/// GET /api/passwords/:id — get one by id
pub async fn get_one(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<Json<PasswordResponse>, StatusCode> {
    let record = password_repo::get_by_id(&pool, &id).await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(PasswordResponse {
        id: record.id,
        password: crypto::decrypt(&record.encrypted_pass),
    }))
}

/// GET /api/passwords/search?q=gmail — search
#[derive(Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

pub async fn search(
    State(pool): State<PgPool>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<PasswordResponse>>, StatusCode> {
    let records = password_repo::search(&pool, &params.q).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = records.into_iter().map(|r| PasswordResponse {
        id: r.id,
        password: crypto::decrypt(&r.encrypted_pass),
    }).collect();

    Ok(Json(response))
}

/// POST /api/passwords — add new
pub async fn add(
    State(pool): State<PgPool>,
    Json(body): Json<PasswordRequest>,
) -> Result<Json<PasswordResponse>, StatusCode> {
    let encrypted = crypto::encrypt(&body.password);

    password_repo::add(&pool, &body.id, &encrypted).await
        .map_err(|_| StatusCode::CONFLICT)?;

    Ok(Json(PasswordResponse {
        id: body.id,
        password: body.password,
    }))
}

/// PUT /api/passwords/:id — update
pub async fn update(
    State(pool): State<PgPool>,
    Path(old_id): Path<String>,
    Json(body): Json<PasswordRequest>,
) -> Result<Json<PasswordResponse>, StatusCode> {
    let encrypted = crypto::encrypt(&body.password);

    let updated = password_repo::update(&pool, &old_id, &body.id, &encrypted).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !updated {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(Json(PasswordResponse {
        id: body.id,
        password: body.password,
    }))
}

/// DELETE /api/passwords/:id — delete
pub async fn delete(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let deleted = password_repo::delete(&pool, &id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !deleted {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}