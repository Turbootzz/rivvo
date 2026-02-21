use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_id: Option<Uuid>,
    pub body: String,
    pub is_admin_reply: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Flat row from JOIN with users.
#[derive(Debug, sqlx::FromRow)]
pub struct CommentWithAuthorRow {
    pub id: Uuid,
    pub body: String,
    pub is_admin_reply: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub author_avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub body: String,
    pub is_admin_reply: bool,
    pub author: Option<CommentAuthor>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CommentAuthor {
    pub id: Uuid,
    pub name: String,
    pub avatar_url: Option<String>,
}
