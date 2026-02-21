use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::{App, test as actix_test, web};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

const JWT_SECRET: &str = "test-secret-that-is-at-least-32-characters-long";

pub fn test_config() -> rivvo::config::Config {
    rivvo::config::Config {
        database_url: String::new(), // pool is shared directly
        jwt_secret: JWT_SECRET.to_string(),
        host: "127.0.0.1".to_string(),
        port: 8080,
        cors_origin: "http://localhost:5173".to_string(),
        max_db_connections: 5,
    }
}

pub async fn create_pool() -> PgPool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");
    sqlx::PgPool::connect(&url)
        .await
        .expect("Failed to connect to test database")
}

pub fn build_app(
    pool: PgPool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let config = test_config();
    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(config))
        .service(
            web::scope("/api")
                .route(
                    "/health",
                    web::get().to(rivvo::handlers::health::health_check),
                )
                .service(
                    web::scope("/auth")
                        .route("/register", web::post().to(rivvo::handlers::auth::register))
                        .route("/login", web::post().to(rivvo::handlers::auth::login))
                        .route("/me", web::get().to(rivvo::handlers::auth::me)),
                )
                .route(
                    "/orgs",
                    web::get().to(rivvo::handlers::organizations::list_user_orgs),
                )
                .service(
                    web::scope("/orgs/{org_id}/boards")
                        .route("", web::get().to(rivvo::handlers::boards::list_boards))
                        .route("", web::post().to(rivvo::handlers::boards::create_board))
                        .route("/{slug}", web::get().to(rivvo::handlers::boards::get_board))
                        .route(
                            "/{slug}",
                            web::put().to(rivvo::handlers::boards::update_board),
                        )
                        .route(
                            "/{slug}",
                            web::delete().to(rivvo::handlers::boards::delete_board),
                        ),
                )
                .service(
                    web::scope("/boards/{board_id}/posts")
                        .route("", web::get().to(rivvo::handlers::posts::list_posts))
                        .route("", web::post().to(rivvo::handlers::posts::create_post))
                        .route(
                            "/{post_id}",
                            web::get().to(rivvo::handlers::posts::get_post),
                        )
                        .route(
                            "/{post_id}",
                            web::put().to(rivvo::handlers::posts::update_post),
                        )
                        .route(
                            "/{post_id}",
                            web::delete().to(rivvo::handlers::posts::delete_post),
                        )
                        .route(
                            "/{post_id}/status",
                            web::put().to(rivvo::handlers::posts::update_status),
                        ),
                )
                .service(
                    web::scope("/boards/{board_id}/tags")
                        .route("", web::get().to(rivvo::handlers::tags::list_tags))
                        .route("", web::post().to(rivvo::handlers::tags::create_tag)),
                )
                .route(
                    "/posts/{post_id}/vote",
                    web::post().to(rivvo::handlers::votes::toggle_vote),
                )
                .service(
                    web::scope("/posts/{post_id}/comments")
                        .route("", web::get().to(rivvo::handlers::comments::list_comments))
                        .route(
                            "",
                            web::post().to(rivvo::handlers::comments::create_comment),
                        ),
                )
                .route(
                    "/comments/{comment_id}",
                    web::delete().to(rivvo::handlers::comments::delete_comment),
                )
                .route(
                    "/tags/{tag_id}",
                    web::delete().to(rivvo::handlers::tags::delete_tag),
                )
                .route(
                    "/posts/{post_id}/tags/{tag_id}",
                    web::post().to(rivvo::handlers::tags::add_tag_to_post),
                )
                .route(
                    "/posts/{post_id}/tags/{tag_id}",
                    web::delete().to(rivvo::handlers::tags::remove_tag_from_post),
                ),
        )
}

/// Register a user with a unique email and return (token, user_id, org_id).
pub async fn register_user(pool: &PgPool) -> (String, Uuid, Uuid) {
    let suffix = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("test-{suffix}@example.com");
    let app = actix_test::init_service(build_app(pool.clone())).await;

    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "name": format!("Test User {suffix}"),
            "password": "password123"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201, "register failed");
    let body: Value = actix_test::read_body_json(resp).await;
    let token = body["token"].as_str().unwrap().to_string();
    let user_id = Uuid::parse_str(body["user"]["id"].as_str().unwrap()).unwrap();

    // Fetch the auto-created org
    let orgs: Vec<crate::common::OrgRow> =
        sqlx::query_as("SELECT o.id FROM organizations o JOIN org_members m ON m.org_id = o.id WHERE m.user_id = $1")
            .bind(user_id)
            .fetch_all(pool)
            .await
            .unwrap();
    let org_id = orgs[0].id;

    (token, user_id, org_id)
}

#[derive(sqlx::FromRow)]
pub struct OrgRow {
    pub id: Uuid,
}

/// Register a second user and add them as a member (not admin) of the given org.
pub async fn register_member(pool: &PgPool, org_id: Uuid) -> (String, Uuid) {
    let suffix = Uuid::new_v4().to_string()[..8].to_string();
    let email = format!("member-{suffix}@example.com");
    let app = actix_test::init_service(build_app(pool.clone())).await;

    let req = actix_test::TestRequest::post()
        .uri("/api/auth/register")
        .set_json(serde_json::json!({
            "email": email,
            "name": format!("Member {suffix}"),
            "password": "password123"
        }))
        .to_request();

    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201);
    let body: Value = actix_test::read_body_json(resp).await;
    let token = body["token"].as_str().unwrap().to_string();
    let user_id = Uuid::parse_str(body["user"]["id"].as_str().unwrap()).unwrap();

    // Add to the admin's org as a regular member
    sqlx::query("INSERT INTO org_members (org_id, user_id, role) VALUES ($1, $2, 'member')")
        .bind(org_id)
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap();

    (token, user_id)
}

/// Helper to create a board via API and return (board_id, slug).
pub async fn create_board(pool: &PgPool, token: &str, org_id: Uuid, name: &str) -> (Uuid, String) {
    let app = actix_test::init_service(build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/orgs/{org_id}/boards"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "name": name }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201, "create_board failed");
    let body: Value = actix_test::read_body_json(resp).await;
    let id = Uuid::parse_str(body["id"].as_str().unwrap()).unwrap();
    let slug = body["slug"].as_str().unwrap().to_string();
    (id, slug)
}

/// Helper to create a post via API and return post_id.
pub async fn create_post(pool: &PgPool, token: &str, board_id: Uuid, title: &str) -> Uuid {
    let app = actix_test::init_service(build_app(pool.clone())).await;
    let req = actix_test::TestRequest::post()
        .uri(&format!("/api/boards/{board_id}/posts"))
        .insert_header(("Authorization", format!("Bearer {token}")))
        .set_json(serde_json::json!({ "title": title, "description": "Test description" }))
        .to_request();
    let resp = actix_test::call_service(&app, req).await;
    assert_eq!(resp.status(), 201, "create_post failed");
    let body: Value = actix_test::read_body_json(resp).await;
    Uuid::parse_str(body["id"].as_str().unwrap()).unwrap()
}
