use thiserror::Error;

//==================================================================================
//= 自定义error
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    AnyHow(#[from] anyhow::Error),
    // #[error("{0}")]
    // DbErr(#[from] sea_orm::DbErr),
    // #[error("{0}")]
    // ParseErr(#[from] salvo::http::ParseError),
    // #[error("{0}")]
    // ValidateErr(#[from] validator::ValidationErrors),
    #[error("{0}")]
    BcryptErr(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    JwtErr(#[from] jsonwebtoken::errors::Error),
    #[error("{0}")]
    VarErr(#[from] std::env::VarError),
    #[error("{0}")]
    LibErr(#[from] libloading::Error),
}

pub type AppResult<T> = Result<T, AppError>;
