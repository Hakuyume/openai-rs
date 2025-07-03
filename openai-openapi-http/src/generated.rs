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
                let path = format!(
                    "/assistants/{assistant_id}",
                    assistant_id = params.assistant_id
                );
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
                let path = format!(
                    "/chat/completions/{completion_id}",
                    completion_id = params.completion_id
                );
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
                let path = format!("/evals/{eval_id}", eval_id = params.eval_id);
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
                let path = format!("/evals/{eval_id}/runs", eval_id = params.eval_id);
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
                let path = format!(
                    "/fine_tuning/checkpoints/{fine_tuned_model_checkpoint}/permissions",
                    fine_tuned_model_checkpoint = params.fine_tuned_model_checkpoint
                );
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
                let path = format!(
                    "/organization/projects/{project_id}",
                    project_id = params.project_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/certificates/activate",
                    project_id = params.project_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/certificates/deactivate",
                    project_id = params.project_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/rate_limits/{rate_limit_id}",
                    project_id = params.project_id,
                    rate_limit_id = params.rate_limit_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/service_accounts",
                    project_id = params.project_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/users",
                    project_id = params.project_id
                );
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
                let path = format!(
                    "/organization/projects/{project_id}/users/{user_id}",
                    project_id = params.project_id,
                    user_id = params.user_id
                );
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
                let path = format!("/organization/users/{user_id}", user_id = params.user_id);
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
                let path = format!("/threads/{thread_id}", thread_id = params.thread_id);
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
                let path = format!(
                    "/threads/{thread_id}/messages",
                    thread_id = params.thread_id
                );
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
                let path = format!(
                    "/threads/{thread_id}/messages/{message_id}",
                    thread_id = params.thread_id,
                    message_id = params.message_id
                );
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
                    }
                    let mut path =
                        format!("/threads/{thread_id}/runs", thread_id = params.thread_id);
                    let crate::__types::CreateRunParams { include, .. } = params;
                    let query = serde_urlencoded::to_string(Query { include })?;
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
                let path = format!(
                    "/threads/{thread_id}/runs/{run_id}",
                    thread_id = params.thread_id,
                    run_id = params.run_id
                );
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
                let path = format!(
                    "/threads/{thread_id}/runs/{run_id}/submit_tool_outputs",
                    thread_id = params.thread_id,
                    run_id = params.run_id
                );
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
                let path = format!(
                    "/uploads/{upload_id}/complete",
                    upload_id = params.upload_id
                );
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
                let path = format!(
                    "/vector_stores/{vector_store_id}",
                    vector_store_id = params.vector_store_id
                );
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
                let path = format!(
                    "/vector_stores/{vector_store_id}/file_batches",
                    vector_store_id = params.vector_store_id
                );
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
                let path = format!(
                    "/vector_stores/{vector_store_id}/files",
                    vector_store_id = params.vector_store_id
                );
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
                let path = format!(
                    "/vector_stores/{vector_store_id}/files/{file_id}",
                    vector_store_id = params.vector_store_id,
                    file_id = params.file_id
                );
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
                let path = format!(
                    "/vector_stores/{vector_store_id}/search",
                    vector_store_id = params.vector_store_id
                );
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
