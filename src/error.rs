use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct Error(pub anyhow::Error);

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("{:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error(err)
    }
}
pub type Result<T> = core::result::Result<T, Error>;
