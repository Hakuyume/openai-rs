use crate::Error;
use http_body_util::BodyExt;
use serde::Deserialize;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[pin_project::pin_project]
pub(crate) struct Json<B, E, T>(
    #[pin] http_body_util::combinators::Collect<B>,
    Option<http::response::Parts>,
    PhantomData<fn() -> (E, T)>,
)
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>;

impl<B, E, T> Json<B, E, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn new(response: http::Response<B>) -> Self {
        let (parts, body) = response.into_parts();
        Self(body.collect(), Some(parts), PhantomData)
    }
}

impl<B, E, T> Future for Json<B, E, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    type Output = Result<T, Error<E, B::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let body = ready!(this.0.poll(cx)).map_err(Error::Body)?;
        let response = http::Response::from_parts(this.1.take().unwrap(), body.to_bytes());
        tracing::debug!(?response);
        Poll::Ready(Ok(serde_json::from_slice(response.body())?))
    }
}
