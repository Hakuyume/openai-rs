use crate::{ApiError, Error};
use http_body_util::BodyExt;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[allow(clippy::large_enum_variant)]
#[pin_project::pin_project(project = SendProj)]
pub(crate) enum Send<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    S0 {
        #[pin]
        f: Fut,
        status: http::StatusCode,
    },
    S1 {
        e: Option<Error<E, B::Error>>,
    },
    S2 {
        #[pin]
        f: http_body_util::combinators::Collect<B>,
        status: http::StatusCode,
    },
}

impl<Fut, B, E> Send<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    pub(crate) fn new<S, R>(service: S, request: R, status: http::StatusCode) -> Self
    where
        S: FnOnce(http::Request<String>) -> Fut,
        R: FnOnce() -> Result<http::Request<String>, Error<E, B::Error>>,
    {
        match request() {
            Ok(request) => {
                let (parts, body) = request.into_parts();
                tracing::debug!(request.parts = ?parts, request.body = ?body);
                Self::S0 {
                    f: service(http::Request::from_parts(parts, body)),
                    status,
                }
            }
            Err(e) => Self::S1 { e: Some(e) },
        }
    }
}

impl<Fut, B, E> Future for Send<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    type Output = Result<B, Error<E, B::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                SendProj::S0 { f, status } => {
                    let response = ready!(f.poll(cx)).map_err(Error::Service)?;
                    let (parts, body) = response.into_parts();
                    if parts.status == *status {
                        break Poll::Ready(Ok(body));
                    } else {
                        self.set(Self::S2 {
                            f: body.collect(),
                            status: parts.status,
                        });
                    }
                }
                SendProj::S1 { e } => {
                    break Poll::Ready(Err(e.take().unwrap()));
                }
                SendProj::S2 { f, status } => {
                    let body = ready!(f.poll(cx)).map_err(Error::Body)?;
                    let body = body.to_bytes();
                    tracing::debug!(?body);
                    let openai_openapi_types::ErrorResponse {
                        error: openai_openapi_types::Error { code, message, .. },
                    } = serde_json::from_slice(&body)?;
                    Err(ApiError {
                        status: Some(*status),
                        code,
                        message,
                    })?;
                }
            }
        }
    }
}
