use db::{diesel::Connection, diesel_migrations::MigrationHarness};
use web::diesel_async::{async_connection_wrapper::AsyncConnectionWrapper, AsyncPgConnection};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Connecting to: {}", database_url);

    let database_url_clone = database_url.clone();
    tokio::task::spawn_blocking(move || {
        let mut temporary_connection_wrapper =
            AsyncConnectionWrapper::<AsyncPgConnection>::establish(&database_url_clone)
                .expect("Failed to establish temporary connection for migrations");
        temporary_connection_wrapper
            .run_pending_migrations(db::MIGRATIONS)
            .expect("Failed to run migrations");
    })
    .await
    .expect("Failed to execute migrations");

    let state = web::AppState {
        db: web::build_connection_pool(&database_url),
        jwt_config: web::auth::JWTConfig::init(),
    };

    let web_app_handle = tokio::spawn(web::serve_web_app(state));

    let (web_app_result,) = tokio::join!(web_app_handle);

    web_app_result.unwrap();
}
