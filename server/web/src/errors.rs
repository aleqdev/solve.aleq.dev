use axum::http::StatusCode;
use axum::Json;

pub type Ty = (StatusCode, Json<serde_json::Value>);


pub fn database_error(e: db::diesel::result::Error) -> Ty {
    let error_response = serde_json::json!({
      "status": "error",
      "message": format!("Database error: {}", e),
    });

    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
}

pub fn user_exists() -> Ty {
  let error_response = serde_json::json!({
    "status": "error",
    "message": "User with that email already exists",
  });
  (StatusCode::CONFLICT, Json(error_response))
}

pub fn invalid_username_or_password() -> Ty {
    let error_response = serde_json::json!({
        "status": "error",
        "message": "Invalid email or password",
    });
    (StatusCode::BAD_REQUEST, Json(error_response))
}

pub fn missing_token() -> Ty {
  let error_response = serde_json::json!({
    "status": "error",
    "message": "You are not logged in, please provide token"
  });
  (StatusCode::UNAUTHORIZED, Json(error_response))
}

pub fn invalid_token() -> Ty {
  let error_response = serde_json::json!({
    "status": "error",
    "message": "invalid_token"
  });
  (StatusCode::UNAUTHORIZED, Json(error_response))
}