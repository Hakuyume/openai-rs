use crate::{ApiError, Error};
use futures::Stream;
use http_body_util::BodyExt;
use serde::Deserialize;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[pin_project::pin_project]
pub struct EventStream<B, T>(
    #[pin]
    #[allow(clippy::type_complexity)]
    http_body_server_sent_events::Decode<
        http_body_util::combinators::MapErr<B, fn(B::Error) -> Error<Infallible, B::Error>>,
    >,
    PhantomData<fn() -> T>,
)
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>;

impl<B, T> EventStream<B, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn new(body: B) -> Self {
        Self(
            http_body_server_sent_events::decode(body.map_err(Error::Body)),
            PhantomData,
        )
    }
}

impl<B, T> Stream for EventStream<B, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    type Item = Result<T, Error<Infallible, B::Error>>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            match ready!(this.0.as_mut().poll_frame(cx)?) {
                Some(frame) => {
                    let Ok(event) = frame.into_data() else {
                        continue;
                    };
                    tracing::debug!(?event);
                    let Some(data) = event.data else { continue };
                    if data == "[DONE]" {
                        break Poll::Ready(None);
                    } else if event.event.is_some_and(|event| event == "error") {
                        let openai_openapi_types::ErrorResponse {
                            error: openai_openapi_types::Error { code, message, .. },
                        } = serde_json::from_str(&data)?;
                        Err(ApiError {
                            status: None,
                            code,
                            message,
                        })?;
                    } else {
                        break Poll::Ready(Some(Ok(serde_json::from_str(&data)?)));
                    };
                }
                None => break Poll::Ready(None),
            }
        }
    }
}
