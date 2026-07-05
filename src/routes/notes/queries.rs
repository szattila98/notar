

use sqlx::{PgPool, Result};
use uuid::Uuid;
use super::model::Note;

pub async fn create_note(pool: &PgPool, created_by: Uuid, title: &str) -> Result<Note> {
    let rec = sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (id, created_by, title)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, created_by, created_at
        "#,
        Uuid::new_v4(),
        created_by,
        title
    )
    .fetch_one(pool)
    .await?;

    Ok(rec)
}

pub async fn list_notes(pool: &PgPool, created_by: Uuid) -> Result<Vec<Note>> {
    let rec = sqlx::query_as!(
        Note,
        r#"
        SELECT id, created_by, title, content, created_at
        FROM notes
        WHERE created_by = $1
        ORDER BY created_at DESC
        "#,
        created_by
    )
    .fetch_all(pool)
    .await?;

    Ok(rec)
}

pub async fn update_note(pool: &PgPool, created_by: Uuid, id: Uuid, title: &str, content: &serde_json::Value) -> Result<Note> {
    let rec = sqlx::query_as!(
        Note,
        r#"
        UPDATE notes
        SET title = $3, content = $4
        WHERE id = $2 AND created_by = $1
        RETURNING id, created_by, title, content, created_at
        "#,
        created_by, id, title, content
    ).fetch_one(pool).await?;

    Ok(rec)
}

pub async fn delete_note(pool: &PgPool, created_by: Uuid, id: Uuid) -> Result<()> {
    sqlx::query!(
        r#"
        DELETE FROM notes
        WHERE id = $1 AND created_by = $2
        "#,
        id, created_by
    ).execute(pool).await?;

    Ok(())
}
