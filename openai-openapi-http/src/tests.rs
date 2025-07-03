use std::convert::Infallible;
use std::future;

#[futures_test::test]
async fn test_get_chat_completion_messages() {
    let response = crate::get_chat_completion_messages(
        |request| {
            assert_eq!(
                request.uri().to_string(),
                "/chat/completions/foo/messages?limit=42&order=desc",
            );
            let response = http::Response::builder()
                .status(http::StatusCode::INTERNAL_SERVER_ERROR)
                .body(String::new())
                .unwrap();
            future::ready(Ok::<_, Infallible>(response))
        },
        &openai_openapi_types::GetChatCompletionMessagesParams::builder()
            .completion_id("foo".to_owned())
            .limit(Some(42))
            .order(Some(
                openai_openapi_types::GetChatCompletionMessagesParamsOrder::Desc,
            ))
            .build(),
    )
    .await;
    assert!(matches!(response, Err(crate::Error::Api(_))));
}

#[cfg(feature = "test-openai")]
mod openai {
    use futures::TryStreamExt;
    use std::env;
    use std::future::{self, Ready};
    use ureq::RequestExt;

    fn client()
    -> impl FnOnce(http::Request<String>) -> Ready<Result<http::Response<String>, ureq::Error>>
    {
        fn run(
            mut request: http::Request<String>,
            api_key: String,
        ) -> Result<http::Response<String>, ureq::Error> {
            *request.uri_mut() = format!("https://api.openai.com/v1{}", request.uri().path())
                .parse()
                .map_err(http::Error::from)?;
            request.headers_mut().insert(
                http::header::AUTHORIZATION,
                format!("Bearer {api_key}")
                    .parse()
                    .map_err(http::Error::from)?,
            );
            let response = request
                .with_default_agent()
                .configure()
                .http_status_as_error(false)
                .run()?;
            let (parts, mut body) = response.into_parts();
            let body = body.read_to_string()?;
            Ok(http::Response::from_parts(parts, body))
        }

        let api_key = env::var("OPENAI_API_KEY").unwrap();
        move |request| future::ready(run(request, api_key))
    }

    #[futures_test::test]
    async fn test_list_models() {
        let response = crate::list_models(client()).await.unwrap();
        dbg!(response);
    }

    #[futures_test::test]
    async fn test_create_chat_completion() {
        let response = crate::create_chat_completion(
            client(),
            &openai_openapi_types::CreateChatCompletionRequest::builder()
                .model(openai_openapi_types::ModelIdsShared::Gpt4_1Nano)
                .messages(vec![
                openai_openapi_types::ChatCompletionRequestMessage::User(
                    openai_openapi_types::ChatCompletionRequestUserMessage::builder()
                        .content(
                            openai_openapi_types::ChatCompletionRequestUserMessageContent::String(
                                "hello world".to_owned(),
                            ),
                        )
                        .build(),
                ),
            ])
                .build(),
        )
        .await
        .unwrap();
        dbg!(response);
    }

    #[futures_test::test]
    async fn test_create_chat_completion_stream() {
        let mut stream = crate::create_chat_completion_stream(
            client(),
            &openai_openapi_types::CreateChatCompletionRequest::builder()
                .model(openai_openapi_types::ModelIdsShared::Gpt4_1Nano)
                .messages(vec![
                openai_openapi_types::ChatCompletionRequestMessage::User(
                    openai_openapi_types::ChatCompletionRequestUserMessage::builder()
                        .content(
                            openai_openapi_types::ChatCompletionRequestUserMessageContent::String(
                                "hello world".to_owned(),
                            ),
                        )
                        .build(),
                ),
            ])
                .stream(Some(true))
                .build(),
        )
        .await
        .unwrap();
        while let Some(response) = stream.try_next().await.unwrap() {
            dbg!(response);
        }
    }
}
