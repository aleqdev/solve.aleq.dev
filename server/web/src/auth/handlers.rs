use std::sync::Arc;

use super::model::{GetSaltSchema, LoginUserSchema, RegisterUserSchema, GetMeSchema, TokenClaims};
use crate::AppState;
use axum::{
    extract::State,
    http::{header, Response, StatusCode},
    response::IntoResponse,
    Extension, Json, body::Body,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;

pub async fn register_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut conn = state.db.get().await.unwrap();

    let user_exists: bool = db::orm::User::exists_with_username(&mut conn, &body.username)
        .await
        .map_err(crate::errors::database_error)
        .map_err(IntoResponse::into_response)?;

    if user_exists {
        return Err(crate::errors::user_exists_htmx());
    }

    let salt = body.salt;
    let hashed_password = body.hashed_password;

    let user = db::orm::User::create(
        &mut conn,
        &body.username,
        salt.as_bytes(),
        hashed_password.as_bytes(),
    )
    .await
    .map_err(crate::errors::database_error)
    .map_err(IntoResponse::into_response)?;

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_config.secret.as_bytes()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    let headers = response.headers_mut();
        
    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    headers.insert("HX-Trigger", "reload".parse().unwrap());

    Ok(response)
}

pub async fn login_user_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut conn = state.db.get().await.unwrap();

    let user = db::orm::User::get_by_username(&mut conn, &body.username)
        .await
        .map_err(crate::errors::database_error)
        .map_err(IntoResponse::into_response)?
        .ok_or_else(crate::errors::invalid_username_or_password_htmx)?;

    let is_valid = user.password_hash == body.hashed_password.as_bytes();

    if !is_valid {
        return Err(crate::errors::invalid_username_or_password_htmx());
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_config.secret.as_bytes()),
    )
    .unwrap();

    let cookie = Cookie::build(("token", token.to_owned()))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    let headers = response.headers_mut();

    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    headers.insert("HX-Trigger", "reload".parse().unwrap());

    Ok(response)
}

pub async fn get_salt_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<GetSaltSchema>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut conn = state.db.get().await.unwrap();

    let user = db::orm::User::get_by_username(&mut conn, &body.username)
        .await
        .map_err(crate::errors::database_error)
        .map_err(IntoResponse::into_response)?
        .ok_or_else(crate::errors::invalid_username_or_password_htmx)?;

    let salt = std::str::from_utf8(&user.salt).unwrap();
    let hx_trigger = format!(r#"{{"try_login":{{"salt": "{salt}"}}}}"#);

    let mut response = Response::new(json!({"status": "success"}).to_string());
    response
        .headers_mut()
        .insert("HX-Trigger", hx_trigger.parse().unwrap());

    Ok::<_, Response<Body>>(response)
}

pub async fn logout_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true);

    let mut response = Response::new(json!({"status": "success"}).to_string());
    let headers = response.headers_mut();

    headers.insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    headers.insert("HX-Trigger", "reload".parse().unwrap());

    Ok(response)
}

pub async fn get_me_handler(
    Extension(user): Extension<db::orm::User>,
    Json(body): Json<GetMeSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
  match body.query.as_str() {
    "all" => return Ok(Json(serde_json::json!({
      "status":  "success",
      "data": serde_json::json!({
          "user": filter_user_record(&user)
      })
    })).into_response()),
    "username" => return Ok(user.username.into_response()),
    _ => {
      let error_response = serde_json::json!({
        "status": "error",
        "message": "Invalid query",
      });
      return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }
  }
}

#[derive(Debug, serde::Serialize)]
pub struct FilteredUser<'a> {
    pub id: uuid::Uuid,
    pub username: &'a str,
}

fn filter_user_record(user: &db::orm::User) -> FilteredUser<'_> {
    FilteredUser {
        id: user.id,
        username: &user.username,
    }
}
