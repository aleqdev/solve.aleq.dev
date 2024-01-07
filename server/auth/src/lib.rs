use argon2::{*, password_hash::{SaltString, rand_core::OsRng}};
use password_hash::Salt;
use wasm_bindgen::prelude::*;


pub fn generate_secrets(password: &str) -> password_hash::Result<(SaltString, String)> {
    let salt = SaltString::generate(&mut OsRng);
  
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())?;

    Ok((salt, hashed_password))
}


#[wasm_bindgen]
pub fn generate_secrets_wasm(password: &str) -> Option<Vec<String>> {
  generate_secrets(password).map(|(s, h)| vec![s.to_string(), h]).ok()
}

#[wasm_bindgen]
pub fn hash_login_password_wasm(password: &str, salt: &str) -> Option<String> {
  Salt::from_b64(salt).ok().and_then(|salt| {
    Argon2::default()
        .hash_password(password.as_bytes(), salt)
        .map(|hash| hash.to_string()).ok()
  })
}