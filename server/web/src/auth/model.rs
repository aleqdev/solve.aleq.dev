use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}


#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub username: String,
    pub salt: String,
    pub hashed_password: String,
}


#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub username: String,
    pub hashed_password: String,
}

#[derive(Debug, Deserialize)]
pub struct GetSaltSchema {
    pub username: String,
}
