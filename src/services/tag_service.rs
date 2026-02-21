use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::tag::Tag;

pub async fn create_tag(
    pool: &PgPool,
    board_id: Uuid,
    name: &str,
    color: Option<&str>,
) -> Result<Tag, AppError> {
    sqlx::query_as(
        "INSERT INTO tags (board_id, name, color) VALUES ($1, $2, COALESCE($3, '#6366f1')) RETURNING *",
    )
    .bind(board_id)
    .bind(name)
    .bind(color)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            AppError::BadRequest("A tag with this name already exists on this board".to_string())
        }
        _ => AppError::DatabaseError(e),
    })
}

pub async fn get_tags(pool: &PgPool, board_id: Uuid) -> Result<Vec<Tag>, AppError> {
    let tags = sqlx::query_as("SELECT * FROM tags WHERE board_id = $1 ORDER BY name ASC")
        .bind(board_id)
        .fetch_all(pool)
        .await?;
    Ok(tags)
}

pub async fn get_post_tags(pool: &PgPool, post_id: Uuid) -> Result<Vec<Tag>, AppError> {
    let tags = sqlx::query_as(
        r#"
        SELECT t.* FROM tags t
        JOIN post_tags pt ON pt.tag_id = t.id
        WHERE pt.post_id = $1
        ORDER BY t.name ASC
        "#,
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;
    Ok(tags)
}

pub async fn delete_tag(pool: &PgPool, tag_id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(tag_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tag not found".to_string()));
    }
    Ok(())
}

pub async fn add_tag_to_post(pool: &PgPool, post_id: Uuid, tag_id: Uuid) -> Result<(), AppError> {
    sqlx::query("INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(post_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn remove_tag_from_post(
    pool: &PgPool,
    post_id: Uuid,
    tag_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM post_tags WHERE post_id = $1 AND tag_id = $2")
        .bind(post_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
    Ok(())
}
