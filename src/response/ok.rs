use axum::{response::{IntoResponse, Response}, http::StatusCode};

pub struct KvReadOk(pub String);

impl IntoResponse for KvReadOk {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.0).into_response()
    }
}

pub struct KvWriteOk(pub String);

impl IntoResponse for KvWriteOk {
    fn into_response(self) -> Response {
        (StatusCode::OK, self.0).into_response()
    }
}

