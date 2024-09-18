use axum::{http::StatusCode, Json};

use crate::error::AppError;

pub type ApiResponse<T> = Result<(StatusCode, T), AppError>;

pub type ResponseBody<T> = Json<T>;

pub trait IntoApiResponse<T> {
    fn from_parts(data: T, status: Option<StatusCode>) -> (StatusCode, Json<T>);
}

impl<T> IntoApiResponse<T> for ApiResponse<T> {
    fn from_parts(data: T, status: Option<StatusCode>) -> (StatusCode, Json<T>) {
        let status_code = status.unwrap_or_default();
        (status_code, Json(data))
    }
}
