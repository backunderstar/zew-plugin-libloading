use interface::error::AppResult;
use jsonwebtoken::{decode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::{config::get, utils::env_conversion::Convert};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    username: String,
    user_id: i32,
    issuer: String,
    exp: i64,
}

pub fn get_token(username: String, user_id: i32) -> AppResult<(String, i64)> {
    let exp = OffsetDateTime::now_utc() + Duration::minutes(get("jwt_expire")?.to_i64()?);
    let claim = JwtClaims {
        username,
        user_id,
        issuer: get("jwt_issuer")?,
        exp: exp.unix_timestamp(),
    };
    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(get("jwt_secret")?.as_bytes()),
    )?;
    Ok((token, exp.unix_timestamp()))
}

pub fn decode_token(token: &str) -> AppResult<bool> {
    let validation = Validation::new(Algorithm::HS256);
    Ok(decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(get("jwt_secret")?.as_bytes()),
        &validation,
    )
    .is_ok())
}
