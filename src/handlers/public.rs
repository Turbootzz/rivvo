use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::OptionalAuthenticatedUser;
use crate::models::board::BoardResponse;
use crate::models::changelog::ChangelogResponse;
use crate::models::post::PostListResponse;
use crate::services::{
    board_service, changelog_service, comment_service, org_service, post_service, tag_service,
};

use super::comments::build_comment_list;
use super::posts::build_post_list_response;

#[derive(Deserialize)]
pub struct PublicPostQuery {
    pub sort: Option<String>,
    pub status: Option<String>,
    pub search: Option<String>,
}

/// GET /api/public/{org_slug}/boards
pub async fn list_boards(
    pool: web::Data<PgPool>,
    org_slug: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let org = org_service::get_org_by_slug(pool.get_ref(), &org_slug).await?;
    let boards = board_service::get_boards(pool.get_ref(), org.id).await?;
    let response: Vec<BoardResponse> = boards.into_iter().map(BoardResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/public/{org_slug}/boards/{board_slug}/posts
pub async fn list_posts(
    pool: web::Data<PgPool>,
    opt_auth: OptionalAuthenticatedUser,
    path: web::Path<(String, String)>,
    query: web::Query<PublicPostQuery>,
) -> Result<HttpResponse, AppError> {
    let (org_slug, board_slug) = path.into_inner();
    let org = org_service::get_org_by_slug(pool.get_ref(), &org_slug).await?;
    let board = board_service::get_board_by_slug(pool.get_ref(), org.id, &board_slug).await?;

    let sort = query.sort.as_deref().unwrap_or("votes");
    let status_filter = query.status.as_deref();
    let search = query.search.as_deref();

    let rows = post_service::get_posts(
        pool.get_ref(),
        board.id,
        opt_auth.user_id,
        sort,
        status_filter,
        search,
    )
    .await?;

    let post_ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();
    let mut tags_map = tag_service::get_tags_for_post_ids(pool.get_ref(), &post_ids).await?;
    let response = build_post_list_response(rows, &mut tags_map);

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/public/{org_slug}/boards/{board_slug}/posts/{post_id}
pub async fn get_post(
    pool: web::Data<PgPool>,
    opt_auth: OptionalAuthenticatedUser,
    path: web::Path<(String, String, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (org_slug, board_slug, post_id) = path.into_inner();
    let org = org_service::get_org_by_slug(pool.get_ref(), &org_slug).await?;
    let board = board_service::get_board_by_slug(pool.get_ref(), org.id, &board_slug).await?;

    let detail = post_service::get_post(pool.get_ref(), post_id, opt_auth.user_id).await?;
    if detail.board_id != board.id {
        return Err(AppError::NotFound("Post not found".to_string()));
    }

    let tags = tag_service::get_post_tags(pool.get_ref(), post_id).await?;
    Ok(HttpResponse::Ok().json(super::posts::build_detail_response(detail, tags)))
}

/// GET /api/public/{org_slug}/boards/{board_slug}/posts/{post_id}/comments
pub async fn list_comments(
    pool: web::Data<PgPool>,
    path: web::Path<(String, String, Uuid)>,
) -> Result<HttpResponse, AppError> {
    let (_org_slug, _board_slug, post_id) = path.into_inner();
    let rows = comment_service::get_comments(pool.get_ref(), post_id).await?;
    let response = build_comment_list(rows);
    Ok(HttpResponse::Ok().json(response))
}

#[derive(Serialize)]
pub struct RoadmapResponse {
    pub planned: Vec<PostListResponse>,
    pub in_progress: Vec<PostListResponse>,
    pub done: Vec<PostListResponse>,
}

/// GET /api/public/{org_slug}/boards/{board_slug}/roadmap
pub async fn roadmap(
    pool: web::Data<PgPool>,
    opt_auth: OptionalAuthenticatedUser,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, AppError> {
    let (org_slug, board_slug) = path.into_inner();
    let org = org_service::get_org_by_slug(pool.get_ref(), &org_slug).await?;
    let board = board_service::get_board_by_slug(pool.get_ref(), org.id, &board_slug).await?;

    let user_id = opt_auth.user_id;
    let planned = post_service::get_posts(
        pool.get_ref(),
        board.id,
        user_id,
        "votes",
        Some("planned"),
        None,
    )
    .await?;
    let in_progress = post_service::get_posts(
        pool.get_ref(),
        board.id,
        user_id,
        "votes",
        Some("in_progress"),
        None,
    )
    .await?;
    let done = post_service::get_posts(
        pool.get_ref(),
        board.id,
        user_id,
        "votes",
        Some("done"),
        None,
    )
    .await?;

    // Batch fetch tags for all posts
    let all_ids: Vec<Uuid> = planned
        .iter()
        .chain(in_progress.iter())
        .chain(done.iter())
        .map(|r| r.id)
        .collect();
    let mut tags_map = tag_service::get_tags_for_post_ids(pool.get_ref(), &all_ids).await?;

    let response = RoadmapResponse {
        planned: build_post_list_response(planned, &mut tags_map),
        in_progress: build_post_list_response(in_progress, &mut tags_map),
        done: build_post_list_response(done, &mut tags_map),
    };

    Ok(HttpResponse::Ok().json(response))
}

/// GET /api/public/{org_slug}/changelog
pub async fn list_changelog(
    pool: web::Data<PgPool>,
    org_slug: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    let org = org_service::get_org_by_slug(pool.get_ref(), &org_slug).await?;
    let entries = changelog_service::get_published(pool.get_ref(), org.id).await?;
    let response: Vec<ChangelogResponse> =
        entries.into_iter().map(ChangelogResponse::from).collect();
    Ok(HttpResponse::Ok().json(response))
}
