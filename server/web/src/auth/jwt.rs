use std::sync::Arc;

use axum::{extract::State, http::header, middleware::Next, response::IntoResponse};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{auth::model::TokenClaims, AppState};

pub async fn jwt_layer(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: axum::extract::Request,
    next: Next,
) -> Result<impl IntoResponse, crate::errors::Ty> {
    let mut conn = &mut data.db.get().await.unwrap();

    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let token = token.ok_or_else(crate::errors::missing_token)?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.jwt_config.secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| crate::errors::invalid_token())?
    .claims;

    let user_id = uuid::Uuid::parse_str(&claims.sub).map_err(|_| crate::errors::invalid_token())?;

    let user = db::orm::User::get(&mut conn, user_id)
        .await
        .map_err(crate::errors::database_error)?;

    let user = user.ok_or_else(crate::errors::invalid_token)?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
