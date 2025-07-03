#[doc = "Returns a list of assistants."]
pub fn list_assistants<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListAssistantsParams,
) -> ListAssistants<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListAssistants(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListAssistantsParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/assistants",);
                    let crate::__types::ListAssistantsParams {
                        limit,
                        order,
                        after,
                        before,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListAssistants,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListAssistantsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListAssistantsResponse>,
    >,
    crate::__types::ListAssistantsResponse
);
#[doc = "Create an assistant with a model and instructions."]
pub fn create_assistant<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateAssistantRequest,
) -> CreateAssistant<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/assistants";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Retrieves an assistant."]
pub fn get_assistant<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetAssistantParams,
) -> GetAssistant<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/assistants/{assistant_id}",
                        assistant_id = params.assistant_id
                    );
                    let crate::__types::GetAssistantParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Modifies an assistant."]
pub fn modify_assistant<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyAssistantParams,
    request: &crate::__types::ModifyAssistantRequest,
) -> ModifyAssistant<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/assistants/{assistant_id}",
                        assistant_id = params.assistant_id
                    );
                    let crate::__types::ModifyAssistantParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Delete an assistant."]
pub fn delete_assistant<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteAssistantParams,
) -> DeleteAssistant<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/assistants/{assistant_id}",
                        assistant_id = params.assistant_id
                    );
                    let crate::__types::DeleteAssistantParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteAssistantResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteAssistantResponse>,
    >,
    crate::__types::DeleteAssistantResponse
);
#[doc = "Generates audio from the input text."]
pub fn create_speech_stream<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateSpeechRequest,
) -> CreateSpeechStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateSpeechStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/audio/speech";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateSpeechStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (B) -> crate :: __combinators :: EventStream < B , crate :: __types :: CreateSpeechResponseStreamEvent > , > , crate :: __combinators :: EventStream < B , crate :: __types :: CreateSpeechResponseStreamEvent >);
#[doc = "Creates and executes a batch from an uploaded file of requests"]
pub fn create_batch<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateBatchRequest,
) -> CreateBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/batches";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "List your organization's batches."]
pub fn list_batches<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListBatchesParams,
) -> ListBatches<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListBatches(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/batches",);
                    let crate::__types::ListBatchesParams { after, limit, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListBatches,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListBatchesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListBatchesResponse>,
    >,
    crate::__types::ListBatchesResponse
);
#[doc = "Retrieves a batch."]
pub fn retrieve_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveBatchParams,
) -> RetrieveBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/batches/{batch_id}", batch_id = params.batch_id);
                    let crate::__types::RetrieveBatchParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "Cancels an in-progress batch. The batch will be in status `cancelling` for up to 10 minutes, before changing to `cancelled`, where it will have partial results (if any) available in the output file."]
pub fn cancel_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelBatchParams,
) -> CancelBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/batches/{batch_id}/cancel", batch_id = params.batch_id);
                    let crate::__types::CancelBatchParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "List stored Chat Completions. Only Chat Completions that have been stored\nwith the `store` parameter set to `true` will be returned.\n"]
pub fn list_chat_completions<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListChatCompletionsParams,
) -> ListChatCompletions<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListChatCompletions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "model")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        model: &'a Option<String>,
                        #[serde(rename = "metadata")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        metadata: &'a Option<crate::__types::Metadata>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListChatCompletionsParamsOrder>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions",);
                    let crate::__types::ListChatCompletionsParams {
                        model,
                        metadata,
                        after,
                        limit,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        model,
                        metadata,
                        after,
                        limit,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListChatCompletions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionList>,
    >,
    crate::__types::ChatCompletionList
);
#[doc = "**Starting a new project?** We recommend trying [Responses](https://platform.openai.com/docs/api-reference/responses) \nto take advantage of the latest OpenAI platform features. Compare\n[Chat Completions with Responses](https://platform.openai.com/docs/guides/responses-vs-chat-completions?api-mode=responses).\n\n---\n\nCreates a model response for the given chat conversation. Learn more in the\n[text generation](https://platform.openai.com/docs/guides/text-generation), [vision](https://platform.openai.com/docs/guides/vision),\nand [audio](https://platform.openai.com/docs/guides/audio) guides.\n\nParameter support can differ depending on the model used to generate the\nresponse, particularly for newer reasoning models. Parameters that are only\nsupported for reasoning models are noted below. For the current state of \nunsupported parameters in reasoning models, \n[refer to the reasoning guide](https://platform.openai.com/docs/guides/reasoning).\n"]
pub fn create_chat_completion<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateChatCompletionRequest,
) -> CreateChatCompletion<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/chat/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "**Starting a new project?** We recommend trying [Responses](https://platform.openai.com/docs/api-reference/responses) \nto take advantage of the latest OpenAI platform features. Compare\n[Chat Completions with Responses](https://platform.openai.com/docs/guides/responses-vs-chat-completions?api-mode=responses).\n\n---\n\nCreates a model response for the given chat conversation. Learn more in the\n[text generation](https://platform.openai.com/docs/guides/text-generation), [vision](https://platform.openai.com/docs/guides/vision),\nand [audio](https://platform.openai.com/docs/guides/audio) guides.\n\nParameter support can differ depending on the model used to generate the\nresponse, particularly for newer reasoning models. Parameters that are only\nsupported for reasoning models are noted below. For the current state of \nunsupported parameters in reasoning models, \n[refer to the reasoning guide](https://platform.openai.com/docs/guides/reasoning).\n"]
pub fn create_chat_completion_stream<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateChatCompletionRequest,
) -> CreateChatCompletionStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateChatCompletionStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/chat/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateChatCompletionStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (B) -> crate :: __combinators :: EventStream < B , crate :: __types :: CreateChatCompletionStreamResponse > , > , crate :: __combinators :: EventStream < B , crate :: __types :: CreateChatCompletionStreamResponse >);
#[doc = "Get a stored chat completion. Only Chat Completions that have been created\nwith the `store` parameter set to `true` will be returned.\n"]
pub fn get_chat_completion<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetChatCompletionParams,
) -> GetChatCompletion<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/chat/completions/{completion_id}",
                        completion_id = params.completion_id
                    );
                    let crate::__types::GetChatCompletionParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "Modify a stored chat completion. Only Chat Completions that have been\ncreated with the `store` parameter set to `true` can be modified. Currently,\nthe only supported modification is to update the `metadata` field.\n"]
pub fn update_chat_completion<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UpdateChatCompletionParams,
    request: &crate::__types::UpdateChatCompletionRequest,
) -> UpdateChatCompletion<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/chat/completions/{completion_id}",
                        completion_id = params.completion_id
                    );
                    let crate::__types::UpdateChatCompletionParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "Delete a stored chat completion. Only Chat Completions that have been\ncreated with the `store` parameter set to `true` can be deleted.\n"]
pub fn delete_chat_completion<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteChatCompletionParams,
) -> DeleteChatCompletion<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/chat/completions/{completion_id}",
                        completion_id = params.completion_id
                    );
                    let crate::__types::DeleteChatCompletionParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionDeleted>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionDeleted>,
    >,
    crate::__types::ChatCompletionDeleted
);
#[doc = "Get the messages in a stored chat completion. Only Chat Completions that\nhave been created with the `store` parameter set to `true` will be\nreturned.\n"]
pub fn get_chat_completion_messages<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetChatCompletionMessagesParams,
) -> GetChatCompletionMessages<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetChatCompletionMessages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::GetChatCompletionMessagesParamsOrder>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/chat/completions/{completion_id}/messages",
                        completion_id = params.completion_id
                    );
                    let crate::__types::GetChatCompletionMessagesParams {
                        after,
                        limit,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetChatCompletionMessages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionMessageList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionMessageList>,
    >,
    crate::__types::ChatCompletionMessageList
);
#[doc = "Creates a completion for the provided prompt and parameters."]
pub fn create_completion<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateCompletionRequest,
) -> CreateCompletion<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateCompletionResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateCompletionResponse>,
    >,
    crate::__types::CreateCompletionResponse
);
#[doc = "List Containers"]
pub fn list_containers<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListContainersParams,
) -> ListContainers<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListContainers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListContainersParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers",);
                    let crate::__types::ListContainersParams {
                        limit,
                        order,
                        after,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListContainers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerListResource>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ContainerListResource>,
    >,
    crate::__types::ContainerListResource
);
#[doc = "Retrieve Container"]
pub fn retrieve_container<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveContainerParams,
) -> RetrieveContainer<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveContainer(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/containers/{container_id}",
                        container_id = params.container_id
                    );
                    let crate::__types::RetrieveContainerParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveContainer,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerResource>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ContainerResource>,
    >,
    crate::__types::ContainerResource
);
#[doc = "List Container files"]
pub fn list_container_files<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListContainerFilesParams,
) -> ListContainerFiles<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListContainerFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListContainerFilesParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/containers/{container_id}/files",
                        container_id = params.container_id
                    );
                    let crate::__types::ListContainerFilesParams {
                        limit,
                        order,
                        after,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListContainerFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerFileListResource>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ContainerFileListResource>,
    >,
    crate::__types::ContainerFileListResource
);
#[doc = "Retrieve Container File"]
pub fn retrieve_container_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveContainerFileParams,
) -> RetrieveContainerFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveContainerFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/containers/{container_id}/files/{file_id}",
                        container_id = params.container_id,
                        file_id = params.file_id
                    );
                    let crate::__types::RetrieveContainerFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveContainerFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerFileResource>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ContainerFileResource>,
    >,
    crate::__types::ContainerFileResource
);
#[doc = "Creates an embedding vector representing the input text."]
pub fn create_embedding<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateEmbeddingRequest,
) -> CreateEmbedding<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEmbedding(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/embeddings";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEmbedding,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateEmbeddingResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateEmbeddingResponse>,
    >,
    crate::__types::CreateEmbeddingResponse
);
#[doc = "List evaluations for a project.\n"]
pub fn list_evals<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListEvalsParams,
) -> ListEvals<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListEvals(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListEvalsParamsOrder>,
                        #[serde(rename = "order_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order_by: &'a Option<crate::__types::ListEvalsParamsOrderBy>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals",);
                    let crate::__types::ListEvalsParams {
                        after,
                        limit,
                        order,
                        order_by,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        order,
                        order_by,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListEvals,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalList>,
    >,
    crate::__types::EvalList
);
#[doc = "Create the structure of an evaluation that can be used to test a model's performance.\nAn evaluation is a set of testing criteria and the config for a data source, which dictates the schema of the data used in the evaluation. After creating an evaluation, you can run it on different models and model parameters. We support several types of graders and datasources.\nFor more information, see the [Evals guide](https://platform.openai.com/docs/guides/evals).\n"]
pub fn create_eval<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateEvalRequest,
) -> CreateEval<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/evals";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(201u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Get an evaluation by ID.\n"]
pub fn get_eval<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetEvalParams,
) -> GetEval<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}", eval_id = params.eval_id);
                    let crate::__types::GetEvalParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Update certain properties of an evaluation.\n"]
pub fn update_eval<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UpdateEvalParams,
    request: &crate::__types::UpdateEvalRequest,
) -> UpdateEval<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}", eval_id = params.eval_id);
                    let crate::__types::UpdateEvalParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Delete an evaluation.\n"]
pub fn delete_eval<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteEvalParams,
) -> DeleteEval<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}", eval_id = params.eval_id);
                    let crate::__types::DeleteEvalParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteEvalResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteEvalResponse>,
    >,
    crate::__types::DeleteEvalResponse
);
#[doc = "Get a list of runs for an evaluation.\n"]
pub fn get_eval_runs<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetEvalRunsParams,
) -> GetEvalRuns<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRuns(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::GetEvalRunsParamsOrder>,
                        #[serde(rename = "status")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        status: &'a Option<crate::__types::GetEvalRunsParamsStatus>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs", eval_id = params.eval_id);
                    let crate::__types::GetEvalRunsParams {
                        after,
                        limit,
                        order,
                        status,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        order,
                        status,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRuns,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRunList>,
    >,
    crate::__types::EvalRunList
);
#[doc = "Kicks off a new run for a given evaluation, specifying the data source, and what model configuration to use to test. The datasource will be validated against the schema specified in the config of the evaluation.\n"]
pub fn create_eval_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateEvalRunParams,
    request: &crate::__types::CreateEvalRunRequest,
) -> CreateEvalRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs", eval_id = params.eval_id);
                    let crate::__types::CreateEvalRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(201u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Get an evaluation run by ID.\n"]
pub fn get_eval_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetEvalRunParams,
) -> GetEvalRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/evals/{eval_id}/runs/{run_id}",
                        eval_id = params.eval_id,
                        run_id = params.run_id
                    );
                    let crate::__types::GetEvalRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Cancel an ongoing evaluation run.\n"]
pub fn cancel_eval_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelEvalRunParams,
) -> CancelEvalRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/evals/{eval_id}/runs/{run_id}",
                        eval_id = params.eval_id,
                        run_id = params.run_id
                    );
                    let crate::__types::CancelEvalRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Delete an eval run.\n"]
pub fn delete_eval_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteEvalRunParams,
) -> DeleteEvalRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/evals/{eval_id}/runs/{run_id}",
                        eval_id = params.eval_id,
                        run_id = params.run_id
                    );
                    let crate::__types::DeleteEvalRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteEvalRunResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteEvalRunResponse>,
    >,
    crate::__types::DeleteEvalRunResponse
);
#[doc = "Get a list of output items for an evaluation run.\n"]
pub fn get_eval_run_output_items<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetEvalRunOutputItemsParams,
) -> GetEvalRunOutputItems<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRunOutputItems(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "status")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        status: &'a Option<crate::__types::GetEvalRunOutputItemsParamsStatus>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::GetEvalRunOutputItemsParamsOrder>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/evals/{eval_id}/runs/{run_id}/output_items",
                        eval_id = params.eval_id,
                        run_id = params.run_id
                    );
                    let crate::__types::GetEvalRunOutputItemsParams {
                        after,
                        limit,
                        status,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        status,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRunOutputItems,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItemList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItemList>,
    >,
    crate::__types::EvalRunOutputItemList
);
#[doc = "Get an evaluation run output item by ID.\n"]
pub fn get_eval_run_output_item<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetEvalRunOutputItemParams,
) -> GetEvalRunOutputItem<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRunOutputItem(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/evals/{eval_id}/runs/{run_id}/output_items/{output_item_id}",
                        eval_id = params.eval_id,
                        run_id = params.run_id,
                        output_item_id = params.output_item_id
                    );
                    let crate::__types::GetEvalRunOutputItemParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRunOutputItem,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItem>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItem>,
    >,
    crate::__types::EvalRunOutputItem
);
#[doc = "Returns a list of files."]
pub fn list_files<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListFilesParams,
) -> ListFiles<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "purpose")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        purpose: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListFilesParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files",);
                    let crate::__types::ListFilesParams {
                        purpose,
                        limit,
                        order,
                        after,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        purpose,
                        limit,
                        order,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListFilesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListFilesResponse>,
    >,
    crate::__types::ListFilesResponse
);
#[doc = "Delete a file."]
pub fn delete_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteFileParams,
) -> DeleteFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}", file_id = params.file_id);
                    let crate::__types::DeleteFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteFileResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteFileResponse>,
    >,
    crate::__types::DeleteFileResponse
);
#[doc = "Returns information about a specific file."]
pub fn retrieve_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveFileParams,
) -> RetrieveFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}", file_id = params.file_id);
                    let crate::__types::RetrieveFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::OpenAiFile>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::OpenAiFile>,
    >,
    crate::__types::OpenAiFile
);
#[doc = "Returns the contents of the specified file."]
pub fn download_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DownloadFileParams,
) -> DownloadFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DownloadFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}/content", file_id = params.file_id);
                    let crate::__types::DownloadFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DownloadFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, String>,
        fn(B) -> crate::__combinators::Json<B, E, String>,
    >,
    String
);
#[doc = "Run a grader.\n"]
pub fn run_grader<S, Fut, B, E>(
    service: S,
    request: &crate::__types::RunGraderRequest,
) -> RunGrader<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RunGrader(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/fine_tuning/alpha/graders/run";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RunGrader,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunGraderResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunGraderResponse>,
    >,
    crate::__types::RunGraderResponse
);
#[doc = "Validate a grader.\n"]
pub fn validate_grader<S, Fut, B, E>(
    service: S,
    request: &crate::__types::ValidateGraderRequest,
) -> ValidateGrader<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ValidateGrader(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/fine_tuning/alpha/graders/validate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ValidateGrader,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ValidateGraderResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ValidateGraderResponse>,
    >,
    crate::__types::ValidateGraderResponse
);
#[doc = "**NOTE:** This endpoint requires an [admin API key](../admin-api-keys).\n\nOrganization owners can use this endpoint to view all permissions for a fine-tuned model checkpoint.\n"]
pub fn list_fine_tuning_checkpoint_permissions<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListFineTuningCheckpointPermissionsParams,
) -> ListFineTuningCheckpointPermissions<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningCheckpointPermissions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "project_id")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_id: &'a Option<String>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<
                            crate::__types::ListFineTuningCheckpointPermissionsParamsOrder,
                        >,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions",
                        fine_tuned_model_checkpoint = params.fine_tuned_model_checkpoint
                    );
                    let crate::__types::ListFineTuningCheckpointPermissionsParams {
                        project_id,
                        after,
                        limit,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        project_id,
                        after,
                        limit,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFineTuningCheckpointPermissions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
        fn(
            B,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::ListFineTuningCheckpointPermissionResponse
);
#[doc = "**NOTE:** Calling this endpoint requires an [admin API key](../admin-api-keys).\n\nThis enables organization owners to share fine-tuned models with other projects in their organization.\n"]
pub fn create_fine_tuning_checkpoint_permission<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateFineTuningCheckpointPermissionParams,
    request: &crate::__types::CreateFineTuningCheckpointPermissionRequest,
) -> CreateFineTuningCheckpointPermission<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateFineTuningCheckpointPermission(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions",
                        fine_tuned_model_checkpoint = params.fine_tuned_model_checkpoint
                    );
                    let crate::__types::CreateFineTuningCheckpointPermissionParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateFineTuningCheckpointPermission,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
        fn(
            B,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::ListFineTuningCheckpointPermissionResponse
);
#[doc = "**NOTE:** This endpoint requires an [admin API key](../admin-api-keys).\n\nOrganization owners can use this endpoint to delete a permission for a fine-tuned model checkpoint.\n"]
pub fn delete_fine_tuning_checkpoint_permission<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteFineTuningCheckpointPermissionParams,
) -> DeleteFineTuningCheckpointPermission<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteFineTuningCheckpointPermission(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id}",
                        fine_tuned_model_checkpoint = params.fine_tuned_model_checkpoint,
                        permission_id = params.permission_id
                    );
                    let crate::__types::DeleteFineTuningCheckpointPermissionParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteFineTuningCheckpointPermission,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<
            B,
            E,
            crate::__types::DeleteFineTuningCheckpointPermissionResponse,
        >,
        fn(
            B,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::DeleteFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::DeleteFineTuningCheckpointPermissionResponse
);
#[doc = "Creates a fine-tuning job which begins the process of creating a new model from a given dataset.\n\nResponse includes details of the enqueued job including job status and the name of the fine-tuned models once complete.\n\n[Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization)\n"]
pub fn create_fine_tuning_job<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateFineTuningJobRequest,
) -> CreateFineTuningJob<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/fine_tuning/jobs";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "List your organization's fine-tuning jobs\n"]
pub fn list_paginated_fine_tuning_jobs<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListPaginatedFineTuningJobsParams,
) -> ListPaginatedFineTuningJobs<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListPaginatedFineTuningJobs(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "metadata")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        metadata: &'a Option<indexmap::IndexMap<String, String>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs",);
                    let crate::__types::ListPaginatedFineTuningJobsParams {
                        after,
                        limit,
                        metadata,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        metadata,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListPaginatedFineTuningJobs,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListPaginatedFineTuningJobsResponse>,
        fn(
            B,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ListPaginatedFineTuningJobsResponse>,
    >,
    crate::__types::ListPaginatedFineTuningJobsResponse
);
#[doc = "Get info about a fine-tuning job.\n\n[Learn more about fine-tuning](https://platform.openai.com/docs/guides/model-optimization)\n"]
pub fn retrieve_fine_tuning_job<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveFineTuningJobParams,
) -> RetrieveFineTuningJob<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::RetrieveFineTuningJobParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Immediately cancel a fine-tune job.\n"]
pub fn cancel_fine_tuning_job<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelFineTuningJobParams,
) -> CancelFineTuningJob<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}/cancel",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::CancelFineTuningJobParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "List checkpoints for a fine-tuning job.\n"]
pub fn list_fine_tuning_job_checkpoints<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListFineTuningJobCheckpointsParams,
) -> ListFineTuningJobCheckpoints<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningJobCheckpoints(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::ListFineTuningJobCheckpointsParams { after, limit, .. } =
                        params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFineTuningJobCheckpoints,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListFineTuningJobCheckpointsResponse>,
        fn(
            B,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningJobCheckpointsResponse,
        >,
    >,
    crate::__types::ListFineTuningJobCheckpointsResponse
);
#[doc = "Get status updates for a fine-tuning job.\n"]
pub fn list_fine_tuning_events<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListFineTuningEventsParams,
) -> ListFineTuningEvents<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningEvents(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}/events",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::ListFineTuningEventsParams { after, limit, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        limit,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFineTuningEvents,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListFineTuningJobEventsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListFineTuningJobEventsResponse>,
    >,
    crate::__types::ListFineTuningJobEventsResponse
);
#[doc = "Pause a fine-tune job.\n"]
pub fn pause_fine_tuning_job<S, Fut, B, E>(
    service: S,
    params: &crate::__types::PauseFineTuningJobParams,
) -> PauseFineTuningJob<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    PauseFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}/pause",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::PauseFineTuningJobParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    PauseFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Resume a fine-tune job.\n"]
pub fn resume_fine_tuning_job<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ResumeFineTuningJobParams,
) -> ResumeFineTuningJob<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ResumeFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/jobs/{fine_tuning_job_id}/resume",
                        fine_tuning_job_id = params.fine_tuning_job_id
                    );
                    let crate::__types::ResumeFineTuningJobParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ResumeFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Creates an image given a prompt. [Learn more](https://platform.openai.com/docs/guides/images).\n"]
pub fn create_image<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateImageRequest,
) -> CreateImage<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateImage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/images/generations";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateImage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ImagesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ImagesResponse>,
    >,
    crate::__types::ImagesResponse
);
#[doc = "Lists the currently available models, and provides basic information about each one such as the owner and availability."]
pub fn list_models<S, Fut, B, E>(service: S) -> ListModels<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListModels(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/models";
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListModels,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListModelsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListModelsResponse>,
    >,
    crate::__types::ListModelsResponse
);
#[doc = "Retrieves a model instance, providing basic information about the model such as the owner and permissioning."]
pub fn retrieve_model<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveModelParams,
) -> RetrieveModel<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveModel(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/models/{model}", model = params.model);
                    let crate::__types::RetrieveModelParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveModel,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Model>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Model>,
    >,
    crate::__types::Model
);
#[doc = "Delete a fine-tuned model. You must have the Owner role in your organization to delete a model."]
pub fn delete_model<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteModelParams,
) -> DeleteModel<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteModel(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/models/{model}", model = params.model);
                    let crate::__types::DeleteModelParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteModel,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteModelResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteModelResponse>,
    >,
    crate::__types::DeleteModelResponse
);
#[doc = "Classifies if text and/or image inputs are potentially harmful. Learn\nmore in the [moderation guide](https://platform.openai.com/docs/guides/moderation).\n"]
pub fn create_moderation<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateModerationRequest,
) -> CreateModeration<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateModeration(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/moderations";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateModeration,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateModerationResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::CreateModerationResponse>,
    >,
    crate::__types::CreateModerationResponse
);
#[doc = "List organization API keys"]
pub fn admin_api_keys_list<S, Fut, B, E>(
    service: S,
    params: &crate::__types::AdminApiKeysListParams,
) -> AdminApiKeysList<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysList(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::AdminApiKeysListParamsOrder>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/admin_api_keys",);
                    let crate::__types::AdminApiKeysListParams {
                        after,
                        order,
                        limit,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        after,
                        order,
                        limit,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysList,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ApiKeyList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ApiKeyList>,
    >,
    crate::__types::ApiKeyList
);
#[doc = "Create an organization admin API key"]
pub fn admin_api_keys_create<S, Fut, B, E>(
    service: S,
    request: &crate::__types::AdminApiKeysCreateRequest,
) -> AdminApiKeysCreate<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysCreate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/admin_api_keys";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysCreate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
    >,
    crate::__types::AdminApiKey
);
#[doc = "Retrieve a single organization API key"]
pub fn admin_api_keys_get<S, Fut, B, E>(
    service: S,
    params: &crate::__types::AdminApiKeysGetParams,
) -> AdminApiKeysGet<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysGet(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/admin_api_keys/{key_id}",
                        key_id = params.key_id
                    );
                    let crate::__types::AdminApiKeysGetParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysGet,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
    >,
    crate::__types::AdminApiKey
);
#[doc = "Delete an organization admin API key"]
pub fn admin_api_keys_delete<S, Fut, B, E>(
    service: S,
    params: &crate::__types::AdminApiKeysDeleteParams,
) -> AdminApiKeysDelete<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysDelete(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/admin_api_keys/{key_id}",
                        key_id = params.key_id
                    );
                    let crate::__types::AdminApiKeysDeleteParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysDelete,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AdminApiKeysDeleteResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::AdminApiKeysDeleteResponse>,
    >,
    crate::__types::AdminApiKeysDeleteResponse
);
#[doc = "List user actions and configuration changes within this organization."]
pub fn list_audit_logs<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListAuditLogsParams,
) -> ListAuditLogs<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListAuditLogs(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "effective_at")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        effective_at: &'a Option<crate::__types::ListAuditLogsParamsEffectiveAt>,
                        #[serde(rename = "project_ids[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "event_types[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        event_types: &'a Option<Vec<crate::__types::AuditLogEventType>>,
                        #[serde(rename = "actor_ids[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        actor_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "actor_emails[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        actor_emails: &'a Option<Vec<String>>,
                        #[serde(rename = "resource_ids[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        resource_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/audit_logs",);
                    let crate::__types::ListAuditLogsParams {
                        effective_at,
                        project_ids,
                        event_types,
                        actor_ids,
                        actor_emails,
                        resource_ids,
                        limit,
                        after,
                        before,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        effective_at,
                        project_ids,
                        event_types,
                        actor_ids,
                        actor_emails,
                        resource_ids,
                        limit,
                        after,
                        before,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListAuditLogs,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListAuditLogsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListAuditLogsResponse>,
    >,
    crate::__types::ListAuditLogsResponse
);
#[doc = "List uploaded certificates for this organization."]
pub fn list_organization_certificates<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListOrganizationCertificatesParams,
) -> ListOrganizationCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListOrganizationCertificatesParamsOrder>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/certificates",);
                    let crate::__types::ListOrganizationCertificatesParams {
                        limit,
                        after,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Upload a certificate to the organization. This does **not** automatically activate the certificate.\n\nOrganizations can upload up to 50 certificates.\n"]
pub fn upload_certificate<S, Fut, B, E>(
    service: S,
    request: &crate::__types::UploadCertificateRequest,
) -> UploadCertificate<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UploadCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/certificates";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UploadCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Activate certificates at the organization level.\n\nYou can atomically and idempotently activate up to 10 certificates at a time.\n"]
pub fn activate_organization_certificates<S, Fut, B, E>(
    service: S,
    request: &crate::__types::ToggleCertificatesRequest,
) -> ActivateOrganizationCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ActivateOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/certificates/activate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ActivateOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Deactivate certificates at the organization level.\n\nYou can atomically and idempotently deactivate up to 10 certificates at a time.\n"]
pub fn deactivate_organization_certificates<S, Fut, B, E>(
    service: S,
    request: &crate::__types::ToggleCertificatesRequest,
) -> DeactivateOrganizationCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeactivateOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/certificates/deactivate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeactivateOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Get a certificate that has been uploaded to the organization.\n\nYou can get a certificate regardless of whether it is active or not.\n"]
pub fn get_certificate<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetCertificateParams,
) -> GetCertificate<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::GetCertificateParamsInclude>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/certificates/{certificate_id}",
                        certificate_id = params.certificate_id
                    );
                    let crate::__types::GetCertificateParams { include, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        include,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Modify a certificate. Note that only the name can be modified.\n"]
pub fn modify_certificate<S, Fut, B, E>(
    service: S,
    request: &crate::__types::ModifyCertificateRequest,
) -> ModifyCertificate<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/certificates/{certificate_id}";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Delete a certificate from the organization.\n\nThe certificate must be inactive for the organization and all projects.\n"]
pub fn delete_certificate<S, Fut, B, E>(service: S) -> DeleteCertificate<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/certificates/{certificate_id}";
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteCertificateResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteCertificateResponse>,
    >,
    crate::__types::DeleteCertificateResponse
);
#[doc = "Get costs details for the organization."]
pub fn usage_costs<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageCostsParams,
) -> UsageCosts<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCosts(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<crate::__types::UsageCostsParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageCostsParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/costs",);
                    let crate::__types::UsageCostsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCosts,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Returns a list of invites in the organization."]
pub fn list_invites<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListInvitesParams,
) -> ListInvites<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListInvites(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/invites",);
                    let crate::__types::ListInvitesParams { limit, after, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListInvites,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::InviteListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::InviteListResponse>,
    >,
    crate::__types::InviteListResponse
);
#[doc = "Create an invite for a user to the organization. The invite must be accepted by the user before they have access to the organization."]
pub fn invite_user<S, Fut, B, E>(
    service: S,
    request: &crate::__types::InviteRequest,
) -> InviteUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    InviteUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/invites";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    InviteUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Invite>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Invite>,
    >,
    crate::__types::Invite
);
#[doc = "Retrieves an invite."]
pub fn retrieve_invite<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveInviteParams,
) -> RetrieveInvite<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveInvite(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/invites/{invite_id}",
                        invite_id = params.invite_id
                    );
                    let crate::__types::RetrieveInviteParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveInvite,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Invite>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Invite>,
    >,
    crate::__types::Invite
);
#[doc = "Delete an invite. If the invite has already been accepted, it cannot be deleted."]
pub fn delete_invite<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteInviteParams,
) -> DeleteInvite<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteInvite(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/invites/{invite_id}",
                        invite_id = params.invite_id
                    );
                    let crate::__types::DeleteInviteParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteInvite,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::InviteDeleteResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::InviteDeleteResponse>,
    >,
    crate::__types::InviteDeleteResponse
);
#[doc = "Returns a list of projects."]
pub fn list_projects<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectsParams,
) -> ListProjects<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjects(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "include_archived")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include_archived: &'a Option<bool>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects",);
                    let crate::__types::ListProjectsParams {
                        limit,
                        after,
                        include_archived,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        include_archived,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjects,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectListResponse>,
    >,
    crate::__types::ProjectListResponse
);
#[doc = "Create a new project in the organization. Projects can be created and archived, but cannot be deleted."]
pub fn create_project<S, Fut, B, E>(
    service: S,
    request: &crate::__types::ProjectCreateRequest,
) -> CreateProject<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/organization/projects";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "Retrieves a project."]
pub fn retrieve_project<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveProjectParams,
) -> RetrieveProject<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}",
                        project_id = params.project_id
                    );
                    let crate::__types::RetrieveProjectParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "Modifies a project in the organization."]
pub fn modify_project<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyProjectParams,
    request: &crate::__types::ProjectUpdateRequest,
) -> ModifyProject<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}",
                        project_id = params.project_id
                    );
                    let crate::__types::ModifyProjectParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "Returns a list of API keys in the project."]
pub fn list_project_api_keys<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectApiKeysParams,
) -> ListProjectApiKeys<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectApiKeys(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/api_keys",
                        project_id = params.project_id
                    );
                    let crate::__types::ListProjectApiKeysParams { limit, after, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectApiKeys,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyListResponse>,
    >,
    crate::__types::ProjectApiKeyListResponse
);
#[doc = "Retrieves an API key in the project."]
pub fn retrieve_project_api_key<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveProjectApiKeyParams,
) -> RetrieveProjectApiKey<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectApiKey(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/api_keys/{key_id}",
                        project_id = params.project_id,
                        key_id = params.key_id
                    );
                    let crate::__types::RetrieveProjectApiKeyParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectApiKey,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKey>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKey>,
    >,
    crate::__types::ProjectApiKey
);
#[doc = "Deletes an API key from the project."]
pub fn delete_project_api_key<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteProjectApiKeyParams,
) -> DeleteProjectApiKey<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectApiKey(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/api_keys/{key_id}",
                        project_id = params.project_id,
                        key_id = params.key_id
                    );
                    let crate::__types::DeleteProjectApiKeyParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteProjectApiKey,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyDeleteResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyDeleteResponse>,
    >,
    crate::__types::ProjectApiKeyDeleteResponse
);
#[doc = "Archives a project in the organization. Archived projects cannot be used or updated."]
pub fn archive_project<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ArchiveProjectParams,
) -> ArchiveProject<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ArchiveProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/archive",
                        project_id = params.project_id
                    );
                    let crate::__types::ArchiveProjectParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ArchiveProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "List certificates for this project."]
pub fn list_project_certificates<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectCertificatesParams,
) -> ListProjectCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListProjectCertificatesParamsOrder>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/certificates",
                        project_id = params.project_id
                    );
                    let crate::__types::ListProjectCertificatesParams {
                        limit,
                        after,
                        order,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        order,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Activate certificates at the project level.\n\nYou can atomically and idempotently activate up to 10 certificates at a time.\n"]
pub fn activate_project_certificates<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ActivateProjectCertificatesParams,
    request: &crate::__types::ToggleCertificatesRequest,
) -> ActivateProjectCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ActivateProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/certificates/activate",
                        project_id = params.project_id
                    );
                    let crate::__types::ActivateProjectCertificatesParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ActivateProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Deactivate certificates at the project level. You can atomically and \nidempotently deactivate up to 10 certificates at a time.\n"]
pub fn deactivate_project_certificates<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeactivateProjectCertificatesParams,
    request: &crate::__types::ToggleCertificatesRequest,
) -> DeactivateProjectCertificates<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeactivateProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/certificates/deactivate",
                        project_id = params.project_id
                    );
                    let crate::__types::DeactivateProjectCertificatesParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeactivateProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Returns the rate limits per model for a project."]
pub fn list_project_rate_limits<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectRateLimitsParams,
) -> ListProjectRateLimits<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectRateLimits(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/rate_limits",
                        project_id = params.project_id
                    );
                    let crate::__types::ListProjectRateLimitsParams {
                        limit,
                        after,
                        before,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        before,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectRateLimits,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectRateLimitListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectRateLimitListResponse>,
    >,
    crate::__types::ProjectRateLimitListResponse
);
#[doc = "Updates a project rate limit."]
pub fn update_project_rate_limits<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UpdateProjectRateLimitsParams,
    request: &crate::__types::ProjectRateLimitUpdateRequest,
) -> UpdateProjectRateLimits<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateProjectRateLimits(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/rate_limits/{rate_limit_id}",
                        project_id = params.project_id,
                        rate_limit_id = params.rate_limit_id
                    );
                    let crate::__types::UpdateProjectRateLimitsParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateProjectRateLimits,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectRateLimit>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectRateLimit>,
    >,
    crate::__types::ProjectRateLimit
);
#[doc = "Returns a list of service accounts in the project."]
pub fn list_project_service_accounts<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectServiceAccountsParams,
) -> ListProjectServiceAccounts<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectServiceAccounts(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts",
                        project_id = params.project_id
                    );
                    let crate::__types::ListProjectServiceAccountsParams { limit, after, .. } =
                        params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectServiceAccounts,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountListResponse>,
        fn(
            B,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountListResponse>,
    >,
    crate::__types::ProjectServiceAccountListResponse
);
#[doc = "Creates a new service account in the project. This also returns an unredacted API key for the service account."]
pub fn create_project_service_account<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateProjectServiceAccountParams,
    request: &crate::__types::ProjectServiceAccountCreateRequest,
) -> CreateProjectServiceAccount<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts",
                        project_id = params.project_id
                    );
                    let crate::__types::CreateProjectServiceAccountParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateProjectServiceAccount,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountCreateResponse>,
        fn(
            B,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountCreateResponse>,
    >,
    crate::__types::ProjectServiceAccountCreateResponse
);
#[doc = "Retrieves a service account in the project."]
pub fn retrieve_project_service_account<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveProjectServiceAccountParams,
) -> RetrieveProjectServiceAccount<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts/{service_account_id}",
                        project_id = params.project_id,
                        service_account_id = params.service_account_id
                    );
                    let crate::__types::RetrieveProjectServiceAccountParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectServiceAccount,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccount>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccount>,
    >,
    crate::__types::ProjectServiceAccount
);
#[doc = "Deletes a service account from the project."]
pub fn delete_project_service_account<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteProjectServiceAccountParams,
) -> DeleteProjectServiceAccount<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts/{service_account_id}",
                        project_id = params.project_id,
                        service_account_id = params.service_account_id
                    );
                    let crate::__types::DeleteProjectServiceAccountParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteProjectServiceAccount,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountDeleteResponse>,
        fn(
            B,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountDeleteResponse>,
    >,
    crate::__types::ProjectServiceAccountDeleteResponse
);
#[doc = "Returns a list of users in the project."]
pub fn list_project_users<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListProjectUsersParams,
) -> ListProjectUsers<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectUsers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/users",
                        project_id = params.project_id
                    );
                    let crate::__types::ListProjectUsersParams { limit, after, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectUsers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUserListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectUserListResponse>,
    >,
    crate::__types::ProjectUserListResponse
);
#[doc = "Adds a user to the project. Users must already be members of the organization to be added to a project."]
pub fn create_project_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateProjectUserParams,
    request: &crate::__types::ProjectUserCreateRequest,
) -> CreateProjectUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/users",
                        project_id = params.project_id
                    );
                    let crate::__types::CreateProjectUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Retrieves a user in the project."]
pub fn retrieve_project_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveProjectUserParams,
) -> RetrieveProjectUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/users/{user_id}",
                        project_id = params.project_id,
                        user_id = params.user_id
                    );
                    let crate::__types::RetrieveProjectUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Modifies a user's role in the project."]
pub fn modify_project_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyProjectUserParams,
    request: &crate::__types::ProjectUserUpdateRequest,
) -> ModifyProjectUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/users/{user_id}",
                        project_id = params.project_id,
                        user_id = params.user_id
                    );
                    let crate::__types::ModifyProjectUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Deletes a user from the project."]
pub fn delete_project_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteProjectUserParams,
) -> DeleteProjectUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/users/{user_id}",
                        project_id = params.project_id,
                        user_id = params.user_id
                    );
                    let crate::__types::DeleteProjectUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUserDeleteResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ProjectUserDeleteResponse>,
    >,
    crate::__types::ProjectUserDeleteResponse
);
#[doc = "Get audio speeches usage details for the organization."]
pub fn usage_audio_speeches<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageAudioSpeechesParams,
) -> UsageAudioSpeeches<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageAudioSpeeches(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width:
                            &'a Option<crate::__types::UsageAudioSpeechesParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageAudioSpeechesParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/audio_speeches",);
                    let crate::__types::UsageAudioSpeechesParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageAudioSpeeches,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get audio transcriptions usage details for the organization."]
pub fn usage_audio_transcriptions<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageAudioTranscriptionsParams,
) -> UsageAudioTranscriptions<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageAudioTranscriptions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width:
                            &'a Option<crate::__types::UsageAudioTranscriptionsParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by:
                            &'a Option<Vec<crate::__types::UsageAudioTranscriptionsParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/audio_transcriptions",);
                    let crate::__types::UsageAudioTranscriptionsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageAudioTranscriptions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get code interpreter sessions usage details for the organization."]
pub fn usage_code_interpreter_sessions<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageCodeInterpreterSessionsParams,
) -> UsageCodeInterpreterSessions<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCodeInterpreterSessions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<
                            crate::__types::UsageCodeInterpreterSessionsParamsBucketWidth,
                        >,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<
                            Vec<crate::__types::UsageCodeInterpreterSessionsParamsGroupBy>,
                        >,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/code_interpreter_sessions",);
                    let crate::__types::UsageCodeInterpreterSessionsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCodeInterpreterSessions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get completions usage details for the organization."]
pub fn usage_completions<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageCompletionsParams,
) -> UsageCompletions<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCompletions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<crate::__types::UsageCompletionsParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "batch")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        batch: &'a Option<bool>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageCompletionsParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/completions",);
                    let crate::__types::UsageCompletionsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        batch,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        batch,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCompletions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get embeddings usage details for the organization."]
pub fn usage_embeddings<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageEmbeddingsParams,
) -> UsageEmbeddings<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageEmbeddings(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<crate::__types::UsageEmbeddingsParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageEmbeddingsParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/embeddings",);
                    let crate::__types::UsageEmbeddingsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageEmbeddings,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get images usage details for the organization."]
pub fn usage_images<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageImagesParams,
) -> UsageImages<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageImages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<crate::__types::UsageImagesParamsBucketWidth>,
                        #[serde(rename = "sources")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        sources: &'a Option<Vec<crate::__types::UsageImagesParamsSources>>,
                        #[serde(rename = "sizes")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        sizes: &'a Option<Vec<crate::__types::UsageImagesParamsSizes>>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageImagesParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/images",);
                    let crate::__types::UsageImagesParams {
                        start_time,
                        end_time,
                        bucket_width,
                        sources,
                        sizes,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        sources,
                        sizes,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageImages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get moderations usage details for the organization."]
pub fn usage_moderations<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageModerationsParams,
) -> UsageModerations<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageModerations(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width: &'a Option<crate::__types::UsageModerationsParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "user_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        user_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "api_key_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        api_key_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "models")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        models: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageModerationsParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/moderations",);
                    let crate::__types::UsageModerationsParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        user_ids,
                        api_key_ids,
                        models,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageModerations,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Get vector stores usage details for the organization."]
pub fn usage_vector_stores<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UsageVectorStoresParams,
) -> UsageVectorStores<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageVectorStores(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "start_time")]
                        start_time: &'a i64,
                        #[serde(rename = "end_time")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        end_time: &'a Option<i64>,
                        #[serde(rename = "bucket_width")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        bucket_width:
                            &'a Option<crate::__types::UsageVectorStoresParamsBucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<Vec<crate::__types::UsageVectorStoresParamsGroupBy>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/vector_stores",);
                    let crate::__types::UsageVectorStoresParams {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageVectorStores,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Lists all of the users in the organization."]
pub fn list_users<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListUsersParams,
) -> ListUsers<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListUsers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "emails")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        emails: &'a Option<Vec<String>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/users",);
                    let crate::__types::ListUsersParams {
                        limit,
                        after,
                        emails,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        after,
                        emails,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListUsers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UserListResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UserListResponse>,
    >,
    crate::__types::UserListResponse
);
#[doc = "Retrieves a user by their identifier."]
pub fn retrieve_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveUserParams,
) -> RetrieveUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/users/{user_id}", user_id = params.user_id);
                    let crate::__types::RetrieveUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::User>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::User>,
    >,
    crate::__types::User
);
#[doc = "Modifies a user's role in the organization."]
pub fn modify_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyUserParams,
    request: &crate::__types::UserRoleUpdateRequest,
) -> ModifyUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/users/{user_id}", user_id = params.user_id);
                    let crate::__types::ModifyUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::User>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::User>,
    >,
    crate::__types::User
);
#[doc = "Deletes a user from the organization."]
pub fn delete_user<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteUserParams,
) -> DeleteUser<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/users/{user_id}", user_id = params.user_id);
                    let crate::__types::DeleteUserParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UserDeleteResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::UserDeleteResponse>,
    >,
    crate::__types::UserDeleteResponse
);
#[doc = "Create an ephemeral API token for use in client-side applications with the\nRealtime API. Can be configured with the same session parameters as the\n`session.update` client event.\n\nIt responds with a session object, plus a `client_secret` key which contains\na usable ephemeral API token that can be used to authenticate browser clients\nfor the Realtime API.\n"]
pub fn create_realtime_session<S, Fut, B, E>(
    service: S,
    request: &crate::__types::RealtimeSessionCreateRequest,
) -> CreateRealtimeSession<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRealtimeSession(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/realtime/sessions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateRealtimeSession,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RealtimeSessionCreateResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RealtimeSessionCreateResponse>,
    >,
    crate::__types::RealtimeSessionCreateResponse
);
#[doc = "Create an ephemeral API token for use in client-side applications with the\nRealtime API specifically for realtime transcriptions. \nCan be configured with the same session parameters as the `transcription_session.update` client event.\n\nIt responds with a session object, plus a `client_secret` key which contains\na usable ephemeral API token that can be used to authenticate browser clients\nfor the Realtime API.\n"]
pub fn create_realtime_transcription_session<S, Fut, B, E>(
    service: S,
    request: &crate::__types::RealtimeTranscriptionSessionCreateRequest,
) -> CreateRealtimeTranscriptionSession<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRealtimeTranscriptionSession(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/realtime/transcription_sessions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateRealtimeTranscriptionSession,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<
            B,
            E,
            crate::__types::RealtimeTranscriptionSessionCreateResponse,
        >,
        fn(
            B,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::RealtimeTranscriptionSessionCreateResponse,
        >,
    >,
    crate::__types::RealtimeTranscriptionSessionCreateResponse
);
#[doc = "Creates a model response. Provide [text](https://platform.openai.com/docs/guides/text) or\n[image](https://platform.openai.com/docs/guides/images) inputs to generate [text](https://platform.openai.com/docs/guides/text)\nor [JSON](https://platform.openai.com/docs/guides/structured-outputs) outputs. Have the model call\nyour own [custom code](https://platform.openai.com/docs/guides/function-calling) or use built-in\n[tools](https://platform.openai.com/docs/guides/tools) like [web search](https://platform.openai.com/docs/guides/tools-web-search)\nor [file search](https://platform.openai.com/docs/guides/tools-file-search) to use your own data\nas input for the model's response.\n"]
pub fn create_response<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateResponse,
) -> CreateResponse<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/responses";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "Creates a model response. Provide [text](https://platform.openai.com/docs/guides/text) or\n[image](https://platform.openai.com/docs/guides/images) inputs to generate [text](https://platform.openai.com/docs/guides/text)\nor [JSON](https://platform.openai.com/docs/guides/structured-outputs) outputs. Have the model call\nyour own [custom code](https://platform.openai.com/docs/guides/function-calling) or use built-in\n[tools](https://platform.openai.com/docs/guides/tools) like [web search](https://platform.openai.com/docs/guides/tools-web-search)\nor [file search](https://platform.openai.com/docs/guides/tools-file-search) to use your own data\nas input for the model's response.\n"]
pub fn create_response_stream<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateResponse,
) -> CreateResponseStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateResponseStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/responses";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateResponseStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (B) -> crate :: __combinators :: EventStream < B , crate :: __types :: ResponseStreamEvent > , > , crate :: __combinators :: EventStream < B , crate :: __types :: ResponseStreamEvent >);
#[doc = "Retrieves a model response with the given ID.\n"]
pub fn get_response<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetResponseParams,
) -> GetResponse<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::Includable>>,
                        #[serde(rename = "stream")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        stream: &'a Option<bool>,
                        #[serde(rename = "starting_after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        starting_after: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/responses/{response_id}", response_id = params.response_id);
                    let crate::__types::GetResponseParams {
                        include,
                        stream,
                        starting_after,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        include,
                        stream,
                        starting_after,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "Cancels a model response with the given ID. Only responses created with\nthe `background` parameter set to `true` can be cancelled. \n[Learn more](https://platform.openai.com/docs/guides/background).\n"]
pub fn cancel_response<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelResponseParams,
) -> CancelResponse<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/responses/{response_id}/cancel",
                        response_id = params.response_id
                    );
                    let crate::__types::CancelResponseParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "Returns a list of input items for a given response."]
pub fn list_input_items<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListInputItemsParams,
) -> ListInputItems<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListInputItems(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListInputItemsParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "include")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::Includable>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/responses/{response_id}/input_items",
                        response_id = params.response_id
                    );
                    let crate::__types::ListInputItemsParams {
                        limit,
                        order,
                        after,
                        before,
                        include,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        include,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListInputItems,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ResponseItemList>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ResponseItemList>,
    >,
    crate::__types::ResponseItemList
);
#[doc = "Create a thread and run it in one request."]
pub fn create_thread_and_run<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateThreadAndRunRequest,
) -> CreateThreadAndRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateThreadAndRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/threads/runs";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateThreadAndRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Retrieves a thread."]
pub fn get_thread<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetThreadParams,
) -> GetThread<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}", thread_id = params.thread_id);
                    let crate::__types::GetThreadParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
    >,
    crate::__types::ThreadObject
);
#[doc = "Modifies a thread."]
pub fn modify_thread<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyThreadParams,
    request: &crate::__types::ModifyThreadRequest,
) -> ModifyThread<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}", thread_id = params.thread_id);
                    let crate::__types::ModifyThreadParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
    >,
    crate::__types::ThreadObject
);
#[doc = "Delete a thread."]
pub fn delete_thread<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteThreadParams,
) -> DeleteThread<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}", thread_id = params.thread_id);
                    let crate::__types::DeleteThreadParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteThreadResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteThreadResponse>,
    >,
    crate::__types::DeleteThreadResponse
);
#[doc = "Returns a list of messages for a given thread."]
pub fn list_messages<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListMessagesParams,
) -> ListMessages<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListMessages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListMessagesParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "run_id")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        run_id: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/messages",
                        thread_id = params.thread_id
                    );
                    let crate::__types::ListMessagesParams {
                        limit,
                        order,
                        after,
                        before,
                        run_id,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        run_id,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListMessages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListMessagesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListMessagesResponse>,
    >,
    crate::__types::ListMessagesResponse
);
#[doc = "Create a message."]
pub fn create_message<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateMessageParams,
    request: &crate::__types::CreateMessageRequest,
) -> CreateMessage<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/messages",
                        thread_id = params.thread_id
                    );
                    let crate::__types::CreateMessageParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Retrieve a message."]
pub fn get_message<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetMessageParams,
) -> GetMessage<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/messages/{message_id}",
                        thread_id = params.thread_id,
                        message_id = params.message_id
                    );
                    let crate::__types::GetMessageParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Modifies a message."]
pub fn modify_message<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyMessageParams,
    request: &crate::__types::ModifyMessageRequest,
) -> ModifyMessage<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/messages/{message_id}",
                        thread_id = params.thread_id,
                        message_id = params.message_id
                    );
                    let crate::__types::ModifyMessageParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Deletes a message."]
pub fn delete_message<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteMessageParams,
) -> DeleteMessage<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/messages/{message_id}",
                        thread_id = params.thread_id,
                        message_id = params.message_id
                    );
                    let crate::__types::DeleteMessageParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteMessageResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteMessageResponse>,
    >,
    crate::__types::DeleteMessageResponse
);
#[doc = "Returns a list of runs belonging to a thread."]
pub fn list_runs<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListRunsParams,
) -> ListRuns<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListRuns(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListRunsParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/threads/{thread_id}/runs", thread_id = params.thread_id);
                    let crate::__types::ListRunsParams {
                        limit,
                        order,
                        after,
                        before,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListRuns,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListRunsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListRunsResponse>,
    >,
    crate::__types::ListRunsResponse
);
#[doc = "Create a run."]
pub fn create_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateRunParams,
    request: &crate::__types::CreateRunRequest,
) -> CreateRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::CreateRunParamsInclude>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/threads/{thread_id}/runs", thread_id = params.thread_id);
                    let crate::__types::CreateRunParams { include, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        include,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Retrieves a run."]
pub fn get_run<S, Fut, B, E>(service: S, params: &crate::__types::GetRunParams) -> GetRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}",
                        thread_id = params.thread_id,
                        run_id = params.run_id
                    );
                    let crate::__types::GetRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Modifies a run."]
pub fn modify_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyRunParams,
    request: &crate::__types::ModifyRunRequest,
) -> ModifyRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}",
                        thread_id = params.thread_id,
                        run_id = params.run_id
                    );
                    let crate::__types::ModifyRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Cancels a run that is `in_progress`."]
pub fn cancel_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelRunParams,
) -> CancelRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}/cancel",
                        thread_id = params.thread_id,
                        run_id = params.run_id
                    );
                    let crate::__types::CancelRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Returns a list of run steps belonging to a run."]
pub fn list_run_steps<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListRunStepsParams,
) -> ListRunSteps<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListRunSteps(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListRunStepsParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::ListRunStepsParamsInclude>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}/steps",
                        thread_id = params.thread_id,
                        run_id = params.run_id
                    );
                    let crate::__types::ListRunStepsParams {
                        limit,
                        order,
                        after,
                        before,
                        include,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        include,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListRunSteps,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListRunStepsResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListRunStepsResponse>,
    >,
    crate::__types::ListRunStepsResponse
);
#[doc = "Retrieves a run step."]
pub fn get_run_step<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetRunStepParams,
) -> GetRunStep<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetRunStep(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::GetRunStepParamsInclude>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}/steps/{step_id}",
                        thread_id = params.thread_id,
                        run_id = params.run_id,
                        step_id = params.step_id
                    );
                    let crate::__types::GetRunStepParams { include, .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        include,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetRunStep,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunStepObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunStepObject>,
    >,
    crate::__types::RunStepObject
);
#[doc = "When a run has the `status: \"requires_action\"` and `required_action.type` is `submit_tool_outputs`, this endpoint can be used to submit the outputs from the tool calls once they're all completed. All outputs must be submitted in a single request.\n"]
pub fn submit_tool_ouputs_to_run<S, Fut, B, E>(
    service: S,
    params: &crate::__types::SubmitToolOuputsToRunParams,
    request: &crate::__types::SubmitToolOutputsRunRequest,
) -> SubmitToolOuputsToRun<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    SubmitToolOuputsToRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/threads/{thread_id}/runs/{run_id}/submit_tool_outputs",
                        thread_id = params.thread_id,
                        run_id = params.run_id
                    );
                    let crate::__types::SubmitToolOuputsToRunParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    SubmitToolOuputsToRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Creates an intermediate [Upload](https://platform.openai.com/docs/api-reference/uploads/object) object\nthat you can add [Parts](https://platform.openai.com/docs/api-reference/uploads/part-object) to.\nCurrently, an Upload can accept at most 8 GB in total and expires after an\nhour after you create it.\n\nOnce you complete the Upload, we will create a\n[File](https://platform.openai.com/docs/api-reference/files/object) object that contains all the parts\nyou uploaded. This File is usable in the rest of our platform as a regular\nFile object.\n\nFor certain `purpose` values, the correct `mime_type` must be specified. \nPlease refer to documentation for the \n[supported MIME types for your use case](https://platform.openai.com/docs/assistants/tools/file-search#supported-files).\n\nFor guidance on the proper filename extensions for each purpose, please\nfollow the documentation on [creating a\nFile](https://platform.openai.com/docs/api-reference/files/create).\n"]
pub fn create_upload<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateUploadRequest,
) -> CreateUpload<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/uploads";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "Cancels the Upload. No Parts may be added after an Upload is cancelled.\n"]
pub fn cancel_upload<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelUploadParams,
) -> CancelUpload<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/uploads/{upload_id}/cancel", upload_id = params.upload_id);
                    let crate::__types::CancelUploadParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "Completes the [Upload](https://platform.openai.com/docs/api-reference/uploads/object). \n\nWithin the returned Upload object, there is a nested [File](https://platform.openai.com/docs/api-reference/files/object) object that is ready to use in the rest of the platform.\n\nYou can specify the order of the Parts by passing in an ordered list of the Part IDs.\n\nThe number of bytes uploaded upon completion must match the number of bytes initially specified when creating the Upload object. No Parts may be added after an Upload is completed.\n"]
pub fn complete_upload<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CompleteUploadParams,
    request: &crate::__types::CompleteUploadRequest,
) -> CompleteUpload<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CompleteUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/uploads/{upload_id}/complete",
                        upload_id = params.upload_id
                    );
                    let crate::__types::CompleteUploadParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CompleteUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "Returns a list of vector stores."]
pub fn list_vector_stores<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListVectorStoresParams,
) -> ListVectorStores<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListVectorStores(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListVectorStoresParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores",);
                    let crate::__types::ListVectorStoresParams {
                        limit,
                        order,
                        after,
                        before,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListVectorStores,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoresResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoresResponse>,
    >,
    crate::__types::ListVectorStoresResponse
);
#[doc = "Create a vector store."]
pub fn create_vector_store<S, Fut, B, E>(
    service: S,
    request: &crate::__types::CreateVectorStoreRequest,
) -> CreateVectorStore<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = "/vector_stores";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Retrieves a vector store."]
pub fn get_vector_store<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetVectorStoreParams,
) -> GetVectorStore<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::GetVectorStoreParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Modifies a vector store."]
pub fn modify_vector_store<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ModifyVectorStoreParams,
    request: &crate::__types::UpdateVectorStoreRequest,
) -> ModifyVectorStore<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::ModifyVectorStoreParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Delete a vector store."]
pub fn delete_vector_store<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteVectorStoreParams,
) -> DeleteVectorStore<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::DeleteVectorStoreParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreResponse>,
    >,
    crate::__types::DeleteVectorStoreResponse
);
#[doc = "Create a vector store file batch."]
pub fn create_vector_store_file_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateVectorStoreFileBatchParams,
    request: &crate::__types::CreateVectorStoreFileBatchRequest,
) -> CreateVectorStoreFileBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/file_batches",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::CreateVectorStoreFileBatchParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "Retrieves a vector store file batch."]
pub fn get_vector_store_file_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetVectorStoreFileBatchParams,
) -> GetVectorStoreFileBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/file_batches/{batch_id}",
                        vector_store_id = params.vector_store_id,
                        batch_id = params.batch_id
                    );
                    let crate::__types::GetVectorStoreFileBatchParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "Cancel a vector store file batch. This attempts to cancel the processing of files in this batch as soon as possible."]
pub fn cancel_vector_store_file_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CancelVectorStoreFileBatchParams,
) -> CancelVectorStoreFileBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel",
                        vector_store_id = params.vector_store_id,
                        batch_id = params.batch_id
                    );
                    let crate::__types::CancelVectorStoreFileBatchParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "Returns a list of vector store files in a batch."]
pub fn list_files_in_vector_store_batch<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListFilesInVectorStoreBatchParams,
) -> ListFilesInVectorStoreBatch<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFilesInVectorStoreBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListFilesInVectorStoreBatchParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "filter")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        filter: &'a Option<crate::__types::ListFilesInVectorStoreBatchParamsFilter>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/file_batches/{batch_id}/files",
                        vector_store_id = params.vector_store_id,
                        batch_id = params.batch_id
                    );
                    let crate::__types::ListFilesInVectorStoreBatchParams {
                        limit,
                        order,
                        after,
                        before,
                        filter,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        filter,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFilesInVectorStoreBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
    >,
    crate::__types::ListVectorStoreFilesResponse
);
#[doc = "Returns a list of vector store files."]
pub fn list_vector_store_files<S, Fut, B, E>(
    service: S,
    params: &crate::__types::ListVectorStoreFilesParams,
) -> ListVectorStoreFiles<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListVectorStoreFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::ListVectorStoreFilesParamsOrder>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "filter")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        filter: &'a Option<crate::__types::ListVectorStoreFilesParamsFilter>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::ListVectorStoreFilesParams {
                        limit,
                        order,
                        after,
                        before,
                        filter,
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(Query {
                        limit,
                        order,
                        after,
                        before,
                        filter,
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListVectorStoreFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
    >,
    crate::__types::ListVectorStoreFilesResponse
);
#[doc = "Create a vector store file by attaching a [File](https://platform.openai.com/docs/api-reference/files) to a [vector store](https://platform.openai.com/docs/api-reference/vector-stores/object)."]
pub fn create_vector_store_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::CreateVectorStoreFileParams,
    request: &crate::__types::CreateVectorStoreFileRequest,
) -> CreateVectorStoreFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::CreateVectorStoreFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Retrieves a vector store file."]
pub fn get_vector_store_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::GetVectorStoreFileParams,
) -> GetVectorStoreFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files/{file_id}",
                        vector_store_id = params.vector_store_id,
                        file_id = params.file_id
                    );
                    let crate::__types::GetVectorStoreFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Delete a vector store file. This will remove the file from the vector store but the file itself will not be deleted. To delete the file, use the [delete file](https://platform.openai.com/docs/api-reference/files/delete) endpoint."]
pub fn delete_vector_store_file<S, Fut, B, E>(
    service: S,
    params: &crate::__types::DeleteVectorStoreFileParams,
) -> DeleteVectorStoreFile<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files/{file_id}",
                        vector_store_id = params.vector_store_id,
                        file_id = params.file_id
                    );
                    let crate::__types::DeleteVectorStoreFileParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreFileResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreFileResponse>,
    >,
    crate::__types::DeleteVectorStoreFileResponse
);
#[doc = "Update attributes on a vector store file."]
pub fn update_vector_store_file_attributes<S, Fut, B, E>(
    service: S,
    params: &crate::__types::UpdateVectorStoreFileAttributesParams,
    request: &crate::__types::UpdateVectorStoreFileAttributesRequest,
) -> UpdateVectorStoreFileAttributes<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateVectorStoreFileAttributes(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files/{file_id}",
                        vector_store_id = params.vector_store_id,
                        file_id = params.file_id
                    );
                    let crate::__types::UpdateVectorStoreFileAttributesParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateVectorStoreFileAttributes,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Retrieve the parsed contents of a vector store file."]
pub fn retrieve_vector_store_file_content<S, Fut, B, E>(
    service: S,
    params: &crate::__types::RetrieveVectorStoreFileContentParams,
) -> RetrieveVectorStoreFileContent<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveVectorStoreFileContent(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/files/{file_id}/content",
                        vector_store_id = params.vector_store_id,
                        file_id = params.file_id
                    );
                    let crate::__types::RetrieveVectorStoreFileContentParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveVectorStoreFileContent,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileContentResponse>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileContentResponse>,
    >,
    crate::__types::VectorStoreFileContentResponse
);
#[doc = "Search a vector store for relevant chunks based on a query and file attributes filter."]
pub fn search_vector_store<S, Fut, B, E>(
    service: S,
    params: &crate::__types::SearchVectorStoreParams,
    request: &crate::__types::VectorStoreSearchRequest,
) -> SearchVectorStore<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    SearchVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            service,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/vector_stores/{vector_store_id}/search",
                        vector_store_id = params.vector_store_id
                    );
                    let crate::__types::SearchVectorStoreParams { .. } = params;
                    let query = serde_urlencoded::to_string(Query {
                        _phantom: std::marker::PhantomData,
                    })?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .body(body)?)
            },
            http::StatusCode::from_u16(200u16).unwrap(),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    SearchVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreSearchResultsPage>,
        fn(B) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreSearchResultsPage>,
    >,
    crate::__types::VectorStoreSearchResultsPage
);
