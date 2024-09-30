use salvo::macros::Extractible;
use serde::Deserialize;

pub mod login_dto;
pub mod user_dto;





#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "param")))]
pub struct CommonIDRequest {
    pub id: i32,
}