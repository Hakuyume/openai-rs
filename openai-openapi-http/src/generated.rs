#[doc = "List assistants"]
pub fn list_assistants<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_assistants::Params,
) -> ListAssistants<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListAssistants(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_assistants::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_assistants::Params {
                        limit,
                        order,
                        after,
                        before,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/assistants");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListAssistants,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListAssistantsResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListAssistantsResponse>,
    >,
    crate::__types::ListAssistantsResponse
);
#[doc = "Create assistant"]
pub fn create_assistant<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateAssistantRequest,
) -> CreateAssistant<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/assistants";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Retrieve assistant"]
pub fn get_assistant<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_assistant::Params,
) -> GetAssistant<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_assistant::Params { assistant_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/assistants/{assistant_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Modify assistant"]
pub fn modify_assistant<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_assistant::Params,
    request: &crate::__types::ModifyAssistantRequest,
) -> ModifyAssistant<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_assistant::Params { assistant_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/assistants/{assistant_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::AssistantObject>,
    >,
    crate::__types::AssistantObject
);
#[doc = "Delete assistant"]
pub fn delete_assistant<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_assistant::Params,
) -> DeleteAssistant<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteAssistant(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_assistant::Params { assistant_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/assistants/{assistant_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteAssistant,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteAssistantResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteAssistantResponse>,
    >,
    crate::__types::DeleteAssistantResponse
);
#[doc = "Create speech"]
pub fn create_speech_stream<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateSpeechRequest,
) -> CreateSpeechStream<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateSpeechStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/audio/speech";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::TEXT_EVENT_STREAM),
            ),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateSpeechStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (http :: Response < B >) -> crate :: __combinators :: EventStream < B , crate :: __types :: CreateSpeechResponseStreamEvent > , > , crate :: __combinators :: EventStream < B , crate :: __types :: CreateSpeechResponseStreamEvent >);
#[doc = "Create batch"]
pub fn create_batch<C, Fut, B, E>(
    client: C,
    request: &crate::__types::create_batch::Request,
) -> CreateBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/batches";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "List batch"]
pub fn list_batches<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_batches::Params,
) -> ListBatches<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListBatches(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_batches::Params { after, limit } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/batches");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListBatches,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListBatchesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListBatchesResponse>,
    >,
    crate::__types::ListBatchesResponse
);
#[doc = "Retrieve batch"]
pub fn retrieve_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_batch::Params,
) -> RetrieveBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_batch::Params { batch_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/batches/{batch_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "Cancel batch"]
pub fn cancel_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_batch::Params,
) -> CancelBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_batch::Params { batch_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/batches/{batch_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Batch>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Batch>,
    >,
    crate::__types::Batch
);
#[doc = "List Chat Completions"]
pub fn list_chat_completions<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_chat_completions::Params,
) -> ListChatCompletions<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListChatCompletions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_chat_completions::params::Order>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_chat_completions::Params {
                        model,
                        metadata,
                        after,
                        limit,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListChatCompletions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionList>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionList>,
    >,
    crate::__types::ChatCompletionList
);
#[doc = "Create chat completion"]
pub fn create_chat_completion<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateChatCompletionRequest,
) -> CreateChatCompletion<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/chat/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "Create chat completion"]
pub fn create_chat_completion_stream<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateChatCompletionRequest,
) -> CreateChatCompletionStream<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateChatCompletionStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/chat/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::TEXT_EVENT_STREAM),
            ),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateChatCompletionStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (http :: Response < B >) -> crate :: __combinators :: EventStream < B , crate :: __types :: CreateChatCompletionStreamResponse > , > , crate :: __combinators :: EventStream < B , crate :: __types :: CreateChatCompletionStreamResponse >);
#[doc = "Get chat completion"]
pub fn get_chat_completion<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_chat_completion::Params,
) -> GetChatCompletion<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_chat_completion::Params { completion_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions/{completion_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "Update chat completion"]
pub fn update_chat_completion<C, Fut, B, E>(
    client: C,
    params: &crate::__types::update_chat_completion::Params,
    request: &crate::__types::update_chat_completion::Request,
) -> UpdateChatCompletion<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::update_chat_completion::Params { completion_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions/{completion_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::CreateChatCompletionResponse>,
    >,
    crate::__types::CreateChatCompletionResponse
);
#[doc = "Delete chat completion"]
pub fn delete_chat_completion<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_chat_completion::Params,
) -> DeleteChatCompletion<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteChatCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_chat_completion::Params { completion_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions/{completion_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteChatCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionDeleted>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionDeleted>,
    >,
    crate::__types::ChatCompletionDeleted
);
#[doc = "Get chat messages"]
pub fn get_chat_completion_messages<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_chat_completion_messages::Params,
) -> GetChatCompletionMessages<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetChatCompletionMessages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order:
                            &'a Option<crate::__types::get_chat_completion_messages::params::Order>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_chat_completion_messages::Params {
                        completion_id,
                        after,
                        limit,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/chat/completions/{completion_id}/messages");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetChatCompletionMessages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ChatCompletionMessageList>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ChatCompletionMessageList>,
    >,
    crate::__types::ChatCompletionMessageList
);
#[doc = "Create completion"]
pub fn create_completion<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateCompletionRequest,
) -> CreateCompletion<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateCompletion(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/completions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateCompletion,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateCompletionResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::CreateCompletionResponse>,
    >,
    crate::__types::CreateCompletionResponse
);
#[doc = "List containers"]
pub fn list_containers<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_containers::Params,
) -> ListContainers<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListContainers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_containers::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_containers::Params {
                        limit,
                        order,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListContainers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerListResource>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ContainerListResource>,
    >,
    crate::__types::ContainerListResource
);
#[doc = "Retrieve container"]
pub fn retrieve_container<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_container::Params,
) -> RetrieveContainer<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveContainer(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_container::Params { container_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveContainer,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerResource>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ContainerResource>,
    >,
    crate::__types::ContainerResource
);
#[doc = "Delete a container"]
pub fn delete_container<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_container::Params,
) -> DeleteContainer<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteContainer(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_container::Params { container_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}");
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
            (http::StatusCode::from_u16(200u16).unwrap(), None),
        ),
        |_| (),
    ))
}
future!(
    DeleteContainer,
    futures::future::MapOk<crate::__combinators::Send<Fut, B, E>, fn(http::Response<B>) -> ()>,
    ()
);
#[doc = "List container files"]
pub fn list_container_files<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_container_files::Params,
) -> ListContainerFiles<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListContainerFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_container_files::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_container_files::Params {
                        container_id,
                        limit,
                        order,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}/files");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListContainerFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerFileListResource>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ContainerFileListResource>,
    >,
    crate::__types::ContainerFileListResource
);
#[doc = "Retrieve container file"]
pub fn retrieve_container_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_container_file::Params,
) -> RetrieveContainerFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveContainerFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_container_file::Params {
                        container_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}/files/{file_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveContainerFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ContainerFileResource>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ContainerFileResource>,
    >,
    crate::__types::ContainerFileResource
);
#[doc = "Delete a container file"]
pub fn delete_container_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_container_file::Params,
) -> DeleteContainerFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteContainerFile(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_container_file::Params {
                        container_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}/files/{file_id}");
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
            (http::StatusCode::from_u16(200u16).unwrap(), None),
        ),
        |_| (),
    ))
}
future!(
    DeleteContainerFile,
    futures::future::MapOk<crate::__combinators::Send<Fut, B, E>, fn(http::Response<B>) -> ()>,
    ()
);
#[doc = "Retrieve container file content"]
pub fn retrieve_container_file_content<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_container_file_content::Params,
) -> RetrieveContainerFileContent<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveContainerFileContent(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_container_file_content::Params {
                        container_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/containers/{container_id}/files/{file_id}/content");
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
            (http::StatusCode::from_u16(200u16).unwrap(), None),
        ),
        http::Response::into_body,
    ))
}
future!(
    RetrieveContainerFileContent,
    futures::future::MapOk<crate::__combinators::Send<Fut, B, E>, fn(http::Response<B>) -> B>,
    B
);
#[doc = "Create embeddings"]
pub fn create_embedding<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateEmbeddingRequest,
) -> CreateEmbedding<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEmbedding(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/embeddings";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEmbedding,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateEmbeddingResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::CreateEmbeddingResponse>,
    >,
    crate::__types::CreateEmbeddingResponse
);
#[doc = "List evals"]
pub fn list_evals<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_evals::Params,
) -> ListEvals<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListEvals(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_evals::params::Order>,
                        #[serde(rename = "order_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order_by: &'a Option<crate::__types::list_evals::params::OrderBy>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_evals::Params {
                        after,
                        limit,
                        order,
                        order_by,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListEvals,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalList>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::EvalList>,
    >,
    crate::__types::EvalList
);
#[doc = "Create eval"]
pub fn create_eval<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateEvalRequest,
) -> CreateEval<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/evals";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(201u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Get an eval"]
pub fn get_eval<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_eval::Params,
) -> GetEval<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_eval::Params { eval_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Update an eval"]
pub fn update_eval<C, Fut, B, E>(
    client: C,
    params: &crate::__types::update_eval::Params,
    request: &crate::__types::update_eval::Request,
) -> UpdateEval<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::update_eval::Params { eval_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Eval>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Eval>,
    >,
    crate::__types::Eval
);
#[doc = "Delete an eval"]
pub fn delete_eval<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_eval::Params,
) -> DeleteEval<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteEval(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_eval::Params { eval_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteEval,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::delete_eval::Response>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::delete_eval::Response>,
    >,
    crate::__types::delete_eval::Response
);
#[doc = "Get eval runs"]
pub fn get_eval_runs<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_eval_runs::Params,
) -> GetEvalRuns<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRuns(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::get_eval_runs::params::Order>,
                        #[serde(rename = "status")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        status: &'a Option<crate::__types::get_eval_runs::params::Status>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_eval_runs::Params {
                        eval_id,
                        after,
                        limit,
                        order,
                        status,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRuns,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunList>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::EvalRunList>,
    >,
    crate::__types::EvalRunList
);
#[doc = "Create eval run"]
pub fn create_eval_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_eval_run::Params,
    request: &crate::__types::CreateEvalRunRequest,
) -> CreateEvalRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_eval_run::Params { eval_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(201u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Get an eval run"]
pub fn get_eval_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_eval_run::Params,
) -> GetEvalRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_eval_run::Params { eval_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs/{run_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Cancel eval run"]
pub fn cancel_eval_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_eval_run::Params,
) -> CancelEvalRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_eval_run::Params { eval_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs/{run_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRun>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::EvalRun>,
    >,
    crate::__types::EvalRun
);
#[doc = "Delete eval run"]
pub fn delete_eval_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_eval_run::Params,
) -> DeleteEvalRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteEvalRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_eval_run::Params { eval_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs/{run_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteEvalRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::delete_eval_run::Response>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::delete_eval_run::Response>,
    >,
    crate::__types::delete_eval_run::Response
);
#[doc = "Get eval run output items"]
pub fn get_eval_run_output_items<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_eval_run_output_items::Params,
) -> GetEvalRunOutputItems<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRunOutputItems(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        status:
                            &'a Option<crate::__types::get_eval_run_output_items::params::Status>,
                        #[serde(rename = "order")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        order: &'a Option<crate::__types::get_eval_run_output_items::params::Order>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_eval_run_output_items::Params {
                        eval_id,
                        run_id,
                        after,
                        limit,
                        status,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/evals/{eval_id}/runs/{run_id}/output_items");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRunOutputItems,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItemList>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItemList>,
    >,
    crate::__types::EvalRunOutputItemList
);
#[doc = "Get an output item of an eval run"]
pub fn get_eval_run_output_item<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_eval_run_output_item::Params,
) -> GetEvalRunOutputItem<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetEvalRunOutputItem(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_eval_run_output_item::Params {
                        eval_id,
                        run_id,
                        output_item_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/evals/{eval_id}/runs/{run_id}/output_items/{output_item_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetEvalRunOutputItem,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItem>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::EvalRunOutputItem>,
    >,
    crate::__types::EvalRunOutputItem
);
#[doc = "List files"]
pub fn list_files<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_files::Params,
) -> ListFiles<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_files::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_files::Params {
                        purpose,
                        limit,
                        order,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListFilesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListFilesResponse>,
    >,
    crate::__types::ListFilesResponse
);
#[doc = "Delete file"]
pub fn delete_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_file::Params,
) -> DeleteFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_file::Params { file_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteFileResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteFileResponse>,
    >,
    crate::__types::DeleteFileResponse
);
#[doc = "Retrieve file"]
pub fn retrieve_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_file::Params,
) -> RetrieveFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_file::Params { file_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::OpenAiFile>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::OpenAiFile>,
    >,
    crate::__types::OpenAiFile
);
#[doc = "Retrieve file content"]
pub fn download_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::download_file::Params,
) -> DownloadFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DownloadFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::download_file::Params { file_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/files/{file_id}/content");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DownloadFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, String>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, String>,
    >,
    String
);
#[doc = "Run grader"]
pub fn run_grader<C, Fut, B, E>(
    client: C,
    request: &crate::__types::RunGraderRequest,
) -> RunGrader<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RunGrader(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/fine_tuning/alpha/graders/run";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RunGrader,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunGraderResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::RunGraderResponse>,
    >,
    crate::__types::RunGraderResponse
);
#[doc = "Validate grader"]
pub fn validate_grader<C, Fut, B, E>(
    client: C,
    request: &crate::__types::ValidateGraderRequest,
) -> ValidateGrader<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ValidateGrader(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/fine_tuning/alpha/graders/validate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ValidateGrader,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ValidateGraderResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ValidateGraderResponse>,
    >,
    crate::__types::ValidateGraderResponse
);
#[doc = "List checkpoint permissions"]
pub fn list_fine_tuning_checkpoint_permissions<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_fine_tuning_checkpoint_permissions::Params,
) -> ListFineTuningCheckpointPermissions<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningCheckpointPermissions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            crate::__types::list_fine_tuning_checkpoint_permissions::params::Order,
                        >,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_fine_tuning_checkpoint_permissions::Params {
                        fine_tuned_model_checkpoint,
                        project_id,
                        after,
                        limit,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions"
                    );
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::ListFineTuningCheckpointPermissionResponse
);
#[doc = "Create checkpoint permissions"]
pub fn create_fine_tuning_checkpoint_permission<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_fine_tuning_checkpoint_permission::Params,
    request: &crate::__types::CreateFineTuningCheckpointPermissionRequest,
) -> CreateFineTuningCheckpointPermission<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateFineTuningCheckpointPermission(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_fine_tuning_checkpoint_permission::Params {
                        fine_tuned_model_checkpoint,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions"
                    );
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::ListFineTuningCheckpointPermissionResponse
);
#[doc = "Delete checkpoint permission"]
pub fn delete_fine_tuning_checkpoint_permission<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_fine_tuning_checkpoint_permission::Params,
) -> DeleteFineTuningCheckpointPermission<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteFineTuningCheckpointPermission(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_fine_tuning_checkpoint_permission::Params {
                        fine_tuned_model_checkpoint,
                        permission_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions/{permission_id}"
                    );
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::DeleteFineTuningCheckpointPermissionResponse,
        >,
    >,
    crate::__types::DeleteFineTuningCheckpointPermissionResponse
);
#[doc = "Create fine-tuning job"]
pub fn create_fine_tuning_job<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateFineTuningJobRequest,
) -> CreateFineTuningJob<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/fine_tuning/jobs";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "List fine-tuning jobs"]
pub fn list_paginated_fine_tuning_jobs<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_paginated_fine_tuning_jobs::Params,
) -> ListPaginatedFineTuningJobs<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListPaginatedFineTuningJobs(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_paginated_fine_tuning_jobs::Params {
                        after,
                        limit,
                        metadata,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ListPaginatedFineTuningJobsResponse>,
    >,
    crate::__types::ListPaginatedFineTuningJobsResponse
);
#[doc = "Retrieve fine-tuning job"]
pub fn retrieve_fine_tuning_job<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_fine_tuning_job::Params,
) -> RetrieveFineTuningJob<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_fine_tuning_job::Params { fine_tuning_job_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Cancel fine-tuning"]
pub fn cancel_fine_tuning_job<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_fine_tuning_job::Params,
) -> CancelFineTuningJob<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_fine_tuning_job::Params { fine_tuning_job_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "List fine-tuning checkpoints"]
pub fn list_fine_tuning_job_checkpoints<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_fine_tuning_job_checkpoints::Params,
) -> ListFineTuningJobCheckpoints<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningJobCheckpoints(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_fine_tuning_job_checkpoints::Params {
                        fine_tuning_job_id,
                        after,
                        limit,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}/checkpoints");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::ListFineTuningJobCheckpointsResponse,
        >,
    >,
    crate::__types::ListFineTuningJobCheckpointsResponse
);
#[doc = "List fine-tuning events"]
pub fn list_fine_tuning_events<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_fine_tuning_events::Params,
) -> ListFineTuningEvents<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFineTuningEvents(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_fine_tuning_events::Params {
                        fine_tuning_job_id,
                        after,
                        limit,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}/events");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFineTuningEvents,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListFineTuningJobEventsResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ListFineTuningJobEventsResponse>,
    >,
    crate::__types::ListFineTuningJobEventsResponse
);
#[doc = "Pause fine-tuning"]
pub fn pause_fine_tuning_job<C, Fut, B, E>(
    client: C,
    params: &crate::__types::pause_fine_tuning_job::Params,
) -> PauseFineTuningJob<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    PauseFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::pause_fine_tuning_job::Params { fine_tuning_job_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}/pause");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    PauseFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Resume fine-tuning"]
pub fn resume_fine_tuning_job<C, Fut, B, E>(
    client: C,
    params: &crate::__types::resume_fine_tuning_job::Params,
) -> ResumeFineTuningJob<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ResumeFineTuningJob(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::resume_fine_tuning_job::Params { fine_tuning_job_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/fine_tuning/jobs/{fine_tuning_job_id}/resume");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ResumeFineTuningJob,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::FineTuningJob>,
    >,
    crate::__types::FineTuningJob
);
#[doc = "Create image"]
pub fn create_image<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateImageRequest,
) -> CreateImage<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateImage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/images/generations";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateImage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ImagesResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ImagesResponse>,
    >,
    crate::__types::ImagesResponse
);
#[doc = "Create image"]
pub fn create_image_stream<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateImageRequest,
) -> CreateImageStream<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateImageStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/images/generations";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::TEXT_EVENT_STREAM),
            ),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateImageStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (http :: Response < B >) -> crate :: __combinators :: EventStream < B , crate :: __types :: ImageGenStreamEvent > , > , crate :: __combinators :: EventStream < B , crate :: __types :: ImageGenStreamEvent >);
#[doc = "List models"]
pub fn list_models<C, Fut, B, E>(client: C) -> ListModels<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListModels(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/models";
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::GET)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListModels,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListModelsResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListModelsResponse>,
    >,
    crate::__types::ListModelsResponse
);
#[doc = "Retrieve model"]
pub fn retrieve_model<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_model::Params,
) -> RetrieveModel<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveModel(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_model::Params { model } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/models/{model}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveModel,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Model>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Model>,
    >,
    crate::__types::Model
);
#[doc = "Delete a fine-tuned model"]
pub fn delete_model<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_model::Params,
) -> DeleteModel<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteModel(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_model::Params { model } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/models/{model}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteModel,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteModelResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteModelResponse>,
    >,
    crate::__types::DeleteModelResponse
);
#[doc = "Create moderation"]
pub fn create_moderation<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateModerationRequest,
) -> CreateModeration<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateModeration(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/moderations";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateModeration,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::CreateModerationResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::CreateModerationResponse>,
    >,
    crate::__types::CreateModerationResponse
);
#[doc = "List all organization and project API keys."]
pub fn admin_api_keys_list<C, Fut, B, E>(
    client: C,
    params: &crate::__types::admin_api_keys_list::Params,
) -> AdminApiKeysList<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysList(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::admin_api_keys_list::params::Order>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::admin_api_keys_list::Params {
                        after,
                        order,
                        limit,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/admin_api_keys");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysList,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ApiKeyList>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ApiKeyList>,
    >,
    crate::__types::ApiKeyList
);
#[doc = "Create admin API key"]
pub fn admin_api_keys_create<C, Fut, B, E>(
    client: C,
    request: &crate::__types::admin_api_keys_create::Request,
) -> AdminApiKeysCreate<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysCreate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/admin_api_keys";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysCreate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
    >,
    crate::__types::AdminApiKey
);
#[doc = "Retrieve admin API key"]
pub fn admin_api_keys_get<C, Fut, B, E>(
    client: C,
    params: &crate::__types::admin_api_keys_get::Params,
) -> AdminApiKeysGet<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysGet(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::admin_api_keys_get::Params { key_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/admin_api_keys/{key_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysGet,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::AdminApiKey>,
    >,
    crate::__types::AdminApiKey
);
#[doc = "Delete admin API key"]
pub fn admin_api_keys_delete<C, Fut, B, E>(
    client: C,
    params: &crate::__types::admin_api_keys_delete::Params,
) -> AdminApiKeysDelete<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    AdminApiKeysDelete(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::admin_api_keys_delete::Params { key_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/admin_api_keys/{key_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    AdminApiKeysDelete,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::admin_api_keys_delete::Response>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::admin_api_keys_delete::Response>,
    >,
    crate::__types::admin_api_keys_delete::Response
);
#[doc = "List audit logs"]
pub fn list_audit_logs<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_audit_logs::Params,
) -> ListAuditLogs<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListAuditLogs(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "effective_at")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        effective_at:
                            &'a Option<crate::__types::list_audit_logs::params::EffectiveAt>,
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
                    let crate::__types::list_audit_logs::Params {
                        effective_at,
                        project_ids,
                        event_types,
                        actor_ids,
                        actor_emails,
                        resource_ids,
                        limit,
                        after,
                        before,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/audit_logs");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListAuditLogs,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListAuditLogsResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListAuditLogsResponse>,
    >,
    crate::__types::ListAuditLogsResponse
);
#[doc = "List organization certificates"]
pub fn list_organization_certificates<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_organization_certificates::Params,
) -> ListOrganizationCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<
                            crate::__types::list_organization_certificates::params::Order,
                        >,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_organization_certificates::Params {
                        limit,
                        after,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/certificates");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Upload certificate"]
pub fn upload_certificate<C, Fut, B, E>(
    client: C,
    request: &crate::__types::UploadCertificateRequest,
) -> UploadCertificate<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UploadCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/certificates";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UploadCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Activate certificates for organization"]
pub fn activate_organization_certificates<C, Fut, B, E>(
    client: C,
    request: &crate::__types::ToggleCertificatesRequest,
) -> ActivateOrganizationCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ActivateOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/certificates/activate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ActivateOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Deactivate certificates for organization"]
pub fn deactivate_organization_certificates<C, Fut, B, E>(
    client: C,
    request: &crate::__types::ToggleCertificatesRequest,
) -> DeactivateOrganizationCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeactivateOrganizationCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/certificates/deactivate";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeactivateOrganizationCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Get certificate"]
pub fn get_certificate<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_certificate::Params,
) -> GetCertificate<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include:
                            &'a Option<Vec<crate::__types::get_certificate::params::include::Item>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_certificate::Params {
                        certificate_id,
                        include,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/certificates/{certificate_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Modify certificate"]
pub fn modify_certificate<C, Fut, B, E>(
    client: C,
    request: &crate::__types::ModifyCertificateRequest,
) -> ModifyCertificate<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/certificates/{certificate_id}";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Certificate>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Certificate>,
    >,
    crate::__types::Certificate
);
#[doc = "Delete certificate"]
pub fn delete_certificate<C, Fut, B, E>(client: C) -> DeleteCertificate<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteCertificate(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/certificates/{certificate_id}";
                let body = String::new();
                Ok(http::Request::builder()
                    .method(http::Method::DELETE)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteCertificate,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteCertificateResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteCertificateResponse>,
    >,
    crate::__types::DeleteCertificateResponse
);
#[doc = "Costs"]
pub fn usage_costs<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_costs::Params,
) -> UsageCosts<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCosts(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        bucket_width: &'a Option<crate::__types::usage_costs::params::BucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by:
                            &'a Option<Vec<crate::__types::usage_costs::params::group_by::Item>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::usage_costs::Params {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/costs");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCosts,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "List invites"]
pub fn list_invites<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_invites::Params,
) -> ListInvites<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListInvites(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_invites::Params { limit, after } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/invites");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListInvites,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::InviteListResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::InviteListResponse>,
    >,
    crate::__types::InviteListResponse
);
#[doc = "Create invite"]
pub fn invite_user<C, Fut, B, E>(
    client: C,
    request: &crate::__types::InviteRequest,
) -> InviteUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    InviteUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/invites";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    InviteUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Invite>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Invite>,
    >,
    crate::__types::Invite
);
#[doc = "Retrieve invite"]
pub fn retrieve_invite<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_invite::Params,
) -> RetrieveInvite<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveInvite(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_invite::Params { invite_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/invites/{invite_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveInvite,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Invite>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Invite>,
    >,
    crate::__types::Invite
);
#[doc = "Delete invite"]
pub fn delete_invite<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_invite::Params,
) -> DeleteInvite<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteInvite(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_invite::Params { invite_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/invites/{invite_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteInvite,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::InviteDeleteResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::InviteDeleteResponse>,
    >,
    crate::__types::InviteDeleteResponse
);
#[doc = "List projects"]
pub fn list_projects<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_projects::Params,
) -> ListProjects<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjects(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_projects::Params {
                        limit,
                        after,
                        include_archived,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjects,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectListResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ProjectListResponse>,
    >,
    crate::__types::ProjectListResponse
);
#[doc = "Create project"]
pub fn create_project<C, Fut, B, E>(
    client: C,
    request: &crate::__types::ProjectCreateRequest,
) -> CreateProject<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/organization/projects";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "Retrieve project"]
pub fn retrieve_project<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_project::Params,
) -> RetrieveProject<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_project::Params { project_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "Modify project"]
pub fn modify_project<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_project::Params,
    request: &crate::__types::ProjectUpdateRequest,
) -> ModifyProject<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_project::Params { project_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "List project API keys"]
pub fn list_project_api_keys<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_project_api_keys::Params,
) -> ListProjectApiKeys<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectApiKeys(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_project_api_keys::Params {
                        project_id,
                        limit,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/api_keys");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectApiKeys,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyListResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyListResponse>,
    >,
    crate::__types::ProjectApiKeyListResponse
);
#[doc = "Retrieve project API key"]
pub fn retrieve_project_api_key<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_project_api_key::Params,
) -> RetrieveProjectApiKey<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectApiKey(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_project_api_key::Params { project_id, key_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/api_keys/{key_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectApiKey,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKey>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKey>,
    >,
    crate::__types::ProjectApiKey
);
#[doc = "Delete project API key"]
pub fn delete_project_api_key<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_project_api_key::Params,
) -> DeleteProjectApiKey<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectApiKey(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_project_api_key::Params { project_id, key_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/api_keys/{key_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteProjectApiKey,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyDeleteResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectApiKeyDeleteResponse>,
    >,
    crate::__types::ProjectApiKeyDeleteResponse
);
#[doc = "Archive project"]
pub fn archive_project<C, Fut, B, E>(
    client: C,
    params: &crate::__types::archive_project::Params,
) -> ArchiveProject<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ArchiveProject(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::archive_project::Params { project_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/archive");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ArchiveProject,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Project>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Project>,
    >,
    crate::__types::Project
);
#[doc = "List project certificates"]
pub fn list_project_certificates<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_project_certificates::Params,
) -> ListProjectCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_project_certificates::params::Order>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_project_certificates::Params {
                        project_id,
                        limit,
                        after,
                        order,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/certificates");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Activate certificates for project"]
pub fn activate_project_certificates<C, Fut, B, E>(
    client: C,
    params: &crate::__types::activate_project_certificates::Params,
    request: &crate::__types::ToggleCertificatesRequest,
) -> ActivateProjectCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ActivateProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::activate_project_certificates::Params { project_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/projects/{project_id}/certificates/activate");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ActivateProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "Deactivate certificates for project"]
pub fn deactivate_project_certificates<C, Fut, B, E>(
    client: C,
    params: &crate::__types::deactivate_project_certificates::Params,
    request: &crate::__types::ToggleCertificatesRequest,
) -> DeactivateProjectCertificates<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeactivateProjectCertificates(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::deactivate_project_certificates::Params { project_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/projects/{project_id}/certificates/deactivate");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeactivateProjectCertificates,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListCertificatesResponse>,
    >,
    crate::__types::ListCertificatesResponse
);
#[doc = "List project rate limits"]
pub fn list_project_rate_limits<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_project_rate_limits::Params,
) -> ListProjectRateLimits<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectRateLimits(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_project_rate_limits::Params {
                        project_id,
                        limit,
                        after,
                        before,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/rate_limits");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectRateLimits,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectRateLimitListResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectRateLimitListResponse>,
    >,
    crate::__types::ProjectRateLimitListResponse
);
#[doc = "Modify project rate limit"]
pub fn update_project_rate_limits<C, Fut, B, E>(
    client: C,
    params: &crate::__types::update_project_rate_limits::Params,
    request: &crate::__types::ProjectRateLimitUpdateRequest,
) -> UpdateProjectRateLimits<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateProjectRateLimits(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::update_project_rate_limits::Params {
                        project_id,
                        rate_limit_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/organization/projects/{project_id}/rate_limits/{rate_limit_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateProjectRateLimits,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectRateLimit>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ProjectRateLimit>,
    >,
    crate::__types::ProjectRateLimit
);
#[doc = "List project service accounts"]
pub fn list_project_service_accounts<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_project_service_accounts::Params,
) -> ListProjectServiceAccounts<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectServiceAccounts(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_project_service_accounts::Params {
                        project_id,
                        limit,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/service_accounts");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountListResponse>,
    >,
    crate::__types::ProjectServiceAccountListResponse
);
#[doc = "Create project service account"]
pub fn create_project_service_account<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_project_service_account::Params,
    request: &crate::__types::ProjectServiceAccountCreateRequest,
) -> CreateProjectServiceAccount<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_project_service_account::Params { project_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/service_accounts");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountCreateResponse>,
    >,
    crate::__types::ProjectServiceAccountCreateResponse
);
#[doc = "Retrieve project service account"]
pub fn retrieve_project_service_account<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_project_service_account::Params,
) -> RetrieveProjectServiceAccount<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_project_service_account::Params {
                        project_id,
                        service_account_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts/{service_account_id}"
                    );
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectServiceAccount,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccount>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccount>,
    >,
    crate::__types::ProjectServiceAccount
);
#[doc = "Delete project service account"]
pub fn delete_project_service_account<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_project_service_account::Params,
) -> DeleteProjectServiceAccount<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectServiceAccount(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_project_service_account::Params {
                        project_id,
                        service_account_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!(
                        "/organization/projects/{project_id}/service_accounts/{service_account_id}"
                    );
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ProjectServiceAccountDeleteResponse>,
    >,
    crate::__types::ProjectServiceAccountDeleteResponse
);
#[doc = "List project users"]
pub fn list_project_users<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_project_users::Params,
) -> ListProjectUsers<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListProjectUsers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_project_users::Params {
                        project_id,
                        limit,
                        after,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/users");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListProjectUsers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUserListResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ProjectUserListResponse>,
    >,
    crate::__types::ProjectUserListResponse
);
#[doc = "Create project user"]
pub fn create_project_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_project_user::Params,
    request: &crate::__types::ProjectUserCreateRequest,
) -> CreateProjectUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_project_user::Params { project_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/users");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Retrieve project user"]
pub fn retrieve_project_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_project_user::Params,
) -> RetrieveProjectUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_project_user::Params {
                        project_id,
                        user_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/users/{user_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Modify project user"]
pub fn modify_project_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_project_user::Params,
    request: &crate::__types::ProjectUserUpdateRequest,
) -> ModifyProjectUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_project_user::Params {
                        project_id,
                        user_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/users/{user_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ProjectUser>,
    >,
    crate::__types::ProjectUser
);
#[doc = "Delete project user"]
pub fn delete_project_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_project_user::Params,
) -> DeleteProjectUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteProjectUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_project_user::Params {
                        project_id,
                        user_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/projects/{project_id}/users/{user_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteProjectUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ProjectUserDeleteResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ProjectUserDeleteResponse>,
    >,
    crate::__types::ProjectUserDeleteResponse
);
#[doc = "Audio speeches"]
pub fn usage_audio_speeches<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_audio_speeches::Params,
) -> UsageAudioSpeeches<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageAudioSpeeches(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            &'a Option<crate::__types::usage_audio_speeches::params::BucketWidth>,
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
                        group_by: &'a Option<
                            Vec<crate::__types::usage_audio_speeches::params::group_by::Item>,
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
                    let crate::__types::usage_audio_speeches::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/audio_speeches");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageAudioSpeeches,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Audio transcriptions"]
pub fn usage_audio_transcriptions<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_audio_transcriptions::Params,
) -> UsageAudioTranscriptions<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageAudioTranscriptions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            crate::__types::usage_audio_transcriptions::params::BucketWidth,
                        >,
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
                        group_by: &'a Option<
                            Vec<crate::__types::usage_audio_transcriptions::params::group_by::Item>,
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
                    let crate::__types::usage_audio_transcriptions::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/audio_transcriptions");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageAudioTranscriptions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Code interpreter sessions"]
pub fn usage_code_interpreter_sessions<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_code_interpreter_sessions::Params,
) -> UsageCodeInterpreterSessions<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCodeInterpreterSessions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query < 'a > { # [serde (rename = "start_time")] start_time : & 'a i64 , # [serde (rename = "end_time")] # [serde (skip_serializing_if = "Option::is_none")] end_time : & 'a Option < i64 > , # [serde (rename = "bucket_width")] # [serde (skip_serializing_if = "Option::is_none")] bucket_width : & 'a Option < crate :: __types :: usage_code_interpreter_sessions :: params :: BucketWidth > , # [serde (rename = "project_ids")] # [serde (skip_serializing_if = "Option::is_none")] project_ids : & 'a Option < Vec < String > > , # [serde (rename = "group_by")] # [serde (skip_serializing_if = "Option::is_none")] group_by : & 'a Option < Vec < crate :: __types :: usage_code_interpreter_sessions :: params :: group_by :: Item > > , # [serde (rename = "limit")] # [serde (skip_serializing_if = "Option::is_none")] limit : & 'a Option < i64 > , # [serde (rename = "page")] # [serde (skip_serializing_if = "Option::is_none")] page : & 'a Option < String > , # [serde (skip_serializing)] _phantom : std :: marker :: PhantomData < & 'a () > , }
                    let crate::__types::usage_code_interpreter_sessions::Params {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/code_interpreter_sessions");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCodeInterpreterSessions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Completions"]
pub fn usage_completions<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_completions::Params,
) -> UsageCompletions<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageCompletions(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            &'a Option<crate::__types::usage_completions::params::BucketWidth>,
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
                        group_by: &'a Option<
                            Vec<crate::__types::usage_completions::params::group_by::Item>,
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
                    let crate::__types::usage_completions::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/completions");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageCompletions,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Embeddings"]
pub fn usage_embeddings<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_embeddings::Params,
) -> UsageEmbeddings<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageEmbeddings(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            &'a Option<crate::__types::usage_embeddings::params::BucketWidth>,
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
                        group_by: &'a Option<
                            Vec<crate::__types::usage_embeddings::params::group_by::Item>,
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
                    let crate::__types::usage_embeddings::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/embeddings");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageEmbeddings,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Images"]
pub fn usage_images<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_images::Params,
) -> UsageImages<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageImages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        bucket_width: &'a Option<crate::__types::usage_images::params::BucketWidth>,
                        #[serde(rename = "sources")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        sources:
                            &'a Option<Vec<crate::__types::usage_images::params::sources::Item>>,
                        #[serde(rename = "sizes")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        sizes: &'a Option<Vec<crate::__types::usage_images::params::sizes::Item>>,
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
                            &'a Option<Vec<crate::__types::usage_images::params::group_by::Item>>,
                        #[serde(rename = "limit")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        limit: &'a Option<i64>,
                        #[serde(rename = "page")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        page: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::usage_images::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/images");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageImages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Moderations"]
pub fn usage_moderations<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_moderations::Params,
) -> UsageModerations<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageModerations(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            &'a Option<crate::__types::usage_moderations::params::BucketWidth>,
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
                        group_by: &'a Option<
                            Vec<crate::__types::usage_moderations::params::group_by::Item>,
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
                    let crate::__types::usage_moderations::Params {
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
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/moderations");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageModerations,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "Vector stores"]
pub fn usage_vector_stores<C, Fut, B, E>(
    client: C,
    params: &crate::__types::usage_vector_stores::Params,
) -> UsageVectorStores<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UsageVectorStores(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                            &'a Option<crate::__types::usage_vector_stores::params::BucketWidth>,
                        #[serde(rename = "project_ids")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        project_ids: &'a Option<Vec<String>>,
                        #[serde(rename = "group_by")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        group_by: &'a Option<
                            Vec<crate::__types::usage_vector_stores::params::group_by::Item>,
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
                    let crate::__types::usage_vector_stores::Params {
                        start_time,
                        end_time,
                        bucket_width,
                        project_ids,
                        group_by,
                        limit,
                        page,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/usage/vector_stores");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UsageVectorStores,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UsageResponse>,
    >,
    crate::__types::UsageResponse
);
#[doc = "List users"]
pub fn list_users<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_users::Params,
) -> ListUsers<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListUsers(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                    let crate::__types::list_users::Params {
                        limit,
                        after,
                        emails,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/users");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListUsers,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UserListResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::UserListResponse>,
    >,
    crate::__types::UserListResponse
);
#[doc = "Retrieve user"]
pub fn retrieve_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_user::Params,
) -> RetrieveUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_user::Params { user_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/users/{user_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::User>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::User>,
    >,
    crate::__types::User
);
#[doc = "Modify user"]
pub fn modify_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_user::Params,
    request: &crate::__types::UserRoleUpdateRequest,
) -> ModifyUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_user::Params { user_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/users/{user_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::User>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::User>,
    >,
    crate::__types::User
);
#[doc = "Delete user"]
pub fn delete_user<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_user::Params,
) -> DeleteUser<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteUser(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_user::Params { user_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/organization/users/{user_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteUser,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::UserDeleteResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::UserDeleteResponse>,
    >,
    crate::__types::UserDeleteResponse
);
#[doc = "Create session"]
pub fn create_realtime_session<C, Fut, B, E>(
    client: C,
    request: &crate::__types::RealtimeSessionCreateRequest,
) -> CreateRealtimeSession<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRealtimeSession(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/realtime/sessions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateRealtimeSession,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RealtimeSessionCreateResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::RealtimeSessionCreateResponse>,
    >,
    crate::__types::RealtimeSessionCreateResponse
);
#[doc = "Create transcription session"]
pub fn create_realtime_transcription_session<C, Fut, B, E>(
    client: C,
    request: &crate::__types::RealtimeTranscriptionSessionCreateRequest,
) -> CreateRealtimeTranscriptionSession<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRealtimeTranscriptionSession(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/realtime/transcription_sessions";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
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
            http::Response<B>,
        ) -> crate::__combinators::Json<
            B,
            E,
            crate::__types::RealtimeTranscriptionSessionCreateResponse,
        >,
    >,
    crate::__types::RealtimeTranscriptionSessionCreateResponse
);
#[doc = "Create a model response"]
pub fn create_response<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateResponse,
) -> CreateResponse<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/responses";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "Create a model response"]
pub fn create_response_stream<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateResponse,
) -> CreateResponseStream<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateResponseStream(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/responses";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::TEXT_EVENT_STREAM),
            ),
        ),
        crate::__combinators::EventStream::new,
    ))
}
future ! (CreateResponseStream , futures :: future :: MapOk < crate :: __combinators :: Send < Fut , B , E > , fn (http :: Response < B >) -> crate :: __combinators :: EventStream < B , crate :: __types :: ResponseStreamEvent > , > , crate :: __combinators :: EventStream < B , crate :: __types :: ResponseStreamEvent >);
#[doc = "Get a model response"]
pub fn get_response<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_response::Params,
) -> GetResponse<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        #[serde(rename = "include_obfuscation")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include_obfuscation: &'a Option<bool>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_response::Params {
                        response_id,
                        include,
                        stream,
                        starting_after,
                        include_obfuscation,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/responses/{response_id}");
                    let query = serde_urlencoded::to_string(Query {
                        include,
                        stream,
                        starting_after,
                        include_obfuscation,
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "Delete a model response"]
pub fn delete_response<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_response::Params,
) -> DeleteResponse<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteResponse(futures::TryFutureExt::map_ok(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_response::Params { response_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/responses/{response_id}");
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
            (http::StatusCode::from_u16(200u16).unwrap(), None),
        ),
        |_| (),
    ))
}
future!(
    DeleteResponse,
    futures::future::MapOk<crate::__combinators::Send<Fut, B, E>, fn(http::Response<B>) -> ()>,
    ()
);
#[doc = "Cancel a response"]
pub fn cancel_response<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_response::Params,
) -> CancelResponse<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelResponse(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_response::Params { response_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/responses/{response_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelResponse,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Response>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Response>,
    >,
    crate::__types::Response
);
#[doc = "List input items"]
pub fn list_input_items<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_input_items::Params,
) -> ListInputItems<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListInputItems(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_input_items::params::Order>,
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
                    let crate::__types::list_input_items::Params {
                        response_id,
                        limit,
                        order,
                        after,
                        before,
                        include,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/responses/{response_id}/input_items");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListInputItems,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ResponseItemList>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ResponseItemList>,
    >,
    crate::__types::ResponseItemList
);
#[doc = "Create thread and run"]
pub fn create_thread_and_run<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateThreadAndRunRequest,
) -> CreateThreadAndRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateThreadAndRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/threads/runs";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateThreadAndRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Retrieve thread"]
pub fn get_thread<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_thread::Params,
) -> GetThread<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_thread::Params { thread_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
    >,
    crate::__types::ThreadObject
);
#[doc = "Modify thread"]
pub fn modify_thread<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_thread::Params,
    request: &crate::__types::ModifyThreadRequest,
) -> ModifyThread<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_thread::Params { thread_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ThreadObject>,
    >,
    crate::__types::ThreadObject
);
#[doc = "Delete thread"]
pub fn delete_thread<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_thread::Params,
) -> DeleteThread<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteThread(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_thread::Params { thread_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteThread,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteThreadResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteThreadResponse>,
    >,
    crate::__types::DeleteThreadResponse
);
#[doc = "List messages"]
pub fn list_messages<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_messages::Params,
) -> ListMessages<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListMessages(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_messages::params::Order>,
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
                    let crate::__types::list_messages::Params {
                        thread_id,
                        limit,
                        order,
                        after,
                        before,
                        run_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/messages");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListMessages,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListMessagesResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListMessagesResponse>,
    >,
    crate::__types::ListMessagesResponse
);
#[doc = "Create message"]
pub fn create_message<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_message::Params,
    request: &crate::__types::CreateMessageRequest,
) -> CreateMessage<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_message::Params { thread_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/messages");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Retrieve message"]
pub fn get_message<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_message::Params,
) -> GetMessage<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_message::Params {
                        thread_id,
                        message_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/messages/{message_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Modify message"]
pub fn modify_message<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_message::Params,
    request: &crate::__types::ModifyMessageRequest,
) -> ModifyMessage<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_message::Params {
                        thread_id,
                        message_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/messages/{message_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::MessageObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::MessageObject>,
    >,
    crate::__types::MessageObject
);
#[doc = "Delete message"]
pub fn delete_message<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_message::Params,
) -> DeleteMessage<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteMessage(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_message::Params {
                        thread_id,
                        message_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/messages/{message_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteMessage,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteMessageResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteMessageResponse>,
    >,
    crate::__types::DeleteMessageResponse
);
#[doc = "List runs"]
pub fn list_runs<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_runs::Params,
) -> ListRuns<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListRuns(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_runs::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_runs::Params {
                        thread_id,
                        limit,
                        order,
                        after,
                        before,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListRuns,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListRunsResponse>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::ListRunsResponse>,
    >,
    crate::__types::ListRunsResponse
);
#[doc = "Create run"]
pub fn create_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_run::Params,
    request: &crate::__types::CreateRunRequest,
) -> CreateRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include: &'a Option<Vec<crate::__types::create_run::params::include::Item>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_run::Params { thread_id, include } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Retrieve run"]
pub fn get_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_run::Params,
) -> GetRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_run::Params { thread_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs/{run_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Modify run"]
pub fn modify_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_run::Params,
    request: &crate::__types::ModifyRunRequest,
) -> ModifyRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_run::Params { thread_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs/{run_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Cancel a run"]
pub fn cancel_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_run::Params,
) -> CancelRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_run::Params { thread_id, run_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs/{run_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "List run steps"]
pub fn list_run_steps<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_run_steps::Params,
) -> ListRunSteps<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListRunSteps(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_run_steps::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include:
                            &'a Option<Vec<crate::__types::list_run_steps::params::include::Item>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_run_steps::Params {
                        thread_id,
                        run_id,
                        limit,
                        order,
                        after,
                        before,
                        include,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs/{run_id}/steps");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListRunSteps,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListRunStepsResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListRunStepsResponse>,
    >,
    crate::__types::ListRunStepsResponse
);
#[doc = "Retrieve run step"]
pub fn get_run_step<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_run_step::Params,
) -> GetRunStep<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetRunStep(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(rename = "include[]")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        include:
                            &'a Option<Vec<crate::__types::get_run_step::params::include::Item>>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_run_step::Params {
                        thread_id,
                        run_id,
                        step_id,
                        include,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/threads/{thread_id}/runs/{run_id}/steps/{step_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetRunStep,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunStepObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunStepObject>,
    >,
    crate::__types::RunStepObject
);
#[doc = "Submit tool outputs to run"]
pub fn submit_tool_ouputs_to_run<C, Fut, B, E>(
    client: C,
    params: &crate::__types::submit_tool_ouputs_to_run::Params,
    request: &crate::__types::SubmitToolOutputsRunRequest,
) -> SubmitToolOuputsToRun<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    SubmitToolOuputsToRun(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::submit_tool_ouputs_to_run::Params { thread_id, run_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/threads/{thread_id}/runs/{run_id}/submit_tool_outputs");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    SubmitToolOuputsToRun,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::RunObject>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::RunObject>,
    >,
    crate::__types::RunObject
);
#[doc = "Create upload"]
pub fn create_upload<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateUploadRequest,
) -> CreateUpload<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/uploads";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "Cancel upload"]
pub fn cancel_upload<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_upload::Params,
) -> CancelUpload<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_upload::Params { upload_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/uploads/{upload_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "Complete upload"]
pub fn complete_upload<C, Fut, B, E>(
    client: C,
    params: &crate::__types::complete_upload::Params,
    request: &crate::__types::CompleteUploadRequest,
) -> CompleteUpload<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CompleteUpload(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::complete_upload::Params { upload_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/uploads/{upload_id}/complete");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CompleteUpload,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::Upload>,
        fn(http::Response<B>) -> crate::__combinators::Json<B, E, crate::__types::Upload>,
    >,
    crate::__types::Upload
);
#[doc = "List vector stores"]
pub fn list_vector_stores<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_vector_stores::Params,
) -> ListVectorStores<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListVectorStores(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_vector_stores::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_vector_stores::Params {
                        limit,
                        order,
                        after,
                        before,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListVectorStores,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoresResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoresResponse>,
    >,
    crate::__types::ListVectorStoresResponse
);
#[doc = "Create vector store"]
pub fn create_vector_store<C, Fut, B, E>(
    client: C,
    request: &crate::__types::CreateVectorStoreRequest,
) -> CreateVectorStore<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = "/vector_stores";
                let body = serde_json::to_string(request)?;
                Ok(http::Request::builder()
                    .method(http::Method::POST)
                    .uri(path)
                    .header(http::header::CONTENT_LENGTH, body.len())
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Retrieve vector store"]
pub fn get_vector_store<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_vector_store::Params,
) -> GetVectorStore<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_vector_store::Params { vector_store_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Modify vector store"]
pub fn modify_vector_store<C, Fut, B, E>(
    client: C,
    params: &crate::__types::modify_vector_store::Params,
    request: &crate::__types::UpdateVectorStoreRequest,
) -> ModifyVectorStore<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ModifyVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::modify_vector_store::Params { vector_store_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ModifyVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreObject>,
    >,
    crate::__types::VectorStoreObject
);
#[doc = "Delete vector store"]
pub fn delete_vector_store<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_vector_store::Params,
) -> DeleteVectorStore<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_vector_store::Params { vector_store_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreResponse>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreResponse>,
    >,
    crate::__types::DeleteVectorStoreResponse
);
#[doc = "Create vector store file batch"]
pub fn create_vector_store_file_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_vector_store_file_batch::Params,
    request: &crate::__types::CreateVectorStoreFileBatchRequest,
) -> CreateVectorStoreFileBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_vector_store_file_batch::Params { vector_store_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/file_batches");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "Retrieve vector store file batch"]
pub fn get_vector_store_file_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_vector_store_file_batch::Params,
) -> GetVectorStoreFileBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_vector_store_file_batch::Params {
                        vector_store_id,
                        batch_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/vector_stores/{vector_store_id}/file_batches/{batch_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "Cancel vector store file batch"]
pub fn cancel_vector_store_file_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::cancel_vector_store_file_batch::Params,
) -> CancelVectorStoreFileBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CancelVectorStoreFileBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::cancel_vector_store_file_batch::Params {
                        vector_store_id,
                        batch_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/vector_stores/{vector_store_id}/file_batches/{batch_id}/cancel");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CancelVectorStoreFileBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileBatchObject>,
    >,
    crate::__types::VectorStoreFileBatchObject
);
#[doc = "List vector store files in a batch"]
pub fn list_files_in_vector_store_batch<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_files_in_vector_store_batch::Params,
) -> ListFilesInVectorStoreBatch<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListFilesInVectorStoreBatch(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<
                            crate::__types::list_files_in_vector_store_batch::params::Order,
                        >,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "filter")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        filter: &'a Option<
                            crate::__types::list_files_in_vector_store_batch::params::Filter,
                        >,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_files_in_vector_store_batch::Params {
                        vector_store_id,
                        batch_id,
                        limit,
                        order,
                        after,
                        before,
                        filter,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/vector_stores/{vector_store_id}/file_batches/{batch_id}/files");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListFilesInVectorStoreBatch,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
    >,
    crate::__types::ListVectorStoreFilesResponse
);
#[doc = "List vector store files"]
pub fn list_vector_store_files<C, Fut, B, E>(
    client: C,
    params: &crate::__types::list_vector_store_files::Params,
) -> ListVectorStoreFiles<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    ListVectorStoreFiles(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
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
                        order: &'a Option<crate::__types::list_vector_store_files::params::Order>,
                        #[serde(rename = "after")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        after: &'a Option<String>,
                        #[serde(rename = "before")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        before: &'a Option<String>,
                        #[serde(rename = "filter")]
                        #[serde(skip_serializing_if = "Option::is_none")]
                        filter: &'a Option<crate::__types::list_vector_store_files::params::Filter>,
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::list_vector_store_files::Params {
                        vector_store_id,
                        limit,
                        order,
                        after,
                        before,
                        filter,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/files");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    ListVectorStoreFiles,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::ListVectorStoreFilesResponse>,
    >,
    crate::__types::ListVectorStoreFilesResponse
);
#[doc = "Create vector store file"]
pub fn create_vector_store_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::create_vector_store_file::Params,
    request: &crate::__types::CreateVectorStoreFileRequest,
) -> CreateVectorStoreFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    CreateVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::create_vector_store_file::Params { vector_store_id } =
                        params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/files");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    CreateVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Retrieve vector store file"]
pub fn get_vector_store_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::get_vector_store_file::Params,
) -> GetVectorStoreFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    GetVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::get_vector_store_file::Params {
                        vector_store_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/files/{file_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    GetVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Delete vector store file"]
pub fn delete_vector_store_file<C, Fut, B, E>(
    client: C,
    params: &crate::__types::delete_vector_store_file::Params,
) -> DeleteVectorStoreFile<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    DeleteVectorStoreFile(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::delete_vector_store_file::Params {
                        vector_store_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/files/{file_id}");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    DeleteVectorStoreFile,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreFileResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::DeleteVectorStoreFileResponse>,
    >,
    crate::__types::DeleteVectorStoreFileResponse
);
#[doc = "Update vector store file attributes"]
pub fn update_vector_store_file_attributes<C, Fut, B, E>(
    client: C,
    params: &crate::__types::update_vector_store_file_attributes::Params,
    request: &crate::__types::UpdateVectorStoreFileAttributesRequest,
) -> UpdateVectorStoreFileAttributes<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    UpdateVectorStoreFileAttributes(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::update_vector_store_file_attributes::Params {
                        vector_store_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/files/{file_id}");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    UpdateVectorStoreFileAttributes,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
        fn(
            http::Response<B>,
        ) -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileObject>,
    >,
    crate::__types::VectorStoreFileObject
);
#[doc = "Retrieve vector store file content"]
pub fn retrieve_vector_store_file_content<C, Fut, B, E>(
    client: C,
    params: &crate::__types::retrieve_vector_store_file_content::Params,
) -> RetrieveVectorStoreFileContent<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    RetrieveVectorStoreFileContent(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::retrieve_vector_store_file_content::Params {
                        vector_store_id,
                        file_id,
                    } = params;
                    #[allow(clippy::useless_format)]
                    let mut path =
                        format!("/vector_stores/{vector_store_id}/files/{file_id}/content");
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
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    RetrieveVectorStoreFileContent,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreFileContentResponse>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::VectorStoreFileContentResponse>,
    >,
    crate::__types::VectorStoreFileContentResponse
);
#[doc = "Search vector store"]
pub fn search_vector_store<C, Fut, B, E>(
    client: C,
    params: &crate::__types::search_vector_store::Params,
    request: &crate::__types::VectorStoreSearchRequest,
) -> SearchVectorStore<Fut, B, E>
where
    C: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    SearchVectorStore(futures::TryFutureExt::and_then(
        crate::__combinators::Send::new(
            client,
            || {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde :: Serialize)]
                    struct Query<'a> {
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }
                    let crate::__types::search_vector_store::Params { vector_store_id } = params;
                    #[allow(clippy::useless_format)]
                    let mut path = format!("/vector_stores/{vector_store_id}/search");
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
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(body)?)
            },
            (
                http::StatusCode::from_u16(200u16).unwrap(),
                Some(mime::APPLICATION_JSON),
            ),
        ),
        crate::__combinators::Json::new,
    ))
}
future!(
    SearchVectorStore,
    futures::future::AndThen<
        crate::__combinators::Send<Fut, B, E>,
        crate::__combinators::Json<B, E, crate::__types::VectorStoreSearchResultsPage>,
        fn(
            http::Response<B>,
        )
            -> crate::__combinators::Json<B, E, crate::__types::VectorStoreSearchResultsPage>,
    >,
    crate::__types::VectorStoreSearchResultsPage
);
