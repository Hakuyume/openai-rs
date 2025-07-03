#[cfg(feature = "test-openai")]
fn service()
-> impl FnOnce(http::Request<String>) -> std::future::Ready<Result<http::Response<String>, ureq::Error>>
{
    use std::env;
    use std::future;
    use ureq::RequestExt;

    fn run(mut request: http::Request<String>) -> Result<http::Response<String>, ureq::Error> {
        let api_key = env::var("OPENAI_API_KEY").unwrap();
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

    |request| future::ready(run(request))
}

#[cfg(feature = "test-openai")]
#[futures_test::test]
async fn test_list_models() {
    let response = super::list_models(service()).await.unwrap();
    dbg!(response);
}

#[cfg(feature = "test-openai")]
#[futures_test::test]
async fn test_create_chat_completion() {
    let response = super::create_chat_completion(
        service(),
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

#[cfg(feature = "test-openai")]
#[futures_test::test]
async fn test_create_chat_completion_stream() {
    use futures::TryStreamExt;

    let mut stream = super::create_chat_completion_stream(
        service(),
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
