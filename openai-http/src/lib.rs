use futures::{Stream, TryFutureExt, TryStreamExt};
use http::header::{CONTENT_LENGTH, CONTENT_TYPE};
use http_body_util::BodyExt;
use serde::{Deserialize, Serialize};
use std::future;

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
    Api(ApiError),
}

#[derive(Debug, thiserror::Error)]
#[error("{}", message)]
pub struct ApiError {
    pub status: Option<http::StatusCode>,
    pub code: Option<String>,
    pub message: String,
}

pub trait StreamRequest: Serialize {
    const PATH: &'static str;
    type Response: for<'de> Deserialize<'de>;
    fn enable_stream(self) -> Self;
}

pub fn call_stream<F, B, E, R>(
    service: F,
    request: R,
) -> impl Stream<Item = Result<R::Response, Error<E, B::Error>>>
where
    F: AsyncFnOnce(http::Request<String>) -> Result<http::Response<B>, E>,
    B: http_body::Body,
    R: StreamRequest,
{
    let request = serde_json::to_string(&request.enable_stream()).map(|body| {
        tracing::debug!(?body);
        http::Request::post(R::PATH)
            .header(CONTENT_LENGTH, body.len())
            .header(CONTENT_TYPE, "application/json")
            .body(body)
    });
    async move {
        let response = service(request??).map_err(Error::Service).await?;
        let (part, body) = response.into_parts();
        let body = body.map_err(Error::Body);
        if part.status.is_success() {
            Ok(http_body_server_sent_events::decode(body).into_event_stream())
        } else {
            let body = body.collect().await?.to_bytes();
            tracing::debug!(?body);
            let openai_openapi_types::ErrorResponse {
                error: openai_openapi_types::Error { code, message, .. },
            } = serde_json::from_slice(&body)?;
            Err(Error::Api(ApiError {
                status: Some(part.status),
                code,
                message,
            }))
        }
    }
    .try_flatten_stream()
    .inspect_ok(|event| tracing::debug!(?event))
    .try_take_while(|event| futures::future::ok(event.data.as_deref() != Some("[DONE]")))
    .try_filter_map(|event| futures::future::ok(event.data.map(|data| (event.event, data))))
    .and_then(|(event, data)| {
        future::ready(if event.as_deref() == Some("error") {
            match serde_json::from_str(&data) {
                Ok(openai_openapi_types::ErrorResponse {
                    error: openai_openapi_types::Error { code, message, .. },
                }) => Err(Error::Api(ApiError {
                    status: None,
                    code,
                    message,
                })),
                Err(e) => Err(Error::Json(e)),
            }
        } else {
            serde_json::from_str(&data).map_err(Error::Json)
        })
    })
}

impl StreamRequest for openai_openapi_types::CreateChatCompletionRequest {
    const PATH: &'static str = "/chat/completions";
    type Response = openai_openapi_types::CreateChatCompletionStreamResponse;
    fn enable_stream(mut self) -> Self {
        self.stream = Some(true);
        self
    }
}

impl StreamRequest for openai_openapi_types::CreateResponse {
    const PATH: &'static str = "/responses";
    type Response = openai_openapi_types::ResponseStreamEvent;
    fn enable_stream(mut self) -> Self {
        self.stream = Some(true);
        self
    }
}
