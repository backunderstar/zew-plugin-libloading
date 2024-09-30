use bcrypt::hash;
use interface::error::AppResult;

use crate::config::get;

use super::env_conversion::Convert;

//==================================================================================
//= 加密
pub async fn encrypt(string: String) -> AppResult<String> {
    let encrypted_string = hash(string, get("encrypt_cost")?.to_u32()?)?;

    Ok(encrypted_string)
}

//==================================================================================
//= 验证
pub async fn verify(string: String, encrypted_string: String) -> AppResult<bool> {
    let is_valid = bcrypt::verify(string, &encrypted_string)?;

    Ok(is_valid)
}
