use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};
use tracing_subscriber::EnvFilter;

mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;
mod vault;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    vault::fetch_secrets().await;

    let config = config::Config::from_env().expect("Failed to load configuration");

    let pool = db::create_pool(&config.database_url, config.max_db_connections)
        .await
        .expect("Failed to create database pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run database migrations");

    let host = config.host.clone();
    let port = config.port;

    tracing::info!("Starting server at http://{}:{}", host, port);

    let cors_origin = config.cors_origin.clone();
    let config_data = web::Data::new(config);
    let pool_data = web::Data::new(pool);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(pool_data.clone())
            .app_data(config_data.clone())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(handlers::health::health_check))
                    // Auth
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(handlers::auth::register))
                            .route("/login", web::post().to(handlers::auth::login))
                            .route("/me", web::get().to(handlers::auth::me)),
                    )
                    // Organizations
                    .route(
                        "/orgs",
                        web::get().to(handlers::organizations::list_user_orgs),
                    )
                    // Boards (under orgs)
                    .service(
                        web::scope("/orgs/{org_id}/boards")
                            .route("", web::get().to(handlers::boards::list_boards))
                            .route("", web::post().to(handlers::boards::create_board))
                            .route("/{slug}", web::get().to(handlers::boards::get_board))
                            .route("/{slug}", web::put().to(handlers::boards::update_board))
                            .route("/{slug}", web::delete().to(handlers::boards::delete_board)),
                    )
                    // Posts (under boards)
                    .service(
                        web::scope("/boards/{board_id}/posts")
                            .route("", web::get().to(handlers::posts::list_posts))
                            .route("", web::post().to(handlers::posts::create_post))
                            .route("/{post_id}", web::get().to(handlers::posts::get_post))
                            .route("/{post_id}", web::put().to(handlers::posts::update_post))
                            .route("/{post_id}", web::delete().to(handlers::posts::delete_post))
                            .route(
                                "/{post_id}/status",
                                web::put().to(handlers::posts::update_status),
                            ),
                    )
                    // Tags (under boards)
                    .service(
                        web::scope("/boards/{board_id}/tags")
                            .route("", web::get().to(handlers::tags::list_tags))
                            .route("", web::post().to(handlers::tags::create_tag)),
                    )
                    // Direct post lookup (no board_id required)
                    .route(
                        "/posts/{post_id}",
                        web::get().to(handlers::posts::get_post_direct),
                    )
                    // Votes
                    .route(
                        "/posts/{post_id}/vote",
                        web::post().to(handlers::votes::toggle_vote),
                    )
                    // Comments
                    .service(
                        web::scope("/posts/{post_id}/comments")
                            .route("", web::get().to(handlers::comments::list_comments))
                            .route("", web::post().to(handlers::comments::create_comment)),
                    )
                    // Comment delete + Tag delete + post tag assignment
                    .route(
                        "/comments/{comment_id}",
                        web::delete().to(handlers::comments::delete_comment),
                    )
                    .route(
                        "/tags/{tag_id}",
                        web::delete().to(handlers::tags::delete_tag),
                    )
                    .route(
                        "/posts/{post_id}/tags/{tag_id}",
                        web::post().to(handlers::tags::add_tag_to_post),
                    )
                    .route(
                        "/posts/{post_id}/tags/{tag_id}",
                        web::delete().to(handlers::tags::remove_tag_from_post),
                    ),
            )
    })
    .bind(format!("{host}:{port}"))?
    .run()
    .await
}
