use anyhow::anyhow;
use std::string::String;

pub trait Convert {
    fn to_bool(&self) -> Result<bool, anyhow::Error>;
    fn to_i32(&self) -> Result<i32, anyhow::Error>;
    fn to_i64(&self) -> Result<i64, anyhow::Error>;
    fn to_u32(&self) -> Result<u32, anyhow::Error>;
}

impl Convert for String {
    //==================================================================================
    //= String 类型转换为 bool 类型
    //= String to bool
    fn to_bool(&self) -> Result<bool, anyhow::Error> {
        match self.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(anyhow!("Failed to convert String to bool")),
        }
    }

    //==================================================================================
    //= String 类型转换为 i32 类型
    //= String to i32
    fn to_i32(&self) -> Result<i32, anyhow::Error> {
        match self.parse::<i32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to convert String to i32")),
        }
    }

    //==================================================================================
    //= String 类型转换为 i64 类型
    //= String to i64
    fn to_i64(&self) -> Result<i64, anyhow::Error> {
        match self.parse::<i64>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to parse to i64")),
        }
    }

    //==================================================================================
    //= String 类型转换为 u32 类型
    //= String to u32
    fn to_u32(&self) -> Result<u32, anyhow::Error> {
        match self.parse::<u32>() {
            Ok(i) => Ok(i),
            Err(_) => Err(anyhow!("Failed to parse to u32")),
        }
    }
}
