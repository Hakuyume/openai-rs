use crate::{ApiError, Error};
use bytes::Bytes;
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
        expected: (http::StatusCode, Option<mime::Mime>),
    },
    S1 {
        e: Option<Error<E, B::Error>>,
    },
    S2 {
        #[pin]
        f: http_body_util::combinators::Collect<B>,
        parts: Option<http::response::Parts>,
    },
}

impl<Fut, B, E> Send<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    pub(crate) fn new<S, R>(
        service: S,
        request: R,
        expected: (http::StatusCode, Option<mime::Mime>),
    ) -> Self
    where
        S: FnOnce(http::Request<String>) -> Fut,
        R: FnOnce() -> Result<http::Request<String>, Error<E, B::Error>>,
    {
        match request() {
            Ok(request) => {
                tracing::debug!(?request);
                Self::S0 {
                    f: service(request),
                    expected,
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
    type Output = Result<http::Response<B>, Error<E, B::Error>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            match self.as_mut().project() {
                SendProj::S0 { f, expected } => {
                    let response = ready!(f.poll(cx)).map_err(Error::Service)?;
                    let content_type = response
                        .headers()
                        .get(http::header::CONTENT_TYPE)
                        .and_then(|value| value.to_str().ok()?.parse::<mime::Mime>().ok());
                    if response.status() == expected.0
                        && expected.1.as_ref().is_some_and(|expected| {
                            content_type.as_ref().map(mime::Mime::essence_str)
                                == Some(expected.essence_str())
                        })
                    {
                        break Poll::Ready(Ok(response));
                    } else if response.status() != expected.0
                        && content_type == Some(mime::APPLICATION_JSON)
                    {
                        let (parts, body) = response.into_parts();
                        self.set(Self::S2 {
                            f: body.collect(),
                            parts: Some(parts),
                        });
                    } else {
                        let (parts, _) = response.into_parts();
                        tracing::debug!(response.parts = ?parts);
                        break Poll::Ready(Err(Error::Api(ApiError {
                            code: None,
                            message: None,
                            response: Some(http::Response::from_parts(parts, Bytes::new())),
                        })));
                    }
                }
                SendProj::S1 { e } => {
                    break Poll::Ready(Err(e.take().unwrap()));
                }
                SendProj::S2 { f, parts } => {
                    let body = ready!(f.poll(cx)).map_err(Error::Body)?;
                    let response =
                        http::Response::from_parts(parts.take().unwrap(), body.to_bytes());
                    tracing::debug!(?response);
                    let openai_openapi_types::ErrorResponse {
                        error: openai_openapi_types::Error { code, message, .. },
                    } = serde_json::from_slice(response.body())?;
                    break Poll::Ready(Err(Error::Api(ApiError {
                        code,
                        message: Some(message),
                        response: Some(response),
                    })));
                }
            }
        }
    }
}
