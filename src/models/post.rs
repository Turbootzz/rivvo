use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub board_id: Uuid,
    pub author_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub category: Option<String>,
    pub vote_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub pinned: Option<bool>,
    pub merged_into_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Flat row returned by the list query (JOIN with users + LEFT JOIN votes).
#[derive(Debug, sqlx::FromRow)]
pub struct PostListRow {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub vote_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub pinned: Option<bool>,
    pub author_name: Option<String>,
    pub has_voted: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostListResponse {
    pub id: Uuid,
    pub title: String,
    pub description_preview: Option<String>,
    pub status: String,
    pub vote_count: i32,
    pub comment_count: i32,
    pub pinned: bool,
    pub author_name: Option<String>,
    pub has_voted: bool,
    pub tags: Vec<super::tag::TagResponse>,
    pub created_at: Option<DateTime<Utc>>,
}

/// Flat row for single-post detail (JOIN with users + LEFT JOIN votes).
#[derive(Debug, sqlx::FromRow)]
pub struct PostDetailRow {
    pub id: Uuid,
    pub board_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<String>,
    pub vote_count: Option<i32>,
    pub comment_count: Option<i32>,
    pub pinned: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub author_id: Option<Uuid>,
    pub author_name: Option<String>,
    pub author_email: Option<String>,
    pub author_avatar_url: Option<String>,
    pub has_voted: bool,
}

#[derive(Debug, Serialize)]
pub struct PostDetailResponse {
    pub id: Uuid,
    pub board_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub vote_count: i32,
    pub comment_count: i32,
    pub pinned: bool,
    pub author: Option<PostAuthor>,
    pub has_voted: bool,
    pub tags: Vec<super::tag::TagResponse>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct PostAuthor {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub avatar_url: Option<String>,
}

/// Status values for posts.
pub const VALID_STATUSES: &[&str] = &["open", "planned", "in_progress", "done", "closed"];
