mod event_stream;
mod send_future;

pub use event_stream::EventStream;
use openai_openapi_types as types;
pub use send_future::SendFuture;

#[derive(Debug, thiserror::Error)]
pub enum Error<S, B> {
    #[error(transparent)]
    Service(S),
    #[error(transparent)]
    Body(B),
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Api(#[from] ApiError),
}

#[derive(Debug, thiserror::Error)]
#[error("{}", message)]
pub struct ApiError {
    pub status: Option<http::StatusCode>,
    pub code: Option<String>,
    pub message: String,
}

mod generated;
pub use generated::*;
