use salvo::macros::Extractible;
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct UserAddRequest {
    pub role: i8,
    pub username: String,
    pub nickname: String,
    pub password: String,
    pub email: String,
    pub avatar: String,
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct UserUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Default)]
pub struct UserInfoResponse {
    pub id: i32,
    pub role: i8,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}
