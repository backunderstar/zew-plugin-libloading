use crate::error::{AppError, AppResult};
use salvo::{async_trait, http::StatusCode, writing::Json, Depot, Request, Response, Writer};
use serde::Serialize;

//==================================================================================
//= 通用响应结构体
//= Universal response structure 
#[derive(Serialize, Debug)]
pub struct ResponseBuilder<T> {
    pub code: i32,
    pub message: String,
    pub data: T,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponseBuilder {
    pub code: i32,
    pub message: String,
    #[serde(skip)]
    pub source_error: AppError,
}

//==================================================================================
//= 为通用结构体响应定义响应方法
//= Define response methods for common structure responses 
impl<T: Serialize + Send + Default> ResponseBuilder<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: 0,
            data,
            message: "success".to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn with_data_msg(data: T, msg: &str) -> Self {
        Self {
            code: 0,
            data,
            message: msg.to_string(),
        }
    }
    pub fn into_response(self, res: &mut Response) {
        res.render(Json(self));
    }
    // TODO: 文档
}
//= 错误响应构建的方法
//= Define error response builder methods
impl ErrorResponseBuilder {
    pub fn with_err(err: AppError) -> Self {
        let (code, msg) = match &err {
            AppError::AnyHow(e) => (500, e.to_string()),
            // AppError::DbErr(e) => (500, e.to_string()),
            // AppError::ParseErr(e) => (400, e.to_string()),
            // AppError::ValidateErr(e) => (400, e.to_string()),
            AppError::BcryptErr(e) => (400, e.to_string()),
            AppError::JwtErr(e) => (400, e.to_string()),
            AppError::VarErr(e) => (500, e.to_string()),
            AppError::LibErr(e) => (500, e.to_string()),
            // 更多错误
            // _ => (500, "internal server error".to_string()),
        };
        Self {
            code,
            message: msg,
            source_error: err,
        }
    }
    pub fn into_response(self, res: &mut Response) {
        let status_code = match self.source_error {
            AppError::AnyHow(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // AppError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // AppError::ParseErr(_) => StatusCode::BAD_REQUEST,
            // AppError::ValidateErr(_) => StatusCode::BAD_REQUEST,
            AppError::BcryptErr(_) => StatusCode::BAD_REQUEST,
            AppError::JwtErr(_) => StatusCode::BAD_REQUEST,
            AppError::VarErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::LibErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            // 更多错误
            // _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        res.stuff(status_code, Json(self));
    }
    // TODO: openapi 文档
}

//==================================================================================
//= 自定义writer结构体
//= Custom writer structure
pub struct AppWriter<T>(pub AppResult<T>);

//==================================================================================
//= 自定义Writer，实现自动通用响应
//= Implement automatic universal response for custom writer
#[async_trait]
impl<T: Serialize + Default + Send> Writer for AppWriter<T> {
    async fn write(self, req: &mut Request, depot: &mut Depot, res: &mut Response) {
        match self.0 {
            Ok(data) => ResponseBuilder::with_data(data).into_response(res),
            Err(e) => e.write(req, depot, res).await,
        }
    }
}
//= 自定义AppError的writer
//= Define custom AppError writer
#[async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        ErrorResponseBuilder::with_err(self).into_response(res)
    }
}

//==================================================================================
//= 允许AppResult<T>、AppError转换为AppWriter，方便进一步处理或者返回（也许会用到？）
//= Allow AppResult<T> and AppError to be converted to AppWriter for further processing or return (maybe?)
//= 用AppResult<>.into()使用转换
//= Use transformation with AppResult<>.into() 
impl<T> From<AppResult<T>> for AppWriter<T> {
    fn from(result: AppResult<T>) -> Self {
        AppWriter(result)
    }
}
impl<T> From<AppError> for AppWriter<T> {
    fn from(result: AppError) -> Self {
        AppWriter(Err(result))
    }
}
