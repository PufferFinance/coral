mod error_kind;
mod error_type;

use axum::body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use serde::{Deserialize, Serialize};

pub use self::error_kind::AppErrorKind;
pub use self::error_type::AppError;

pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppServerResult<T> = Result<T, ServerErrorResponse>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerSuccessBody<T: Serialize> {
    pub success: bool,
    pub result: T,
}

impl<T: Serialize> ServerSuccessBody<T> {
    pub fn new(result: T) -> Self {
        Self {
            success: true,
            result,
        }
    }
}

impl<T: Serialize> IntoResponse for ServerSuccessBody<T> {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header("Content-type", "application/json")
            .status(StatusCode::ACCEPTED)
            .body(body::Body::new(json_body));
        response.unwrap()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorBody {
    pub success: bool,
    pub result: ServerErrorResult,
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorResult {
    pub error_code: usize,
    pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ServerErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub response: ServerErrorBody,
}

impl std::fmt::Display for ServerErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.response.result.message)
    }
}

impl ServerErrorResponse {
    pub fn new(status_code: StatusCode, error_code: usize, message: &str) -> Self {
        Self {
            status_code,
            response: ServerErrorBody {
                success: false,
                result: ServerErrorResult {
                    error_code,
                    message: message.to_owned(),
                },
            },
        }
    }
}

impl IntoResponse for ServerErrorResponse {
    fn into_response(self) -> Response<body::Body> {
        let json_body = serde_json::to_string(&self.response).unwrap_or("{}".to_owned());
        let response = Response::builder()
            .header("Content-type", "application/json")
            .status(self.status_code)
            .body(body::Body::new(json_body));
        response.unwrap()
    }
}
