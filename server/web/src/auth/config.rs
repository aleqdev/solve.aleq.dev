#[derive(Debug, Clone)]
pub struct JWTConfig {
    pub secret: String,
    pub expires_in: String,
    pub maxage: i32,
}

impl JWTConfig {
    pub fn init() -> JWTConfig {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Self {
            secret,
            expires_in,
            maxage: maxage.parse::<i32>().unwrap(),
        }
    }
}
