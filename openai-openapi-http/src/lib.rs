mod event_stream;
mod json;
mod send;

mod __combinators {
    pub(super) use crate::event_stream::EventStream;
    pub(super) use crate::json::Json;
    pub(super) use crate::send::Send;
}
pub use event_stream::EventStream;
pub use generated::*;
use openai_openapi_types as __types;

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
    Urlencode(#[from] serde_urlencoded::ser::Error),
    #[error(transparent)]
    Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)]
    Api(#[from] ApiError),
}

impl<S, B> Error<S, B> {
    pub fn boxed(self) -> Box<dyn std::error::Error + Send + Sync + 'static>
    where
        S: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
        B: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
    {
        match self {
            Self::Service(e) => e.into(),
            Self::Body(e) => e.into(),
            Self::Http(e) => e.into(),
            Self::Json(e) => e.into(),
            Self::Urlencode(e) => e.into(),
            Self::Utf8(e) => e.into(),
            Self::Api(e) => e.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{}", message)]
pub struct ApiError {
    pub status: Option<http::StatusCode>,
    pub code: Option<String>,
    pub message: String,
}

macro_rules! future {
    ($ident:ident, $fut:ty, $output:ty) => {
        #[pin_project::pin_project]
        pub struct $ident<Fut, B, E>(
            #[pin]
            #[allow(clippy::type_complexity)]
            $fut,
        )
        where
            Fut: Future<Output = Result<http::Response<B>, E>>,
            B: http_body::Body;

        impl<Fut, B, E> Future for $ident<Fut, B, E>
        where
            Fut: Future<Output = Result<http::Response<B>, E>>,
            B: http_body::Body,
        {
            type Output = Result<$output, crate::Error<E, B::Error>>;
            fn poll(
                self: std::pin::Pin<&mut Self>,
                cx: &mut std::task::Context<'_>,
            ) -> std::task::Poll<Self::Output> {
                self.project().0.poll(cx)
            }
        }
    };
}

mod generated;
