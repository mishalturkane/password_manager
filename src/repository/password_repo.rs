use sqlx::PgPool;
use crate::models::password::PasswordRecord;

/// Insert new encrypted password
pub async fn add(pool: &PgPool, id: &str, encrypted_pass: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO passwords (id, encrypted_pass) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(encrypted_pass)
    .execute(pool)
    .await?;

    Ok(())
}

/// Get by id
pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<PasswordRecord, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT id, encrypted_pass FROM passwords WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(PasswordRecord { id: row.0, encrypted_pass: row.1 })
}

/// Get all records
pub async fn get_all(pool: &PgPool) -> Result<Vec<PasswordRecord>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String)>(
        "SELECT id, encrypted_pass FROM passwords ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| PasswordRecord {
        id: r.0,
        encrypted_pass: r.1,
    }).collect())
}

/// Search by id (partial match)
pub async fn search(pool: &PgPool, query: &str) -> Result<Vec<PasswordRecord>, sqlx::Error> {
    let pattern = format!("%{}%", query);

    let rows = sqlx::query_as::<_, (String, String)>(
        "SELECT id, encrypted_pass FROM passwords WHERE id ILIKE $1 ORDER BY id"
    )
    .bind(pattern)
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| PasswordRecord {
        id: r.0,
        encrypted_pass: r.1,
    }).collect())
}

/// Update id and/or password
pub async fn update(
    pool: &PgPool,
    old_id: &str,
    new_id: &str,
    new_encrypted_pass: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE passwords SET id = $1, encrypted_pass = $2 WHERE id = $3"
    )
    .bind(new_id)
    .bind(new_encrypted_pass)
    .bind(old_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete by id
pub async fn delete(pool: &PgPool, id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM passwords WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}