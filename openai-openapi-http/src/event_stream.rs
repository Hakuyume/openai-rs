use crate::{ApiError, Error};
use futures::Stream;
use http_body_util::BodyExt;
use serde::Deserialize;
use std::convert::Infallible;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll, ready};

#[pin_project::pin_project]
pub struct EventStream<B, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    #[pin]
    decode: Option<http_body_server_sent_events::Decode<Body<B, B::Error>>>,
    _phantom: PhantomData<fn() -> T>,
}
type Body<B, E> = http_body_util::combinators::MapErr<B, fn(E) -> Error<Infallible, E>>;

impl<B, T> EventStream<B, T>
where
    B: http_body::Body,
    T: for<'de> Deserialize<'de>,
{
    pub(crate) fn new(body: B) -> Self {
        Self {
            decode: Some(http_body_server_sent_events::decode(
                body.map_err(Error::Body),
            )),
            _phantom: PhantomData,
        }
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
            let Some(decode) = this.decode.as_mut().as_pin_mut() else {
                break Poll::Ready(None);
            };
            match ready!(decode.poll_frame(cx)?) {
                Some(frame) => {
                    let Ok(event) = frame.into_data() else {
                        continue;
                    };
                    tracing::debug!(?event);
                    let Some(data) = event.data else { continue };
                    if data == "[DONE]" {
                        this.decode.set(None);
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
