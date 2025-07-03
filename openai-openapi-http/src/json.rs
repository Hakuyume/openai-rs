use crate::Error;
use http_body_util::BodyExt;
use serde::Deserialize;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[pin_project::pin_project]
pub(crate) struct Json<B, E, T>(
    #[pin] http_body_util::combinators::Collect<B>,
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
    pub(crate) fn new(body: B) -> Self {
        Self(body.collect(), PhantomData)
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
        let body = body.to_bytes();
        tracing::debug!(?body);
        Poll::Ready(Ok(serde_json::from_slice(&body)?))
    }
}
