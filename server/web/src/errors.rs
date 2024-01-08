use axum::body::Body;
use axum::{response::{IntoResponse, Response}, http::StatusCode};
use axum::Json;

pub type TyString = (StatusCode, String);
pub type TyJson = (StatusCode, Json<serde_json::Value>);

pub fn database_error(e: db::diesel::result::Error) -> TyJson {
    let error_response = serde_json::json!({
      "status": "error",
      "message": format!("Database error: {}", e),
    });

    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}

pub fn user_exists() -> TyString {
    (StatusCode::CONFLICT, "* ID уже занят".to_owned())
}

pub fn user_exists_htmx() -> Response<Body> {
  let mut response = user_exists().into_response();
  response.headers_mut().insert("HX-Reswap", "innerHTML".parse().unwrap());
  response
}

pub fn invalid_username_or_password() -> TyString {
    (StatusCode::BAD_REQUEST, "* Неверные данные".to_owned())
}

pub fn invalid_username_or_password_htmx() -> Response<Body> {
  let mut response = invalid_username_or_password().into_response();
  response.headers_mut().insert("HX-Reswap", "innerHTML".parse().unwrap());
  response
}

pub fn missing_token() -> TyJson {
    let error_response = serde_json::json!({
      "status": "error",
      "message": "You are not logged in, please provide token"
    });
    (StatusCode::UNAUTHORIZED, Json(error_response))
}

pub fn invalid_token() -> TyJson {
    let error_response = serde_json::json!({
      "status": "error",
      "message": "invalid_token"
    });
    (StatusCode::UNAUTHORIZED, Json(error_response))
}
