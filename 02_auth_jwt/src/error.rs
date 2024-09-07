use axum::{http::StatusCode, response::IntoResponse};

pub enum AppError {
    InvalidPortBinding,
    JwtError,
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         match self {
//             AppError::InvalidPortBinding => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "Unfortunately your request could not be handled at this time",
//             ),
//         }
//     }
// }
