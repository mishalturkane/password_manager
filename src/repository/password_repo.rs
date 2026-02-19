use sqlx::PgPool;
use crate::models::password::Password;

/// Insert new password record
pub async fn add(pool: &PgPool, id: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO passwords (id, password) VALUES ($1, $2)",
        id,
        password
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch password by id
pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<Password, sqlx::Error> {
    let row = sqlx::query!(
        "SELECT id, password FROM passwords WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(Password {
        id: row.id,
        password: row.password,
    })
}

/// Fetch all password ids
pub async fn get_all_ids(pool: &PgPool) -> Result<Vec<String>, sqlx::Error> {
    let rows = sqlx::query!(
        "SELECT id FROM passwords ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.id).collect())
}

/// Update id and/or password
pub async fn update(
    pool: &PgPool,
    old_id: &str,
    new_id: &str,
    new_password: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE passwords SET id = $1, password = $2 WHERE id = $3",
        new_id,
        new_password,
        old_id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete password by id
pub async fn delete(pool: &PgPool, id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "DELETE FROM passwords WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}