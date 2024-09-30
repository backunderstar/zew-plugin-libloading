use anyhow::anyhow;
use interface::writer::ErrorResponseBuilder;
use salvo::{handler, http::StatusCode, writing::Json, FlowCtrl, Response};

#[handler]
pub async fn cather_all(res: &mut Response, ctrl: &mut FlowCtrl) {
    if let Some(StatusCode::UNAUTHORIZED) = res.status_code {
        res.stuff(
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponseBuilder {
                code: 401,
                message: "Unauthorized".to_string(),
                source_error: anyhow!("Unauthorized").into(),
            }),
        );
        ctrl.skip_rest();
    }
    // if let Some(StatusCode::FORBIDDEN) = res.status_code {
    //     res.stuff(
    //         StatusCode::FORBIDDEN,
    //         Json(ErrorResponseBuilder {
    //             code: 403,
    //             message: "Forbidden".to_string(),
    //             source_error: anyhow!("Forbidden").into(),
    //         }),
    //     );
    //     ctrl.skip_rest();
    // }
    // if let Some(StatusCode::NOT_FOUND) = res.status_code {
    //     res.stuff(
    //         StatusCode::NOT_FOUND,
    //         Json(ErrorResponseBuilder {
    //             code: 404,
    //             message: "Not Found".to_string(),
    //             source_error: anyhow!("Not Found").into(),
    //         }),
    //     );
    //     ctrl.skip_rest();
    // }
}
