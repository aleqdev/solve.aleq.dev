pub mod templates;
pub mod auth;
pub mod errors;

use std::sync::Arc;

use axum::{http::{HeaderValue, Method, header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE}}, routing::{post, get}, Router, middleware};
use diesel_async::{pg::AsyncPgConnection, pooled_connection::{AsyncDieselConnectionManager, deadpool::Pool}};
use tower_http::{cors::CorsLayer, services::ServeDir};

pub use diesel_async;


#[derive(Clone)]
pub struct AppState {
  pub db: Pool<AsyncPgConnection>,
  pub jwt_config: auth::JWTConfig
}

pub fn build_connection_pool(url: &str) -> Pool<AsyncPgConnection> {
  let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);

  Pool::builder(manager)
      .build()
      .expect("Could not build connection pool")
}

pub async fn serve_web_app(state: AppState) {
  use time::Duration;
  use tower::ServiceBuilder;
  use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
  use axum::{*, http::StatusCode, error_handling::HandleErrorLayer};

  let state = std::sync::Arc::new(state);
  

  let session_store = MemoryStore::default();
  let session_service = ServiceBuilder::new()
    .layer(HandleErrorLayer::new(|_: BoxError| async {
        StatusCode::BAD_REQUEST
    }))
    .layer(
        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_expiry(Expiry::OnInactivity(Duration::seconds(10))),
    );

  let cors = CorsLayer::new()
    .allow_origin("http://localhost:32055".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

  let app = create_router(state)
    .layer(session_service)
    .layer(cors);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:32055").await.unwrap();
  axum::serve(listener, app).await.unwrap()
}


pub fn create_router(state: Arc<AppState>) -> Router {
  Router::new()
    .nest_service("/static/", ServeDir::new("static"))
    .route("/", get(|| async { templates::IndexTemplate { title: "Главная" } }))
    .route("/api/auth/register", post(auth::register_user_handler))
    .route("/api/auth/login", post(auth::login_user_handler))
    .route("/api/auth/get_salt", post(auth::get_salt_handler))
    .route(
        "/api/auth/logout",
        get(auth::logout_handler)
            .route_layer(middleware::from_fn_with_state(state.clone(), auth::jwt_layer)),
    )
    .route(
        "/api/users/me",
        get(auth::get_me_handler)
            .route_layer(middleware::from_fn_with_state(state.clone(), auth::jwt_layer)),
    )
    .with_state(state)
}