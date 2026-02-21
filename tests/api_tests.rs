mod common;

use actix_web::test as actix_test;
use serde_json::Value;
use uuid::Uuid;

// ============================================================
// Auth tests
// ============================================================

#[actix_web::test]
async fn register_returns_token_and_user() {
    let pool = common::create_pool().await;
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let suffix = Uuid::new_v4().to_string()[..8].to_string();

    let name = format!("Alice {suffix}");
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": format!("reg-{suffix}@example.com"),
            "name": &name,
            "password": "password123"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    let body: Value = actix_test::read_body_json(resp).await;
    assert!(body["token"].is_string());
    assert_eq!(body["user"]["name"], name);
    assert!(body["user"]["id"].is_string());
}

#[actix_web::test]
async fn register_creates_org_automatically() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;

    // The org should be accessible via the API
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri("/api/orgs")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    let orgs = body.as_array().unwrap();
    assert!(!orgs.is_empty());
    assert_eq!(orgs[0]["id"], org_id.to_string());
    assert_eq!(orgs[0]["role"], "admin");
}

#[actix_web::test]
async fn register_duplicate_email_fails() {
    let pool = common::create_pool().await;
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let suffix = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("dup-{suffix}@example.com");

    let body = serde_json::json!({
        "email": email,
        "name": format!("Duplicate {suffix}"),
        "password": "password123"
    });

    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&body)
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    // Second attempt with same email
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(&body)
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn register_validation_rejects_short_password() {
    let pool = common::create_pool().await;
    let app = actix_test::init_service(common::build_app(pool.clone())).await;

    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "valid@example.com",
            "name": "Valid Name",
            "password": "short"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}

#[actix_web::test]
async fn register_validation_rejects_invalid_email() {
    let pool = common::create_pool().await;
    let app = actix_test::init_service(common::build_app(pool.clone())).await;

    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": "not-an-email",
            "name": "Valid Name",
            "password": "password123"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 422);
}

#[actix_web::test]
async fn login_returns_token() {
    let pool = common::create_pool().await;
    let suffix = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("login-{suffix}@example.com");

    // Register first
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": &email,
            "name": format!("Login {suffix}"),
            "password": "password123"
        }))
        .to_request();
    actix_test::call_service(&app, req).await;

    // Login
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": &email,
            "password": "password123"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert!(body["token"].is_string());
}

#[actix_web::test]
async fn login_wrong_password_returns_401() {
    let pool = common::create_pool().await;
    let suffix = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("wrong-{suffix}@example.com");

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": &email,
            "name": format!("Wrong {suffix}"),
            "password": "password123"
        }))
        .to_request();
    actix_test::call_service(&app, req).await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(serde_json::json!({
            "email": &email,
            "password": "wrongpassword"
        }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

#[actix_web::test]
async fn me_returns_current_user() {
    let pool = common::create_pool().await;
    let (token, _user_id, _org_id) = common::register_user(&pool).await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri("/api/auth/me")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert!(body["email"].is_string());
}

#[actix_web::test]
async fn me_without_token_returns_401() {
    let pool = common::create_pool().await;
    let app = actix_test::init_service(common::build_app(pool.clone())).await;

    let req = actix_test::TestRequest::get()
        .uri("/api/auth/me")
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401);
}

// ============================================================
// Board tests
// ============================================================

#[actix_web::test]
async fn create_board_as_admin() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/orgs/{org_id}/boards"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "name": "Feature Requests",
            "description": "Vote on features"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["name"], "Feature Requests");
    assert_eq!(body["slug"], "feature-requests");
    assert_eq!(body["post_count"], 0);
}

#[actix_web::test]
async fn create_board_as_member_returns_403() {
    let pool = common::create_pool().await;
    let (_admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/orgs/{org_id}/boards"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "name": "Nope" }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn list_boards() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    common::create_board(&pool, &token, org_id, "Board A").await;
    common::create_board(&pool, &token, org_id, "Board B").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/orgs/{org_id}/boards"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    let boards = body.as_array().unwrap();
    assert!(boards.len() >= 2);
}

#[actix_web::test]
async fn get_board_by_slug() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (_board_id, slug) = common::create_board(&pool, &token, org_id, "My Board").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/orgs/{org_id}/boards/{slug}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["name"], "My Board");
}

#[actix_web::test]
async fn update_board() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (_board_id, slug) = common::create_board(&pool, &token, org_id, "Old Name").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/orgs/{org_id}/boards/{slug}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "name": "New Name",
            "description": "Updated"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["name"], "New Name");
}

#[actix_web::test]
async fn delete_board() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (_board_id, slug) = common::create_board(&pool, &token, org_id, "To Delete").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/orgs/{org_id}/boards/{slug}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn get_nonexistent_board_returns_404() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/orgs/{org_id}/boards/nonexistent-slug"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

// ============================================================
// Post tests
// ============================================================

#[actix_web::test]
async fn create_and_get_post() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Posts Board").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/posts"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "title": "Dark mode please",
            "description": "We need dark mode"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["title"], "Dark mode please");
    assert_eq!(body["status"], "open");
    assert_eq!(body["vote_count"], 0);
    let post_id = body["id"].as_str().unwrap();

    // Get the post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["title"], "Dark mode please");
}

#[actix_web::test]
async fn list_posts_with_sort_and_filter() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Sorted Board").await;

    common::create_post(&pool, &token, board_id, "Post A").await;
    common::create_post(&pool, &token, board_id, "Post B").await;

    // List with default sort
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 2);

    // List with status filter
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts?status=planned"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 0);

    // List with sort=recent
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts?sort=recent"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
}

#[actix_web::test]
async fn update_post_by_author() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Update Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Original Title").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({
            "title": "Updated Title",
            "description": "Updated desc"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["title"], "Updated Title");
}

#[actix_web::test]
async fn update_post_by_non_author_non_admin_returns_403() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) = common::create_board(&pool, &admin_token, org_id, "Auth Board").await;
    let post_id = common::create_post(&pool, &admin_token, board_id, "Admin Post").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "title": "Hacked" }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn delete_post_by_author() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Del Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "To Delete").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn update_status_as_admin() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Status Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Status Post").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}/status"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "status": "planned" }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["status"], "planned");
}

#[actix_web::test]
async fn update_status_as_member_returns_403() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) =
        common::create_board(&pool, &admin_token, org_id, "Status Auth Board").await;
    let post_id = common::create_post(&pool, &member_token, board_id, "Member Post").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}/status"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "status": "done" }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn update_status_invalid_value_returns_400() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Bad Status Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Bad Status").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}/status"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "status": "invalid_status" }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

// ============================================================
// Vote tests
// ============================================================

#[actix_web::test]
async fn toggle_vote_on_and_off() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Vote Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Vote Post").await;

    // Vote on
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/vote"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["voted"], true);
    assert_eq!(body["vote_count"], 1);

    // Vote off
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/vote"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["voted"], false);
    assert_eq!(body["vote_count"], 0);
}

#[actix_web::test]
async fn multiple_users_can_vote() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) =
        common::create_board(&pool, &admin_token, org_id, "Multi Vote Board").await;
    let post_id = common::create_post(&pool, &admin_token, board_id, "Popular Post").await;

    // Admin votes
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/vote"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["vote_count"], 1);

    // Member votes
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/vote"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["vote_count"], 2);
}

// ============================================================
// Comment tests
// ============================================================

#[actix_web::test]
async fn create_and_list_comments() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Comment Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Comment Post").await;

    // Create comment
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "body": "Great idea!" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["body"], "Great idea!");
    assert_eq!(body["is_admin_reply"], true); // user is admin

    // List comments
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[actix_web::test]
async fn member_comment_not_admin_reply() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) =
        common::create_board(&pool, &admin_token, org_id, "Member Comment Board").await;
    let post_id = common::create_post(&pool, &admin_token, board_id, "Post for Comment").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "body": "Member comment" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["is_admin_reply"], false);
}

#[actix_web::test]
async fn delete_own_comment() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Del Comment Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Post for Del").await;

    // Create comment
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "body": "To be deleted" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    let comment_id = body["id"].as_str().unwrap();

    // Delete it
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/comments/{comment_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn delete_others_comment_returns_403() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) =
        common::create_board(&pool, &admin_token, org_id, "Other Comment Board").await;
    let post_id = common::create_post(&pool, &admin_token, board_id, "Other Comment Post").await;

    // Admin creates comment
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({ "body": "Admin's comment" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    let comment_id = body["id"].as_str().unwrap();

    // Member tries to delete
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/comments/{comment_id}"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

#[actix_web::test]
async fn comment_increments_post_count() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Count Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Count Post").await;

    // Add comment
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "body": "Comment 1" }))
        .to_request();
    actix_test::call_service(&app, req).await;

    // Check post comment_count
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["comment_count"], 1);
}

// ============================================================
// Tag tests
// ============================================================

#[actix_web::test]
async fn create_and_list_tags() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Tag Board").await;

    // Create tag
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "name": "UX", "color": "#8b5cf6" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["name"], "UX");
    assert_eq!(body["color"], "#8b5cf6");

    // List tags
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body.as_array().unwrap().len(), 1);
}

#[actix_web::test]
async fn tag_default_color() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) =
        common::create_board(&pool, &token, org_id, "Default Color Board").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "name": "NoColor" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["color"], "#6366f1");
}

#[actix_web::test]
async fn delete_tag() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Del Tag Board").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "name": "ToDelete" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    let tag_id = body["id"].as_str().unwrap();

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/tags/{tag_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn assign_and_remove_tag_from_post() {
    let pool = common::create_pool().await;
    let (token, _user_id, org_id) = common::register_user(&pool).await;
    let (board_id, _slug) = common::create_board(&pool, &token, org_id, "Tag Assign Board").await;
    let post_id = common::create_post(&pool, &token, board_id, "Tagged Post").await;

    // Create tag
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "name": "Backend" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    let tag_id = body["id"].as_str().unwrap();

    // Assign tag to post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/tags/{tag_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // Verify tag shows on post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    let tags = body["tags"].as_array().unwrap();
    assert_eq!(tags.len(), 1);
    assert_eq!(tags[0]["name"], "Backend");

    // Remove tag from post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::delete()
        .uri(&format!("/api/posts/{post_id}/tags/{tag_id}"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);
}

#[actix_web::test]
async fn member_cannot_create_tag() {
    let pool = common::create_pool().await;
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;
    let (board_id, _slug) = common::create_board(&pool, &admin_token, org_id, "No Tag Board").await;

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "name": "Nope" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 403);
}

// ============================================================
// Full flow test
// ============================================================

#[actix_web::test]
async fn full_workflow() {
    let pool = common::create_pool().await;

    // 1. Register admin
    let (admin_token, _admin_id, org_id) = common::register_user(&pool).await;

    // 2. Register member
    let (member_token, _member_id) = common::register_member(&pool, org_id).await;

    // 3. Admin creates board
    let (board_id, _slug) =
        common::create_board(&pool, &admin_token, org_id, "Full Flow Board").await;

    // 4. Member creates post
    let post_id = common::create_post(&pool, &member_token, board_id, "Full Flow Post").await;

    // 5. Admin votes on post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/vote"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["voted"], true);
    assert_eq!(body["vote_count"], 1);

    // 6. Member adds comment
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/comments"))
        .insert_header(("Authorization", format!("Bearer {member_token}")))
        .set_json(serde_json::json!({ "body": "Thanks!" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);

    // 7. Admin changes status
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::put()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}/status"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({ "status": "in_progress" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["status"], "in_progress");

    // 8. Admin creates tag and assigns to post
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/tags"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .set_json(serde_json::json!({ "name": "Priority" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let tag_body: Value = actix_test::read_body_json(resp).await;
    let tag_id = tag_body["id"].as_str().unwrap();

    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/posts/{post_id}/tags/{tag_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 204);

    // 9. Verify final post state
    let app = actix_test::init_service(common::build_app(pool.clone())).await;
    let req = actix_test::TestRequest::get()
        .uri(&format!("/api/boards/{board_id}/posts/{post_id}"))
        .insert_header(("Authorization", format!("Bearer {admin_token}")))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    let body: Value = actix_test::read_body_json(resp).await;
    assert_eq!(body["status"], "in_progress");
    assert_eq!(body["vote_count"], 1);
    assert_eq!(body["comment_count"], 1);
    assert_eq!(body["has_voted"], true);
    assert_eq!(body["tags"].as_array().unwrap().len(), 1);
}
