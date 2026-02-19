use sqlx::PgPool;
use crate::models::password::Password;

/// Insert new password record
pub async fn add(pool: &PgPool, id: &str, password: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO passwords (id, password) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(password)
    .execute(pool)
    .await?;

    Ok(())
}

/// Fetch password by id
pub async fn get_by_id(pool: &PgPool, id: &str) -> Result<Password, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT id, password FROM passwords WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(Password {
        id: row.0,
        password: row.1,
    })
}

/// Fetch all passwords (id + password)
pub async fn get_all(pool: &PgPool) -> Result<Vec<Password>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, String)>(
        "SELECT id, password FROM passwords ORDER BY id"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| Password {
        id: r.0,
        password: r.1,
    }).collect())
}
/// Update id and/or password
pub async fn update(
    pool: &PgPool,
    old_id: &str,
    new_id: &str,
    new_password: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE passwords SET id = $1, password = $2 WHERE id = $3"
    )
    .bind(new_id)
    .bind(new_password)
    .bind(old_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Delete password by id
pub async fn delete(pool: &PgPool, id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM passwords WHERE id = $1"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}