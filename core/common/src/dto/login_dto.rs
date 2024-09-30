use salvo::macros::Extractible;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Default)]
pub struct LoginResponse {
    pub token: String,
}
