use axum::{response::{IntoResponse, Response}, http::StatusCode};

use crate::util::MAX_PAYLOAD_LEN;

pub enum KvReadError {
    NotFound { key: String },
}

impl IntoResponse for KvReadError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            KvReadError::NotFound { key } => (
                StatusCode::NOT_FOUND,
                format!("Couldn't find a value with key: {}", key),
            ),
        };

        (status_code, message).into_response()
    }
}

pub enum KvWriteError {
    SizeTooBig(usize),
}

impl IntoResponse for KvWriteError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            KvWriteError::SizeTooBig(size) => (
                StatusCode::BAD_REQUEST,
                format!(
                    "Your payload is {} long, but the maximum amount is {}",
                    size, MAX_PAYLOAD_LEN
                ),
            ),
        };

        (status_code, message).into_response()
    }
}
