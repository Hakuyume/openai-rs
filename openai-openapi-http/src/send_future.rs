use crate::Error;
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http_body_util::BodyExt;
use serde::Serialize;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[pin_project::pin_project]
pub struct SendFuture<Fut, B, E>(#[pin] State<Fut, B, E>)
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body;

#[allow(clippy::large_enum_variant)]
#[pin_project::pin_project(project = StateProj)]
enum State<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    S0 {
        #[pin]
        f: Fut,
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

impl<Fut, B, E> SendFuture<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    pub(crate) fn new<S, T>(service: S, method: http::Method, path: &str, body: &T) -> Self
    where
        S: FnOnce(http::Request<String>) -> Fut,
        T: Serialize,
    {
        let request = serde_json::to_string(body)
            .map_err(Error::Json)
            .and_then(|body| {
                tracing::debug!(?body);
                http::Request::builder()
                    .method(method)
                    .uri(path)
                    .header(CONTENT_LENGTH, body.len())
                    .header(CONTENT_TYPE, "application/json")
                    .body(body)
                    .map_err(Error::Http)
            });
        match request {
            Ok(request) => Self(State::S0 {
                f: service(request),
            }),
            Err(e) => Self(State::S1 { e: Some(e) }),
        }
    }
}

impl<Fut, B, E> Future for SendFuture<Fut, B, E>
where
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    type Output = Result<B, Error<E, B::Error>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            match this.0.as_mut().project() {
                StateProj::S0 { f } => {
                    let response = ready!(f.poll(cx)).map_err(Error::Service)?;
                    let (parts, body) = response.into_parts();
                    if parts.status.is_success() {
                        break Poll::Ready(Ok(body));
                    } else {
                        this.0.set(State::S2 {
                            f: body.collect(),
                            status: parts.status,
                        });
                    }
                }
                StateProj::S1 { e } => {
                    break Poll::Ready(Err(e.take().unwrap()));
                }
                StateProj::S2 { f, status } => {
                    let body = ready!(f.poll(cx)).map_err(Error::Body)?;
                    let body = body.to_bytes();
                    tracing::debug!(?body);
                    let openai_openapi_types::ErrorResponse {
                        error: openai_openapi_types::Error { code, message, .. },
                    } = serde_json::from_slice(&body)?;
                    break Poll::Ready(Err(Error::Api(crate::ApiError {
                        status: Some(*status),
                        code,
                        message,
                    })));
                }
            }
        }
    }
}
