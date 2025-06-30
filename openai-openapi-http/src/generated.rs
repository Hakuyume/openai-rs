#[doc = "Generates audio from the input text."]
pub fn create_speech_stream<S, Fut, B, E>(
    service: S,
    body: &crate::types::CreateSpeechRequest,
) -> CreateSpeechStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    futures::TryFutureExt::map_ok(
        crate::SendFuture::new(service, http::Method::POST, "/audio/speech", body),
        crate::EventStream::new,
    )
}
pub type CreateSpeechStream<Fut, B, E> = futures::future::MapOk<
    crate::SendFuture<Fut, B, E>,
    fn(B) -> crate::EventStream<B, crate::types::CreateSpeechResponseStreamEvent>,
>;
#[doc = "**Starting a new project?** We recommend trying [Responses](https://platform.openai.com/docs/api-reference/responses) \nto take advantage of the latest OpenAI platform features. Compare\n[Chat Completions with Responses](https://platform.openai.com/docs/guides/responses-vs-chat-completions?api-mode=responses).\n\n---\n\nCreates a model response for the given chat conversation. Learn more in the\n[text generation](https://platform.openai.com/docs/guides/text-generation), [vision](https://platform.openai.com/docs/guides/vision),\nand [audio](https://platform.openai.com/docs/guides/audio) guides.\n\nParameter support can differ depending on the model used to generate the\nresponse, particularly for newer reasoning models. Parameters that are only\nsupported for reasoning models are noted below. For the current state of \nunsupported parameters in reasoning models, \n[refer to the reasoning guide](https://platform.openai.com/docs/guides/reasoning).\n"]
pub fn create_chat_completion_stream<S, Fut, B, E>(
    service: S,
    body: &crate::types::CreateChatCompletionRequest,
) -> CreateChatCompletionStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    futures::TryFutureExt::map_ok(
        crate::SendFuture::new(service, http::Method::POST, "/chat/completions", body),
        crate::EventStream::new,
    )
}
pub type CreateChatCompletionStream<Fut, B, E> = futures::future::MapOk<
    crate::SendFuture<Fut, B, E>,
    fn(B) -> crate::EventStream<B, crate::types::CreateChatCompletionStreamResponse>,
>;
#[doc = "Creates a model response. Provide [text](https://platform.openai.com/docs/guides/text) or\n[image](https://platform.openai.com/docs/guides/images) inputs to generate [text](https://platform.openai.com/docs/guides/text)\nor [JSON](https://platform.openai.com/docs/guides/structured-outputs) outputs. Have the model call\nyour own [custom code](https://platform.openai.com/docs/guides/function-calling) or use built-in\n[tools](https://platform.openai.com/docs/guides/tools) like [web search](https://platform.openai.com/docs/guides/tools-web-search)\nor [file search](https://platform.openai.com/docs/guides/tools-file-search) to use your own data\nas input for the model's response.\n"]
pub fn create_response_stream<S, Fut, B, E>(
    service: S,
    body: &crate::types::CreateResponse,
) -> CreateResponseStream<Fut, B, E>
where
    S: FnOnce(http::Request<String>) -> Fut,
    Fut: Future<Output = Result<http::Response<B>, E>>,
    B: http_body::Body,
{
    futures::TryFutureExt::map_ok(
        crate::SendFuture::new(service, http::Method::POST, "/responses", body),
        crate::EventStream::new,
    )
}
pub type CreateResponseStream<Fut, B, E> = futures::future::MapOk<
    crate::SendFuture<Fut, B, E>,
    fn(B) -> crate::EventStream<B, crate::types::ResponseStreamEvent>,
>;
