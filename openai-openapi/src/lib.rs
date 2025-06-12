#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AddUploadPartRequest {
    #[doc = "The chunk of bytes for this Part.\n"]
    #[serde(rename = "data")]
    pub data: Vec<u8>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AdminApiKeyOwner {
    #[doc = "Always `user`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "The object type, which is always organization.user"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the user"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The Unix timestamp (in seconds) of when the user was created"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "created_at")]
    pub created_at: Option<u64>,
    #[doc = "Always `owner`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "Represents an individual Admin API key in an org."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AdminApiKey {
    #[doc = "The object type, which is always `organization.admin_api_key`"]
    #[serde(rename = "object")]
    pub object: String,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the API key"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The redacted value of the API key"]
    #[serde(rename = "redacted_value")]
    pub redacted_value: String,
    #[doc = "The value of the API key. Only shown on create."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the API key was last used"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_used_at")]
    pub last_used_at: Option<u64>,
    #[builder(default)]
    #[serde(rename = "owner")]
    pub owner: AdminApiKeyOwner,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ApiKeyList {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<Vec<AdminApiKey>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
}
#[doc = "The object type, which is always `assistant`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantObjectObject {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AssistantObjectTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantObjectToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantObjectToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<AssistantObjectToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<AssistantObjectToolResourcesFileSearch>,
}
#[doc = "Represents an `assistant` that can call the model and use tools."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AssistantObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `assistant`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: AssistantObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the assistant was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    #[serde(rename = "tools")]
    pub tools: Vec<AssistantObjectTool>,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<AssistantObjectToolResources>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[doc = "Represents an event emitted when streaming a Run.\n\nEach event in a server-sent events stream has an `event` and `data` property:\n\n```\nevent: thread.created\ndata: {\"id\": \"thread_123\", \"object\": \"thread\", ...}\n```\n\nWe emit events whenever a new object is created, transitions to a new state, or is being\nstreamed in parts (deltas). For example, we emit `thread.run.created` when a new run\nis created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses\nto create a message during a run, we emit a `thread.message.created event`, a\n`thread.message.in_progress` event, many `thread.message.delta` events, and finally a\n`thread.message.completed` event.\n\nWe may add additional events over time, so we recommend handling unknown events gracefully\nin your code. See the [Assistants API quickstart](/docs/assistants/overview) to learn how to\nintegrate the Assistants API with streaming.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AssistantStreamEvent {
    ThreadStreamEvent(ThreadStreamEvent),
    RunStreamEvent(RunStreamEvent),
    RunStepStreamEvent(RunStepStreamEvent),
    MessageStreamEvent(MessageStreamEvent),
    Error(ErrorEvent),
    Done(DoneEvent),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AssistantSupportedModels {
    #[serde(rename = "gpt-4.1")]
    Gpt41,
    #[serde(rename = "gpt-4.1-mini")]
    Gpt41Mini,
    #[serde(rename = "gpt-4.1-nano")]
    Gpt41Nano,
    #[serde(rename = "gpt-4.1-2025-04-14")]
    Gpt4120250414,
    #[serde(rename = "gpt-4.1-mini-2025-04-14")]
    Gpt41Mini20250414,
    #[serde(rename = "gpt-4.1-nano-2025-04-14")]
    Gpt41Nano20250414,
    #[serde(rename = "o3-mini")]
    O3Mini,
    #[serde(rename = "o3-mini-2025-01-31")]
    O3Mini20250131,
    #[serde(rename = "o1")]
    O1,
    #[serde(rename = "o1-2024-12-17")]
    O120241217,
    #[serde(rename = "gpt-4o")]
    Gpt4o,
    #[serde(rename = "gpt-4o-2024-11-20")]
    Gpt4o20241120,
    #[serde(rename = "gpt-4o-2024-08-06")]
    Gpt4o20240806,
    #[serde(rename = "gpt-4o-2024-05-13")]
    Gpt4o20240513,
    #[serde(rename = "gpt-4o-mini")]
    Gpt4oMini,
    #[serde(rename = "gpt-4o-mini-2024-07-18")]
    Gpt4oMini20240718,
    #[serde(rename = "gpt-4.5-preview")]
    Gpt45Preview,
    #[serde(rename = "gpt-4.5-preview-2025-02-27")]
    Gpt45Preview20250227,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "gpt-4-turbo-2024-04-09")]
    Gpt4Turbo20240409,
    #[serde(rename = "gpt-4-0125-preview")]
    Gpt40125Preview,
    #[serde(rename = "gpt-4-turbo-preview")]
    Gpt4TurboPreview,
    #[serde(rename = "gpt-4-1106-preview")]
    Gpt41106Preview,
    #[serde(rename = "gpt-4-vision-preview")]
    Gpt4VisionPreview,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4-0314")]
    Gpt40314,
    #[serde(rename = "gpt-4-0613")]
    Gpt40613,
    #[serde(rename = "gpt-4-32k")]
    Gpt432k,
    #[serde(rename = "gpt-4-32k-0314")]
    Gpt432k0314,
    #[serde(rename = "gpt-4-32k-0613")]
    Gpt432k0613,
    #[serde(rename = "gpt-3.5-turbo")]
    Gpt35Turbo,
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt35Turbo16k,
    #[serde(rename = "gpt-3.5-turbo-0613")]
    Gpt35Turbo0613,
    #[serde(rename = "gpt-3.5-turbo-1106")]
    Gpt35Turbo1106,
    #[serde(rename = "gpt-3.5-turbo-0125")]
    Gpt35Turbo0125,
    #[serde(rename = "gpt-3.5-turbo-16k-0613")]
    Gpt35Turbo16k0613,
}
#[doc = "The type of tool being defined: `code_interpreter`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantToolsCodeType {
    #[default]
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantToolsCode {
    #[doc = "The type of tool being defined: `code_interpreter`"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: AssistantToolsCodeType,
}
#[doc = "The type of tool being defined: `file_search`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantToolsFileSearchType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}
#[doc = "Overrides for the file search tool."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantToolsFileSearchFileSearch {
    #[doc = "The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.\n\nNote that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_num_results")]
    pub max_num_results: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<FileSearchRankingOptions>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantToolsFileSearch {
    #[doc = "The type of tool being defined: `file_search`"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFileSearchType,
    #[doc = "Overrides for the file search tool."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<AssistantToolsFileSearchFileSearch>,
}
#[doc = "The type of tool being defined: `file_search`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantToolsFileSearchTypeOnlyType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AssistantToolsFileSearchTypeOnly {
    #[doc = "The type of tool being defined: `file_search`"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFileSearchTypeOnlyType,
}
#[doc = "The type of tool being defined: `function`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantToolsFunctionType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AssistantToolsFunction {
    #[doc = "The type of tool being defined: `function`"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: AssistantToolsFunctionType,
    #[serde(rename = "function")]
    pub function: FunctionObject,
}
#[doc = "`auto` is the default value\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AssistantsApiResponseFormatOption0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Specifies the format that the model must output. Compatible with [GPT-4o](/docs/models#gpt-4o), [GPT-4 Turbo](/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables JSON mode, which ensures the message the model generates is valid JSON.\n\n**Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly \"stuck\" request. Also note that the message content may be partially cut off if `finish_reason=\"length\"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AssistantsApiResponseFormatOption {
    _0(AssistantsApiResponseFormatOption0),
    Text(ResponseFormatText),
    JsonObject(ResponseFormatJsonObject),
    JsonSchema(ResponseFormatJsonSchema),
}
#[doc = "`none` means the model will not call any tools and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools before responding to the user.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AssistantsApiToolChoiceOption0 {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tools and instead generates a message.\n`auto` is the default value and means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools before responding to the user.\nSpecifying a particular tool like `{\"type\": \"file_search\"}` or `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum AssistantsApiToolChoiceOption {
    _0(AssistantsApiToolChoiceOption0),
    AssistantsNamedToolChoice(AssistantsNamedToolChoice),
}
#[doc = "The type of the tool. If type is `function`, the function name must be set"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AssistantsNamedToolChoiceType {
    #[serde(rename = "function")]
    Function,
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "file_search")]
    FileSearch,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AssistantsNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific tool."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AssistantsNamedToolChoice {
    #[doc = "The type of the tool. If type is `function`, the function name must be set"]
    #[serde(rename = "type")]
    pub type_: AssistantsNamedToolChoiceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function")]
    pub function: Option<AssistantsNamedToolChoiceFunction>,
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is `json`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AudioResponseFormat {
    #[default]
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    #[serde(rename = "vtt")]
    Vtt,
}
#[doc = "The project that the action was scoped to. Absent for actions not scoped to projects."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProject {
    #[doc = "The project ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The project title."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The payload used to create the API key."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogApiKeyCreatedData {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogApiKeyCreated {
    #[doc = "The tracking ID of the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to create the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogApiKeyCreatedData>,
}
#[doc = "The payload used to update the API key."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogApiKeyUpdatedChangesRequested {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "scopes")]
    pub scopes: Option<Vec<String>>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogApiKeyUpdated {
    #[doc = "The tracking ID of the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to update the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogApiKeyUpdatedChangesRequested>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogApiKeyDeleted {
    #[doc = "The tracking ID of the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The payload used to create the checkpoint permission."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCheckpointPermissionCreatedData {
    #[doc = "The ID of the project that the checkpoint permission was created for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "The ID of the fine-tuned model checkpoint."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fine_tuned_model_checkpoint")]
    pub fine_tuned_model_checkpoint: Option<String>,
}
#[doc = "The project and fine-tuned model checkpoint that the checkpoint permission was created for."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCheckpointPermissionCreated {
    #[doc = "The ID of the checkpoint permission."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to create the checkpoint permission."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogCheckpointPermissionCreatedData>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCheckpointPermissionDeleted {
    #[doc = "The ID of the checkpoint permission."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The payload used to create the invite."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogInviteSentData {
    #[doc = "The email invited to the organization."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[doc = "The role the email was invited to be. Is either `owner` or `member`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogInviteSent {
    #[doc = "The ID of the invite."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to create the invite."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogInviteSentData>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogInviteAccepted {
    #[doc = "The ID of the invite."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogInviteDeleted {
    #[doc = "The ID of the invite."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogLoginFailed {
    #[doc = "The error code of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    #[doc = "The error message of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogLogoutFailed {
    #[doc = "The error code of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
    #[doc = "The error message of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error_message")]
    pub error_message: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogOrganizationUpdatedChangesRequestedSettings {
    #[doc = "Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threads_ui_visibility")]
    pub threads_ui_visibility: Option<String>,
    #[doc = "Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage_dashboard_visibility")]
    pub usage_dashboard_visibility: Option<String>,
}
#[doc = "The payload used to update the organization settings."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogOrganizationUpdatedChangesRequested {
    #[doc = "The organization title."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[doc = "The organization description."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The organization name."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "settings")]
    pub settings: Option<AuditLogOrganizationUpdatedChangesRequestedSettings>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogOrganizationUpdated {
    #[doc = "The organization ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to update the organization settings."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogOrganizationUpdatedChangesRequested>,
}
#[doc = "The payload used to create the project."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProjectCreatedData {
    #[doc = "The project name."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The title of the project as seen on the dashboard."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProjectCreated {
    #[doc = "The project ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to create the project."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogProjectCreatedData>,
}
#[doc = "The payload used to update the project."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProjectUpdatedChangesRequested {
    #[doc = "The title of the project as seen on the dashboard."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "title")]
    pub title: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProjectUpdated {
    #[doc = "The project ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to update the project."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogProjectUpdatedChangesRequested>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogProjectArchived {
    #[doc = "The project ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The payload used to update the rate limits."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogRateLimitUpdatedChangesRequested {
    #[doc = "The maximum requests per minute."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_requests_per_1_minute")]
    pub max_requests_per_1_minute: Option<u64>,
    #[doc = "The maximum tokens per minute."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_tokens_per_1_minute")]
    pub max_tokens_per_1_minute: Option<u64>,
    #[doc = "The maximum images per minute. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_images_per_1_minute")]
    pub max_images_per_1_minute: Option<u64>,
    #[doc = "The maximum audio megabytes per minute. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_audio_megabytes_per_1_minute")]
    pub max_audio_megabytes_per_1_minute: Option<u64>,
    #[doc = "The maximum requests per day. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_requests_per_1_day")]
    pub max_requests_per_1_day: Option<u64>,
    #[doc = "The maximum batch input tokens per day. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_1_day_max_input_tokens")]
    pub batch_1_day_max_input_tokens: Option<u64>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogRateLimitUpdated {
    #[doc = "The rate limit ID"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to update the rate limits."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogRateLimitUpdatedChangesRequested>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogRateLimitDeleted {
    #[doc = "The rate limit ID"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The payload used to create the service account."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogServiceAccountCreatedData {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogServiceAccountCreated {
    #[doc = "The service account ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to create the service account."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogServiceAccountCreatedData>,
}
#[doc = "The payload used to updated the service account."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogServiceAccountUpdated {
    #[doc = "The service account ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to updated the service account."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogServiceAccountUpdatedChangesRequested>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogServiceAccountDeleted {
    #[doc = "The service account ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The payload used to add the user to the project."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogUserAddedData {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogUserAdded {
    #[doc = "The user ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to add the user to the project."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<AuditLogUserAddedData>,
}
#[doc = "The payload used to update the user."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogUserUpdatedChangesRequested {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogUserUpdated {
    #[doc = "The project ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The payload used to update the user."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "changes_requested")]
    pub changes_requested: Option<AuditLogUserUpdatedChangesRequested>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogUserDeleted {
    #[doc = "The user ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificateCreated {
    #[doc = "The certificate ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificateUpdated {
    #[doc = "The certificate ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificateDeleted {
    #[doc = "The certificate ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The certificate content in PEM format."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificate")]
    pub certificate: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificatesActivatedCertificate {
    #[doc = "The certificate ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificatesActivated {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificates")]
    pub certificates: Option<Vec<AuditLogCertificatesActivatedCertificate>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificatesDeactivatedCertificate {
    #[doc = "The certificate ID."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The name of the certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The details for events with this `type`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogCertificatesDeactivated {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificates")]
    pub certificates: Option<Vec<AuditLogCertificatesDeactivatedCertificate>>,
}
#[doc = "A log of a user action or configuration change within this organization."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct AuditLog {
    #[doc = "The ID of this log."]
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub type_: AuditLogEventType,
    #[doc = "The Unix timestamp (in seconds) of the event."]
    #[serde(rename = "effective_at")]
    pub effective_at: u64,
    #[doc = "The project that the action was scoped to. Absent for actions not scoped to projects."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project")]
    pub project: Option<AuditLogProject>,
    #[builder(default)]
    #[serde(rename = "actor")]
    pub actor: AuditLogActor,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key.created")]
    pub api_key_created: Option<AuditLogApiKeyCreated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key.updated")]
    pub api_key_updated: Option<AuditLogApiKeyUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key.deleted")]
    pub api_key_deleted: Option<AuditLogApiKeyDeleted>,
    #[doc = "The project and fine-tuned model checkpoint that the checkpoint permission was created for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "checkpoint_permission.created")]
    pub checkpoint_permission_created: Option<AuditLogCheckpointPermissionCreated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "checkpoint_permission.deleted")]
    pub checkpoint_permission_deleted: Option<AuditLogCheckpointPermissionDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "invite.sent")]
    pub invite_sent: Option<AuditLogInviteSent>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "invite.accepted")]
    pub invite_accepted: Option<AuditLogInviteAccepted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "invite.deleted")]
    pub invite_deleted: Option<AuditLogInviteDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "login.failed")]
    pub login_failed: Option<AuditLogLoginFailed>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logout.failed")]
    pub logout_failed: Option<AuditLogLogoutFailed>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "organization.updated")]
    pub organization_updated: Option<AuditLogOrganizationUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project.created")]
    pub project_created: Option<AuditLogProjectCreated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project.updated")]
    pub project_updated: Option<AuditLogProjectUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project.archived")]
    pub project_archived: Option<AuditLogProjectArchived>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rate_limit.updated")]
    pub rate_limit_updated: Option<AuditLogRateLimitUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rate_limit.deleted")]
    pub rate_limit_deleted: Option<AuditLogRateLimitDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_account.created")]
    pub service_account_created: Option<AuditLogServiceAccountCreated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_account.updated")]
    pub service_account_updated: Option<AuditLogServiceAccountUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_account.deleted")]
    pub service_account_deleted: Option<AuditLogServiceAccountDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user.added")]
    pub user_added: Option<AuditLogUserAdded>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user.updated")]
    pub user_updated: Option<AuditLogUserUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user.deleted")]
    pub user_deleted: Option<AuditLogUserDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificate.created")]
    pub certificate_created: Option<AuditLogCertificateCreated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificate.updated")]
    pub certificate_updated: Option<AuditLogCertificateUpdated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificate.deleted")]
    pub certificate_deleted: Option<AuditLogCertificateDeleted>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificates.activated")]
    pub certificates_activated: Option<AuditLogCertificatesActivated>,
    #[doc = "The details for events with this `type`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "certificates.deactivated")]
    pub certificates_deactivated: Option<AuditLogCertificatesDeactivated>,
}
#[doc = "The type of actor. Is either `session` or `api_key`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AuditLogActorType {
    #[serde(rename = "session")]
    Session,
    #[serde(rename = "api_key")]
    ApiKey,
}
#[doc = "The actor who performed the audit logged action."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogActor {
    #[doc = "The type of actor. Is either `session` or `api_key`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<AuditLogActorType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "session")]
    pub session: Option<AuditLogActorSession>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key")]
    pub api_key: Option<AuditLogActorApiKey>,
}
#[doc = "The type of API key. Can be either `user` or `service_account`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AuditLogActorApiKeyType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "service_account")]
    ServiceAccount,
}
#[doc = "The API Key used to perform the audit logged action."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogActorApiKey {
    #[doc = "The tracking id of the API key."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of API key. Can be either `user` or `service_account`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<AuditLogActorApiKeyType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<AuditLogActorUser>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_account")]
    pub service_account: Option<AuditLogActorServiceAccount>,
}
#[doc = "The service account that performed the audit logged action."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogActorServiceAccount {
    #[doc = "The service account id."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
}
#[doc = "The session in which the audit logged action was performed."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogActorSession {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<AuditLogActorUser>,
    #[doc = "The IP address from which the action was performed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ip_address")]
    pub ip_address: Option<String>,
}
#[doc = "The user who performed the audit logged action."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AuditLogActorUser {
    #[doc = "The user id."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The user email."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "email")]
    pub email: Option<String>,
}
#[doc = "The event type."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum AuditLogEventType {
    #[serde(rename = "api_key.created")]
    ApiKeyCreated,
    #[serde(rename = "api_key.updated")]
    ApiKeyUpdated,
    #[serde(rename = "api_key.deleted")]
    ApiKeyDeleted,
    #[serde(rename = "checkpoint_permission.created")]
    CheckpointPermissionCreated,
    #[serde(rename = "checkpoint_permission.deleted")]
    CheckpointPermissionDeleted,
    #[serde(rename = "invite.sent")]
    InviteSent,
    #[serde(rename = "invite.accepted")]
    InviteAccepted,
    #[serde(rename = "invite.deleted")]
    InviteDeleted,
    #[serde(rename = "login.succeeded")]
    LoginSucceeded,
    #[serde(rename = "login.failed")]
    LoginFailed,
    #[serde(rename = "logout.succeeded")]
    LogoutSucceeded,
    #[serde(rename = "logout.failed")]
    LogoutFailed,
    #[serde(rename = "organization.updated")]
    OrganizationUpdated,
    #[serde(rename = "project.created")]
    ProjectCreated,
    #[serde(rename = "project.updated")]
    ProjectUpdated,
    #[serde(rename = "project.archived")]
    ProjectArchived,
    #[serde(rename = "service_account.created")]
    ServiceAccountCreated,
    #[serde(rename = "service_account.updated")]
    ServiceAccountUpdated,
    #[serde(rename = "service_account.deleted")]
    ServiceAccountDeleted,
    #[serde(rename = "rate_limit.updated")]
    RateLimitUpdated,
    #[serde(rename = "rate_limit.deleted")]
    RateLimitDeleted,
    #[serde(rename = "user.added")]
    UserAdded,
    #[serde(rename = "user.updated")]
    UserUpdated,
    #[serde(rename = "user.deleted")]
    UserDeleted,
}
#[doc = "Always `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum AutoChunkingStrategyRequestParamType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct AutoChunkingStrategyRequestParam {
    #[doc = "Always `auto`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: AutoChunkingStrategyRequestParamType,
}
#[doc = "The object type, which is always `batch`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum BatchObject {
    #[default]
    #[serde(rename = "batch")]
    Batch,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchErrorsDatum {
    #[doc = "An error code identifying the error type."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "A human-readable message providing more details about the error."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[doc = "The name of the parameter that caused the error, if applicable."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
    #[doc = "The line number of the input file where the error occurred, if applicable."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "line")]
    pub line: Option<u64>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchErrors {
    #[doc = "The object type, which is always `list`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<Vec<BatchErrorsDatum>>,
}
#[doc = "The current status of the batch."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum BatchStatus {
    #[serde(rename = "validating")]
    Validating,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finalizing")]
    Finalizing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
}
#[doc = "The request counts for different statuses within the batch."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct BatchRequestCounts {
    #[doc = "Total number of requests in the batch."]
    #[serde(rename = "total")]
    pub total: u64,
    #[doc = "Number of requests that have been completed successfully."]
    #[serde(rename = "completed")]
    pub completed: u64,
    #[doc = "Number of requests that have failed."]
    #[serde(rename = "failed")]
    pub failed: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Batch {
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `batch`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: BatchObject,
    #[doc = "The OpenAI API endpoint used by the batch."]
    #[serde(rename = "endpoint")]
    pub endpoint: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "errors")]
    pub errors: Option<BatchErrors>,
    #[doc = "The ID of the input file for the batch."]
    #[serde(rename = "input_file_id")]
    pub input_file_id: String,
    #[doc = "The time frame within which the batch should be processed."]
    #[serde(rename = "completion_window")]
    pub completion_window: String,
    #[doc = "The current status of the batch."]
    #[serde(rename = "status")]
    pub status: BatchStatus,
    #[doc = "The ID of the file containing the outputs of successfully executed requests."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_file_id")]
    pub output_file_id: Option<String>,
    #[doc = "The ID of the file containing the outputs of requests with errors."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error_file_id")]
    pub error_file_id: Option<String>,
    #[doc = "The Unix timestamp (in seconds) for when the batch was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch started processing."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "in_progress_at")]
    pub in_progress_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch will expire."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch started finalizing."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finalizing_at")]
    pub finalizing_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch was completed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completed_at")]
    pub completed_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch failed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failed_at")]
    pub failed_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch expired."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expired_at")]
    pub expired_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch started cancelling."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelling_at")]
    pub cancelling_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the batch was cancelled."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<u64>,
    #[doc = "The request counts for different statuses within the batch."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request_counts")]
    pub request_counts: Option<BatchRequestCounts>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[doc = "The HTTP method to be used for the request. Currently only `POST` is supported."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum BatchRequestInputMethod {
    #[default]
    #[serde(rename = "POST")]
    Post,
}
#[doc = "The per-line object of the batch input file"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchRequestInput {
    #[doc = "A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "custom_id")]
    pub custom_id: Option<String>,
    #[doc = "The HTTP method to be used for the request. Currently only `POST` is supported."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "method")]
    pub method: Option<BatchRequestInputMethod>,
    #[doc = "The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "url")]
    pub url: Option<String>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchRequestOutputResponse {
    #[doc = "The HTTP status code of the response"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status_code")]
    pub status_code: Option<u64>,
    #[doc = "An unique identifier for the OpenAI API request. Please include this request ID when contacting support."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "request_id")]
    pub request_id: Option<String>,
    #[doc = "The JSON body of the response"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "body")]
    pub body: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[doc = "For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchRequestOutputError {
    #[doc = "A machine-readable error code."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "A human-readable error message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message")]
    pub message: Option<String>,
}
#[doc = "The per-line object of the batch output and error files"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct BatchRequestOutput {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "A developer-provided per-request id that will be used to match outputs to inputs."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "custom_id")]
    pub custom_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response")]
    pub response: Option<BatchRequestOutputResponse>,
    #[doc = "For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<BatchRequestOutputError>,
}
#[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CertificateObject {
    #[default]
    #[serde(rename = "certificate")]
    Certificate,
    #[serde(rename = "organization.certificate")]
    OrganizationCertificate,
    #[serde(rename = "organization.project.certificate")]
    OrganizationProjectCertificate,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CertificateCertificateDetails {
    #[doc = "The Unix timestamp (in seconds) of when the certificate becomes valid."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valid_at")]
    pub valid_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) of when the certificate expires."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<u64>,
    #[doc = "The content of the certificate in PEM format."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<String>,
}
#[doc = "Represents an individual `certificate` uploaded to the organization."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Certificate {
    #[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CertificateObject,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the certificate."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The Unix timestamp (in seconds) of when the certificate was uploaded."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[builder(default)]
    #[serde(rename = "certificate_details")]
    pub certificate_details: CertificateCertificateDetails,
    #[doc = "Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "active")]
    pub active: Option<bool>,
}
#[doc = "The type of object being deleted."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionDeletedObject {
    #[default]
    #[serde(rename = "chat.completion.deleted")]
    ChatCompletionDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionDeleted {
    #[doc = "The type of object being deleted."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ChatCompletionDeletedObject,
    #[doc = "The ID of the chat completion that was deleted."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Whether the chat completion was deleted."]
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[doc = "Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionFunctionCallOption {
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionFunctions {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    #[serde(rename = "name")]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<FunctionParameters>,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "An object representing a list of Chat Completions.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionList {
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ChatCompletionListObject,
    #[doc = "An array of chat completion objects.\n"]
    #[serde(rename = "data")]
    pub data: Vec<CreateChatCompletionResponse>,
    #[doc = "The identifier of the first chat completion in the data array."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The identifier of the last chat completion in the data array."]
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[doc = "Indicates whether there are more Chat Completions available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionMessageListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageListDatum {
    #[builder(default)]
    #[serde(flatten)]
    pub chat_completion_response_message: ChatCompletionResponseMessage,
    #[doc = "The identifier of the chat message."]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "An object representing a list of chat completion messages.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageList {
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ChatCompletionMessageListObject,
    #[doc = "An array of chat completion message objects.\n"]
    #[serde(rename = "data")]
    pub data: Vec<ChatCompletionMessageListDatum>,
    #[doc = "The identifier of the first chat message in the data array."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The identifier of the last chat message in the data array."]
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[doc = "Indicates whether there are more chat messages available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionMessageToolCallType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "The function that the model called."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageToolCallFunction {
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[serde(rename = "arguments")]
    pub arguments: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageToolCall {
    #[doc = "The ID of the tool call."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionMessageToolCallType,
    #[doc = "The function that the model called."]
    #[serde(rename = "function")]
    pub function: ChatCompletionMessageToolCallFunction,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionMessageToolCallChunkType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageToolCallChunkFunction {
    #[doc = "The name of the function to call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionMessageToolCallChunk {
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "The ID of the tool call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<ChatCompletionMessageToolCallChunkType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function")]
    pub function: Option<ChatCompletionMessageToolCallChunkFunction>,
}
pub type ChatCompletionMessageToolCalls = Vec<ChatCompletionMessageToolCall>;
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ChatCompletionModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
pub type ChatCompletionModalities = Vec<ChatCompletionModality>;
#[doc = "The type of the tool. Currently, only `function` is supported."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionNamedToolChoiceType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific function."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionNamedToolChoice {
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionNamedToolChoiceType,
    #[serde(rename = "function")]
    pub function: ChatCompletionNamedToolChoiceFunction,
}
#[doc = "The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestAssistantMessageContent {
    _0(String),
    _1(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `assistant`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestAssistantMessageRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
#[doc = "Data about a previous audio response from the model. \n[Learn more](/docs/guides/audio).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestAssistantMessageAudio {
    #[doc = "Unique identifier for a previous audio response from the model.\n"]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestAssistantMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[serde(rename = "arguments")]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "Messages sent by the model in response to user messages.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestAssistantMessage {
    #[doc = "The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<ChatCompletionRequestAssistantMessageContent>,
    #[doc = "The refusal message by the assistant."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
    #[doc = "The role of the messages author, in this case `assistant`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestAssistantMessageRole,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "Data about a previous audio response from the model. \n[Learn more](/docs/guides/audio).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<ChatCompletionRequestAssistantMessageAudio>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<ChatCompletionMessageToolCalls>,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function_call")]
    pub function_call: Option<ChatCompletionRequestAssistantMessageFunctionCall>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    Refusal(ChatCompletionRequestMessageContentPartRefusal),
}
#[doc = "The contents of the developer message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    _0(String),
    _1(Vec<ChatCompletionRequestMessageContentPartText>),
}
#[doc = "The role of the messages author, in this case `developer`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestDeveloperMessageRole {
    #[default]
    #[serde(rename = "developer")]
    Developer,
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, `developer` messages\nreplace the previous `system` messages.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestDeveloperMessage {
    #[doc = "The contents of the developer message."]
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestDeveloperMessageContent,
    #[doc = "The role of the messages author, in this case `developer`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestDeveloperMessageRole,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The role of the messages author, in this case `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestFunctionMessageRole {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestFunctionMessage {
    #[doc = "The role of the messages author, in this case `function`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestFunctionMessageRole,
    #[doc = "The contents of the function message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestMessage {
    Developer(ChatCompletionRequestDeveloperMessage),
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    Assistant(ChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}
#[doc = "The type of the content part. Always `input_audio`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartAudioType {
    #[default]
    #[serde(rename = "input_audio")]
    InputAudio,
}
#[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
    #[doc = "Base64 encoded audio data."]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
    #[serde(rename = "format")]
    pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}
#[doc = "Learn about [audio inputs](/docs/guides/audio).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartAudio {
    #[doc = "The type of the content part. Always `input_audio`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartAudioType,
    #[serde(rename = "input_audio")]
    pub input_audio: ChatCompletionRequestMessageContentPartAudioInputAudio,
}
#[doc = "The type of the content part. Always `file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartFileType {
    #[default]
    #[serde(rename = "file")]
    File,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartFileFile {
    #[doc = "The name of the file, used when passing the file to the model as a \nstring.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    #[doc = "The base64 encoded file data, used when passing the file to the model \nas a string.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_data")]
    pub file_data: Option<String>,
    #[doc = "The ID of an uploaded file to use as input.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
}
#[doc = "Learn about [file inputs](/docs/guides/text) for text generation.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartFile {
    #[doc = "The type of the content part. Always `file`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartFileType,
    #[builder(default)]
    #[serde(rename = "file")]
    pub file: ChatCompletionRequestMessageContentPartFileFile,
}
#[doc = "The type of the content part."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartImageType {
    #[default]
    #[serde(rename = "image_url")]
    ImageUrl,
}
#[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding)."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding)."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail")]
    pub detail: Option<ChatCompletionRequestMessageContentPartImageImageUrlDetail>,
}
#[doc = "Learn about [image inputs](/docs/guides/vision).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartImage {
    #[doc = "The type of the content part."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartImageType,
    #[serde(rename = "image_url")]
    pub image_url: ChatCompletionRequestMessageContentPartImageImageUrl,
}
#[doc = "The type of the content part."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartRefusalType {
    #[default]
    #[serde(rename = "refusal")]
    Refusal,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartRefusal {
    #[doc = "The type of the content part."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartRefusalType,
    #[doc = "The refusal message generated by the model."]
    #[serde(rename = "refusal")]
    pub refusal: String,
}
#[doc = "The type of the content part."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestMessageContentPartTextType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[doc = "Learn about [text inputs](/docs/guides/text-generation).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestMessageContentPartText {
    #[doc = "The type of the content part."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionRequestMessageContentPartTextType,
    #[doc = "The text content."]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The contents of the system message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestSystemMessageContent {
    _0(String),
    _1(Vec<ChatCompletionRequestSystemMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `system`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestSystemMessageRole {
    #[default]
    #[serde(rename = "system")]
    System,
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, use `developer` messages\nfor this purpose instead.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestSystemMessage {
    #[doc = "The contents of the system message."]
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestSystemMessageContent,
    #[doc = "The role of the messages author, in this case `system`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestSystemMessageRole,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestSystemMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}
#[doc = "The role of the messages author, in this case `tool`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestToolMessageRole {
    #[default]
    #[serde(rename = "tool")]
    Tool,
}
#[doc = "The contents of the tool message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestToolMessageContent {
    _0(String),
    _1(Vec<ChatCompletionRequestToolMessageContentPart>),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestToolMessage {
    #[doc = "The role of the messages author, in this case `tool`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestToolMessageRole,
    #[doc = "The contents of the tool message."]
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestToolMessageContent,
    #[doc = "Tool call that this message is responding to."]
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestToolMessageContentPart {
    ChatCompletionRequestMessageContentPartText(ChatCompletionRequestMessageContentPartText),
}
#[doc = "The contents of the user message.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestUserMessageContent {
    _0(String),
    _1(Vec<ChatCompletionRequestUserMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `user`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionRequestUserMessageRole {
    #[default]
    #[serde(rename = "user")]
    User,
}
#[doc = "Messages sent by an end user, containing prompts or additional context\ninformation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionRequestUserMessage {
    #[doc = "The contents of the user message.\n"]
    #[serde(rename = "content")]
    pub content: ChatCompletionRequestUserMessageContent,
    #[doc = "The role of the messages author, in this case `user`."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionRequestUserMessageRole,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestUserMessageContentPart {
    Text(ChatCompletionRequestMessageContentPartText),
    ImageUrl(ChatCompletionRequestMessageContentPartImage),
    InputAudio(ChatCompletionRequestMessageContentPartAudio),
    File(ChatCompletionRequestMessageContentPartFile),
}
#[doc = "The type of the URL citation. Always `url_citation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionResponseMessageAnnotationType {
    #[default]
    #[serde(rename = "url_citation")]
    UrlCitation,
}
#[doc = "A URL citation when using web search."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionResponseMessageAnnotationUrlCitation {
    #[doc = "The index of the last character of the URL citation in the message."]
    #[serde(rename = "end_index")]
    pub end_index: u64,
    #[doc = "The index of the first character of the URL citation in the message."]
    #[serde(rename = "start_index")]
    pub start_index: u64,
    #[doc = "The URL of the web resource."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "The title of the web resource."]
    #[serde(rename = "title")]
    pub title: String,
}
#[doc = "A URL citation when using web search.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionResponseMessageAnnotation {
    #[doc = "The type of the URL citation. Always `url_citation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionResponseMessageAnnotationType,
    #[doc = "A URL citation when using web search."]
    #[serde(rename = "url_citation")]
    pub url_citation: ChatCompletionResponseMessageAnnotationUrlCitation,
}
#[doc = "The role of the author of this message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionResponseMessageRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionResponseMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[serde(rename = "arguments")]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "If the audio output modality is requested, this object contains data\nabout the audio response from the model. [Learn more](/docs/guides/audio).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionResponseMessageAudio {
    #[doc = "Unique identifier for this audio response."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when this audio response will\nno longer be accessible on the server for use in multi-turn\nconversations.\n"]
    #[serde(rename = "expires_at")]
    pub expires_at: u64,
    #[doc = "Base64 encoded audio bytes generated by the model, in the format\nspecified in the request.\n"]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "Transcript of the audio generated by the model."]
    #[serde(rename = "transcript")]
    pub transcript: String,
}
#[doc = "A chat completion message generated by the model."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionResponseMessage {
    #[doc = "The contents of the message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[doc = "The refusal message generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<ChatCompletionMessageToolCalls>,
    #[doc = "Annotations for the message, when applicable, as when using the\n[web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotations")]
    pub annotations: Option<Vec<ChatCompletionResponseMessageAnnotation>>,
    #[doc = "The role of the author of this message."]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ChatCompletionResponseMessageRole,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function_call")]
    pub function_call: Option<ChatCompletionResponseMessageFunctionCall>,
    #[doc = "If the audio output modality is requested, this object contains data\nabout the audio response from the model. [Learn more](/docs/guides/audio).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<ChatCompletionResponseMessageAudio>,
}
#[doc = "The role of the author of a message"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ChatCompletionRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "function")]
    Function,
}
#[doc = "Options for streaming response. Only set this when you set `stream: true`.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionStreamOptions {
    #[doc = "If set, an additional chunk will be streamed before the `data: [DONE]`\nmessage. The `usage` field on this chunk shows the token usage statistics\nfor the entire request, and the `choices` field will always be an empty\narray. \n\nAll other chunks will also include a `usage` field, but with a null\nvalue. **NOTE:** If the stream is interrupted, you may not receive the\nfinal usage chunk which contains the total token usage for the request.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include_usage")]
    pub include_usage: Option<bool>,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionStreamResponseDeltaFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    #[doc = "The name of the function to call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
}
#[doc = "The role of the author of this message."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ChatCompletionStreamResponseDeltaRole {
    #[serde(rename = "developer")]
    Developer,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "tool")]
    Tool,
}
#[doc = "A chat completion delta generated by streamed model responses."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionStreamResponseDelta {
    #[doc = "The contents of the chunk message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<String>,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function_call")]
    pub function_call: Option<ChatCompletionStreamResponseDeltaFunctionCall>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<ChatCompletionMessageToolCallChunk>>,
    #[doc = "The role of the author of this message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<ChatCompletionStreamResponseDeltaRole>,
    #[doc = "The refusal message generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionTokenLogprobTopLogprob {
    #[doc = "The token."]
    #[serde(rename = "token")]
    pub token: String,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    #[serde(rename = "logprob")]
    pub logprob: f64,
    #[doc = "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<u64>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionTokenLogprob {
    #[doc = "The token."]
    #[serde(rename = "token")]
    pub token: String,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    #[serde(rename = "logprob")]
    pub logprob: f64,
    #[doc = "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<u64>>,
    #[doc = "List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned."]
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Vec<ChatCompletionTokenLogprobTopLogprob>,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ChatCompletionToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ChatCompletionTool {
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ChatCompletionToolType,
    #[serde(rename = "function")]
    pub function: FunctionObject,
}
#[doc = "`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ChatCompletionToolChoiceOption0 {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tool and instead generates a message.\n`auto` means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools.\nSpecifying a particular tool via `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n\n`none` is the default when no tools are present. `auto` is the default if tools are present.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionToolChoiceOption {
    _0(ChatCompletionToolChoiceOption0),
    ChatCompletionNamedToolChoice(ChatCompletionNamedToolChoice),
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ChunkingStrategyRequestParam {
    Auto(AutoChunkingStrategyRequestParam),
    Static(StaticChunkingStrategyRequestParam),
}
#[doc = "Specifies the event type. For a click action, this property is \nalways set to `click`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ClickType {
    #[default]
    #[serde(rename = "click")]
    Click,
}
#[doc = "Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ClickButton {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "wheel")]
    Wheel,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}
#[doc = "A click action.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Click {
    #[doc = "Specifies the event type. For a click action, this property is \nalways set to `click`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ClickType,
    #[doc = "Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.\n"]
    #[serde(rename = "button")]
    pub button: ClickButton,
    #[doc = "The x-coordinate where the click occurred.\n"]
    #[serde(rename = "x")]
    pub x: u64,
    #[doc = "The y-coordinate where the click occurred.\n"]
    #[serde(rename = "y")]
    pub y: u64,
}
#[doc = "The type of the code interpreter file output. Always `files`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CodeInterpreterFileOutputType {
    #[default]
    #[serde(rename = "files")]
    Files,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CodeInterpreterFileOutputFile {
    #[doc = "The MIME type of the file.\n"]
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    #[doc = "The ID of the file.\n"]
    #[serde(rename = "file_id")]
    pub file_id: String,
}
#[doc = "The output of a code interpreter tool call that is a file.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CodeInterpreterFileOutput {
    #[doc = "The type of the code interpreter file output. Always `files`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterFileOutputType,
    #[serde(rename = "files")]
    pub files: Vec<CodeInterpreterFileOutputFile>,
}
#[doc = "The type of the code interpreter text output. Always `logs`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CodeInterpreterTextOutputType {
    #[default]
    #[serde(rename = "logs")]
    Logs,
}
#[doc = "The output of a code interpreter tool call that is text.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CodeInterpreterTextOutput {
    #[doc = "The type of the code interpreter text output. Always `logs`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterTextOutputType,
    #[doc = "The logs of the code interpreter tool call.\n"]
    #[serde(rename = "logs")]
    pub logs: String,
}
#[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CodeInterpreterToolCallType {
    #[default]
    #[serde(rename = "code_interpreter_call")]
    CodeInterpreterCall,
}
#[doc = "The status of the code interpreter tool call.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CodeInterpreterToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "interpreting")]
    Interpreting,
    #[serde(rename = "completed")]
    Completed,
}
#[doc = "A tool call to run code.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CodeInterpreterToolCall {
    #[doc = "The unique ID of the code interpreter tool call.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CodeInterpreterToolCallType,
    #[doc = "The code to run.\n"]
    #[serde(rename = "code")]
    pub code: String,
    #[doc = "The status of the code interpreter tool call.\n"]
    #[serde(rename = "status")]
    pub status: CodeInterpreterToolCallStatus,
    #[doc = "The results of the code interpreter tool call.\n"]
    #[serde(rename = "results")]
    pub results: Vec<CodeInterpreterToolOutput>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CodeInterpreterToolOutput {
    Logs(CodeInterpreterTextOutput),
    Files(CodeInterpreterFileOutput),
}
#[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComparisonFilterType {
    #[default]
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "gt")]
    Gt,
    #[serde(rename = "gte")]
    Gte,
    #[serde(rename = "lt")]
    Lt,
    #[serde(rename = "lte")]
    Lte,
}
#[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ComparisonFilterValue {
    _0(String),
    _1(f64),
    _2(bool),
}
#[doc = "A filter used to compare a specified attribute key to a given value using a defined comparison operation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComparisonFilter {
    #[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComparisonFilterType,
    #[doc = "The key to compare against the value."]
    #[serde(rename = "key")]
    pub key: String,
    #[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
    #[serde(rename = "value")]
    pub value: ComparisonFilterValue,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CompleteUploadRequest {
    #[doc = "The ordered list of Part IDs.\n"]
    #[serde(rename = "part_ids")]
    pub part_ids: Vec<String>,
    #[doc = "The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "md5")]
    pub md5: Option<String>,
}
#[doc = "Breakdown of tokens used in a completion."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CompletionUsageCompletionTokensDetails {
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that appeared in the completion.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accepted_prediction_tokens")]
    pub accepted_prediction_tokens: Option<u64>,
    #[doc = "Audio input tokens generated by the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio_tokens")]
    pub audio_tokens: Option<u64>,
    #[doc = "Tokens generated by the model for reasoning."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_tokens")]
    pub reasoning_tokens: Option<u64>,
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that did not appear in the completion. However, like\nreasoning tokens, these tokens are still counted in the total\ncompletion tokens for purposes of billing, output, and context window\nlimits.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rejected_prediction_tokens")]
    pub rejected_prediction_tokens: Option<u64>,
}
#[doc = "Breakdown of tokens used in the prompt."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CompletionUsagePromptTokensDetails {
    #[doc = "Audio input tokens present in the prompt."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio_tokens")]
    pub audio_tokens: Option<u64>,
    #[doc = "Cached tokens present in the prompt."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: Option<u64>,
}
#[doc = "Usage statistics for the completion request."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CompletionUsage {
    #[doc = "Number of tokens in the generated completion."]
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[doc = "Number of tokens in the prompt."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "Total number of tokens used in the request (prompt + completion)."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
    #[doc = "Breakdown of tokens used in a completion."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completion_tokens_details")]
    pub completion_tokens_details: Option<CompletionUsageCompletionTokensDetails>,
    #[doc = "Breakdown of tokens used in the prompt."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt_tokens_details")]
    pub prompt_tokens_details: Option<CompletionUsagePromptTokensDetails>,
}
#[doc = "Type of operation: `and` or `or`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CompoundFilterType {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CompoundFilterFilter {
    ComparisonFilter(ComparisonFilter),
    CompoundFilter(CompoundFilter),
}
#[doc = "Combine multiple filters using `and` or `or`."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CompoundFilter {
    #[doc = "Type of operation: `and` or `or`."]
    #[serde(rename = "type")]
    pub type_: CompoundFilterType,
    #[doc = "Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`."]
    #[serde(rename = "filters")]
    pub filters: Vec<CompoundFilterFilter>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ComputerAction {
    Click(Click),
    DoubleClick(DoubleClick),
    Drag(Drag),
    Keypress(KeyPress),
    Move(Move),
    Screenshot(Screenshot),
    Scroll(Scroll),
    Type(Type),
    Wait(Wait),
}
#[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComputerScreenshotImageType {
    #[default]
    #[serde(rename = "computer_screenshot")]
    ComputerScreenshot,
}
#[doc = "A computer screenshot image used with the computer use tool.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ComputerScreenshotImage {
    #[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComputerScreenshotImageType,
    #[doc = "The URL of the screenshot image."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[doc = "The identifier of an uploaded file that contains the screenshot."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
}
#[doc = "The type of the computer call. Always `computer_call`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComputerToolCallType {
    #[default]
    #[serde(rename = "computer_call")]
    ComputerCall,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ComputerToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "A tool call to a computer use tool. See the \n[computer use guide](/docs/guides/tools-computer-use) for more information.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerToolCall {
    #[doc = "The type of the computer call. Always `computer_call`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComputerToolCallType,
    #[doc = "The unique ID of the computer call."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "An identifier used when responding to the tool call with output.\n"]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[serde(rename = "action")]
    pub action: ComputerAction,
    #[doc = "The pending safety checks for the computer call.\n"]
    #[serde(rename = "pending_safety_checks")]
    pub pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[serde(rename = "status")]
    pub status: ComputerToolCallStatus,
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComputerToolCallOutputType {
    #[default]
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ComputerToolCallOutputStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The output of a computer tool call.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerToolCallOutput {
    #[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComputerToolCallOutputType,
    #[doc = "The ID of the computer tool call output.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The ID of the computer tool call that produced the output.\n"]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The safety checks reported by the API that have been acknowledged by the \ndeveloper.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "acknowledged_safety_checks")]
    pub acknowledged_safety_checks: Option<Vec<ComputerToolCallSafetyCheck>>,
    #[builder(default)]
    #[serde(rename = "output")]
    pub output: ComputerScreenshotImage,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ComputerToolCallOutputStatus>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerToolCallOutputResource {
    #[serde(flatten)]
    pub computer_tool_call_output: ComputerToolCallOutput,
    #[doc = "The unique ID of the computer call tool output.\n"]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "A pending safety check for the computer call.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerToolCallSafetyCheck {
    #[doc = "The ID of the pending safety check."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the pending safety check."]
    #[serde(rename = "code")]
    pub code: String,
    #[doc = "Details about the pending safety check."]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "Multi-modal input and output contents.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Content {
    InputContent(InputContent),
    OutputContent(OutputContent),
}
#[doc = "An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Coordinate {
    #[doc = "The x-coordinate.\n"]
    #[serde(rename = "x")]
    pub x: u64,
    #[doc = "The y-coordinate.\n"]
    #[serde(rename = "y")]
    pub y: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CostsResultObject {
    #[default]
    #[serde(rename = "organization.costs.result")]
    OrganizationCostsResult,
}
#[doc = "The monetary value in its associated currency."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CostsResultAmount {
    #[doc = "The numeric value of the cost."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<f64>,
    #[doc = "Lowercase ISO-4217 currency e.g. \"usd\""]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "currency")]
    pub currency: Option<String>,
}
#[doc = "The aggregated costs details of the specific time bucket."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CostsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CostsResultObject,
    #[doc = "The monetary value in its associated currency."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "amount")]
    pub amount: Option<CostsResultAmount>,
    #[doc = "When `group_by=line_item`, this field provides the line item of the grouped costs result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "line_item")]
    pub line_item: Option<String>,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped costs result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[doc = "Always `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAutoType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAuto {
    #[doc = "Always `auto`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAutoType,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticStatic {
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: u64,
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStatic {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticType,
    #[serde(rename = "static")]
    pub static_:
        CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticStatic,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategy {
    Auto(CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAuto),
    Static(CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStatic),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch0VectorStore {
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy:
        Option<CreateAssistantRequestToolResourcesFileSearch0VectorStoreChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch0 {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Vec<String>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_stores")]
    pub vector_stores: Option<Vec<CreateAssistantRequestToolResourcesFileSearch0VectorStore>>,
}
#[doc = "Always `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAutoType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAuto {
    #[doc = "Always `auto`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAutoType,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticStatic {
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: u64,
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStatic {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticType,
    #[serde(rename = "static")]
    pub static_:
        CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticStatic,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategy {
    Auto(CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAuto),
    Static(CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStatic),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch1VectorStore {
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy:
        Option<CreateAssistantRequestToolResourcesFileSearch1VectorStoreChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResourcesFileSearch1 {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[serde(rename = "vector_stores")]
    pub vector_stores: Vec<CreateAssistantRequestToolResourcesFileSearch1VectorStore>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestToolResourcesFileSearch {
    _0(CreateAssistantRequestToolResourcesFileSearch0),
    _1(CreateAssistantRequestToolResourcesFileSearch1),
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequestToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<CreateAssistantRequestToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<CreateAssistantRequestToolResourcesFileSearch>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateAssistantRequest {
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<CreateAssistantRequestTool>>,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<CreateAssistantRequestToolResources>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[doc = "The type of location approximation. Always `approximate`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateChatCompletionRequestWebSearchOptionsUserLocationType {
    #[default]
    #[serde(rename = "approximate")]
    Approximate,
}
#[doc = "Approximate location parameters for the search.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionRequestWebSearchOptionsUserLocation {
    #[doc = "The type of location approximation. Always `approximate`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateChatCompletionRequestWebSearchOptionsUserLocationType,
    #[builder(default)]
    #[serde(rename = "approximate")]
    pub approximate: WebSearchLocation,
}
#[doc = "This tool searches the web for relevant results to use in a response.\nLearn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionRequestWebSearchOptions {
    #[doc = "Approximate location parameters for the search.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_location")]
    pub user_location: Option<CreateChatCompletionRequestWebSearchOptionsUserLocation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "search_context_size")]
    pub search_context_size: Option<WebSearchContextSize>,
}
#[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequestResponseFormat {
    Text(ResponseFormatText),
    JsonSchema(ResponseFormatJsonSchema),
    JsonObject(ResponseFormatJsonObject),
}
#[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateChatCompletionRequestAudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "pcm16")]
    Pcm16,
}
#[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](/docs/guides/audio).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionRequestAudio {
    #[doc = "The voice the model uses to respond. Supported voices are \n`alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`, `sage`, and `shimmer`.\n"]
    #[serde(rename = "voice")]
    pub voice: VoiceIdsShared,
    #[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
    #[serde(rename = "format")]
    pub format: CreateChatCompletionRequestAudioFormat,
}
#[doc = "Configuration for a [Predicted Output](/docs/guides/predicted-outputs),\nwhich can greatly improve response times when large parts of the model\nresponse are known ahead of time. This is most common when you are\nregenerating a file with only minor changes to most of the content.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequestPrediction {
    PredictionContent(PredictionContent),
}
#[doc = "`none` means the model will not call a function and instead generates a message. `auto` means the model can pick between generating a message or calling a function.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateChatCompletionRequestFunctionCall0 {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequestFunctionCall {
    _0(CreateChatCompletionRequestFunctionCall0),
    ChatCompletionFunctionCallOption(ChatCompletionFunctionCallOption),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionRequest {
    #[builder(default)]
    #[serde(flatten)]
    pub create_model_response_properties: CreateModelResponseProperties,
    #[doc = "A list of messages comprising the conversation so far. Depending on the\n[model](/docs/models) you use, different message types (modalities) are\nsupported, like [text](/docs/guides/text-generation),\n[images](/docs/guides/vision), and [audio](/docs/guides/audio).\n"]
    #[serde(rename = "messages")]
    pub messages: Vec<ChatCompletionRequestMessage>,
    #[doc = "Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](/docs/models)\nto browse and compare available models.\n"]
    #[serde(rename = "model")]
    pub model: ModelIdsShared,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<ResponseModalities>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[doc = "An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\ntheir existing frequency in the text so far, decreasing the model's\nlikelihood to repeat the same line verbatim.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\nwhether they appear in the text so far, increasing the model's likelihood\nto talk about new topics.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f64>,
    #[doc = "This tool searches the web for relevant results to use in a response.\nLearn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "web_search_options")]
    pub web_search_options: Option<CreateChatCompletionRequestWebSearchOptions>,
    #[doc = "An integer between 0 and 20 specifying the number of most likely tokens to\nreturn at each token position, each with an associated log probability.\n`logprobs` must be set to `true` if this parameter is used.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<u64>,
    #[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateChatCompletionRequestResponseFormat>,
    #[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](/docs/guides/audio).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<CreateChatCompletionRequestAudio>,
    #[doc = "Whether or not to store the output of this chat completion request for \nuse in our [model distillation](/docs/guides/distillation) or\n[evals](/docs/guides/evals) products.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "store")]
    pub store: Option<bool>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\nSee the [Streaming section below](/docs/api-reference/chat/streaming)\nfor more information, along with the [streaming responses](/docs/guides/streaming-responses)\nguide for more information on how to handle the streaming events.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stop")]
    pub stop: Option<StopConfiguration>,
    #[doc = "Modify the likelihood of specified tokens appearing in the completion.\n\nAccepts a JSON object that maps tokens (specified by their token ID in the\ntokenizer) to an associated bias value from -100 to 100. Mathematically,\nthe bias is added to the logits generated by the model prior to sampling.\nThe exact effect will vary per model, but values between -1 and 1 should\ndecrease or increase likelihood of selection; values like -100 or 100\nshould result in a ban or exclusive selection of the relevant token.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logit_bias")]
    pub logit_bias: Option<Vec<u64>>,
    #[doc = "Whether to return log probabilities of the output tokens or not. If true,\nreturns the log probabilities of each output token returned in the\n`content` of `message`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<bool>,
    #[doc = "The maximum number of [tokens](/tokenizer) that can be generated in the\nchat completion. This value can be used to control\n[costs](https://openai.com/api/pricing/) for text generated via API.\n\nThis value is now deprecated in favor of `max_completion_tokens`, and is\nnot compatible with [o-series models](/docs/guides/reasoning).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_tokens")]
    pub max_tokens: Option<u64>,
    #[doc = "How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n: Option<u64>,
    #[doc = "Configuration for a [Predicted Output](/docs/guides/predicted-outputs),\nwhich can greatly improve response times when large parts of the model\nresponse are known ahead of time. This is most common when you are\nregenerating a file with only minor changes to most of the content.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prediction")]
    pub prediction: Option<CreateChatCompletionRequestPrediction>,
    #[doc = "This feature is in Beta.\nIf specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.\nDeterminism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream_options")]
    pub stream_options: Option<ChatCompletionStreamOptions>,
    #[doc = "A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<ChatCompletionToolChoiceOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<ParallelToolCalls>,
    #[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function_call")]
    pub function_call: Option<CreateChatCompletionRequestFunctionCall>,
    #[doc = "Deprecated in favor of `tools`.\n\nA list of functions the model may generate JSON inputs for.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functions")]
    pub functions: Option<Vec<ChatCompletionFunctions>>,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateChatCompletionResponseChoiceFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}
#[doc = "Log probability information for the choice."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionResponseChoiceLogprobs {
    #[doc = "A list of message content tokens with log probability information."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
    #[doc = "A list of message refusal tokens with log probability information."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionResponseChoice {
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
    #[serde(rename = "finish_reason")]
    pub finish_reason: CreateChatCompletionResponseChoiceFinishReason,
    #[doc = "The index of the choice in the list of choices."]
    #[serde(rename = "index")]
    pub index: u64,
    #[builder(default)]
    #[serde(rename = "message")]
    pub message: ChatCompletionResponseMessage,
    #[doc = "Log probability information for the choice."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<CreateChatCompletionResponseChoiceLogprobs>,
}
#[doc = "The object type, which is always `chat.completion`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateChatCompletionResponseObject {
    #[default]
    #[serde(rename = "chat.completion")]
    ChatCompletion,
}
#[doc = "Represents a chat completion response returned by model, based on the provided input."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionResponse {
    #[doc = "A unique identifier for the chat completion."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "A list of chat completion choices. Can be more than one if `n` is greater than 1."]
    #[serde(rename = "choices")]
    pub choices: Vec<CreateChatCompletionResponseChoice>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created."]
    #[serde(rename = "created")]
    pub created: u64,
    #[doc = "The model used for the chat completion."]
    #[serde(rename = "model")]
    pub model: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_tier")]
    pub service_tier: Option<ServiceTier>,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\n\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    #[doc = "The object type, which is always `chat.completion`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CreateChatCompletionResponseObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<CompletionUsage>,
}
#[doc = "Log probability information for the choice."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionStreamResponseChoiceLogprobs {
    #[doc = "A list of message content tokens with log probability information."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<ChatCompletionTokenLogprob>>,
    #[doc = "A list of message refusal tokens with log probability information."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<Vec<ChatCompletionTokenLogprob>>,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateChatCompletionStreamResponseChoiceFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionStreamResponseChoice {
    #[builder(default)]
    #[serde(rename = "delta")]
    pub delta: ChatCompletionStreamResponseDelta,
    #[doc = "Log probability information for the choice."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<CreateChatCompletionStreamResponseChoiceLogprobs>,
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finish_reason")]
    pub finish_reason: Option<CreateChatCompletionStreamResponseChoiceFinishReason>,
    #[doc = "The index of the choice in the list of choices."]
    #[serde(rename = "index")]
    pub index: u64,
}
#[doc = "The object type, which is always `chat.completion.chunk`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateChatCompletionStreamResponseObject {
    #[default]
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}
#[doc = "Represents a streamed chunk of a chat completion response returned\nby the model, based on the provided input. \n[Learn more](/docs/guides/streaming-responses).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateChatCompletionStreamResponse {
    #[doc = "A unique identifier for the chat completion. Each chunk has the same ID."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the\nlast chunk if you set `stream_options: {\"include_usage\": true}`.\n"]
    #[serde(rename = "choices")]
    pub choices: Vec<CreateChatCompletionStreamResponseChoice>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp."]
    #[serde(rename = "created")]
    pub created: u64,
    #[doc = "The model to generate the completion."]
    #[serde(rename = "model")]
    pub model: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_tier")]
    pub service_tier: Option<ServiceTier>,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    #[doc = "The object type, which is always `chat.completion.chunk`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CreateChatCompletionStreamResponseObject,
    #[doc = "An optional field that will only be present when you set\n`stream_options: {\"include_usage\": true}` in your request. When present, it\ncontains a null value **except for the last chunk** which contains the\ntoken usage statistics for the entire request.\n\n**NOTE:** If the stream is interrupted or cancelled, you may not\nreceive the final usage chunk which contains the total token usage for\nthe request.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<CompletionUsage>,
}
#[doc = "The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.\n\nNote that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateCompletionRequestPrompt {
    _0(String),
    _1(Vec<String>),
    _2(Vec<u64>),
    _3(Vec<Vec<u64>>),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateCompletionRequest {
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.\n\nNote that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<CreateCompletionRequestPrompt>,
    #[doc = "Generates `best_of` completions server-side and returns the \"best\" (the one with the highest log probability per token). Results cannot be streamed.\n\nWhen used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.\n\n**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "best_of")]
    pub best_of: Option<u64>,
    #[doc = "Echo back the prompt in addition to the completion\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "echo")]
    pub echo: Option<bool>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.\n\n[See more information about frequency and presence penalties.](/docs/guides/text-generation)\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "frequency_penalty")]
    pub frequency_penalty: Option<f64>,
    #[doc = "Modify the likelihood of specified tokens appearing in the completion.\n\nAccepts a JSON object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.\n\nAs an example, you can pass `{\"50256\": -100}` to prevent the <|endoftext|> token from being generated.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logit_bias")]
    pub logit_bias: Option<Vec<u64>>,
    #[doc = "Include the log probabilities on the `logprobs` most likely output tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.\n\nThe maximum value for `logprobs` is 5.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<u64>,
    #[doc = "The maximum number of [tokens](/tokenizer) that can be generated in the completion.\n\nThe token count of your prompt plus `max_tokens` cannot exceed the model's context length. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_tokens")]
    pub max_tokens: Option<u64>,
    #[doc = "How many completions to generate for each prompt.\n\n**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n: Option<u64>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.\n\n[See more information about frequency and presence penalties.](/docs/guides/text-generation)\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "presence_penalty")]
    pub presence_penalty: Option<f64>,
    #[doc = "If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.\n\nDeterminism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stop")]
    pub stop: Option<StopConfiguration>,
    #[doc = "Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream_options")]
    pub stream_options: Option<ChatCompletionStreamOptions>,
    #[doc = "The suffix that comes after a completion of inserted text.\n\nThis parameter is only supported for `gpt-3.5-turbo-instruct`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suffix")]
    pub suffix: Option<String>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n\nWe generally recommend altering this or `top_p` but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or `temperature` but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\nor `content_filter` if content was omitted due to a flag from our content filters.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateCompletionResponseChoiceFinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "content_filter")]
    ContentFilter,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateCompletionResponseChoiceLogprobs {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text_offset")]
    pub text_offset: Option<Vec<u64>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "token_logprobs")]
    pub token_logprobs: Option<Vec<f64>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tokens")]
    pub tokens: Option<Vec<String>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_logprobs")]
    pub top_logprobs: Option<Vec<Vec<f64>>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateCompletionResponseChoice {
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\nor `content_filter` if content was omitted due to a flag from our content filters.\n"]
    #[serde(rename = "finish_reason")]
    pub finish_reason: CreateCompletionResponseChoiceFinishReason,
    #[serde(rename = "index")]
    pub index: u64,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<CreateCompletionResponseChoiceLogprobs>,
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The object type, which is always \"text_completion\""]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateCompletionResponseObject {
    #[default]
    #[serde(rename = "text_completion")]
    TextCompletion,
}
#[doc = "Represents a completion response from the API. Note: both the streamed and non-streamed response objects share the same shape (unlike the chat endpoint).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateCompletionResponse {
    #[doc = "A unique identifier for the completion."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The list of completion choices the model generated for the input prompt."]
    #[serde(rename = "choices")]
    pub choices: Vec<CreateCompletionResponseChoice>,
    #[doc = "The Unix timestamp (in seconds) of when the completion was created."]
    #[serde(rename = "created")]
    pub created: u64,
    #[doc = "The model used for completion."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\n\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "system_fingerprint")]
    pub system_fingerprint: Option<String>,
    #[doc = "The object type, which is always \"text_completion\""]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CreateCompletionResponseObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<CompletionUsage>,
}
#[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEmbeddingRequestInput {
    _0(String),
    _1(Vec<String>),
    _2(Vec<u64>),
    _3(Vec<Vec<u64>>),
}
#[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEmbeddingRequestEncodingFormat {
    #[default]
    #[serde(rename = "float")]
    Float,
    #[serde(rename = "base64")]
    Base64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEmbeddingRequest {
    #[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.\n"]
    #[serde(rename = "input")]
    pub input: CreateEmbeddingRequestInput,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "encoding_format")]
    pub encoding_format: Option<CreateEmbeddingRequestEncodingFormat>,
    #[doc = "The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dimensions")]
    pub dimensions: Option<u64>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
}
#[doc = "The object type, which is always \"list\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEmbeddingResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "The usage information for the request."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEmbeddingResponseUsage {
    #[doc = "The number of tokens used by the prompt."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "The total number of tokens used by the request."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEmbeddingResponse {
    #[doc = "The list of embeddings generated by the model."]
    #[serde(rename = "data")]
    pub data: Vec<Embedding>,
    #[doc = "The name of the model used to generate the embedding."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The object type, which is always \"list\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: CreateEmbeddingResponseObject,
    #[doc = "The usage information for the request."]
    #[serde(rename = "usage")]
    pub usage: CreateEmbeddingResponseUsage,
}
#[doc = "The type of run data source. Always `completions`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalCompletionsRunDataSourceType {
    #[default]
    #[serde(rename = "completions")]
    Completions,
}
#[doc = "The type of input messages. Always `template`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages0Type {
    #[serde(rename = "template")]
    Template,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages0Template {
    EasyInputMessage(EasyInputMessage),
    EvalItem(EvalItem),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalCompletionsRunDataSourceInputMessages0 {
    #[doc = "The type of input messages. Always `template`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalCompletionsRunDataSourceInputMessages0Type,
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    #[serde(rename = "template")]
    pub template: Vec<CreateEvalCompletionsRunDataSourceInputMessages0Template>,
}
#[doc = "The type of input messages. Always `item_reference`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages1Type {
    #[serde(rename = "item_reference")]
    ItemReference,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalCompletionsRunDataSourceInputMessages1 {
    #[doc = "The type of input messages. Always `item_reference`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalCompletionsRunDataSourceInputMessages1Type,
    #[doc = "A reference to a variable in the \"item\" namespace. Ie, \"item.name\""]
    #[serde(rename = "item_reference")]
    pub item_reference: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages {
    _0(CreateEvalCompletionsRunDataSourceInputMessages0),
    _1(CreateEvalCompletionsRunDataSourceInputMessages1),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
    #[doc = "A higher temperature increases randomness in the outputs."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "The maximum number of tokens in the generated output."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceSource {
    FileContent(EvalJsonlFileContentSource),
    FileId(EvalJsonlFileIdSource),
    StoredCompletions(EvalStoredCompletionsSource),
}
#[doc = "A CompletionsRunDataSource object describing a model sampling configuration.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalCompletionsRunDataSource {
    #[doc = "The type of run data source. Always `completions`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalCompletionsRunDataSourceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_messages")]
    pub input_messages: Option<CreateEvalCompletionsRunDataSourceInputMessages>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sampling_params")]
    pub sampling_params: Option<CreateEvalCompletionsRunDataSourceSamplingParams>,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "source")]
    pub source: CreateEvalCompletionsRunDataSourceSource,
}
#[doc = "The type of data source. Always `custom`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalCustomDataSourceConfigType {
    #[default]
    #[serde(rename = "custom")]
    Custom,
}
#[doc = "A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation runs.\nThis schema is used to define the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalCustomDataSourceConfig {
    #[doc = "The type of data source. Always `custom`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalCustomDataSourceConfigType,
    #[doc = "The json schema for each row in the data source."]
    #[serde(rename = "item_schema")]
    pub item_schema: std::collections::HashMap<String, serde_json::Value>,
    #[doc = "Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source)"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include_sample_schema")]
    pub include_sample_schema: Option<bool>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalItem0 {
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    #[serde(rename = "role")]
    pub role: String,
    #[doc = "The content of the message."]
    #[serde(rename = "content")]
    pub content: String,
}
#[doc = "A chat message that makes up the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalItem {
    _0(CreateEvalItem0),
    EvalItem(EvalItem),
}
#[doc = "The type of data source. Always `jsonl`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalJsonlRunDataSourceType {
    #[default]
    #[serde(rename = "jsonl")]
    Jsonl,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalJsonlRunDataSourceSource {
    FileContent(EvalJsonlFileContentSource),
    FileId(EvalJsonlFileIdSource),
}
#[doc = "A JsonlRunDataSource object with that specifies a JSONL file that matches the eval \n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalJsonlRunDataSource {
    #[doc = "The type of data source. Always `jsonl`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalJsonlRunDataSourceType,
    #[serde(rename = "source")]
    pub source: CreateEvalJsonlRunDataSourceSource,
}
#[doc = "The object type, which is always `label_model`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalLabelModelGraderType {
    #[default]
    #[serde(rename = "label_model")]
    LabelModel,
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalLabelModelGrader {
    #[doc = "The object type, which is always `label_model`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalLabelModelGraderType,
    #[doc = "The name of the grader."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    #[serde(rename = "input")]
    pub input: Vec<CreateEvalItem>,
    #[doc = "The labels to classify to each item in the evaluation."]
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    #[serde(rename = "passing_labels")]
    pub passing_labels: Vec<String>,
}
#[doc = "The type of data source. Always `logs`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalLogsDataSourceConfigType {
    #[default]
    #[serde(rename = "logs")]
    Logs,
}
#[doc = "A data source config which specifies the metadata property of your stored completions query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateEvalLogsDataSourceConfig {
    #[doc = "The type of data source. Always `logs`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalLogsDataSourceConfigType,
    #[doc = "Metadata filters for the logs data source."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[doc = "The configuration for the data source used for the evaluation runs."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalRequestDataSourceConfig {
    Custom(CreateEvalCustomDataSourceConfig),
    Logs(CreateEvalLogsDataSourceConfig),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalRequestTestingCriteria {
    LabelModel(CreateEvalLabelModelGrader),
    StringCheck(EvalStringCheckGrader),
    TextSimilarity(EvalTextSimilarityGrader),
    Python(EvalPythonGrader),
    ScoreModel(EvalScoreModelGrader),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalRequest {
    #[doc = "The name of the evaluation."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "The configuration for the data source used for the evaluation runs."]
    #[serde(rename = "data_source_config")]
    pub data_source_config: CreateEvalRequestDataSourceConfig,
    #[doc = "A list of graders for all eval runs in this group."]
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: Vec<CreateEvalRequestTestingCriteria>,
}
#[doc = "The type of run data source. Always `completions`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateEvalResponsesRunDataSourceType {
    #[default]
    #[serde(rename = "completions")]
    Completions,
}
#[doc = "The type of input messages. Always `template`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateEvalResponsesRunDataSourceInputMessages0Type {
    #[serde(rename = "template")]
    Template,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalResponsesRunDataSourceInputMessages0Template0 {
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    #[serde(rename = "role")]
    pub role: String,
    #[doc = "The content of the message."]
    #[serde(rename = "content")]
    pub content: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceInputMessages0Template {
    _0(CreateEvalResponsesRunDataSourceInputMessages0Template0),
    EvalItem(EvalItem),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalResponsesRunDataSourceInputMessages0 {
    #[doc = "The type of input messages. Always `template`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalResponsesRunDataSourceInputMessages0Type,
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    #[serde(rename = "template")]
    pub template: Vec<CreateEvalResponsesRunDataSourceInputMessages0Template>,
}
#[doc = "The type of input messages. Always `item_reference`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateEvalResponsesRunDataSourceInputMessages1Type {
    #[serde(rename = "item_reference")]
    ItemReference,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalResponsesRunDataSourceInputMessages1 {
    #[doc = "The type of input messages. Always `item_reference`."]
    #[serde(rename = "type")]
    pub type_: CreateEvalResponsesRunDataSourceInputMessages1Type,
    #[doc = "A reference to a variable in the \"item\" namespace. Ie, \"item.name\""]
    #[serde(rename = "item_reference")]
    pub item_reference: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceInputMessages {
    _0(CreateEvalResponsesRunDataSourceInputMessages0),
    _1(CreateEvalResponsesRunDataSourceInputMessages1),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateEvalResponsesRunDataSourceSamplingParams {
    #[doc = "A higher temperature increases randomness in the outputs."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "The maximum number of tokens in the generated output."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceSource {
    FileContent(EvalJsonlFileContentSource),
    FileId(EvalJsonlFileIdSource),
    EvalResponsesSource(EvalResponsesSource),
}
#[doc = "A ResponsesRunDataSource object describing a model sampling configuration.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalResponsesRunDataSource {
    #[doc = "The type of run data source. Always `completions`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateEvalResponsesRunDataSourceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_messages")]
    pub input_messages: Option<CreateEvalResponsesRunDataSourceInputMessages>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sampling_params")]
    pub sampling_params: Option<CreateEvalResponsesRunDataSourceSamplingParams>,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "source")]
    pub source: CreateEvalResponsesRunDataSourceSource,
}
#[doc = "Details about the run's data source."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalRunRequestDataSource {
    Jsonl(CreateEvalJsonlRunDataSource),
    Completions0(CreateEvalCompletionsRunDataSource),
    Completions1(CreateEvalResponsesRunDataSource),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateEvalRunRequest {
    #[doc = "The name of the run."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "Details about the run's data source."]
    #[serde(rename = "data_source")]
    pub data_source: CreateEvalRunRequestDataSource,
}
#[doc = "The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateFileRequestPurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "vision")]
    Vision,
    #[serde(rename = "user_data")]
    UserData,
    #[serde(rename = "evals")]
    Evals,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateFileRequest {
    #[doc = "The File object (not file name) to be uploaded.\n"]
    #[serde(rename = "file")]
    pub file: Vec<u8>,
    #[doc = "The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets\n"]
    #[serde(rename = "purpose")]
    pub purpose: CreateFileRequestPurpose,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateFineTuningCheckpointPermissionRequest {
    #[doc = "The project identifiers to grant access to."]
    #[serde(rename = "project_ids")]
    pub project_ids: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
    _0(CreateFineTuningJobRequestHyperparametersBatchSize0),
    _1(u64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    _0(CreateFineTuningJobRequestHyperparametersLearningRateMultiplier0),
    _1(f64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
    _0(CreateFineTuningJobRequestHyperparametersNEpochs0),
    _1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job.\nThis value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateFineTuningJobRequestHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_size")]
    pub batch_size: Option<CreateFineTuningJobRequestHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier:
        Option<CreateFineTuningJobRequestHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<CreateFineTuningJobRequestHyperparametersNEpochs>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateFineTuningJobRequestIntegrationType0 {
    #[default]
    #[serde(rename = "wandb")]
    Wandb,
}
#[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestIntegrationType {
    _0(CreateFineTuningJobRequestIntegrationType0),
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateFineTuningJobRequestIntegrationWandb {
    #[doc = "The name of the project that the new run will be created under.\n"]
    #[serde(rename = "project")]
    pub project: String,
    #[doc = "A display name to set for the run. If not set, we will use the Job ID as the name.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The entity to use for the run. This allows you to set the team or username of the WandB user that you would\nlike associated with the run. If not set, the default entity for the registered WandB API key is used.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entity")]
    pub entity: Option<String>,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateFineTuningJobRequestIntegration {
    #[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
    #[serde(rename = "type")]
    pub type_: CreateFineTuningJobRequestIntegrationType,
    #[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
    #[serde(rename = "wandb")]
    pub wandb: CreateFineTuningJobRequestIntegrationWandb,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateFineTuningJobRequest {
    #[doc = "The name of the model to fine-tune. You can select one of the\n[supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned).\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The ID of an uploaded file that contains training data.\n\nSee [upload file](/docs/api-reference/files/create) for how to upload a file.\n\nYour dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.\n\nThe contents of the file should differ depending on if the model uses the [chat](/docs/api-reference/fine-tuning/chat-input), [completions](/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](/docs/api-reference/fine-tuning/preference-input) format.\n\nSee the [fine-tuning guide](/docs/guides/fine-tuning) for more details.\n"]
    #[serde(rename = "training_file")]
    pub training_file: String,
    #[doc = "The hyperparameters used for the fine-tuning job.\nThis value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: Option<CreateFineTuningJobRequestHyperparameters>,
    #[doc = "A string of up to 64 characters that will be added to your fine-tuned model name.\n\nFor example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "suffix")]
    pub suffix: Option<String>,
    #[doc = "The ID of an uploaded file that contains validation data.\n\nIf you provide this file, the data is used to generate validation\nmetrics periodically during fine-tuning. These metrics can be viewed in\nthe fine-tuning results file.\nThe same data should not be present in both train and validation files.\n\nYour dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.\n\nSee the [fine-tuning guide](/docs/guides/fine-tuning) for more details.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "validation_file")]
    pub validation_file: Option<String>,
    #[doc = "A list of integrations to enable for your fine-tuning job."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "integrations")]
    pub integrations: Option<Vec<CreateFineTuningJobRequestIntegration>>,
    #[doc = "The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.\nIf a seed is not specified, one will be generated for you.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "seed")]
    pub seed: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "method")]
    pub method: Option<FineTuneMethod>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less \nthan 25MB. You can provide up to 16 images.\n\nFor `dall-e-2`, you can only provide one image, and it should be a square \n`png` file less than 4MB.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateImageEditRequestImage {
    _0(Vec<u8>),
    _1(Vec<Vec<u8>>),
}
#[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageEditRequestSize {
    #[serde(rename = "256x256")]
    _256x256,
    #[serde(rename = "512x512")]
    _512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    _1024x1024,
    #[serde(rename = "1536x1024")]
    _1536x1024,
    #[serde(rename = "1024x1536")]
    _1024x1536,
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageEditRequestResponseFormat {
    #[default]
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}
#[doc = "The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageEditRequestQuality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateImageEditRequest {
    #[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less \nthan 25MB. You can provide up to 16 images.\n\nFor `dall-e-2`, you can only provide one image, and it should be a square \n`png` file less than 4MB.\n"]
    #[serde(rename = "image")]
    pub image: CreateImageEditRequestImage,
    #[doc = "A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`, and 32000 characters for `gpt-image-1`."]
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[doc = "An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "mask")]
    pub mask: Option<Vec<u8>>,
    #[doc = "The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n: Option<u64>,
    #[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "size")]
    pub size: Option<CreateImageEditRequestSize>,
    #[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateImageEditRequestResponseFormat>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
    #[doc = "The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quality")]
    pub quality: Option<CreateImageEditRequestQuality>,
}
#[doc = "The quality of the image that will be generated. \n\n- `auto` (default value) will automatically select the best quality for the given model.\n- `high`, `medium` and `low` are supported for `gpt-image-1`.\n- `hd` and `standard` are supported for `dall-e-3`.\n- `standard` is the only option for `dall-e-2`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestQuality {
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "hd")]
    Hd,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestResponseFormat {
    #[default]
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}
#[doc = "The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestOutputFormat {
    #[default]
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
}
#[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestSize {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "1024x1024")]
    _1024x1024,
    #[serde(rename = "1536x1024")]
    _1536x1024,
    #[serde(rename = "1024x1536")]
    _1024x1536,
    #[serde(rename = "256x256")]
    _256x256,
    #[serde(rename = "512x512")]
    _512x512,
    #[serde(rename = "1792x1024")]
    _1792x1024,
    #[serde(rename = "1024x1792")]
    _1024x1792,
}
#[doc = "Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value)."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestModeration {
    #[serde(rename = "low")]
    Low,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Allows to set transparency for the background of the generated image(s). \nThis parameter is only supported for `gpt-image-1`. Must be one of \n`transparent`, `opaque` or `auto` (default value). When `auto` is used, the \nmodel will automatically determine the best background for the image.\n\nIf `transparent`, the output format needs to support transparency, so it \nshould be set to either `png` (default value) or `webp`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestBackground {
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "opaque")]
    Opaque,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageRequestStyle {
    #[default]
    #[serde(rename = "vivid")]
    Vivid,
    #[serde(rename = "natural")]
    Natural,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateImageRequest {
    #[doc = "A text description of the desired image(s). The maximum length is 32000 characters for `gpt-image-1`, 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`."]
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[doc = "The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n: Option<u64>,
    #[doc = "The quality of the image that will be generated. \n\n- `auto` (default value) will automatically select the best quality for the given model.\n- `high`, `medium` and `low` are supported for `gpt-image-1`.\n- `hd` and `standard` are supported for `dall-e-3`.\n- `standard` is the only option for `dall-e-2`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quality")]
    pub quality: Option<CreateImageRequestQuality>,
    #[doc = "The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateImageRequestResponseFormat>,
    #[doc = "The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_format")]
    pub output_format: Option<CreateImageRequestOutputFormat>,
    #[doc = "The compression level (0-100%) for the generated images. This parameter is only supported for `gpt-image-1` with the `webp` or `jpeg` output formats, and defaults to 100."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_compression")]
    pub output_compression: Option<u64>,
    #[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "size")]
    pub size: Option<CreateImageRequestSize>,
    #[doc = "Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value)."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "moderation")]
    pub moderation: Option<CreateImageRequestModeration>,
    #[doc = "Allows to set transparency for the background of the generated image(s). \nThis parameter is only supported for `gpt-image-1`. Must be one of \n`transparent`, `opaque` or `auto` (default value). When `auto` is used, the \nmodel will automatically determine the best background for the image.\n\nIf `transparent`, the output format needs to support transparency, so it \nshould be set to either `png` (default value) or `webp`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "background")]
    pub background: Option<CreateImageRequestBackground>,
    #[doc = "The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "style")]
    pub style: Option<CreateImageRequestStyle>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
}
#[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageVariationRequestResponseFormat {
    #[default]
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    B64Json,
}
#[doc = "The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateImageVariationRequestSize {
    #[serde(rename = "256x256")]
    _256x256,
    #[serde(rename = "512x512")]
    _512x512,
    #[default]
    #[serde(rename = "1024x1024")]
    _1024x1024,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateImageVariationRequest {
    #[doc = "The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square."]
    #[serde(rename = "image")]
    pub image: Vec<u8>,
    #[doc = "The model to use for image generation. Only `dall-e-2` is supported at this time."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n")]
    pub n: Option<u64>,
    #[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateImageVariationRequestResponseFormat>,
    #[doc = "The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "size")]
    pub size: Option<CreateImageVariationRequestSize>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
}
#[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateMessageRequestRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestContent1 {
    ImageFile(MessageContentImageFileObject),
    ImageUrl(MessageContentImageUrlObject),
    Text(MessageRequestContentTextObject),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestContent {
    _0(String),
    _1(Vec<CreateMessageRequestContent1>),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestAttachmentsTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearchTypeOnly),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateMessageRequestAttachments {
    #[doc = "The ID of the file to attach to the message."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "The tools to add this file to."]
    #[serde(rename = "tools")]
    pub tools: Vec<CreateMessageRequestAttachmentsTool>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateMessageRequest {
    #[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
    #[serde(rename = "role")]
    pub role: CreateMessageRequestRole,
    #[serde(rename = "content")]
    pub content: CreateMessageRequestContent,
    #[doc = "A list of files attached to the message, and the tools they should be added to."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attachments")]
    pub attachments: Option<Vec<CreateMessageRequestAttachments>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
pub type CreateModelResponseProperties = ModelResponseProperties;
#[doc = "Always `image_url`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationRequestInput2ImageUrlType {
    #[default]
    #[serde(rename = "image_url")]
    ImageUrl,
}
#[doc = "Contains either an image URL or a data URL for a base64 encoded image."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationRequestInput2ImageUrlImageUrl {
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    #[serde(rename = "url")]
    pub url: String,
}
#[doc = "An object describing an image to classify."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationRequestInput2ImageUrl {
    #[doc = "Always `image_url`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateModerationRequestInput2ImageUrlType,
    #[doc = "Contains either an image URL or a data URL for a base64 encoded image."]
    #[serde(rename = "image_url")]
    pub image_url: CreateModerationRequestInput2ImageUrlImageUrl,
}
#[doc = "Always `text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationRequestInput2TextType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[doc = "An object describing text to classify."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationRequestInput2Text {
    #[doc = "Always `text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateModerationRequestInput2TextType,
    #[doc = "A string of text to classify."]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateModerationRequestInput2 {
    ImageUrl(CreateModerationRequestInput2ImageUrl),
    Text(CreateModerationRequestInput2Text),
}
#[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateModerationRequestInput {
    _0(String),
    _1(Vec<String>),
    _2(Vec<CreateModerationRequestInput2>),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationRequest {
    #[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
    #[serde(rename = "input")]
    pub input: CreateModerationRequestInput,
    #[doc = "The content moderation model you would like to use. Learn more in\n[the moderation guide](/docs/guides/moderation), and learn about\navailable models [here](/docs/models#moderation).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[doc = "A list of the categories, and whether they are flagged or not."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationResponseResultCategories {
    #[doc = "Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment."]
    #[serde(rename = "hate")]
    pub hate: bool,
    #[doc = "Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[doc = "Content that expresses, incites, or promotes harassing language towards any target."]
    #[serde(rename = "harassment")]
    pub harassment: bool,
    #[doc = "Harassment content that also includes violence or serious harm towards any target."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: bool,
    #[doc = "Content that includes instructions or advice that facilitate the planning or execution of wrongdoing, or that gives advice or instruction on how to commit illicit acts. For example, \"how to shoplift\" would fit this category."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "illicit")]
    pub illicit: Option<bool>,
    #[doc = "Content that includes instructions or advice that facilitate the planning or execution of wrongdoing that also includes violence, or that gives advice or instruction on the procurement of any weapon."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: Option<bool>,
    #[doc = "Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders."]
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    #[doc = "Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: bool,
    #[doc = "Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: bool,
    #[doc = "Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness)."]
    #[serde(rename = "sexual")]
    pub sexual: bool,
    #[doc = "Sexual content that includes an individual who is under 18 years old."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    #[doc = "Content that depicts death, violence, or physical injury."]
    #[serde(rename = "violence")]
    pub violence: bool,
    #[doc = "Content that depicts death, violence, or physical injury in graphic detail."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}
#[doc = "A list of the categories along with their scores as predicted by model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationResponseResultCategoryScores {
    #[doc = "The score for the category 'hate'."]
    #[serde(rename = "hate")]
    pub hate: f64,
    #[doc = "The score for the category 'hate/threatening'."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[doc = "The score for the category 'harassment'."]
    #[serde(rename = "harassment")]
    pub harassment: f64,
    #[doc = "The score for the category 'harassment/threatening'."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening: f64,
    #[doc = "The score for the category 'illicit'."]
    #[serde(rename = "illicit")]
    pub illicit: f64,
    #[doc = "The score for the category 'illicit/violent'."]
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: f64,
    #[doc = "The score for the category 'self-harm'."]
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    #[doc = "The score for the category 'self-harm/intent'."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent: f64,
    #[doc = "The score for the category 'self-harm/instructions'."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions: f64,
    #[doc = "The score for the category 'sexual'."]
    #[serde(rename = "sexual")]
    pub sexual: f64,
    #[doc = "The score for the category 'sexual/minors'."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    #[doc = "The score for the category 'violence'."]
    #[serde(rename = "violence")]
    pub violence: f64,
    #[doc = "The score for the category 'violence/graphic'."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesHate {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesHateThreatening {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesHarassment {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesHarassmentThreatening {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesIllicit {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesIllicitViolent {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesSelfHarm {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesSelfHarmIntent {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesSelfHarmInstructions {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesSexual {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesSexualMinors {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesViolence {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateModerationResponseResultCategoryAppliedInputTypesViolenceGraphic {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}
#[doc = "A list of the categories along with the input type(s) that the score applies to."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationResponseResultCategoryAppliedInputTypes {
    #[doc = "The applied input type(s) for the category 'hate'."]
    #[serde(rename = "hate")]
    pub hate: Vec<CreateModerationResponseResultCategoryAppliedInputTypesHate>,
    #[doc = "The applied input type(s) for the category 'hate/threatening'."]
    #[serde(rename = "hate/threatening")]
    pub hate_threatening:
        Vec<CreateModerationResponseResultCategoryAppliedInputTypesHateThreatening>,
    #[doc = "The applied input type(s) for the category 'harassment'."]
    #[serde(rename = "harassment")]
    pub harassment: Vec<CreateModerationResponseResultCategoryAppliedInputTypesHarassment>,
    #[doc = "The applied input type(s) for the category 'harassment/threatening'."]
    #[serde(rename = "harassment/threatening")]
    pub harassment_threatening:
        Vec<CreateModerationResponseResultCategoryAppliedInputTypesHarassmentThreatening>,
    #[doc = "The applied input type(s) for the category 'illicit'."]
    #[serde(rename = "illicit")]
    pub illicit: Vec<CreateModerationResponseResultCategoryAppliedInputTypesIllicit>,
    #[doc = "The applied input type(s) for the category 'illicit/violent'."]
    #[serde(rename = "illicit/violent")]
    pub illicit_violent: Vec<CreateModerationResponseResultCategoryAppliedInputTypesIllicitViolent>,
    #[doc = "The applied input type(s) for the category 'self-harm'."]
    #[serde(rename = "self-harm")]
    pub self_harm: Vec<CreateModerationResponseResultCategoryAppliedInputTypesSelfHarm>,
    #[doc = "The applied input type(s) for the category 'self-harm/intent'."]
    #[serde(rename = "self-harm/intent")]
    pub self_harm_intent:
        Vec<CreateModerationResponseResultCategoryAppliedInputTypesSelfHarmIntent>,
    #[doc = "The applied input type(s) for the category 'self-harm/instructions'."]
    #[serde(rename = "self-harm/instructions")]
    pub self_harm_instructions:
        Vec<CreateModerationResponseResultCategoryAppliedInputTypesSelfHarmInstructions>,
    #[doc = "The applied input type(s) for the category 'sexual'."]
    #[serde(rename = "sexual")]
    pub sexual: Vec<CreateModerationResponseResultCategoryAppliedInputTypesSexual>,
    #[doc = "The applied input type(s) for the category 'sexual/minors'."]
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: Vec<CreateModerationResponseResultCategoryAppliedInputTypesSexualMinors>,
    #[doc = "The applied input type(s) for the category 'violence'."]
    #[serde(rename = "violence")]
    pub violence: Vec<CreateModerationResponseResultCategoryAppliedInputTypesViolence>,
    #[doc = "The applied input type(s) for the category 'violence/graphic'."]
    #[serde(rename = "violence/graphic")]
    pub violence_graphic:
        Vec<CreateModerationResponseResultCategoryAppliedInputTypesViolenceGraphic>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationResponseResult {
    #[doc = "Whether any of the below categories are flagged."]
    #[serde(rename = "flagged")]
    pub flagged: bool,
    #[doc = "A list of the categories, and whether they are flagged or not."]
    #[serde(rename = "categories")]
    pub categories: CreateModerationResponseResultCategories,
    #[doc = "A list of the categories along with their scores as predicted by model."]
    #[serde(rename = "category_scores")]
    pub category_scores: CreateModerationResponseResultCategoryScores,
    #[doc = "A list of the categories along with the input type(s) that the score applies to."]
    #[serde(rename = "category_applied_input_types")]
    pub category_applied_input_types: CreateModerationResponseResultCategoryAppliedInputTypes,
}
#[doc = "Represents if a given text input is potentially harmful."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateModerationResponse {
    #[doc = "The unique identifier for the moderation request."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The model used to generate the moderation results."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "A list of moderation objects."]
    #[serde(rename = "results")]
    pub results: Vec<CreateModerationResponseResult>,
}
#[doc = "Text, image, or file inputs to the model, used to generate a response.\n\nLearn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Image inputs](/docs/guides/images)\n- [File inputs](/docs/guides/pdf-files)\n- [Conversation state](/docs/guides/conversation-state)\n- [Function calling](/docs/guides/function-calling)\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateResponseInput {
    _0(String),
    _1(Vec<InputItem>),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateResponse {
    #[builder(default)]
    #[serde(flatten)]
    pub create_model_response_properties: CreateModelResponseProperties,
    #[builder(default)]
    #[serde(flatten)]
    pub response_properties: ResponseProperties,
    #[doc = "Text, image, or file inputs to the model, used to generate a response.\n\nLearn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Image inputs](/docs/guides/images)\n- [File inputs](/docs/guides/pdf-files)\n- [Conversation state](/docs/guides/conversation-state)\n- [Function calling](/docs/guides/function-calling)\n"]
    #[serde(rename = "input")]
    pub input: CreateResponseInput,
    #[doc = "Specify additional output data to include in the model response. Currently\nsupported values are:\n- `file_search_call.results`: Include the search results of\n  the file search tool call.\n- `message.input_image.image_url`: Include image urls from the input message.\n- `computer_call_output.output.image_url`: Include image urls from the computer call output.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include")]
    pub include: Option<Vec<Includable>>,
    #[doc = "Whether to allow the model to run tool calls in parallel.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<bool>,
    #[doc = "Whether to store the generated model response for later retrieval via\nAPI.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "store")]
    pub store: Option<bool>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\nSee the [Streaming section below](/docs/api-reference/responses-streaming)\nfor more information.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateRunRequestTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateRunRequest {
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run."]
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    #[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[doc = "Overrides the [instructions](/docs/api-reference/assistants/createAssistant) of the assistant. This is useful for modifying the behavior on a per-run basis."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additional_instructions")]
    pub additional_instructions: Option<String>,
    #[doc = "Adds additional messages to the thread before creating the run."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "additional_messages")]
    pub additional_messages: Option<Vec<CreateMessageRequest>>,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<CreateRunRequestTool>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_prompt_tokens")]
    pub max_prompt_tokens: Option<u64>,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: Option<TruncationObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<AssistantsApiToolChoiceOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<ParallelToolCalls>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateSpeechRequestResponseFormat {
    #[default]
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "opus")]
    Opus,
    #[serde(rename = "aac")]
    Aac,
    #[serde(rename = "flac")]
    Flac,
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "pcm")]
    Pcm,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateSpeechRequest {
    #[doc = "One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The text to generate audio for. The maximum length is 4096 characters."]
    #[serde(rename = "input")]
    pub input: String,
    #[doc = "Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](/docs/guides/text-to-speech#voice-options)."]
    #[serde(rename = "voice")]
    pub voice: VoiceIdsShared,
    #[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateSpeechRequestResponseFormat>,
    #[doc = "The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "speed")]
    pub speed: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadAndRunRequestTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadAndRunRequestToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<CreateThreadAndRunRequestToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<CreateThreadAndRunRequestToolResourcesFileSearch>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadAndRunRequest {
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run."]
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "thread")]
    pub thread: Option<CreateThreadRequest>,
    #[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<CreateThreadAndRunRequestTool>>,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<CreateThreadAndRunRequestToolResources>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_prompt_tokens")]
    pub max_prompt_tokens: Option<u64>,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: Option<TruncationObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<AssistantsApiToolChoiceOption>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<ParallelToolCalls>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[doc = "Always `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAutoType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAuto {
    #[doc = "Always `auto`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAutoType,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticStatic {
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: u64,
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStatic {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticType,
    #[serde(rename = "static")]
    pub static_: CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStaticStatic,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategy {
    Auto(CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyAuto),
    Static(CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategyStatic),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch0VectorStore {
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy:
        Option<CreateThreadRequestToolResourcesFileSearch0VectorStoreChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch0 {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Vec<String>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_stores")]
    pub vector_stores: Option<Vec<CreateThreadRequestToolResourcesFileSearch0VectorStore>>,
}
#[doc = "Always `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAutoType {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAuto {
    #[doc = "Always `auto`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAutoType,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticStatic {
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: u64,
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStatic {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticType,
    #[serde(rename = "static")]
    pub static_: CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStaticStatic,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategy {
    Auto(CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyAuto),
    Static(CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategyStatic),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch1VectorStore {
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy:
        Option<CreateThreadRequestToolResourcesFileSearch1VectorStoreChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResourcesFileSearch1 {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[serde(rename = "vector_stores")]
    pub vector_stores: Vec<CreateThreadRequestToolResourcesFileSearch1VectorStore>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadRequestToolResourcesFileSearch {
    _0(CreateThreadRequestToolResourcesFileSearch0),
    _1(CreateThreadRequestToolResourcesFileSearch1),
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequestToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<CreateThreadRequestToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<CreateThreadRequestToolResourcesFileSearch>,
}
#[doc = "Options to create a new thread. If no thread is provided when running a \nrequest, an empty thread will be created.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateThreadRequest {
    #[doc = "A list of [messages](/docs/api-reference/messages) to start the thread with."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messages")]
    pub messages: Option<Vec<CreateMessageRequest>>,
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<CreateThreadRequestToolResources>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateTranscriptionRequestTimestampGranularities {
    #[serde(rename = "word")]
    Word,
    #[serde(rename = "segment")]
    Segment,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranscriptionRequest {
    #[doc = "The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.\n"]
    #[serde(rename = "file")]
    pub file: Vec<u8>,
    #[doc = "ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model).\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should match the audio language.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AudioResponseFormat>,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Additional information to include in the transcription response. \n`logprobs` will return the log probabilities of the tokens in the \nresponse to understand the model's confidence in the transcription. \n`logprobs` only works with response_format set to `json` and only with \nthe models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include[]")]
    pub include: Option<Vec<TranscriptionInclude>>,
    #[doc = "The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timestamp_granularities[]")]
    pub timestamp_granularities: Option<Vec<CreateTranscriptionRequestTimestampGranularities>>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). \nSee the [Streaming section of the Speech-to-Text guide](/docs/guides/speech-to-text?lang=curl#streaming-transcriptions)\nfor more information.\n\nNote: Streaming is not supported for the `whisper-1` model and will be ignored.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateTranscriptionResponseJsonLogprob {
    #[doc = "The token in the transcription."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[doc = "The log probability of the token."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprob")]
    pub logprob: Option<f64>,
    #[doc = "The bytes of the token."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<f64>>,
}
#[doc = "Represents a transcription response returned by model, based on the provided input."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranscriptionResponseJson {
    #[doc = "The transcribed text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<CreateTranscriptionResponseJsonLogprob>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateTranscriptionResponseStreamEvent {
    TranscriptTextDelta(TranscriptTextDeltaEvent),
    TranscriptTextDone(TranscriptTextDoneEvent),
}
#[doc = "Represents a verbose json transcription response returned by model, based on the provided input."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranscriptionResponseVerboseJson {
    #[doc = "The language of the input audio."]
    #[serde(rename = "language")]
    pub language: String,
    #[doc = "The duration of the input audio."]
    #[serde(rename = "duration")]
    pub duration: f64,
    #[doc = "The transcribed text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Extracted words and their corresponding timestamps."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "words")]
    pub words: Option<Vec<TranscriptionWord>>,
    #[doc = "Segments of the transcribed text and their corresponding details."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "segments")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum CreateTranslationRequestResponseFormat {
    #[default]
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "srt")]
    Srt,
    #[serde(rename = "verbose_json")]
    VerboseJson,
    #[serde(rename = "vtt")]
    Vtt,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranslationRequest {
    #[doc = "The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.\n"]
    #[serde(rename = "file")]
    pub file: Vec<u8>,
    #[doc = "ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.\n"]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should be in English.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
    #[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<CreateTranslationRequestResponseFormat>,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranslationResponseJson {
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateTranslationResponseVerboseJson {
    #[doc = "The language of the output translation (always `english`)."]
    #[serde(rename = "language")]
    pub language: String,
    #[doc = "The duration of the input audio."]
    #[serde(rename = "duration")]
    pub duration: f64,
    #[doc = "The translated text."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Segments of the translated text and their corresponding details."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "segments")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}
#[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum CreateUploadRequestPurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "vision")]
    Vision,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateUploadRequest {
    #[doc = "The name of the file to upload.\n"]
    #[serde(rename = "filename")]
    pub filename: String,
    #[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose).\n"]
    #[serde(rename = "purpose")]
    pub purpose: CreateUploadRequestPurpose,
    #[doc = "The number of bytes in the file you are uploading.\n"]
    #[serde(rename = "bytes")]
    pub bytes: u64,
    #[doc = "The MIME type of the file.\n\nThis must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.\n"]
    #[serde(rename = "mime_type")]
    pub mime_type: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateVectorStoreFileBatchRequest {
    #[doc = "A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files."]
    #[serde(rename = "file_ids")]
    pub file_ids: Vec<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<VectorStoreFileAttributes>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct CreateVectorStoreFileRequest {
    #[doc = "A [File](/docs/api-reference/files) ID that the vector store should use. Useful for tools like `file_search` that can access files."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<ChunkingStrategyRequestParam>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<VectorStoreFileAttributes>,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum CreateVectorStoreRequestChunkingStrategy {
    Auto(AutoChunkingStrategyRequestParam),
    Static(StaticChunkingStrategyRequestParam),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct CreateVectorStoreRequest {
    #[doc = "A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
    #[doc = "The name of the vector store."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_after")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<CreateVectorStoreRequestChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteAssistantResponseObject {
    #[default]
    #[serde(rename = "assistant.deleted")]
    AssistantDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteAssistantResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteAssistantResponseObject,
}
#[doc = "The object type, must be `certificate.deleted`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteCertificateResponseObject {
    #[default]
    #[serde(rename = "certificate.deleted")]
    CertificateDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteCertificateResponse {
    #[doc = "The object type, must be `certificate.deleted`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteCertificateResponseObject,
    #[doc = "The ID of the certificate that was deleted."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteFileResponseObject {
    #[default]
    #[serde(rename = "file")]
    File,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteFileResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteFileResponseObject,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteFineTuningCheckpointPermissionResponseObject {
    #[default]
    #[serde(rename = "checkpoint.permission")]
    CheckpointPermission,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteFineTuningCheckpointPermissionResponse {
    #[doc = "The ID of the fine-tuned model checkpoint permission that was deleted."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteFineTuningCheckpointPermissionResponseObject,
    #[doc = "Whether the fine-tuned model checkpoint permission was successfully deleted."]
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteMessageResponseObject {
    #[default]
    #[serde(rename = "thread.message.deleted")]
    ThreadMessageDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteMessageResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteMessageResponseObject,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteModelResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "object")]
    pub object: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteThreadResponseObject {
    #[default]
    #[serde(rename = "thread.deleted")]
    ThreadDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteThreadResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteThreadResponseObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteVectorStoreFileResponseObject {
    #[default]
    #[serde(rename = "vector_store.file.deleted")]
    VectorStoreFileDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteVectorStoreFileResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteVectorStoreFileResponseObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DeleteVectorStoreResponseObject {
    #[default]
    #[serde(rename = "vector_store.deleted")]
    VectorStoreDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DeleteVectorStoreResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: DeleteVectorStoreResponseObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DoneEventEvent {
    #[default]
    #[serde(rename = "done")]
    Done,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DoneEventData {
    #[default]
    #[serde(rename = "[DONE]")]
    Done,
}
#[doc = "Occurs when a stream ends."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct DoneEvent {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: DoneEventEvent,
    #[builder(default)]
    #[serde(rename = "data")]
    pub data: DoneEventData,
}
#[doc = "Specifies the event type. For a double click action, this property is \nalways set to `double_click`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DoubleClickType {
    #[default]
    #[serde(rename = "double_click")]
    DoubleClick,
}
#[doc = "A double click action.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct DoubleClick {
    #[doc = "Specifies the event type. For a double click action, this property is \nalways set to `double_click`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: DoubleClickType,
    #[doc = "The x-coordinate where the double click occurred.\n"]
    #[serde(rename = "x")]
    pub x: u64,
    #[doc = "The y-coordinate where the double click occurred.\n"]
    #[serde(rename = "y")]
    pub y: u64,
}
#[doc = "Specifies the event type. For a drag action, this property is \nalways set to `drag`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum DragType {
    #[default]
    #[serde(rename = "drag")]
    Drag,
}
#[doc = "A drag action.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Drag {
    #[doc = "Specifies the event type. For a drag action, this property is \nalways set to `drag`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: DragType,
    #[doc = "An array of coordinates representing the path of the drag action. Coordinates will appear as an array\nof objects, eg\n```\n[\n  { x: 100, y: 200 },\n  { x: 200, y: 300 }\n]\n```\n"]
    #[serde(rename = "path")]
    pub path: Vec<Coordinate>,
}
#[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum EasyInputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
#[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EasyInputMessageContent {
    _0(String),
    InputMessageContentList(InputMessageContentList),
}
#[doc = "The type of the message input. Always `message`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EasyInputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EasyInputMessage {
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    #[serde(rename = "role")]
    pub role: EasyInputMessageRole,
    #[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
    #[serde(rename = "content")]
    pub content: EasyInputMessageContent,
    #[doc = "The type of the message input. Always `message`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<EasyInputMessageType>,
}
#[doc = "The object type, which is always \"embedding\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EmbeddingObject {
    #[default]
    #[serde(rename = "embedding")]
    Embedding,
}
#[doc = "Represents an embedding vector returned by embedding endpoint.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Embedding {
    #[doc = "The index of the embedding in the list of embeddings."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "The embedding vector, which is a list of floats. The length of vector depends on the model as listed in the [embedding guide](/docs/guides/embeddings).\n"]
    #[serde(rename = "embedding")]
    pub embedding: Vec<f64>,
    #[doc = "The object type, which is always \"embedding\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EmbeddingObject,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Error {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    pub message: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ErrorEventEvent {
    #[default]
    #[serde(rename = "error")]
    Error,
}
#[doc = "Occurs when an [error](/docs/guides/error-codes#api-errors) occurs. This can happen due to an internal server error or a timeout."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ErrorEvent {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: ErrorEventEvent,
    #[serde(rename = "data")]
    pub data: Error,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ErrorResponse {
    #[serde(rename = "error")]
    pub error: Error,
}
#[doc = "The object type."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalObject {
    #[default]
    #[serde(rename = "eval")]
    Eval,
}
#[doc = "Configuration of data sources used in runs of the evaluation."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EvalDataSourceConfig {
    Custom(EvalCustomDataSourceConfig),
    StoredCompletions(EvalStoredCompletionsDataSourceConfig),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EvalTestingCriteria {
    LabelModel(EvalLabelModelGrader),
    StringCheck(EvalStringCheckGrader),
    TextSimilarity(EvalTextSimilarityGrader),
    Python(EvalPythonGrader),
    ScoreModel(EvalScoreModelGrader),
}
#[doc = "An Eval object with a data source config and testing criteria.\nAn Eval represents a task to be done for your LLM integration.\nLike:\n - Improve the quality of my chatbot\n - See how well my chatbot handles customer support\n - Check if o3-mini is better at my usecase than gpt-4o\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Eval {
    #[doc = "The object type."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalObject,
    #[doc = "Unique identifier for the evaluation."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the evaluation."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Configuration of data sources used in runs of the evaluation."]
    #[serde(rename = "data_source_config")]
    pub data_source_config: EvalDataSourceConfig,
    #[doc = "A list of testing criteria."]
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: Vec<EvalTestingCriteria>,
    #[doc = "The Unix timestamp (in seconds) for when the eval was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}
#[doc = "An object representing an error response from the Eval API.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalApiError {
    #[doc = "The error code."]
    #[serde(rename = "code")]
    pub code: String,
    #[doc = "The error message."]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "The type of data source. Always `custom`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalCustomDataSourceConfigType {
    #[default]
    #[serde(rename = "custom")]
    Custom,
}
#[doc = "A CustomDataSourceConfig which specifies the schema of your `item` and optionally `sample` namespaces.\nThe response schema defines the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalCustomDataSourceConfig {
    #[doc = "The type of data source. Always `custom`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalCustomDataSourceConfigType,
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    #[serde(rename = "schema")]
    pub schema: std::collections::HashMap<String, serde_json::Value>,
}
#[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum EvalItemRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
#[doc = "The type of the output text. Always `output_text`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalItemContentOutputTextType {
    #[default]
    #[serde(rename = "output_text")]
    OutputText,
}
#[doc = "A text output from the model.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalItemContentOutputText {
    #[doc = "The type of the output text. Always `output_text`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalItemContentOutputTextType,
    #[doc = "The text output from the model.\n"]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "Text inputs to the model - can contain template strings.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EvalItemContent {
    _0(String),
    InputText(InputTextContent),
    OutputText(EvalItemContentOutputText),
}
#[doc = "The type of the message input. Always `message`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalItemType {
    #[default]
    #[serde(rename = "message")]
    Message,
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalItem {
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    #[serde(rename = "role")]
    pub role: EvalItemRole,
    #[doc = "Text inputs to the model - can contain template strings.\n"]
    #[serde(rename = "content")]
    pub content: EvalItemContent,
    #[doc = "The type of the message input. Always `message`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<EvalItemType>,
}
#[doc = "The type of jsonl source. Always `file_content`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalJsonlFileContentSourceType {
    #[default]
    #[serde(rename = "file_content")]
    FileContent,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalJsonlFileContentSourceContent {
    #[serde(rename = "item")]
    pub item: std::collections::HashMap<String, serde_json::Value>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sample")]
    pub sample: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalJsonlFileContentSource {
    #[doc = "The type of jsonl source. Always `file_content`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalJsonlFileContentSourceType,
    #[doc = "The content of the jsonl file."]
    #[serde(rename = "content")]
    pub content: Vec<EvalJsonlFileContentSourceContent>,
}
#[doc = "The type of jsonl source. Always `file_id`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalJsonlFileIdSourceType {
    #[default]
    #[serde(rename = "file_id")]
    FileId,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalJsonlFileIdSource {
    #[doc = "The type of jsonl source. Always `file_id`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalJsonlFileIdSourceType,
    #[doc = "The identifier of the file."]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "The object type, which is always `label_model`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalLabelModelGraderType {
    #[default]
    #[serde(rename = "label_model")]
    LabelModel,
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalLabelModelGrader {
    #[doc = "The object type, which is always `label_model`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalLabelModelGraderType,
    #[doc = "The name of the grader."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "input")]
    pub input: Vec<EvalItem>,
    #[doc = "The labels to assign to each item in the evaluation."]
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    #[serde(rename = "passing_labels")]
    pub passing_labels: Vec<String>,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "An object representing a list of evals.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalList {
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalListObject,
    #[doc = "An array of eval objects.\n"]
    #[serde(rename = "data")]
    pub data: Vec<Eval>,
    #[doc = "The identifier of the first eval in the data array."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The identifier of the last eval in the data array."]
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[doc = "Indicates whether there are more evals available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The object type, which is always `python`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalPythonGraderType {
    #[default]
    #[serde(rename = "python")]
    Python,
}
#[doc = "A PythonGrader object that runs a python script on the input.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalPythonGrader {
    #[doc = "The object type, which is always `python`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalPythonGraderType,
    #[doc = "The name of the grader."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The source code of the python script."]
    #[serde(rename = "source")]
    pub source: String,
    #[doc = "The threshold for the score."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: Option<f64>,
    #[doc = "The image tag to use for the python script."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_tag")]
    pub image_tag: Option<String>,
}
#[doc = "The type of run data source. Always `responses`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum EvalResponsesSourceType {
    #[serde(rename = "responses")]
    Responses,
}
#[doc = "A EvalResponsesSource object describing a run data source configuration.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalResponsesSource {
    #[doc = "The type of run data source. Always `responses`."]
    #[serde(rename = "type")]
    pub type_: EvalResponsesSourceType,
    #[doc = "Metadata filter for the responses. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[doc = "The name of the model to find responses for. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "Optional search string for instructions. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions_search")]
    pub instructions_search: Option<String>,
    #[doc = "Only include items created after this timestamp (inclusive). This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "created_after")]
    pub created_after: Option<u64>,
    #[doc = "Only include items created before this timestamp (inclusive). This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "created_before")]
    pub created_before: Option<u64>,
    #[doc = "Whether the response has tool calls. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "has_tool_calls")]
    pub has_tool_calls: Option<bool>,
    #[doc = "Optional reasoning effort parameter. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[doc = "Sampling temperature. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Nucleus sampling parameter. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "List of user identifiers. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "users")]
    pub users: Option<Vec<String>>,
    #[doc = "Whether to allow parallel tool calls. This is a query parameter used to select responses."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "allow_parallel_tool_calls")]
    pub allow_parallel_tool_calls: Option<bool>,
}
#[doc = "The type of the object. Always \"eval.run\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalRunObject {
    #[default]
    #[serde(rename = "eval.run")]
    EvalRun,
}
#[doc = "Counters summarizing the outcomes of the evaluation run."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunResultCounts {
    #[doc = "Total number of executed output items."]
    #[serde(rename = "total")]
    pub total: u64,
    #[doc = "Number of output items that resulted in an error."]
    #[serde(rename = "errored")]
    pub errored: u64,
    #[doc = "Number of output items that failed to pass the evaluation."]
    #[serde(rename = "failed")]
    pub failed: u64,
    #[doc = "Number of output items that passed the evaluation."]
    #[serde(rename = "passed")]
    pub passed: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunPerModelUsage {
    #[doc = "The name of the model."]
    #[serde(rename = "model_name")]
    pub model_name: String,
    #[doc = "The number of invocations."]
    #[serde(rename = "invocation_count")]
    pub invocation_count: u64,
    #[doc = "The number of prompt tokens used."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "The number of completion tokens generated."]
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[doc = "The total number of tokens used."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
    #[doc = "The number of tokens retrieved from cache."]
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: u64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunPerTestingCriteriaResult {
    #[doc = "A description of the testing criteria."]
    #[serde(rename = "testing_criteria")]
    pub testing_criteria: String,
    #[doc = "Number of tests passed for this criteria."]
    #[serde(rename = "passed")]
    pub passed: u64,
    #[doc = "Number of tests failed for this criteria."]
    #[serde(rename = "failed")]
    pub failed: u64,
}
#[doc = "Information about the run's data source."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum EvalRunDataSource {
    Jsonl(CreateEvalJsonlRunDataSource),
    Completions0(CreateEvalCompletionsRunDataSource),
    Completions1(CreateEvalResponsesRunDataSource),
}
#[doc = "A schema representing an evaluation run.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRun {
    #[doc = "The type of the object. Always \"eval.run\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalRunObject,
    #[doc = "Unique identifier for the evaluation run."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The identifier of the associated evaluation."]
    #[serde(rename = "eval_id")]
    pub eval_id: String,
    #[doc = "The status of the evaluation run."]
    #[serde(rename = "status")]
    pub status: String,
    #[doc = "The model that is evaluated, if applicable."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The name of the evaluation run."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The URL to the rendered evaluation run report on the UI dashboard."]
    #[serde(rename = "report_url")]
    pub report_url: String,
    #[doc = "Counters summarizing the outcomes of the evaluation run."]
    #[serde(rename = "result_counts")]
    pub result_counts: EvalRunResultCounts,
    #[doc = "Usage statistics for each model during the evaluation run."]
    #[serde(rename = "per_model_usage")]
    pub per_model_usage: Vec<EvalRunPerModelUsage>,
    #[doc = "Results per testing criteria applied during the evaluation run."]
    #[serde(rename = "per_testing_criteria_results")]
    pub per_testing_criteria_results: Vec<EvalRunPerTestingCriteriaResult>,
    #[doc = "Information about the run's data source."]
    #[serde(rename = "data_source")]
    pub data_source: EvalRunDataSource,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "error")]
    pub error: EvalApiError,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalRunListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "An object representing a list of runs for an evaluation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunList {
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalRunListObject,
    #[doc = "An array of eval run objects.\n"]
    #[serde(rename = "data")]
    pub data: Vec<EvalRun>,
    #[doc = "The identifier of the first eval run in the data array."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The identifier of the last eval run in the data array."]
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[doc = "Indicates whether there are more evals available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The type of the object. Always \"eval.run.output_item\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalRunOutputItemObject {
    #[default]
    #[serde(rename = "eval.run.output_item")]
    EvalRunOutputItem,
}
#[doc = "An input message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItemSampleInput {
    #[doc = "The role of the message sender (e.g., system, user, developer)."]
    #[serde(rename = "role")]
    pub role: String,
    #[doc = "The content of the message."]
    #[serde(rename = "content")]
    pub content: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItemSampleOutput {
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<String>,
    #[doc = "The content of the message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<String>,
}
#[doc = "Token usage details for the sample."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItemSampleUsage {
    #[doc = "The total number of tokens used."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
    #[doc = "The number of completion tokens generated."]
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[doc = "The number of prompt tokens used."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "The number of tokens retrieved from cache."]
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: u64,
}
#[doc = "A sample containing the input and output of the evaluation run."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItemSample {
    #[doc = "An array of input messages."]
    #[serde(rename = "input")]
    pub input: Vec<EvalRunOutputItemSampleInput>,
    #[doc = "An array of output messages."]
    #[serde(rename = "output")]
    pub output: Vec<EvalRunOutputItemSampleOutput>,
    #[doc = "The reason why the sample generation was finished."]
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    #[doc = "The model used for generating the sample."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "Token usage details for the sample."]
    #[serde(rename = "usage")]
    pub usage: EvalRunOutputItemSampleUsage,
    #[serde(rename = "error")]
    pub error: EvalApiError,
    #[doc = "The sampling temperature used."]
    #[serde(rename = "temperature")]
    pub temperature: f64,
    #[doc = "The maximum number of tokens allowed for completion."]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: u64,
    #[doc = "The top_p value used for sampling."]
    #[serde(rename = "top_p")]
    pub top_p: f64,
    #[doc = "The seed used for generating the sample."]
    #[serde(rename = "seed")]
    pub seed: u64,
}
#[doc = "A schema representing an evaluation run output item.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItem {
    #[doc = "The type of the object. Always \"eval.run.output_item\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalRunOutputItemObject,
    #[doc = "Unique identifier for the evaluation run output item."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The identifier of the evaluation run associated with this output item."]
    #[serde(rename = "run_id")]
    pub run_id: String,
    #[doc = "The identifier of the evaluation group."]
    #[serde(rename = "eval_id")]
    pub eval_id: String,
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The status of the evaluation run."]
    #[serde(rename = "status")]
    pub status: String,
    #[doc = "The identifier for the data source item."]
    #[serde(rename = "datasource_item_id")]
    pub datasource_item_id: u64,
    #[doc = "Details of the input data source item."]
    #[serde(rename = "datasource_item")]
    pub datasource_item: std::collections::HashMap<String, serde_json::Value>,
    #[doc = "A list of results from the evaluation run."]
    #[serde(rename = "results")]
    pub results: Vec<std::collections::HashMap<String, serde_json::Value>>,
    #[doc = "A sample containing the input and output of the evaluation run."]
    #[serde(rename = "sample")]
    pub sample: EvalRunOutputItemSample,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalRunOutputItemListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "An object representing a list of output items for an evaluation run.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalRunOutputItemList {
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: EvalRunOutputItemListObject,
    #[doc = "An array of eval run output item objects.\n"]
    #[serde(rename = "data")]
    pub data: Vec<EvalRunOutputItem>,
    #[doc = "The identifier of the first eval run output item in the data array."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The identifier of the last eval run output item in the data array."]
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[doc = "Indicates whether there are more eval run output items available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The object type, which is always `score_model`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalScoreModelGraderType {
    #[default]
    #[serde(rename = "score_model")]
    ScoreModel,
}
#[doc = "A ScoreModelGrader object that uses a model to assign a score to the input.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalScoreModelGrader {
    #[doc = "The object type, which is always `score_model`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalScoreModelGraderType,
    #[doc = "The name of the grader."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The model to use for the evaluation."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The sampling parameters for the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sampling_params")]
    pub sampling_params: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[doc = "The input text. This may include template strings."]
    #[serde(rename = "input")]
    pub input: Vec<EvalItem>,
    #[doc = "The threshold for the score."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: Option<f64>,
    #[doc = "The range of the score. Defaults to `[0, 1]`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "range")]
    pub range: Option<Vec<f64>>,
}
#[doc = "The type of data source. Always `stored_completions`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalStoredCompletionsDataSourceConfigType {
    #[default]
    #[serde(rename = "stored_completions")]
    StoredCompletions,
}
#[doc = "A StoredCompletionsDataSourceConfig which specifies the metadata property of your stored completions query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\nThe schema returned by this data source config is used to defined what variables are available in your evals.\n`item` and `sample` are both defined when using this data source config.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalStoredCompletionsDataSourceConfig {
    #[doc = "The type of data source. Always `stored_completions`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalStoredCompletionsDataSourceConfigType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    #[serde(rename = "schema")]
    pub schema: std::collections::HashMap<String, serde_json::Value>,
}
#[doc = "The type of source. Always `stored_completions`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalStoredCompletionsSourceType {
    #[default]
    #[serde(rename = "stored_completions")]
    StoredCompletions,
}
#[doc = "A StoredCompletionsRunDataSource configuration describing a set of filters\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct EvalStoredCompletionsSource {
    #[doc = "The type of source. Always `stored_completions`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalStoredCompletionsSourceType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "An optional model to filter by (e.g., 'gpt-4o')."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "An optional Unix timestamp to filter items created after this time."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "created_after")]
    pub created_after: Option<u64>,
    #[doc = "An optional Unix timestamp to filter items created before this time."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "created_before")]
    pub created_before: Option<u64>,
    #[doc = "An optional maximum number of items to return."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "limit")]
    pub limit: Option<u64>,
}
#[doc = "The object type, which is always `string_check`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalStringCheckGraderType {
    #[default]
    #[serde(rename = "string_check")]
    StringCheck,
}
#[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum EvalStringCheckGraderOperation {
    #[serde(rename = "eq")]
    Eq,
    #[serde(rename = "ne")]
    Ne,
    #[serde(rename = "like")]
    Like,
    #[serde(rename = "ilike")]
    Ilike,
}
#[doc = "A StringCheckGrader object that performs a string comparison between input and reference using a specified operation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalStringCheckGrader {
    #[doc = "The object type, which is always `string_check`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalStringCheckGraderType,
    #[doc = "The name of the grader."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The input text. This may include template strings."]
    #[serde(rename = "input")]
    pub input: String,
    #[doc = "The reference text. This may include template strings."]
    #[serde(rename = "reference")]
    pub reference: String,
    #[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
    #[serde(rename = "operation")]
    pub operation: EvalStringCheckGraderOperation,
}
#[doc = "The type of grader."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum EvalTextSimilarityGraderType {
    #[default]
    #[serde(rename = "text_similarity")]
    TextSimilarity,
}
#[doc = "The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum EvalTextSimilarityGraderEvaluationMetric {
    #[serde(rename = "fuzzy_match")]
    FuzzyMatch,
    #[serde(rename = "bleu")]
    Bleu,
    #[serde(rename = "gleu")]
    Gleu,
    #[serde(rename = "meteor")]
    Meteor,
    #[serde(rename = "rouge_1")]
    Rouge1,
    #[serde(rename = "rouge_2")]
    Rouge2,
    #[serde(rename = "rouge_3")]
    Rouge3,
    #[serde(rename = "rouge_4")]
    Rouge4,
    #[serde(rename = "rouge_5")]
    Rouge5,
    #[serde(rename = "rouge_l")]
    RougeL,
}
#[doc = "A TextSimilarityGrader object which grades text based on similarity metrics.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct EvalTextSimilarityGrader {
    #[doc = "The type of grader."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: EvalTextSimilarityGraderType,
    #[doc = "The name of the grader."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The text being graded."]
    #[serde(rename = "input")]
    pub input: String,
    #[doc = "The text being graded against."]
    #[serde(rename = "reference")]
    pub reference: String,
    #[doc = "A float score where a value greater than or equal indicates a passing grade."]
    #[serde(rename = "pass_threshold")]
    pub pass_threshold: f64,
    #[doc = "The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`."]
    #[serde(rename = "evaluation_metric")]
    pub evaluation_metric: EvalTextSimilarityGraderEvaluationMetric,
}
#[doc = "The type of the file path. Always `file_path`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FilePathType {
    #[default]
    #[serde(rename = "file_path")]
    FilePath,
}
#[doc = "A path to a file.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FilePath {
    #[doc = "The type of the file path. Always `file_path`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FilePathType,
    #[doc = "The ID of the file.\n"]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "The index of the file in the list of files.\n"]
    #[serde(rename = "index")]
    pub index: u64,
}
#[doc = "The ranker to use for the file search. If not specified will use the `auto` ranker."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FileSearchRanker {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default_2024_08_21")]
    Default20240821,
}
#[doc = "The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.\n\nSee the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FileSearchRankingOptions {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranker")]
    pub ranker: Option<FileSearchRanker>,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    #[serde(rename = "score_threshold")]
    pub score_threshold: f64,
}
#[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FileSearchToolCallType {
    #[default]
    #[serde(rename = "file_search_call")]
    FileSearchCall,
}
#[doc = "The status of the file search tool call. One of `in_progress`, \n`searching`, `incomplete` or `failed`,\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FileSearchToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "failed")]
    Failed,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FileSearchToolCallResult {
    #[doc = "The unique ID of the file.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "The text that was retrieved from the file.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[doc = "The name of the file.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<VectorStoreFileAttributes>,
    #[doc = "The relevance score of the file - a value between 0 and 1.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "score")]
    pub score: Option<f64>,
}
#[doc = "The results of a file search tool call. See the \n[file search guide](/docs/guides/tools-file-search) for more information.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FileSearchToolCall {
    #[doc = "The unique ID of the file search tool call.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FileSearchToolCallType,
    #[doc = "The status of the file search tool call. One of `in_progress`, \n`searching`, `incomplete` or `failed`,\n"]
    #[serde(rename = "status")]
    pub status: FileSearchToolCallStatus,
    #[doc = "The queries used to search for files.\n"]
    #[serde(rename = "queries")]
    pub queries: Vec<String>,
    #[doc = "The results of the file search tool call.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "results")]
    pub results: Option<Vec<FileSearchToolCallResult>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneChatCompletionRequestAssistantMessage {
    #[doc = "Controls whether the assistant message is trained against (0 or 1)"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "weight")]
    pub weight: Option<u64>,
    #[builder(default)]
    #[serde(flatten)]
    pub chat_completion_request_assistant_message: ChatCompletionRequestAssistantMessage,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneChatRequestInputMessages {
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}
#[doc = "The per-line training example of a fine-tuning input file for chat models using the supervised method."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneChatRequestInput {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messages")]
    pub messages: Option<Vec<FineTuneChatRequestInputMessages>>,
    #[doc = "A list of tools the model may generate JSON inputs for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<ParallelToolCalls>,
    #[doc = "A list of functions the model may generate JSON inputs for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "functions")]
    pub functions: Option<Vec<ChatCompletionFunctions>>,
}
#[doc = "The per-line training example of a fine-tuning input file for completions models"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneCompletionRequestInput {
    #[doc = "The input prompt for this training example."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
    #[doc = "The desired completion for this training example."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completion")]
    pub completion: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneDpoMethodHyperparametersBeta0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersBeta {
    _0(FineTuneDpoMethodHyperparametersBeta0),
    _1(f64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneDpoMethodHyperparametersBatchSize0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersBatchSize {
    _0(FineTuneDpoMethodHyperparametersBatchSize0),
    _1(u64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneDpoMethodHyperparametersLearningRateMultiplier0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersLearningRateMultiplier {
    _0(FineTuneDpoMethodHyperparametersLearningRateMultiplier0),
    _1(f64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneDpoMethodHyperparametersNEpochs0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersNEpochs {
    _0(FineTuneDpoMethodHyperparametersNEpochs0),
    _1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneDpoMethodHyperparameters {
    #[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "beta")]
    pub beta: Option<FineTuneDpoMethodHyperparametersBeta>,
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_size")]
    pub batch_size: Option<FineTuneDpoMethodHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier: Option<FineTuneDpoMethodHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<FineTuneDpoMethodHyperparametersNEpochs>,
}
#[doc = "Configuration for the DPO fine-tuning method."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneDpoMethod {
    #[doc = "The hyperparameters used for the fine-tuning job."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: Option<FineTuneDpoMethodHyperparameters>,
}
#[doc = "The type of method. Is either `supervised` or `dpo`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FineTuneMethodType {
    #[serde(rename = "supervised")]
    Supervised,
    #[serde(rename = "dpo")]
    Dpo,
}
#[doc = "The method used for fine-tuning."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneMethod {
    #[doc = "The type of method. Is either `supervised` or `dpo`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<FineTuneMethodType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "supervised")]
    pub supervised: Option<FineTuneSupervisedMethod>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dpo")]
    pub dpo: Option<FineTuneDpoMethod>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputInputMessages {
    System(ChatCompletionRequestSystemMessage),
    User(ChatCompletionRequestUserMessage),
    FineTuneChatCompletionRequestAssistantMessage(FineTuneChatCompletionRequestAssistantMessage),
    Tool(ChatCompletionRequestToolMessage),
    Function(ChatCompletionRequestFunctionMessage),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTunePreferenceRequestInputInput {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "messages")]
    pub messages: Option<Vec<FineTunePreferenceRequestInputInputMessages>>,
    #[doc = "A list of tools the model may generate JSON inputs for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<ChatCompletionTool>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: Option<ParallelToolCalls>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputPreferredCompletion {
    ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputNonPreferredCompletion {
    ChatCompletionRequestAssistantMessage(ChatCompletionRequestAssistantMessage),
}
#[doc = "The per-line training example of a fine-tuning input file for chat models using the dpo method."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTunePreferenceRequestInput {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input")]
    pub input: Option<FineTunePreferenceRequestInputInput>,
    #[doc = "The preferred completion message for the output."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "preferred_completion")]
    pub preferred_completion: Option<Vec<FineTunePreferenceRequestInputPreferredCompletion>>,
    #[doc = "The non-preferred completion message for the output."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "non_preferred_completion")]
    pub non_preferred_completion: Option<Vec<FineTunePreferenceRequestInputNonPreferredCompletion>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneSupervisedMethodHyperparametersBatchSize0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersBatchSize {
    _0(FineTuneSupervisedMethodHyperparametersBatchSize0),
    _1(u64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier {
    _0(FineTuneSupervisedMethodHyperparametersLearningRateMultiplier0),
    _1(f64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuneSupervisedMethodHyperparametersNEpochs0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersNEpochs {
    _0(FineTuneSupervisedMethodHyperparametersNEpochs0),
    _1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneSupervisedMethodHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_size")]
    pub batch_size: Option<FineTuneSupervisedMethodHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier:
        Option<FineTuneSupervisedMethodHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<FineTuneSupervisedMethodHyperparametersNEpochs>,
}
#[doc = "Configuration for the supervised fine-tuning method."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuneSupervisedMethod {
    #[doc = "The hyperparameters used for the fine-tuning job."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: Option<FineTuneSupervisedMethodHyperparameters>,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningCheckpointPermissionObject {
    #[default]
    #[serde(rename = "checkpoint.permission")]
    CheckpointPermission,
}
#[doc = "The `checkpoint.permission` object represents a permission for a fine-tuned model checkpoint.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningCheckpointPermission {
    #[doc = "The permission identifier, which can be referenced in the API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the permission was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The project identifier that the permission is for."]
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: FineTuningCheckpointPermissionObject,
}
#[doc = "The type of the integration being enabled for the fine-tuning job"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningIntegrationType {
    #[default]
    #[serde(rename = "wandb")]
    Wandb,
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningIntegrationWandb {
    #[doc = "The name of the project that the new run will be created under.\n"]
    #[serde(rename = "project")]
    pub project: String,
    #[doc = "A display name to set for the run. If not set, we will use the Job ID as the name.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The entity to use for the run. This allows you to set the team or username of the WandB user that you would\nlike associated with the run. If not set, the default entity for the registered WandB API key is used.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "entity")]
    pub entity: Option<String>,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tags")]
    pub tags: Option<Vec<String>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningIntegration {
    #[doc = "The type of the integration being enabled for the fine-tuning job"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FineTuningIntegrationType,
    #[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
    #[serde(rename = "wandb")]
    pub wandb: FineTuningIntegrationWandb,
}
#[doc = "For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningJobError {
    #[doc = "A machine-readable error code."]
    #[serde(rename = "code")]
    pub code: String,
    #[doc = "A human-readable error message."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "The parameter that was invalid, usually `training_file` or `validation_file`. This field will be null if the failure was not parameter-specific."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobHyperparametersBatchSize0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersBatchSize {
    _0(FineTuningJobHyperparametersBatchSize0),
    _1(u64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobHyperparametersLearningRateMultiplier0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersLearningRateMultiplier {
    _0(FineTuningJobHyperparametersLearningRateMultiplier0),
    _1(f64),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobHyperparametersNEpochs0 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersNEpochs {
    _0(FineTuningJobHyperparametersNEpochs0),
    _1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuningJobHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_size")]
    pub batch_size: Option<FineTuningJobHyperparametersBatchSize>,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "learning_rate_multiplier")]
    pub learning_rate_multiplier: Option<FineTuningJobHyperparametersLearningRateMultiplier>,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "n_epochs")]
    pub n_epochs: Option<FineTuningJobHyperparametersNEpochs>,
}
#[doc = "The object type, which is always \"fine_tuning.job\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobObject {
    #[default]
    #[serde(rename = "fine_tuning.job")]
    FineTuningJob,
}
#[doc = "The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FineTuningJobStatus {
    #[serde(rename = "validating_files")]
    ValidatingFiles,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "succeeded")]
    Succeeded,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "cancelled")]
    Cancelled,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobIntegration {
    FineTuningIntegration(FineTuningIntegration),
}
#[doc = "The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningJob {
    #[doc = "The object identifier, which can be referenced in the API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<FineTuningJobError>,
    #[doc = "The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "fine_tuned_model")]
    pub fine_tuned_model: Option<String>,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "finished_at")]
    pub finished_at: Option<u64>,
    #[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
    #[builder(default)]
    #[serde(rename = "hyperparameters")]
    pub hyperparameters: FineTuningJobHyperparameters,
    #[doc = "The base model that is being fine-tuned."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The object type, which is always \"fine_tuning.job\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: FineTuningJobObject,
    #[doc = "The organization that owns the fine-tuning job."]
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[doc = "The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    #[serde(rename = "result_files")]
    pub result_files: Vec<String>,
    #[doc = "The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`."]
    #[serde(rename = "status")]
    pub status: FineTuningJobStatus,
    #[doc = "The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "trained_tokens")]
    pub trained_tokens: Option<u64>,
    #[doc = "The file ID used for training. You can retrieve the training data with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    #[serde(rename = "training_file")]
    pub training_file: String,
    #[doc = "The file ID used for validation. You can retrieve the validation results with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "validation_file")]
    pub validation_file: Option<String>,
    #[doc = "A list of integrations to enable for this fine-tuning job."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "integrations")]
    pub integrations: Option<Vec<FineTuningJobIntegration>>,
    #[doc = "The seed used for the fine-tuning job."]
    #[serde(rename = "seed")]
    pub seed: u64,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "estimated_finish")]
    pub estimated_finish: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "method")]
    pub method: Option<FineTuneMethod>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[doc = "Metrics at the step number during the fine-tuning job."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct FineTuningJobCheckpointMetrics {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "step")]
    pub step: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "train_loss")]
    pub train_loss: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "train_mean_token_accuracy")]
    pub train_mean_token_accuracy: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valid_loss")]
    pub valid_loss: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "valid_mean_token_accuracy")]
    pub valid_mean_token_accuracy: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "full_valid_loss")]
    pub full_valid_loss: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "full_valid_mean_token_accuracy")]
    pub full_valid_mean_token_accuracy: Option<f64>,
}
#[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobCheckpointObject {
    #[default]
    #[serde(rename = "fine_tuning.job.checkpoint")]
    FineTuningJobCheckpoint,
}
#[doc = "The `fine_tuning.job.checkpoint` object represents a model checkpoint for a fine-tuning job that is ready to use.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningJobCheckpoint {
    #[doc = "The checkpoint identifier, which can be referenced in the API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the checkpoint was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The name of the fine-tuned checkpoint model that is created."]
    #[serde(rename = "fine_tuned_model_checkpoint")]
    pub fine_tuned_model_checkpoint: String,
    #[doc = "The step number that the checkpoint was created at."]
    #[serde(rename = "step_number")]
    pub step_number: u64,
    #[doc = "Metrics at the step number during the fine-tuning job."]
    #[builder(default)]
    #[serde(rename = "metrics")]
    pub metrics: FineTuningJobCheckpointMetrics,
    #[doc = "The name of the fine-tuning job that this checkpoint was created from."]
    #[serde(rename = "fine_tuning_job_id")]
    pub fine_tuning_job_id: String,
    #[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: FineTuningJobCheckpointObject,
}
#[doc = "The object type, which is always \"fine_tuning.job.event\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FineTuningJobEventObject {
    #[default]
    #[serde(rename = "fine_tuning.job.event")]
    FineTuningJobEvent,
}
#[doc = "The log level of the event."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FineTuningJobEventLevel {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}
#[doc = "The type of event."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FineTuningJobEventType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "metrics")]
    Metrics,
}
#[doc = "Fine-tuning job event object"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FineTuningJobEvent {
    #[doc = "The object type, which is always \"fine_tuning.job.event\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: FineTuningJobEventObject,
    #[doc = "The object identifier."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The log level of the event."]
    #[serde(rename = "level")]
    pub level: FineTuningJobEventLevel,
    #[doc = "The message of the event."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "The type of event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<FineTuningJobEventType>,
    #[doc = "The data associated with the event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionObject {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    #[serde(rename = "name")]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<FunctionParameters>,
    #[doc = "Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling)."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "strict")]
    pub strict: Option<bool>,
}
pub type FunctionParameters = std::collections::HashMap<String, serde_json::Value>;
#[doc = "The type of the function tool call. Always `function_call`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FunctionToolCallType {
    #[default]
    #[serde(rename = "function_call")]
    FunctionCall,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FunctionToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "A tool call to run a function. See the \n[function calling guide](/docs/guides/function-calling) for more information.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionToolCall {
    #[doc = "The unique ID of the function tool call.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of the function tool call. Always `function_call`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallType,
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The name of the function to run.\n"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "A JSON string of the arguments to pass to the function.\n"]
    #[serde(rename = "arguments")]
    pub arguments: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<FunctionToolCallStatus>,
}
#[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FunctionToolCallOutputType {
    #[default]
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FunctionToolCallOutputStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The output of a function tool call.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionToolCallOutput {
    #[doc = "The unique ID of the function tool call output. Populated when this item\nis returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FunctionToolCallOutputType,
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "A JSON string of the output of the function tool call.\n"]
    #[serde(rename = "output")]
    pub output: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<FunctionToolCallOutputStatus>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionToolCallOutputResource {
    #[serde(flatten)]
    pub function_tool_call_output: FunctionToolCallOutput,
    #[doc = "The unique ID of the function call tool output.\n"]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionToolCallResource {
    #[serde(flatten)]
    pub function_tool_call: FunctionToolCall,
    #[doc = "The unique ID of the function tool call.\n"]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "Represents the content or the URL of an image generated by the OpenAI API."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct Image {
    #[doc = "The base64-encoded JSON of the generated image. Default value for `gpt-image-1`, and only present if `response_format` is set to `b64_json` for `dall-e-2` and `dall-e-3`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "b64_json")]
    pub b64_json: Option<String>,
    #[doc = "When using `dall-e-2` or `dall-e-3`, the URL of the generated image if `response_format` is set to `url` (default value). Unsupported for `gpt-image-1`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[doc = "For `dall-e-3` only, the revised prompt that was used to generate the image."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "revised_prompt")]
    pub revised_prompt: Option<String>,
}
#[doc = "The input tokens detailed information for the image generation."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ImagesResponseUsageInputTokensDetails {
    #[doc = "The number of text tokens in the input prompt."]
    #[serde(rename = "text_tokens")]
    pub text_tokens: u64,
    #[doc = "The number of image tokens in the input prompt."]
    #[serde(rename = "image_tokens")]
    pub image_tokens: u64,
}
#[doc = "For `gpt-image-1` only, the token usage information for the image generation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ImagesResponseUsage {
    #[doc = "The total number of tokens (images and text) used for the image generation."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
    #[doc = "The number of tokens (images and text) in the input prompt."]
    #[serde(rename = "input_tokens")]
    pub input_tokens: u64,
    #[doc = "The number of image tokens in the output image."]
    #[serde(rename = "output_tokens")]
    pub output_tokens: u64,
    #[doc = "The input tokens detailed information for the image generation."]
    #[serde(rename = "input_tokens_details")]
    pub input_tokens_details: ImagesResponseUsageInputTokensDetails,
}
#[doc = "The response from the image generation endpoint."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ImagesResponse {
    #[doc = "The Unix timestamp (in seconds) of when the image was created."]
    #[serde(rename = "created")]
    pub created: u64,
    #[doc = "The list of generated images."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "data")]
    pub data: Option<Vec<Image>>,
    #[doc = "For `gpt-image-1` only, the token usage information for the image generation.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<ImagesResponseUsage>,
}
#[doc = "Specify additional output data to include in the model response. Currently\nsupported values are:\n- `file_search_call.results`: Include the search results of\n  the file search tool call.\n- `message.input_image.image_url`: Include image urls from the input message.\n- `computer_call_output.output.image_url`: Include image urls from the computer call output.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum Includable {
    #[serde(rename = "file_search_call.results")]
    FileSearchCallResults,
    #[serde(rename = "message.input_image.image_url")]
    MessageInputImageImageUrl,
    #[serde(rename = "computer_call_output.output.image_url")]
    ComputerCallOutputOutputImageUrl,
}
#[doc = "The type of the input item. Always `input_audio`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InputAudioType {
    #[default]
    #[serde(rename = "input_audio")]
    InputAudio,
}
#[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InputAudioFormat {
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "wav")]
    Wav,
}
#[doc = "An audio input to the model.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InputAudio {
    #[doc = "The type of the input item. Always `input_audio`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: InputAudioType,
    #[doc = "Base64-encoded audio data.\n"]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
    #[serde(rename = "format")]
    pub format: InputAudioFormat,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum InputContent {
    InputText(InputTextContent),
    InputImage(InputImageContent),
    InputFile(InputFileContent),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum InputItem {
    EasyInputMessage(EasyInputMessage),
    Item(Item),
    ItemReferenceParam(ItemReferenceParam),
}
#[doc = "The type of the message input. Always set to `message`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}
#[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InputMessageRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "developer")]
    Developer,
}
#[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InputMessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InputMessage {
    #[doc = "The type of the message input. Always set to `message`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<InputMessageType>,
    #[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
    #[serde(rename = "role")]
    pub role: InputMessageRole,
    #[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<InputMessageStatus>,
    #[serde(rename = "content")]
    pub content: InputMessageContentList,
}
pub type InputMessageContentList = Vec<InputContent>;
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InputMessageResource {
    #[serde(flatten)]
    pub input_message: InputMessage,
    #[doc = "The unique ID of the message input.\n"]
    #[serde(rename = "id")]
    pub id: String,
}
#[doc = "The object type, which is always `organization.invite`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InviteObject {
    #[default]
    #[serde(rename = "organization.invite")]
    OrganizationInvite,
}
#[doc = "`owner` or `reader`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InviteRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
}
#[doc = "`accepted`,`expired`, or `pending`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InviteStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "pending")]
    Pending,
}
#[doc = "Project membership role"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InviteProjectsRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct InviteProjects {
    #[doc = "Project's public ID"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "Project membership role"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<InviteProjectsRole>,
}
#[doc = "Represents an individual `invite` to the organization."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Invite {
    #[doc = "The object type, which is always `organization.invite`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: InviteObject,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The email address of the individual to whom the invite was sent"]
    #[serde(rename = "email")]
    pub email: String,
    #[doc = "`owner` or `reader`"]
    #[serde(rename = "role")]
    pub role: InviteRole,
    #[doc = "`accepted`,`expired`, or `pending`"]
    #[serde(rename = "status")]
    pub status: InviteStatus,
    #[doc = "The Unix timestamp (in seconds) of when the invite was sent."]
    #[serde(rename = "invited_at")]
    pub invited_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the invite expires."]
    #[serde(rename = "expires_at")]
    pub expires_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the invite was accepted."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accepted_at")]
    pub accepted_at: Option<u64>,
    #[doc = "The projects that were granted membership upon acceptance of the invite."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projects")]
    pub projects: Option<Vec<InviteProjects>>,
}
#[doc = "The object type, which is always `organization.invite.deleted`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InviteDeleteResponseObject {
    #[default]
    #[serde(rename = "organization.invite.deleted")]
    OrganizationInviteDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InviteDeleteResponse {
    #[doc = "The object type, which is always `organization.invite.deleted`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: InviteDeleteResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[doc = "The object type, which is always `list`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InviteListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InviteListResponse {
    #[doc = "The object type, which is always `list`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: InviteListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<Invite>,
    #[doc = "The first `invite_id` in the retrieved `list`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[doc = "The last `invite_id` in the retrieved `list`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[doc = "The `has_more` property is used for pagination to indicate there are additional results."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "has_more")]
    pub has_more: Option<bool>,
}
#[doc = "`owner` or `reader`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InviteRequestRole {
    #[serde(rename = "reader")]
    Reader,
    #[serde(rename = "owner")]
    Owner,
}
#[doc = "Project membership role"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InviteRequestProjectsRole {
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "owner")]
    Owner,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InviteRequestProjects {
    #[doc = "Project's public ID"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Project membership role"]
    #[serde(rename = "role")]
    pub role: InviteRequestProjectsRole,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InviteRequest {
    #[doc = "Send an email to this address"]
    #[serde(rename = "email")]
    pub email: String,
    #[doc = "`owner` or `reader`"]
    #[serde(rename = "role")]
    pub role: InviteRequestRole,
    #[doc = "An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "projects")]
    pub projects: Option<Vec<InviteRequestProjects>>,
}
#[doc = "Content item used to generate a response.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Item {
    InputMessage(InputMessage),
    Message(OutputMessage),
    FileSearchCall(FileSearchToolCall),
    ComputerCall(ComputerToolCall),
    ComputerCallOutput(ComputerCallOutputItemParam),
    WebSearchCall(WebSearchToolCall),
    FunctionCall(FunctionToolCall),
    FunctionCallOutput(FunctionCallOutputItemParam),
    Reasoning(ReasoningItem),
}
#[doc = "Content item used to generate a response.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ItemResource {
    InputMessageResource(InputMessageResource),
    Message(OutputMessage),
    FileSearchCall(FileSearchToolCall),
    ComputerCall(ComputerToolCall),
    ComputerToolCallOutputResource(ComputerToolCallOutputResource),
    WebSearchCall(WebSearchToolCall),
    FunctionToolCallResource(FunctionToolCallResource),
    FunctionToolCallOutputResource(FunctionToolCallOutputResource),
}
#[doc = "Specifies the event type. For a keypress action, this property is \nalways set to `keypress`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum KeyPressType {
    #[default]
    #[serde(rename = "keypress")]
    Keypress,
}
#[doc = "A collection of keypresses the model would like to perform.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct KeyPress {
    #[doc = "Specifies the event type. For a keypress action, this property is \nalways set to `keypress`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: KeyPressType,
    #[doc = "The combination of keys the model is requesting to be pressed. This is an\narray of strings, each representing a key.\n"]
    #[serde(rename = "keys")]
    pub keys: Vec<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListAssistantsResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<AssistantObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListAuditLogsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListAuditLogsResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListAuditLogsResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<AuditLog>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListBatchesResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListBatchesResponse {
    #[serde(rename = "data")]
    pub data: Vec<Batch>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListBatchesResponseObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListCertificatesResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListCertificatesResponse {
    #[serde(rename = "data")]
    pub data: Vec<Certificate>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListCertificatesResponseObject,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListFilesResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<OpenAiFile>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListFineTuningCheckpointPermissionResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListFineTuningCheckpointPermissionResponse {
    #[serde(rename = "data")]
    pub data: Vec<FineTuningCheckpointPermission>,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListFineTuningCheckpointPermissionResponseObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListFineTuningJobCheckpointsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListFineTuningJobCheckpointsResponse {
    #[serde(rename = "data")]
    pub data: Vec<FineTuningJobCheckpoint>,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListFineTuningJobCheckpointsResponseObject,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "first_id")]
    pub first_id: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_id")]
    pub last_id: Option<String>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListFineTuningJobEventsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListFineTuningJobEventsResponse {
    #[serde(rename = "data")]
    pub data: Vec<FineTuningJobEvent>,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListFineTuningJobEventsResponseObject,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListMessagesResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<MessageObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListModelsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListModelsResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListModelsResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<Model>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ListPaginatedFineTuningJobsResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListPaginatedFineTuningJobsResponse {
    #[serde(rename = "data")]
    pub data: Vec<FineTuningJob>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ListPaginatedFineTuningJobsResponseObject,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListRunStepsResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<RunStepObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListRunsResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<RunObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListVectorStoreFilesResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<VectorStoreFileObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ListVectorStoresResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<VectorStoreObject>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "A log probability object.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct LogProbProperties {
    #[doc = "The token that was used to generate the log probability.\n"]
    #[serde(rename = "token")]
    pub token: String,
    #[doc = "The log probability of the token.\n"]
    #[serde(rename = "logprob")]
    pub logprob: f64,
    #[doc = "The bytes that were used to generate the log probability.\n"]
    #[serde(rename = "bytes")]
    pub bytes: Vec<u64>,
}
#[doc = "Always `image_file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentImageFileObjectType {
    #[default]
    #[serde(rename = "image_file")]
    ImageFile,
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentImageFileObjectImageFileDetail {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentImageFileObjectImageFile {
    #[doc = "The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail")]
    pub detail: Option<MessageContentImageFileObjectImageFileDetail>,
}
#[doc = "References an image [File](/docs/api-reference/files) in the content of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentImageFileObject {
    #[doc = "Always `image_file`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentImageFileObjectType,
    #[serde(rename = "image_file")]
    pub image_file: MessageContentImageFileObjectImageFile,
}
#[doc = "The type of the content part."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentImageUrlObjectType {
    #[default]
    #[serde(rename = "image_url")]
    ImageUrl,
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentImageUrlObjectImageUrlDetail {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentImageUrlObjectImageUrl {
    #[doc = "The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail")]
    pub detail: Option<MessageContentImageUrlObjectImageUrlDetail>,
}
#[doc = "References an image URL in the content of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentImageUrlObject {
    #[doc = "The type of the content part."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentImageUrlObjectType,
    #[serde(rename = "image_url")]
    pub image_url: MessageContentImageUrlObjectImageUrl,
}
#[doc = "Always `refusal`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentRefusalObjectType {
    #[default]
    #[serde(rename = "refusal")]
    Refusal,
}
#[doc = "The refusal content generated by the assistant."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentRefusalObject {
    #[doc = "Always `refusal`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentRefusalObjectType,
    #[serde(rename = "refusal")]
    pub refusal: String,
}
#[doc = "Always `file_citation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentTextAnnotationsFileCitationObjectType {
    #[default]
    #[serde(rename = "file_citation")]
    FileCitation,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    #[serde(rename = "file_id")]
    pub file_id: String,
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextAnnotationsFileCitationObject {
    #[doc = "Always `file_citation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentTextAnnotationsFileCitationObjectType,
    #[doc = "The text in the message content that needs to be replaced."]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "file_citation")]
    pub file_citation: MessageContentTextAnnotationsFileCitationObjectFileCitation,
    #[serde(rename = "start_index")]
    pub start_index: u64,
    #[serde(rename = "end_index")]
    pub end_index: u64,
}
#[doc = "Always `file_path`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentTextAnnotationsFilePathObjectType {
    #[default]
    #[serde(rename = "file_path")]
    FilePath,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    #[serde(rename = "file_id")]
    pub file_id: String,
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextAnnotationsFilePathObject {
    #[doc = "Always `file_path`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentTextAnnotationsFilePathObjectType,
    #[doc = "The text in the message content that needs to be replaced."]
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "file_path")]
    pub file_path: MessageContentTextAnnotationsFilePathObjectFilePath,
    #[serde(rename = "start_index")]
    pub start_index: u64,
    #[serde(rename = "end_index")]
    pub end_index: u64,
}
#[doc = "Always `text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageContentTextObjectType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageContentTextObjectTextAnnotation {
    FileCitation(MessageContentTextAnnotationsFileCitationObject),
    FilePath(MessageContentTextAnnotationsFilePathObject),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextObjectText {
    #[doc = "The data that makes up the text."]
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "annotations")]
    pub annotations: Vec<MessageContentTextObjectTextAnnotation>,
}
#[doc = "The text content that is part of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageContentTextObject {
    #[doc = "Always `text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageContentTextObjectType,
    #[serde(rename = "text")]
    pub text: MessageContentTextObjectText,
}
#[doc = "Always `image_file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentImageFileObjectType {
    #[default]
    #[serde(rename = "image_file")]
    ImageFile,
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentImageFileObjectImageFileDetail {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentImageFileObjectImageFile {
    #[doc = "The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail")]
    pub detail: Option<MessageDeltaContentImageFileObjectImageFileDetail>,
}
#[doc = "References an image [File](/docs/api-reference/files) in the content of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentImageFileObject {
    #[doc = "The index of the content part in the message."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `image_file`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentImageFileObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_file")]
    pub image_file: Option<MessageDeltaContentImageFileObjectImageFile>,
}
#[doc = "Always `image_url`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentImageUrlObjectType {
    #[default]
    #[serde(rename = "image_url")]
    ImageUrl,
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentImageUrlObjectImageUrlDetail {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentImageUrlObjectImageUrl {
    #[doc = "The URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "detail")]
    pub detail: Option<MessageDeltaContentImageUrlObjectImageUrlDetail>,
}
#[doc = "References an image URL in the content of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentImageUrlObject {
    #[doc = "The index of the content part in the message."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `image_url`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentImageUrlObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_url")]
    pub image_url: Option<MessageDeltaContentImageUrlObjectImageUrl>,
}
#[doc = "Always `refusal`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentRefusalObjectType {
    #[default]
    #[serde(rename = "refusal")]
    Refusal,
}
#[doc = "The refusal content that is part of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentRefusalObject {
    #[doc = "The index of the refusal part in the message."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `refusal`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentRefusalObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "refusal")]
    pub refusal: Option<String>,
}
#[doc = "Always `file_citation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentTextAnnotationsFileCitationObjectType {
    #[default]
    #[serde(rename = "file_citation")]
    FileCitation,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "The specific quote in the file."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "quote")]
    pub quote: Option<String>,
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    #[doc = "The index of the annotation in the text content part."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `file_citation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextAnnotationsFileCitationObjectType,
    #[doc = "The text in the message content that needs to be replaced."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_citation")]
    pub file_citation: Option<MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start_index")]
    pub start_index: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end_index")]
    pub end_index: Option<u64>,
}
#[doc = "Always `file_path`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentTextAnnotationsFilePathObjectType {
    #[default]
    #[serde(rename = "file_path")]
    FilePath,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    #[doc = "The index of the annotation in the text content part."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `file_path`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextAnnotationsFilePathObjectType,
    #[doc = "The text in the message content that needs to be replaced."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_path")]
    pub file_path: Option<MessageDeltaContentTextAnnotationsFilePathObjectFilePath>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "start_index")]
    pub start_index: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "end_index")]
    pub end_index: Option<u64>,
}
#[doc = "Always `text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaContentTextObjectType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageDeltaContentTextObjectTextAnnotation {
    FileCitation(MessageDeltaContentTextAnnotationsFileCitationObject),
    FilePath(MessageDeltaContentTextAnnotationsFilePathObject),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextObjectText {
    #[doc = "The data that makes up the text."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "annotations")]
    pub annotations: Option<Vec<MessageDeltaContentTextObjectTextAnnotation>>,
}
#[doc = "The text content that is part of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaContentTextObject {
    #[doc = "The index of the content part in the message."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageDeltaContentTextObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<MessageDeltaContentTextObjectText>,
}
#[doc = "The object type, which is always `thread.message.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageDeltaObjectObject {
    #[default]
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta,
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum MessageDeltaObjectDeltaRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageDeltaObjectDeltaContent {
    ImageFile(MessageDeltaContentImageFileObject),
    Text(MessageDeltaContentTextObject),
    Refusal(MessageDeltaContentRefusalObject),
    ImageUrl(MessageDeltaContentImageUrlObject),
}
#[doc = "The delta containing the fields that have changed on the Message."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaObjectDelta {
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<MessageDeltaObjectDeltaRole>,
    #[doc = "The content of the message in array of text and/or images."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<MessageDeltaObjectDeltaContent>>,
}
#[doc = "Represents a message delta i.e. any changed fields on a message during streaming.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageDeltaObject {
    #[doc = "The identifier of the message, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread.message.delta`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: MessageDeltaObjectObject,
    #[doc = "The delta containing the fields that have changed on the Message."]
    #[builder(default)]
    #[serde(rename = "delta")]
    pub delta: MessageDeltaObjectDelta,
}
#[doc = "The object type, which is always `thread.message`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageObjectObject {
    #[default]
    #[serde(rename = "thread.message")]
    ThreadMessage,
}
#[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum MessageObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "completed")]
    Completed,
}
#[doc = "The reason the message is incomplete."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum MessageObjectIncompleteDetailsReason {
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "max_tokens")]
    MaxTokens,
    #[serde(rename = "run_cancelled")]
    RunCancelled,
    #[serde(rename = "run_expired")]
    RunExpired,
    #[serde(rename = "run_failed")]
    RunFailed,
}
#[doc = "On an incomplete message, details about why the message is incomplete."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageObjectIncompleteDetails {
    #[doc = "The reason the message is incomplete."]
    #[serde(rename = "reason")]
    pub reason: MessageObjectIncompleteDetailsReason,
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum MessageObjectRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageObjectContent {
    ImageFile(MessageContentImageFileObject),
    ImageUrl(MessageContentImageUrlObject),
    Text(MessageContentTextObject),
    Refusal(MessageContentRefusalObject),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageObjectAttachmentsTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearchTypeOnly),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct MessageObjectAttachments {
    #[doc = "The ID of the file to attach to the message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "The tools to add this file to."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<MessageObjectAttachmentsTool>>,
}
#[doc = "Represents a message within a [thread](/docs/api-reference/threads)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread.message`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: MessageObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the message was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The [thread](/docs/api-reference/threads) ID that this message belongs to."]
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
    #[serde(rename = "status")]
    pub status: MessageObjectStatus,
    #[doc = "On an incomplete message, details about why the message is incomplete."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: Option<MessageObjectIncompleteDetails>,
    #[doc = "The Unix timestamp (in seconds) for when the message was completed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completed_at")]
    pub completed_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the message was marked as incomplete."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "incomplete_at")]
    pub incomplete_at: Option<u64>,
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    #[serde(rename = "role")]
    pub role: MessageObjectRole,
    #[doc = "The content of the message in array of text and/or images."]
    #[serde(rename = "content")]
    pub content: Vec<MessageObjectContent>,
    #[doc = "If applicable, the ID of the [assistant](/docs/api-reference/assistants) that authored this message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "assistant_id")]
    pub assistant_id: Option<String>,
    #[doc = "The ID of the [run](/docs/api-reference/runs) associated with the creation of this message. Value is `null` when messages are created manually using the create message or create thread endpoints."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "run_id")]
    pub run_id: Option<String>,
    #[doc = "A list of files attached to the message, and the tools they were added to."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attachments")]
    pub attachments: Option<Vec<MessageObjectAttachments>>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}
#[doc = "Always `text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageRequestContentTextObjectType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[doc = "The text content that is part of a message."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageRequestContentTextObject {
    #[doc = "Always `text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MessageRequestContentTextObjectType,
    #[doc = "Text content to be sent to the model"]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageStreamEventThreadMessageCreatedEvent {
    #[default]
    #[serde(rename = "thread.message.created")]
    ThreadMessageCreated,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) is created."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageStreamEventThreadMessageCreated {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: MessageStreamEventThreadMessageCreatedEvent,
    #[serde(rename = "data")]
    pub data: MessageObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageStreamEventThreadMessageInProgressEvent {
    #[default]
    #[serde(rename = "thread.message.in_progress")]
    ThreadMessageInProgress,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) moves to an `in_progress` state."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageStreamEventThreadMessageInProgress {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: MessageStreamEventThreadMessageInProgressEvent,
    #[serde(rename = "data")]
    pub data: MessageObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageStreamEventThreadMessageDeltaEvent {
    #[default]
    #[serde(rename = "thread.message.delta")]
    ThreadMessageDelta,
}
#[doc = "Occurs when parts of a [Message](/docs/api-reference/messages/object) are being streamed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageStreamEventThreadMessageDelta {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: MessageStreamEventThreadMessageDeltaEvent,
    #[serde(rename = "data")]
    pub data: MessageDeltaObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageStreamEventThreadMessageCompletedEvent {
    #[default]
    #[serde(rename = "thread.message.completed")]
    ThreadMessageCompleted,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageStreamEventThreadMessageCompleted {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: MessageStreamEventThreadMessageCompletedEvent,
    #[serde(rename = "data")]
    pub data: MessageObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MessageStreamEventThreadMessageIncompleteEvent {
    #[default]
    #[serde(rename = "thread.message.incomplete")]
    ThreadMessageIncomplete,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) ends before it is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct MessageStreamEventThreadMessageIncomplete {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: MessageStreamEventThreadMessageIncompleteEvent,
    #[serde(rename = "data")]
    pub data: MessageObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum MessageStreamEvent {
    ThreadMessageCreated(MessageStreamEventThreadMessageCreated),
    ThreadMessageInProgress(MessageStreamEventThreadMessageInProgress),
    ThreadMessageDelta(MessageStreamEventThreadMessageDelta),
    ThreadMessageCompleted(MessageStreamEventThreadMessageCompleted),
    ThreadMessageIncomplete(MessageStreamEventThreadMessageIncomplete),
}
pub type Metadata = Vec<String>;
#[doc = "The object type, which is always \"model\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ModelObject {
    #[default]
    #[serde(rename = "model")]
    Model,
}
#[doc = "Describes an OpenAI model offering that can be used with the API."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Model {
    #[doc = "The model identifier, which can be referenced in the API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) when the model was created."]
    #[serde(rename = "created")]
    pub created: u64,
    #[doc = "The object type, which is always \"model\"."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ModelObject,
    #[doc = "The organization that owns the model."]
    #[serde(rename = "owned_by")]
    pub owned_by: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ModelIds {
    ModelIdsShared(ModelIdsShared),
    ModelIdsResponses(ModelIdsResponses),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ModelIdsResponses1 {
    #[serde(rename = "o1-pro")]
    O1Pro,
    #[serde(rename = "o1-pro-2025-03-19")]
    O1Pro20250319,
    #[serde(rename = "computer-use-preview")]
    ComputerUsePreview,
    #[serde(rename = "computer-use-preview-2025-03-11")]
    ComputerUsePreview20250311,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ModelIdsResponses {
    ModelIdsShared(ModelIdsShared),
    _1(ModelIdsResponses1),
}
pub type ModelIdsShared = String;
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModelResponseProperties {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\nWe generally recommend altering this or `top_p` but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling,\nwhere the model considers the results of the tokens with top_p probability\nmass. So 0.1 means only the tokens comprising the top 10% probability mass\nare considered.\n\nWe generally recommend altering this or `temperature` but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_tier")]
    pub service_tier: Option<ServiceTier>,
}
#[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ModifyAssistantRequestModel {
    _0(String),
    AssistantSupportedModels(AssistantSupportedModels),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ModifyAssistantRequestTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "Overrides the list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyAssistantRequestToolResourcesFileSearch {
    #[doc = "Overrides the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyAssistantRequestToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<ModifyAssistantRequestToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<ModifyAssistantRequestToolResourcesFileSearch>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyAssistantRequest {
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<ModifyAssistantRequestModel>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning_effort")]
    pub reasoning_effort: Option<ReasoningEffort>,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<ModifyAssistantRequestTool>>,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<ModifyAssistantRequestToolResources>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ModifyCertificateRequest {
    #[doc = "The updated name for the certificate"]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyMessageRequest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyRunRequest {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyThreadRequestToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyThreadRequestToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<ModifyThreadRequestToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<ModifyThreadRequestToolResourcesFileSearch>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ModifyThreadRequest {
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<ModifyThreadRequestToolResources>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[doc = "Specifies the event type. For a move action, this property is \nalways set to `move`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum MoveType {
    #[default]
    #[serde(rename = "move")]
    Move,
}
#[doc = "A mouse move action.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Move {
    #[doc = "Specifies the event type. For a move action, this property is \nalways set to `move`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: MoveType,
    #[doc = "The x-coordinate to move to.\n"]
    #[serde(rename = "x")]
    pub x: u64,
    #[doc = "The y-coordinate to move to.\n"]
    #[serde(rename = "y")]
    pub y: u64,
}
#[doc = "The object type, which is always `file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OpenAiFileObject {
    #[default]
    #[serde(rename = "file")]
    File,
}
#[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum OpenAiFilePurpose {
    #[serde(rename = "assistants")]
    Assistants,
    #[serde(rename = "assistants_output")]
    AssistantsOutput,
    #[serde(rename = "batch")]
    Batch,
    #[serde(rename = "batch_output")]
    BatchOutput,
    #[serde(rename = "fine-tune")]
    FineTune,
    #[serde(rename = "fine-tune-results")]
    FineTuneResults,
    #[serde(rename = "vision")]
    Vision,
}
#[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum OpenAiFileStatus {
    #[serde(rename = "uploaded")]
    Uploaded,
    #[serde(rename = "processed")]
    Processed,
    #[serde(rename = "error")]
    Error,
}
#[doc = "The `File` object represents a document that has been uploaded to OpenAI."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct OpenAiFile {
    #[doc = "The file identifier, which can be referenced in the API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The size of the file, in bytes."]
    #[serde(rename = "bytes")]
    pub bytes: u64,
    #[doc = "The Unix timestamp (in seconds) for when the file was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the file will expire."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<u64>,
    #[doc = "The name of the file."]
    #[serde(rename = "filename")]
    pub filename: String,
    #[doc = "The object type, which is always `file`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: OpenAiFileObject,
    #[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`."]
    #[serde(rename = "purpose")]
    pub purpose: OpenAiFilePurpose,
    #[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
    #[serde(rename = "status")]
    pub status: OpenAiFileStatus,
    #[doc = "Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status_details")]
    pub status_details: Option<String>,
}
#[doc = "Always `other`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OtherChunkingStrategyResponseParamType {
    #[default]
    #[serde(rename = "other")]
    Other,
}
#[doc = "This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct OtherChunkingStrategyResponseParam {
    #[doc = "Always `other`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: OtherChunkingStrategyResponseParamType,
}
#[doc = "The type of the output audio. Always `output_audio`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OutputAudioType {
    #[default]
    #[serde(rename = "output_audio")]
    OutputAudio,
}
#[doc = "An audio output from the model.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct OutputAudio {
    #[doc = "The type of the output audio. Always `output_audio`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: OutputAudioType,
    #[doc = "Base64-encoded audio data from the model.\n"]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "The transcript of the audio data from the model.\n"]
    #[serde(rename = "transcript")]
    pub transcript: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OutputContent {
    OutputText(OutputTextContent),
    Refusal(RefusalContent),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum OutputItem {
    Message(OutputMessage),
    FileSearchCall(FileSearchToolCall),
    FunctionCall(FunctionToolCall),
    WebSearchCall(WebSearchToolCall),
    ComputerCall(ComputerToolCall),
    Reasoning(ReasoningItem),
}
#[doc = "The type of the output message. Always `message`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OutputMessageType {
    #[default]
    #[serde(rename = "message")]
    Message,
}
#[doc = "The role of the output message. Always `assistant`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OutputMessageRole {
    #[default]
    #[serde(rename = "assistant")]
    Assistant,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum OutputMessageStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "An output message from the model.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct OutputMessage {
    #[doc = "The unique ID of the output message.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the output message. Always `message`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: OutputMessageType,
    #[doc = "The role of the output message. Always `assistant`.\n"]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: OutputMessageRole,
    #[doc = "The content of the output message.\n"]
    #[serde(rename = "content")]
    pub content: Vec<OutputContent>,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    #[serde(rename = "status")]
    pub status: OutputMessageStatus,
}
pub type ParallelToolCalls = bool;
#[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum PredictionContentType {
    #[default]
    #[serde(rename = "content")]
    Content,
}
#[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum PredictionContentContent {
    _0(String),
    _1(Vec<ChatCompletionRequestMessageContentPartText>),
}
#[doc = "Static predicted output content, such as the content of a text file that is\nbeing regenerated.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct PredictionContent {
    #[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: PredictionContentType,
    #[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
    #[serde(rename = "content")]
    pub content: PredictionContentContent,
}
#[doc = "The object type, which is always `organization.project`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectObject {
    #[default]
    #[serde(rename = "organization.project")]
    OrganizationProject,
}
#[doc = "`active` or `archived`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "archived")]
    Archived,
}
#[doc = "Represents an individual project."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Project {
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `organization.project`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectObject,
    #[doc = "The name of the project. This appears in reporting."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The Unix timestamp (in seconds) of when the project was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the project was archived or `null`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "archived_at")]
    pub archived_at: Option<u64>,
    #[doc = "`active` or `archived`"]
    #[serde(rename = "status")]
    pub status: ProjectStatus,
}
#[doc = "The object type, which is always `organization.project.api_key`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectApiKeyObject {
    #[default]
    #[serde(rename = "organization.project.api_key")]
    OrganizationProjectApiKey,
}
#[doc = "`user` or `service_account`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectApiKeyOwnerType {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "service_account")]
    ServiceAccount,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ProjectApiKeyOwner {
    #[doc = "`user` or `service_account`"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<ProjectApiKeyOwnerType>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user")]
    pub user: Option<ProjectUser>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "service_account")]
    pub service_account: Option<ProjectServiceAccount>,
}
#[doc = "Represents an individual API key in a project."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectApiKey {
    #[doc = "The object type, which is always `organization.project.api_key`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectApiKeyObject,
    #[doc = "The redacted value of the API key"]
    #[serde(rename = "redacted_value")]
    pub redacted_value: String,
    #[doc = "The name of the API key"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the API key was last used."]
    #[serde(rename = "last_used_at")]
    pub last_used_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[builder(default)]
    #[serde(rename = "owner")]
    pub owner: ProjectApiKeyOwner,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectApiKeyDeleteResponseObject {
    #[default]
    #[serde(rename = "organization.project.api_key.deleted")]
    OrganizationProjectApiKeyDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectApiKeyDeleteResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectApiKeyDeleteResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectApiKeyListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectApiKeyListResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectApiKeyListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<ProjectApiKey>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectCreateRequest {
    #[doc = "The friendly name of the project, this name appears in reports."]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectListResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<Project>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "The object type, which is always `project.rate_limit`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectRateLimitObject {
    #[default]
    #[serde(rename = "project.rate_limit")]
    ProjectRateLimit,
}
#[doc = "Represents a project rate limit config."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectRateLimit {
    #[doc = "The object type, which is always `project.rate_limit`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectRateLimitObject,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The model this rate limit applies to."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The maximum requests per minute."]
    #[serde(rename = "max_requests_per_1_minute")]
    pub max_requests_per_1_minute: u64,
    #[doc = "The maximum tokens per minute."]
    #[serde(rename = "max_tokens_per_1_minute")]
    pub max_tokens_per_1_minute: u64,
    #[doc = "The maximum images per minute. Only present for relevant models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_images_per_1_minute")]
    pub max_images_per_1_minute: Option<u64>,
    #[doc = "The maximum audio megabytes per minute. Only present for relevant models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_audio_megabytes_per_1_minute")]
    pub max_audio_megabytes_per_1_minute: Option<u64>,
    #[doc = "The maximum requests per day. Only present for relevant models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_requests_per_1_day")]
    pub max_requests_per_1_day: Option<u64>,
    #[doc = "The maximum batch input tokens per day. Only present for relevant models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_1_day_max_input_tokens")]
    pub batch_1_day_max_input_tokens: Option<u64>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectRateLimitListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectRateLimitListResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectRateLimitListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<ProjectRateLimit>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ProjectRateLimitUpdateRequest {
    #[doc = "The maximum requests per minute."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_requests_per_1_minute")]
    pub max_requests_per_1_minute: Option<u64>,
    #[doc = "The maximum tokens per minute."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_tokens_per_1_minute")]
    pub max_tokens_per_1_minute: Option<u64>,
    #[doc = "The maximum images per minute. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_images_per_1_minute")]
    pub max_images_per_1_minute: Option<u64>,
    #[doc = "The maximum audio megabytes per minute. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_audio_megabytes_per_1_minute")]
    pub max_audio_megabytes_per_1_minute: Option<u64>,
    #[doc = "The maximum requests per day. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_requests_per_1_day")]
    pub max_requests_per_1_day: Option<u64>,
    #[doc = "The maximum batch input tokens per day. Only relevant for certain models."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch_1_day_max_input_tokens")]
    pub batch_1_day_max_input_tokens: Option<u64>,
}
#[doc = "The object type, which is always `organization.project.service_account`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountObject {
    #[default]
    #[serde(rename = "organization.project.service_account")]
    OrganizationProjectServiceAccount,
}
#[doc = "`owner` or `member`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectServiceAccountRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
}
#[doc = "Represents an individual service account in a project."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccount {
    #[doc = "The object type, which is always `organization.project.service_account`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectServiceAccountObject,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the service account"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "`owner` or `member`"]
    #[serde(rename = "role")]
    pub role: ProjectServiceAccountRole,
    #[doc = "The Unix timestamp (in seconds) of when the service account was created"]
    #[serde(rename = "created_at")]
    pub created_at: u64,
}
#[doc = "The object type, which is always `organization.project.service_account.api_key`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountApiKeyObject {
    #[default]
    #[serde(rename = "organization.project.service_account.api_key")]
    OrganizationProjectServiceAccountApiKey,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccountApiKey {
    #[doc = "The object type, which is always `organization.project.service_account.api_key`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectServiceAccountApiKeyObject,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccountCreateRequest {
    #[doc = "The name of the service account being created."]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountCreateResponseObject {
    #[default]
    #[serde(rename = "organization.project.service_account")]
    OrganizationProjectServiceAccount,
}
#[doc = "Service accounts can only have one role of type `member`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountCreateResponseRole {
    #[default]
    #[serde(rename = "member")]
    Member,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccountCreateResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectServiceAccountCreateResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Service accounts can only have one role of type `member`"]
    #[builder(default)]
    #[serde(rename = "role")]
    pub role: ProjectServiceAccountCreateResponseRole,
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[serde(rename = "api_key")]
    pub api_key: ProjectServiceAccountApiKey,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountDeleteResponseObject {
    #[default]
    #[serde(rename = "organization.project.service_account.deleted")]
    OrganizationProjectServiceAccountDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccountDeleteResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectServiceAccountDeleteResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectServiceAccountListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectServiceAccountListResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectServiceAccountListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<ProjectServiceAccount>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUpdateRequest {
    #[doc = "The updated name of the project, this name appears in reports."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "The object type, which is always `organization.project.user`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectUserObject {
    #[default]
    #[serde(rename = "organization.project.user")]
    OrganizationProjectUser,
}
#[doc = "`owner` or `member`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectUserRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
}
#[doc = "Represents an individual user in a project."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUser {
    #[doc = "The object type, which is always `organization.project.user`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectUserObject,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the user"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The email address of the user"]
    #[serde(rename = "email")]
    pub email: String,
    #[doc = "`owner` or `member`"]
    #[serde(rename = "role")]
    pub role: ProjectUserRole,
    #[doc = "The Unix timestamp (in seconds) of when the project was added."]
    #[serde(rename = "added_at")]
    pub added_at: u64,
}
#[doc = "`owner` or `member`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectUserCreateRequestRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUserCreateRequest {
    #[doc = "The ID of the user."]
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[doc = "`owner` or `member`"]
    #[serde(rename = "role")]
    pub role: ProjectUserCreateRequestRole,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ProjectUserDeleteResponseObject {
    #[default]
    #[serde(rename = "organization.project.user.deleted")]
    OrganizationProjectUserDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUserDeleteResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ProjectUserDeleteResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUserListResponse {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "data")]
    pub data: Vec<ProjectUser>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "`owner` or `member`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ProjectUserUpdateRequestRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "member")]
    Member,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ProjectUserUpdateRequest {
    #[doc = "`owner` or `member`"]
    #[serde(rename = "role")]
    pub role: ProjectUserUpdateRequestRole,
}
#[doc = "A realtime client event.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeClientEvent {
    ConversationItemCreate(RealtimeClientEventConversationItemCreate),
    ConversationItemDelete(RealtimeClientEventConversationItemDelete),
    ConversationItemRetrieve(RealtimeClientEventConversationItemRetrieve),
    ConversationItemTruncate(RealtimeClientEventConversationItemTruncate),
    InputAudioBufferAppend(RealtimeClientEventInputAudioBufferAppend),
    InputAudioBufferClear(RealtimeClientEventInputAudioBufferClear),
    OutputAudioBufferClear(RealtimeClientEventOutputAudioBufferClear),
    InputAudioBufferCommit(RealtimeClientEventInputAudioBufferCommit),
    ResponseCancel(RealtimeClientEventResponseCancel),
    ResponseCreate(RealtimeClientEventResponseCreate),
    SessionUpdate(RealtimeClientEventSessionUpdate),
    TranscriptionSessionUpdate(RealtimeClientEventTranscriptionSessionUpdate),
}
#[doc = "The event type, must be `conversation.item.create`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventConversationItemCreateType {
    #[default]
    #[serde(rename = "conversation.item.create")]
    ConversationItemCreate,
}
#[doc = "Add a new Item to the Conversation's context, including messages, function \ncalls, and function call responses. This event can be used both to populate a \n\"history\" of the conversation and to add new items mid-stream, but has the \ncurrent limitation that it cannot populate assistant audio messages.\n\nIf successful, the server will respond with a `conversation.item.created` \nevent, otherwise an `error` event will be sent.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventConversationItemCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `conversation.item.create`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemCreateType,
    #[doc = "The ID of the preceding item after which the new item will be inserted. \nIf not set, the new item will be appended to the end of the conversation.\nIf set to `root`, the new item will be added to the beginning of the conversation.\nIf set to an existing ID, it allows an item to be inserted mid-conversation. If the\nID cannot be found, an error will be returned and the item will not be added.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: Option<String>,
    #[builder(default)]
    #[serde(rename = "item")]
    pub item: RealtimeConversationItem,
}
#[doc = "The event type, must be `conversation.item.delete`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventConversationItemDeleteType {
    #[default]
    #[serde(rename = "conversation.item.delete")]
    ConversationItemDelete,
}
#[doc = "Send this event when you want to remove any item from the conversation \nhistory. The server will respond with a `conversation.item.deleted` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventConversationItemDelete {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `conversation.item.delete`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemDeleteType,
    #[doc = "The ID of the item to delete."]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be `conversation.item.retrieve`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventConversationItemRetrieveType {
    #[default]
    #[serde(rename = "conversation.item.retrieve")]
    ConversationItemRetrieve,
}
#[doc = "Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.\nThe server will respond with a `conversation.item.retrieved` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventConversationItemRetrieve {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `conversation.item.retrieve`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemRetrieveType,
    #[doc = "The ID of the item to retrieve."]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be `conversation.item.truncate`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventConversationItemTruncateType {
    #[default]
    #[serde(rename = "conversation.item.truncate")]
    ConversationItemTruncate,
}
#[doc = "Send this event to truncate a previous assistant message’s audio. The server \nwill produce audio faster than realtime, so this event is useful when the user \ninterrupts to truncate audio that has already been sent to the client but not \nyet played. This will synchronize the server's understanding of the audio with \nthe client's playback.\n\nTruncating audio will delete the server-side text transcript to ensure there \nis not text in the context that hasn't been heard by the user.\n\nIf successful, the server will respond with a `conversation.item.truncated` \nevent. \n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventConversationItemTruncate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `conversation.item.truncate`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventConversationItemTruncateType,
    #[doc = "The ID of the assistant message item to truncate. Only assistant message \nitems can be truncated.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the content part to truncate. Set this to 0."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "Inclusive duration up to which audio is truncated, in milliseconds. If \nthe audio_end_ms is greater than the actual audio duration, the server \nwill respond with an error.\n"]
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: u64,
}
#[doc = "The event type, must be `input_audio_buffer.append`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventInputAudioBufferAppendType {
    #[default]
    #[serde(rename = "input_audio_buffer.append")]
    InputAudioBufferAppend,
}
#[doc = "Send this event to append audio bytes to the input audio buffer. The audio \nbuffer is temporary storage you can write to and later commit. In Server VAD \nmode, the audio buffer is used to detect speech and the server will decide \nwhen to commit. When Server VAD is disabled, you must commit the audio buffer\nmanually.\n\nThe client may choose how much audio to place in each event up to a maximum \nof 15 MiB, for example streaming smaller chunks from the client may allow the \nVAD to be more responsive. Unlike made other client events, the server will \nnot send a confirmation response to this event.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventInputAudioBufferAppend {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `input_audio_buffer.append`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferAppendType,
    #[doc = "Base64-encoded audio bytes. This must be in the format specified by the \n`input_audio_format` field in the session configuration.\n"]
    #[serde(rename = "audio")]
    pub audio: String,
}
#[doc = "The event type, must be `input_audio_buffer.clear`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventInputAudioBufferClearType {
    #[default]
    #[serde(rename = "input_audio_buffer.clear")]
    InputAudioBufferClear,
}
#[doc = "Send this event to clear the audio bytes in the buffer. The server will \nrespond with an `input_audio_buffer.cleared` event.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventInputAudioBufferClear {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `input_audio_buffer.clear`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferClearType,
}
#[doc = "The event type, must be `input_audio_buffer.commit`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventInputAudioBufferCommitType {
    #[default]
    #[serde(rename = "input_audio_buffer.commit")]
    InputAudioBufferCommit,
}
#[doc = "Send this event to commit the user input audio buffer, which will create a \nnew user message item in the conversation. This event will produce an error \nif the input audio buffer is empty. When in Server VAD mode, the client does \nnot need to send this event, the server will commit the audio buffer \nautomatically.\n\nCommitting the input audio buffer will trigger input audio transcription \n(if enabled in session configuration), but it will not create a response \nfrom the model. The server will respond with an `input_audio_buffer.committed` \nevent.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventInputAudioBufferCommit {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `input_audio_buffer.commit`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventInputAudioBufferCommitType,
}
#[doc = "The event type, must be `output_audio_buffer.clear`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventOutputAudioBufferClearType {
    #[default]
    #[serde(rename = "output_audio_buffer.clear")]
    OutputAudioBufferClear,
}
#[doc = "**WebRTC Only:** Emit to cut off the current audio response. This will trigger the server to\nstop generating audio and emit a `output_audio_buffer.cleared` event. This \nevent should be preceded by a `response.cancel` client event to stop the \ngeneration of the current response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventOutputAudioBufferClear {
    #[doc = "The unique ID of the client event used for error handling."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `output_audio_buffer.clear`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventOutputAudioBufferClearType,
}
#[doc = "The event type, must be `response.cancel`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventResponseCancelType {
    #[default]
    #[serde(rename = "response.cancel")]
    ResponseCancel,
}
#[doc = "Send this event to cancel an in-progress response. The server will respond \nwith a `response.cancelled` event or an error if there is no response to \ncancel.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventResponseCancel {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `response.cancel`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventResponseCancelType,
    #[doc = "A specific response ID to cancel - if not provided, will cancel an \nin-progress response in the default conversation.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_id")]
    pub response_id: Option<String>,
}
#[doc = "The event type, must be `response.create`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventResponseCreateType {
    #[default]
    #[serde(rename = "response.create")]
    ResponseCreate,
}
#[doc = "This event instructs the server to create a Response, which means triggering \nmodel inference. When in Server VAD mode, the server will create Responses \nautomatically.\n\nA Response will include at least one Item, and may have two, in which case \nthe second will be a function call. These Items will be appended to the \nconversation history.\n\nThe server will respond with a `response.created` event, events for Items \nand content created, and finally a `response.done` event to indicate the \nResponse is complete.\n\nThe `response.create` event includes inference configuration like \n`instructions`, and `temperature`. These fields will override the Session's \nconfiguration for this Response only.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventResponseCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `response.create`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventResponseCreateType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response")]
    pub response: Option<RealtimeResponseCreateParams>,
}
#[doc = "The event type, must be `session.update`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventSessionUpdateType {
    #[default]
    #[serde(rename = "session.update")]
    SessionUpdate,
}
#[doc = "Send this event to update the session’s default configuration.\nThe client may send this event at any time to update any field,\nexcept for `voice`. However, note that once a session has been\ninitialized with a particular `model`, it can’t be changed to\nanother model using `session.update`.\n\nWhen the server receives a `session.update`, it will respond\nwith a `session.updated` event showing the full, effective configuration.\nOnly the fields that are present are updated. To clear a field like\n`instructions`, pass an empty string.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `session.update`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventSessionUpdateType,
    #[builder(default)]
    #[serde(rename = "session")]
    pub session: RealtimeSessionCreateRequest,
}
#[doc = "The event type, must be `transcription_session.update`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeClientEventTranscriptionSessionUpdateType {
    #[default]
    #[serde(rename = "transcription_session.update")]
    TranscriptionSessionUpdate,
}
#[doc = "Send this event to update a transcription session.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
    #[doc = "The event type, must be `transcription_session.update`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeClientEventTranscriptionSessionUpdateType,
    #[builder(default)]
    #[serde(rename = "session")]
    pub session: RealtimeTranscriptionSessionCreateRequest,
}
#[doc = "The type of the item (`message`, `function_call`, `function_call_output`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "function_call")]
    FunctionCall,
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeConversationItemObject {
    #[default]
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
#[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}
#[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemContentType {
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "item_reference")]
    ItemReference,
    #[serde(rename = "text")]
    Text,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeConversationItemContent {
    #[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeConversationItemContentType>,
    #[doc = "The text content, used for `input_text` and `text` content types.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[doc = "ID of a previous conversation item to reference (for `item_reference`\ncontent types in `response.create` events). These can reference both\nclient and server created items.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "Base64-encoded audio bytes, used for `input_audio` content type.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    #[doc = "The transcript of the audio, used for `input_audio` content type.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
}
#[doc = "The item to add to the conversation."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeConversationItem {
    #[doc = "The unique ID of the item, this can be generated by the client to help \nmanage server-side context, but is not required because the server will \ngenerate one if not provided.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of the item (`message`, `function_call`, `function_call_output`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeConversationItemType>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<RealtimeConversationItemObject>,
    #[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<RealtimeConversationItemStatus>,
    #[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<RealtimeConversationItemRole>,
    #[doc = "The content of the message, applicable for `message` items. \n- Message items of role `system` support only `input_text` content\n- Message items of role `user` support `input_text` and `input_audio` \n  content\n- Message items of role `assistant` support `text` content.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<RealtimeConversationItemContent>>,
    #[doc = "The ID of the function call (for `function_call` and \n`function_call_output` items). If passed on a `function_call_output` \nitem, the server will check that a `function_call` item with the same \nID exists in the conversation history.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "call_id")]
    pub call_id: Option<String>,
    #[doc = "The name of the function being called (for `function_call` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The arguments of the function call (for `function_call` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    #[doc = "The output of the function call (for `function_call_output` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<String>,
}
#[doc = "The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemWithReferenceType {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "function_call")]
    FunctionCall,
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeConversationItemWithReferenceObject {
    #[default]
    #[serde(rename = "realtime.item")]
    RealtimeItem,
}
#[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemWithReferenceStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemWithReferenceRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}
#[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeConversationItemWithReferenceContentType {
    #[serde(rename = "input_audio")]
    InputAudio,
    #[serde(rename = "input_text")]
    InputText,
    #[serde(rename = "item_reference")]
    ItemReference,
    #[serde(rename = "text")]
    Text,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeConversationItemWithReferenceContent {
    #[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeConversationItemWithReferenceContentType>,
    #[doc = "The text content, used for `input_text` and `text` content types.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[doc = "ID of a previous conversation item to reference (for `item_reference`\ncontent types in `response.create` events). These can reference both\nclient and server created items.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "Base64-encoded audio bytes, used for `input_audio` content type.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    #[doc = "The transcript of the audio, used for `input_audio` content type.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
}
#[doc = "The item to add to the conversation."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeConversationItemWithReference {
    #[doc = "For an item of type (`message` | `function_call` | `function_call_output`)\nthis field allows the client to assign the unique ID of the item. It is\nnot required because the server will generate one if not provided.\n\nFor an item of type `item_reference`, this field is required and is a\nreference to any item that has previously existed in the conversation.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeConversationItemWithReferenceType>,
    #[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<RealtimeConversationItemWithReferenceObject>,
    #[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<RealtimeConversationItemWithReferenceStatus>,
    #[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "role")]
    pub role: Option<RealtimeConversationItemWithReferenceRole>,
    #[doc = "The content of the message, applicable for `message` items. \n- Message items of role `system` support only `input_text` content\n- Message items of role `user` support `input_text` and `input_audio` \n  content\n- Message items of role `assistant` support `text` content.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<RealtimeConversationItemWithReferenceContent>>,
    #[doc = "The ID of the function call (for `function_call` and \n`function_call_output` items). If passed on a `function_call_output` \nitem, the server will check that a `function_call` item with the same \nID exists in the conversation history.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "call_id")]
    pub call_id: Option<String>,
    #[doc = "The name of the function being called (for `function_call` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The arguments of the function call (for `function_call` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    #[doc = "The output of the function call (for `function_call_output` items).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<String>,
}
#[doc = "The object type, must be `realtime.response`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeResponseObject {
    #[default]
    #[serde(rename = "realtime.response")]
    RealtimeResponse,
}
#[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseStatusDetailsType {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The reason the Response did not complete. For a `cancelled` Response, \none of `turn_detected` (the server VAD detected a new start of speech) \nor `client_cancelled` (the client sent a cancel event). For an \n`incomplete` Response, one of `max_output_tokens` or `content_filter` \n(the server-side safety filter activated and cut off the response).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseStatusDetailsReason {
    #[serde(rename = "turn_detected")]
    TurnDetected,
    #[serde(rename = "client_cancelled")]
    ClientCancelled,
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}
#[doc = "A description of the error that caused the response to fail, \npopulated when the `status` is `failed`.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseStatusDetailsError {
    #[doc = "The type of error."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "Error code, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
}
#[doc = "Additional details about the status."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseStatusDetails {
    #[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeResponseStatusDetailsType>,
    #[doc = "The reason the Response did not complete. For a `cancelled` Response, \none of `turn_detected` (the server VAD detected a new start of speech) \nor `client_cancelled` (the client sent a cancel event). For an \n`incomplete` Response, one of `max_output_tokens` or `content_filter` \n(the server-side safety filter activated and cut off the response).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reason")]
    pub reason: Option<RealtimeResponseStatusDetailsReason>,
    #[doc = "A description of the error that caused the response to fail, \npopulated when the `status` is `failed`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "error")]
    pub error: Option<RealtimeResponseStatusDetailsError>,
}
#[doc = "Details about the input tokens used in the Response."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseUsageInputTokenDetails {
    #[doc = "The number of cached tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: Option<u64>,
    #[doc = "The number of text tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text_tokens")]
    pub text_tokens: Option<u64>,
    #[doc = "The number of audio tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio_tokens")]
    pub audio_tokens: Option<u64>,
}
#[doc = "Details about the output tokens used in the Response."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseUsageOutputTokenDetails {
    #[doc = "The number of text tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text_tokens")]
    pub text_tokens: Option<u64>,
    #[doc = "The number of audio tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio_tokens")]
    pub audio_tokens: Option<u64>,
}
#[doc = "Usage statistics for the Response, this will correspond to billing. A \nRealtime API session will maintain a conversation context and append new \nItems to the Conversation, thus output from previous turns (text and \naudio tokens) will become the input for later turns.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseUsage {
    #[doc = "The total number of tokens in the Response including input and output \ntext and audio tokens.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "total_tokens")]
    pub total_tokens: Option<u64>,
    #[doc = "The number of input tokens used in the Response, including text and \naudio tokens.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_tokens")]
    pub input_tokens: Option<u64>,
    #[doc = "The number of output tokens sent in the Response, including text and \naudio tokens.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_tokens")]
    pub output_tokens: Option<u64>,
    #[doc = "Details about the input tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_token_details")]
    pub input_token_details: Option<RealtimeResponseUsageInputTokenDetails>,
    #[doc = "Details about the output tokens used in the Response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_token_details")]
    pub output_token_details: Option<RealtimeResponseUsageOutputTokenDetails>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseOutputAudioFormat {
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeResponseMaxOutputTokens1 {
    #[default]
    #[serde(rename = "inf")]
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseMaxOutputTokens {
    _0(u64),
    _1(RealtimeResponseMaxOutputTokens1),
}
#[doc = "The response resource."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponse {
    #[doc = "The unique ID of the response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The object type, must be `realtime.response`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<RealtimeResponseObject>,
    #[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<RealtimeResponseStatus>,
    #[doc = "Additional details about the status."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status_details")]
    pub status_details: Option<RealtimeResponseStatusDetails>,
    #[doc = "The list of output items generated by the response."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<Vec<RealtimeConversationItem>>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "Usage statistics for the Response, this will correspond to billing. A \nRealtime API session will maintain a conversation context and append new \nItems to the Conversation, thus output from previous turns (text and \naudio tokens) will become the input for later turns.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<RealtimeResponseUsage>,
    #[doc = "Which conversation the response is added to, determined by the `conversation`\nfield in the `response.create` event. If `auto`, the response will be added to\nthe default conversation and the value of `conversation_id` will be an id like\n`conv_1234`. If `none`, the response will not be added to any conversation and\nthe value of `conversation_id` will be `null`. If responses are being triggered\nby server VAD, the response will be added to the default conversation, thus\nthe `conversation_id` will be an id like `conv_1234`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conversation_id")]
    pub conversation_id: Option<String>,
    #[doc = "The voice the model used to respond.\nCurrent voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voice")]
    pub voice: Option<VoiceIdsShared>,
    #[doc = "The set of modalities the model used to respond. If there are multiple modalities,\nthe model will pick one, for example if `modalities` is `[\"text\", \"audio\"]`, the model\ncould be responding in either text or audio.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeResponseModality>>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<RealtimeResponseOutputAudioFormat>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_output_tokens")]
    pub max_output_tokens: Option<RealtimeResponseMaxOutputTokens>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseCreateParamsModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeResponseCreateParamsOutputAudioFormat {
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "The type of the tool, i.e. `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeResponseCreateParamsToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseCreateParamsTool {
    #[doc = "The type of the tool, i.e. `function`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeResponseCreateParamsToolType>,
    #[doc = "The name of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens1 {
    #[default]
    #[serde(rename = "inf")]
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens {
    _0(u64),
    _1(RealtimeResponseCreateParamsMaxResponseOutputTokens1),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeResponseCreateParamsConversation1 {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "none")]
    None,
}
#[doc = "Controls which conversation the response is added to. Currently supports\n`auto` and `none`, with `auto` as the default value. The `auto` value\nmeans that the contents of the response will be added to the default\nconversation. Set this to `none` to create an out-of-band response which \nwill not add items to default conversation.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseCreateParamsConversation {
    _0(String),
    _1(RealtimeResponseCreateParamsConversation1),
}
#[doc = "Create a new Realtime response with these parameters"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeResponseCreateParams {
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeResponseCreateParamsModality>>,
    #[doc = "The default system instructions (i.e. system message) prepended to model \ncalls. This field allows the client to guide the model on desired \nresponses. The model can be instructed on response content and format, \n(e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good \nresponses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion \ninto your voice\", \"laugh frequently\"). The instructions are not guaranteed \nto be followed by the model, but they provide guidance to the model on the \ndesired behavior.\n\nNote that the server sets default instructions which will be used if this \nfield is not set and are visible in the `session.created` event at the \nstart of the session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voice")]
    pub voice: Option<VoiceIdsShared>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<RealtimeResponseCreateParamsOutputAudioFormat>,
    #[doc = "Tools (functions) available to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<RealtimeResponseCreateParamsTool>>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function, like `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_response_output_tokens")]
    pub max_response_output_tokens: Option<RealtimeResponseCreateParamsMaxResponseOutputTokens>,
    #[doc = "Controls which conversation the response is added to. Currently supports\n`auto` and `none`, with `auto` as the default value. The `auto` value\nmeans that the contents of the response will be added to the default\nconversation. Set this to `none` to create an out-of-band response which \nwill not add items to default conversation.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "conversation")]
    pub conversation: Option<RealtimeResponseCreateParamsConversation>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
    #[doc = "Input items to include in the prompt for the model. Using this field\ncreates a new context for this Response instead of using the default\nconversation. An empty array `[]` will clear the context for this Response.\nNote that this can include references to items from the default conversation.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input")]
    pub input: Option<Vec<RealtimeConversationItemWithReference>>,
}
#[doc = "A realtime server event.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeServerEvent {
    ConversationCreated(RealtimeServerEventConversationCreated),
    ConversationItemCreated(RealtimeServerEventConversationItemCreated),
    ConversationItemDeleted(RealtimeServerEventConversationItemDeleted),
    ConversationItemInputAudioTranscriptionCompleted(
        RealtimeServerEventConversationItemInputAudioTranscriptionCompleted,
    ),
    ConversationItemInputAudioTranscriptionDelta(
        RealtimeServerEventConversationItemInputAudioTranscriptionDelta,
    ),
    ConversationItemInputAudioTranscriptionFailed(
        RealtimeServerEventConversationItemInputAudioTranscriptionFailed,
    ),
    ConversationItemRetrieved(RealtimeServerEventConversationItemRetrieved),
    ConversationItemTruncated(RealtimeServerEventConversationItemTruncated),
    Error(RealtimeServerEventError),
    InputAudioBufferCleared(RealtimeServerEventInputAudioBufferCleared),
    InputAudioBufferCommitted(RealtimeServerEventInputAudioBufferCommitted),
    InputAudioBufferSpeechStarted(RealtimeServerEventInputAudioBufferSpeechStarted),
    InputAudioBufferSpeechStopped(RealtimeServerEventInputAudioBufferSpeechStopped),
    RateLimitsUpdated(RealtimeServerEventRateLimitsUpdated),
    ResponseAudioDelta(RealtimeServerEventResponseAudioDelta),
    ResponseAudioDone(RealtimeServerEventResponseAudioDone),
    ResponseAudioTranscriptDelta(RealtimeServerEventResponseAudioTranscriptDelta),
    ResponseAudioTranscriptDone(RealtimeServerEventResponseAudioTranscriptDone),
    ResponseContentPartAdded(RealtimeServerEventResponseContentPartAdded),
    ResponseContentPartDone(RealtimeServerEventResponseContentPartDone),
    ResponseCreated(RealtimeServerEventResponseCreated),
    ResponseDone(RealtimeServerEventResponseDone),
    ResponseFunctionCallArgumentsDelta(RealtimeServerEventResponseFunctionCallArgumentsDelta),
    ResponseFunctionCallArgumentsDone(RealtimeServerEventResponseFunctionCallArgumentsDone),
    ResponseOutputItemAdded(RealtimeServerEventResponseOutputItemAdded),
    ResponseOutputItemDone(RealtimeServerEventResponseOutputItemDone),
    ResponseTextDelta(RealtimeServerEventResponseTextDelta),
    ResponseTextDone(RealtimeServerEventResponseTextDone),
    SessionCreated(RealtimeServerEventSessionCreated),
    SessionUpdated(RealtimeServerEventSessionUpdated),
    TranscriptionSessionUpdated(RealtimeServerEventTranscriptionSessionUpdated),
    OutputAudioBufferStarted(RealtimeServerEventOutputAudioBufferStarted),
    OutputAudioBufferStopped(RealtimeServerEventOutputAudioBufferStopped),
    OutputAudioBufferCleared(RealtimeServerEventOutputAudioBufferCleared),
}
#[doc = "The event type, must be `conversation.created`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationCreatedType {
    #[default]
    #[serde(rename = "conversation.created")]
    ConversationCreated,
}
#[doc = "The conversation resource."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationCreatedConversation {
    #[doc = "The unique ID of the conversation."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The object type, must be `realtime.conversation`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<String>,
}
#[doc = "Returned when a conversation is created. Emitted right after session creation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationCreated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.created`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationCreatedType,
    #[doc = "The conversation resource."]
    #[builder(default)]
    #[serde(rename = "conversation")]
    pub conversation: RealtimeServerEventConversationCreatedConversation,
}
#[doc = "The event type, must be `conversation.item.created`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemCreatedType {
    #[default]
    #[serde(rename = "conversation.item.created")]
    ConversationItemCreated,
}
#[doc = "Returned when a conversation item is created. There are several scenarios that produce this event:\n  - The server is generating a Response, which if successful will produce \n    either one or two Items, which will be of type `message` \n    (role `assistant`) or type `function_call`.\n  - The input audio buffer has been committed, either by the client or the \n    server (in `server_vad` mode). The server will take the content of the \n    input audio buffer and add it to a new user message Item.\n  - The client has sent a `conversation.item.create` event to add a new Item \n    to the Conversation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemCreated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.item.created`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemCreatedType,
    #[doc = "The ID of the preceding item in the Conversation context, allows the \nclient to understand the order of the conversation.\n"]
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: String,
    #[builder(default)]
    #[serde(rename = "item")]
    pub item: RealtimeConversationItem,
}
#[doc = "The event type, must be `conversation.item.deleted`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemDeletedType {
    #[default]
    #[serde(rename = "conversation.item.deleted")]
    ConversationItemDeleted,
}
#[doc = "Returned when an item in the conversation is deleted by the client with a \n`conversation.item.delete` event. This event is used to synchronize the \nserver's understanding of the conversation history with the client's view.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemDeleted {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.item.deleted`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemDeletedType,
    #[doc = "The ID of the item that was deleted."]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.completed")]
    ConversationItemInputAudioTranscriptionCompleted,
}
#[doc = "This event is the output of audio transcription for user audio written to the \nuser audio buffer. Transcription begins when the input audio buffer is \ncommitted by the client or server (in `server_vad` mode). Transcription runs \nasynchronously with Response creation, so this event may come before or after \nthe Response events.\n\nRealtime API models accept audio natively, and thus input transcription is a \nseparate process run on a separate ASR (Automatic Speech Recognition) model, \ncurrently always `whisper-1`. Thus the transcript may diverge somewhat from \nthe model's interpretation, and should be treated as a rough guide.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType,
    #[doc = "The ID of the user message item containing the audio."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the content part containing the audio."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The transcribed text."]
    #[serde(rename = "transcript")]
    pub transcript: String,
    #[doc = "The log probabilities of the transcription."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<LogProbProperties>>,
}
#[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.delta")]
    ConversationItemInputAudioTranscriptionDelta,
}
#[doc = "Returned when the text value of an input audio transcription content part is updated.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the content part in the item's content array."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content_index")]
    pub content_index: Option<u64>,
    #[doc = "The text delta."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "delta")]
    pub delta: Option<String>,
    #[doc = "The log probabilities of the transcription."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<LogProbProperties>>,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    #[default]
    #[serde(rename = "conversation.item.input_audio_transcription.failed")]
    ConversationItemInputAudioTranscriptionFailed,
}
#[doc = "Details of the transcription error."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    #[doc = "The type of error."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "Error code, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "A human-readable error message."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[doc = "Parameter related to the error, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
}
#[doc = "Returned when input audio transcription is configured, and a transcription \nrequest for a user message failed. These events are separate from other \n`error` events so that the client can identify the related Item.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionFailedType,
    #[doc = "The ID of the user message item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the content part containing the audio."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "Details of the transcription error."]
    #[builder(default)]
    #[serde(rename = "error")]
    pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
}
#[doc = "The event type, must be `conversation.item.retrieved`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemRetrievedType {
    #[default]
    #[serde(rename = "conversation.item.retrieved")]
    ConversationItemRetrieved,
}
#[doc = "Returned when a conversation item is retrieved with `conversation.item.retrieve`.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemRetrieved {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.item.retrieved`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemRetrievedType,
    #[builder(default)]
    #[serde(rename = "item")]
    pub item: RealtimeConversationItem,
}
#[doc = "The event type, must be `conversation.item.truncated`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventConversationItemTruncatedType {
    #[default]
    #[serde(rename = "conversation.item.truncated")]
    ConversationItemTruncated,
}
#[doc = "Returned when an earlier assistant audio message item is truncated by the \nclient with a `conversation.item.truncate` event. This event is used to \nsynchronize the server's understanding of the audio with the client's playback.\n\nThis action will truncate the audio and remove the server-side text transcript \nto ensure there is no text in the context that hasn't been heard by the user.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventConversationItemTruncated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.item.truncated`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventConversationItemTruncatedType,
    #[doc = "The ID of the assistant message item that was truncated."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the content part that was truncated."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The duration up to which the audio was truncated, in milliseconds.\n"]
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: u64,
}
#[doc = "The event type, must be `error`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventErrorType {
    #[default]
    #[serde(rename = "error")]
    Error,
}
#[doc = "Details of the error."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventErrorError {
    #[doc = "The type of error (e.g., \"invalid_request_error\", \"server_error\").\n"]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Error code, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "A human-readable error message."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "Parameter related to the error, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
    #[doc = "The event_id of the client event that caused the error, if applicable.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "event_id")]
    pub event_id: Option<String>,
}
#[doc = "Returned when an error occurs, which could be a client problem or a server \nproblem. Most errors are recoverable and the session will stay open, we \nrecommend to implementors to monitor and log error messages by default.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventError {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `error`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventErrorType,
    #[doc = "Details of the error."]
    #[serde(rename = "error")]
    pub error: RealtimeServerEventErrorError,
}
#[doc = "The event type, must be `input_audio_buffer.cleared`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventInputAudioBufferClearedType {
    #[default]
    #[serde(rename = "input_audio_buffer.cleared")]
    InputAudioBufferCleared,
}
#[doc = "Returned when the input audio buffer is cleared by the client with a \n`input_audio_buffer.clear` event.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventInputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.cleared`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferClearedType,
}
#[doc = "The event type, must be `input_audio_buffer.committed`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventInputAudioBufferCommittedType {
    #[default]
    #[serde(rename = "input_audio_buffer.committed")]
    InputAudioBufferCommitted,
}
#[doc = "Returned when an input audio buffer is committed, either by the client or \nautomatically in server VAD mode. The `item_id` property is the ID of the user\nmessage item that will be created, thus a `conversation.item.created` event \nwill also be sent to the client.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.committed`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferCommittedType,
    #[doc = "The ID of the preceding item after which the new item will be inserted.\n"]
    #[serde(rename = "previous_item_id")]
    pub previous_item_id: String,
    #[doc = "The ID of the user message item that will be created."]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be `input_audio_buffer.speech_started`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventInputAudioBufferSpeechStartedType {
    #[default]
    #[serde(rename = "input_audio_buffer.speech_started")]
    InputAudioBufferSpeechStarted,
}
#[doc = "Sent by the server when in `server_vad` mode to indicate that speech has been \ndetected in the audio buffer. This can happen any time audio is added to the \nbuffer (unless speech is already detected). The client may want to use this \nevent to interrupt audio playback or provide visual feedback to the user. \n\nThe client should expect to receive a `input_audio_buffer.speech_stopped` event \nwhen speech stops. The `item_id` property is the ID of the user message item \nthat will be created when speech stops and will also be included in the \n`input_audio_buffer.speech_stopped` event (unless the client manually commits \nthe audio buffer during VAD activation).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.speech_started`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStartedType,
    #[doc = "Milliseconds from the start of all audio written to the buffer during the \nsession when speech was first detected. This will correspond to the \nbeginning of audio sent to the model, and thus includes the \n`prefix_padding_ms` configured in the Session.\n"]
    #[serde(rename = "audio_start_ms")]
    pub audio_start_ms: u64,
    #[doc = "The ID of the user message item that will be created when speech stops.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventInputAudioBufferSpeechStoppedType {
    #[default]
    #[serde(rename = "input_audio_buffer.speech_stopped")]
    InputAudioBufferSpeechStopped,
}
#[doc = "Returned in `server_vad` mode when the server detects the end of speech in \nthe audio buffer. The server will also send an `conversation.item.created` \nevent with the user message item that is created from the audio buffer.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStoppedType,
    #[doc = "Milliseconds since the session started when speech stopped. This will \ncorrespond to the end of audio sent to the model, and thus includes the \n`min_silence_duration_ms` configured in the Session.\n"]
    #[serde(rename = "audio_end_ms")]
    pub audio_end_ms: u64,
    #[doc = "The ID of the user message item that will be created."]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The event type, must be `output_audio_buffer.cleared`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventOutputAudioBufferClearedType {
    #[default]
    #[serde(rename = "output_audio_buffer.cleared")]
    OutputAudioBufferCleared,
}
#[doc = "**WebRTC Only:** Emitted when the output audio buffer is cleared. This happens either in VAD\nmode when the user has interrupted (`input_audio_buffer.speech_started`),\nor when the client has emitted the `output_audio_buffer.clear` event to manually\ncut off the current audio response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `output_audio_buffer.cleared`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferClearedType,
    #[doc = "The unique ID of the response that produced the audio."]
    #[serde(rename = "response_id")]
    pub response_id: String,
}
#[doc = "The event type, must be `output_audio_buffer.started`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventOutputAudioBufferStartedType {
    #[default]
    #[serde(rename = "output_audio_buffer.started")]
    OutputAudioBufferStarted,
}
#[doc = "**WebRTC Only:** Emitted when the server begins streaming audio to the client. This event is\nemitted after an audio content part has been added (`response.content_part.added`)\nto the response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `output_audio_buffer.started`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferStartedType,
    #[doc = "The unique ID of the response that produced the audio."]
    #[serde(rename = "response_id")]
    pub response_id: String,
}
#[doc = "The event type, must be `output_audio_buffer.stopped`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventOutputAudioBufferStoppedType {
    #[default]
    #[serde(rename = "output_audio_buffer.stopped")]
    OutputAudioBufferStopped,
}
#[doc = "**WebRTC Only:** Emitted when the output audio buffer has been completely drained on the server,\nand no more audio is forthcoming. This event is emitted after the full response\ndata has been sent to the client (`response.done`).\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `output_audio_buffer.stopped`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventOutputAudioBufferStoppedType,
    #[doc = "The unique ID of the response that produced the audio."]
    #[serde(rename = "response_id")]
    pub response_id: String,
}
#[doc = "The event type, must be `rate_limits.updated`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventRateLimitsUpdatedType {
    #[default]
    #[serde(rename = "rate_limits.updated")]
    RateLimitsUpdated,
}
#[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeServerEventRateLimitsUpdatedRateLimitsName {
    #[serde(rename = "requests")]
    Requests,
    #[serde(rename = "tokens")]
    Tokens,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventRateLimitsUpdatedRateLimits {
    #[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<RealtimeServerEventRateLimitsUpdatedRateLimitsName>,
    #[doc = "The maximum allowed value for the rate limit."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "limit")]
    pub limit: Option<u64>,
    #[doc = "The remaining value before the limit is reached."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "remaining")]
    pub remaining: Option<u64>,
    #[doc = "Seconds until the rate limit resets."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reset_seconds")]
    pub reset_seconds: Option<f64>,
}
#[doc = "Emitted at the beginning of a Response to indicate the updated rate limits. \nWhen a Response is created some tokens will be \"reserved\" for the output \ntokens, the rate limits shown here reflect that reservation, which is then \nadjusted accordingly once the Response is completed.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventRateLimitsUpdated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `rate_limits.updated`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventRateLimitsUpdatedType,
    #[doc = "List of rate limit information."]
    #[serde(rename = "rate_limits")]
    pub rate_limits: Vec<RealtimeServerEventRateLimitsUpdatedRateLimits>,
}
#[doc = "The event type, must be `response.audio.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseAudioDeltaType {
    #[default]
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta,
}
#[doc = "Returned when the model-generated audio is updated."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseAudioDelta {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.audio.delta`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioDeltaType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "Base64-encoded audio data delta."]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The event type, must be `response.audio.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseAudioDoneType {
    #[default]
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone,
}
#[doc = "Returned when the model-generated audio is done. Also emitted when a Response\nis interrupted, incomplete, or cancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseAudioDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.audio.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioDoneType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
}
#[doc = "The event type, must be `response.audio_transcript.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseAudioTranscriptDeltaType {
    #[default]
    #[serde(rename = "response.audio_transcript.delta")]
    ResponseAudioTranscriptDelta,
}
#[doc = "Returned when the model-generated transcription of audio output is updated.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.audio_transcript.delta`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioTranscriptDeltaType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The transcript delta."]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The event type, must be `response.audio_transcript.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseAudioTranscriptDoneType {
    #[default]
    #[serde(rename = "response.audio_transcript.done")]
    ResponseAudioTranscriptDone,
}
#[doc = "Returned when the model-generated transcription of audio output is done\nstreaming. Also emitted when a Response is interrupted, incomplete, or\ncancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.audio_transcript.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseAudioTranscriptDoneType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The final transcript of the audio."]
    #[serde(rename = "transcript")]
    pub transcript: String,
}
#[doc = "The event type, must be `response.content_part.added`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseContentPartAddedType {
    #[default]
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}
#[doc = "The content type (\"text\", \"audio\")."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeServerEventResponseContentPartAddedPartType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "text")]
    Text,
}
#[doc = "The content part that was added."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseContentPartAddedPart {
    #[doc = "The content type (\"text\", \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeServerEventResponseContentPartAddedPartType>,
    #[doc = "The text content (if type is \"text\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
}
#[doc = "Returned when a new content part is added to an assistant message item during\nresponse generation.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseContentPartAdded {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.content_part.added`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseContentPartAddedType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item to which the content part was added."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The content part that was added."]
    #[builder(default)]
    #[serde(rename = "part")]
    pub part: RealtimeServerEventResponseContentPartAddedPart,
}
#[doc = "The event type, must be `response.content_part.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseContentPartDoneType {
    #[default]
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}
#[doc = "The content type (\"text\", \"audio\")."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeServerEventResponseContentPartDonePartType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "text")]
    Text,
}
#[doc = "The content part that is done."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseContentPartDonePart {
    #[doc = "The content type (\"text\", \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeServerEventResponseContentPartDonePartType>,
    #[doc = "The text content (if type is \"text\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "audio")]
    pub audio: Option<String>,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "transcript")]
    pub transcript: Option<String>,
}
#[doc = "Returned when a content part is done streaming in an assistant message item.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseContentPartDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.content_part.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseContentPartDoneType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The content part that is done."]
    #[builder(default)]
    #[serde(rename = "part")]
    pub part: RealtimeServerEventResponseContentPartDonePart,
}
#[doc = "The event type, must be `response.created`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseCreatedType {
    #[default]
    #[serde(rename = "response.created")]
    ResponseCreated,
}
#[doc = "Returned when a new Response is created. The first event of response creation,\nwhere the response is in an initial state of `in_progress`.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseCreated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.created`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseCreatedType,
    #[builder(default)]
    #[serde(rename = "response")]
    pub response: RealtimeResponse,
}
#[doc = "The event type, must be `response.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseDoneType {
    #[default]
    #[serde(rename = "response.done")]
    ResponseDone,
}
#[doc = "Returned when a Response is done streaming. Always emitted, no matter the \nfinal state. The Response object included in the `response.done` event will \ninclude all output Items in the Response but will omit the raw audio data.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseDoneType,
    #[builder(default)]
    #[serde(rename = "response")]
    pub response: RealtimeResponse,
}
#[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    #[default]
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}
#[doc = "Returned when the model-generated function call arguments are updated.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDeltaType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the function call item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The ID of the function call."]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The arguments delta as a JSON string."]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    #[default]
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}
#[doc = "Returned when the model-generated function call arguments are done streaming.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDoneType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the function call item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The ID of the function call."]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The final arguments as a JSON string."]
    #[serde(rename = "arguments")]
    pub arguments: String,
}
#[doc = "The event type, must be `response.output_item.added`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseOutputItemAddedType {
    #[default]
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}
#[doc = "Returned when a new Item is created during Response generation."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseOutputItemAdded {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.output_item.added`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseOutputItemAddedType,
    #[doc = "The ID of the Response to which the item belongs."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The index of the output item in the Response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[builder(default)]
    #[serde(rename = "item")]
    pub item: RealtimeConversationItem,
}
#[doc = "The event type, must be `response.output_item.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseOutputItemDoneType {
    #[default]
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}
#[doc = "Returned when an Item is done streaming. Also emitted when a Response is \ninterrupted, incomplete, or cancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseOutputItemDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.output_item.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseOutputItemDoneType,
    #[doc = "The ID of the Response to which the item belongs."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The index of the output item in the Response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[builder(default)]
    #[serde(rename = "item")]
    pub item: RealtimeConversationItem,
}
#[doc = "The event type, must be `response.text.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseTextDeltaType {
    #[default]
    #[serde(rename = "response.text.delta")]
    ResponseTextDelta,
}
#[doc = "Returned when the text value of a \"text\" content part is updated."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseTextDelta {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.text.delta`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseTextDeltaType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The text delta."]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The event type, must be `response.text.done`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventResponseTextDoneType {
    #[default]
    #[serde(rename = "response.text.done")]
    ResponseTextDone,
}
#[doc = "Returned when the text value of a \"text\" content part is done streaming. Also\nemitted when a Response is interrupted, incomplete, or cancelled.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventResponseTextDone {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `response.text.done`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventResponseTextDoneType,
    #[doc = "The ID of the response."]
    #[serde(rename = "response_id")]
    pub response_id: String,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part in the item's content array."]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The final text content."]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The event type, must be `session.created`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventSessionCreatedType {
    #[default]
    #[serde(rename = "session.created")]
    SessionCreated,
}
#[doc = "Returned when a Session is created. Emitted automatically when a new \nconnection is established as the first server event. This event will contain \nthe default Session configuration.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventSessionCreated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `session.created`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventSessionCreatedType,
    #[builder(default)]
    #[serde(rename = "session")]
    pub session: RealtimeSession,
}
#[doc = "The event type, must be `session.updated`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventSessionUpdatedType {
    #[default]
    #[serde(rename = "session.updated")]
    SessionUpdated,
}
#[doc = "Returned when a session is updated with a `session.update` event, unless \nthere is an error.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventSessionUpdated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `session.updated`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventSessionUpdatedType,
    #[builder(default)]
    #[serde(rename = "session")]
    pub session: RealtimeSession,
}
#[doc = "The event type, must be `transcription_session.updated`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeServerEventTranscriptionSessionUpdatedType {
    #[default]
    #[serde(rename = "transcription_session.updated")]
    TranscriptionSessionUpdated,
}
#[doc = "Returned when a transcription session is updated with a `transcription_session.update` event, unless \nthere is an error.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
    #[doc = "The unique ID of the server event."]
    #[serde(rename = "event_id")]
    pub event_id: String,
    #[doc = "The event type, must be `transcription_session.updated`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RealtimeServerEventTranscriptionSessionUpdatedType,
    #[serde(rename = "session")]
    pub session: RealtimeTranscriptionSessionCreateResponse,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The Realtime model used for this session.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionModel {
    #[serde(rename = "gpt-4o-realtime-preview")]
    Gpt4oRealtimePreview,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    Gpt4oRealtimePreview20241001,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    Gpt4oRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-realtime-preview")]
    Gpt4oMiniRealtimePreview,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    Gpt4oMiniRealtimePreview20241217,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionInputAudioFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionOutputAudioFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionInputAudioTranscription {
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
}
#[doc = "Type of turn detection.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionTurnDetectionType {
    #[default]
    #[serde(rename = "server_vad")]
    ServerVad,
    #[serde(rename = "semantic_vad")]
    SemanticVad,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionTurnDetectionEagerness {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionTurnDetection {
    #[doc = "Type of turn detection.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionTurnDetectionType>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eagerness")]
    pub eagerness: Option<RealtimeSessionTurnDetectionEagerness>,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<u64>,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<u64>,
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "create_response")]
    pub create_response: Option<bool>,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "interrupt_response")]
    pub interrupt_response: Option<bool>,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionInputAudioNoiseReductionType {
    #[serde(rename = "near_field")]
    NearField,
    #[serde(rename = "far_field")]
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionInputAudioNoiseReductionType>,
}
#[doc = "The type of the tool, i.e. `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionTool {
    #[doc = "The type of the tool, i.e. `function`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionToolType>,
    #[doc = "The name of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionMaxResponseOutputTokens1 {
    #[default]
    #[serde(rename = "inf")]
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionMaxResponseOutputTokens {
    _0(u64),
    _1(RealtimeSessionMaxResponseOutputTokens1),
}
#[doc = "Realtime session object configuration."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSession {
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeSessionModality>>,
    #[doc = "The Realtime model used for this session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<RealtimeSessionModel>,
    #[doc = "The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good  responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion  into your voice\", \"laugh frequently\"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, \n`shimmer` and `verse`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voice")]
    pub voice: Option<VoiceIdsShared>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<RealtimeSessionInputAudioFormat>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<RealtimeSessionOutputAudioFormat>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription: Option<RealtimeSessionInputAudioTranscription>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<RealtimeSessionTurnDetection>,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_noise_reduction")]
    pub input_audio_noise_reduction: Option<RealtimeSessionInputAudioNoiseReduction>,
    #[doc = "Tools (functions) available to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<RealtimeSessionTool>>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_response_output_tokens")]
    pub max_response_output_tokens: Option<RealtimeSessionMaxResponseOutputTokens>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionCreateRequestModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The Realtime model used for this session.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionCreateRequestModel {
    #[serde(rename = "gpt-4o-realtime-preview")]
    Gpt4oRealtimePreview,
    #[serde(rename = "gpt-4o-realtime-preview-2024-10-01")]
    Gpt4oRealtimePreview20241001,
    #[serde(rename = "gpt-4o-realtime-preview-2024-12-17")]
    Gpt4oRealtimePreview20241217,
    #[serde(rename = "gpt-4o-mini-realtime-preview")]
    Gpt4oMiniRealtimePreview,
    #[serde(rename = "gpt-4o-mini-realtime-preview-2024-12-17")]
    Gpt4oMiniRealtimePreview20241217,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestInputAudioFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestOutputAudioFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
}
#[doc = "Type of turn detection.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestTurnDetectionType {
    #[default]
    #[serde(rename = "server_vad")]
    ServerVad,
    #[serde(rename = "semantic_vad")]
    SemanticVad,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestTurnDetectionEagerness {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateRequestTurnDetection {
    #[doc = "Type of turn detection.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionCreateRequestTurnDetectionType>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eagerness")]
    pub eagerness: Option<RealtimeSessionCreateRequestTurnDetectionEagerness>,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<u64>,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<u64>,
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "create_response")]
    pub create_response: Option<bool>,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "interrupt_response")]
    pub interrupt_response: Option<bool>,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionCreateRequestInputAudioNoiseReductionType {
    #[serde(rename = "near_field")]
    NearField,
    #[serde(rename = "far_field")]
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionCreateRequestInputAudioNoiseReductionType>,
}
#[doc = "The type of the tool, i.e. `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateRequestTool {
    #[doc = "The type of the tool, i.e. `function`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionCreateRequestToolType>,
    #[doc = "The name of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens1 {
    #[default]
    #[serde(rename = "inf")]
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
    _0(u64),
    _1(RealtimeSessionCreateRequestMaxResponseOutputTokens1),
}
#[doc = "Realtime session object configuration."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateRequest {
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeSessionCreateRequestModality>>,
    #[doc = "The Realtime model used for this session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<RealtimeSessionCreateRequestModel>,
    #[doc = "The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good  responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion  into your voice\", \"laugh frequently\"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voice")]
    pub voice: Option<VoiceIdsShared>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<RealtimeSessionCreateRequestInputAudioFormat>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<RealtimeSessionCreateRequestOutputAudioFormat>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription: Option<RealtimeSessionCreateRequestInputAudioTranscription>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<RealtimeSessionCreateRequestTurnDetection>,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_noise_reduction")]
    pub input_audio_noise_reduction: Option<RealtimeSessionCreateRequestInputAudioNoiseReduction>,
    #[doc = "Tools (functions) available to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<RealtimeSessionCreateRequestTool>>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_response_output_tokens")]
    pub max_response_output_tokens: Option<RealtimeSessionCreateRequestMaxResponseOutputTokens>,
}
#[doc = "Ephemeral key returned by the API."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateResponseClientSecret {
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    #[serde(rename = "expires_at")]
    pub expires_at: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeSessionCreateResponseModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be \nset to `null` to turn off once on. Input audio transcription is not native \nto the model, since the model consumes audio directly. Transcription runs \nasynchronously through Whisper and should be treated as rough guidance \nrather than the representation understood by the model.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateResponseInputAudioTranscription {
    #[doc = "The model to use for transcription, `whisper-1` is the only currently \nsupported model.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateResponseTurnDetection {
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[doc = "Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<u64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<u64>,
}
#[doc = "The type of the tool, i.e. `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateResponseToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateResponseTool {
    #[doc = "The type of the tool, i.e. `function`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeSessionCreateResponseToolType>,
    #[doc = "The name of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "Parameters of the function in JSON Schema."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<std::collections::HashMap<String, serde_json::Value>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens1 {
    #[default]
    #[serde(rename = "inf")]
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens {
    _0(u64),
    _1(RealtimeSessionCreateResponseMaxResponseOutputTokens1),
}
#[doc = "A new Realtime session configuration, with an ephermeral key. Default TTL\nfor keys is one minute.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeSessionCreateResponse {
    #[doc = "Ephemeral key returned by the API."]
    #[serde(rename = "client_secret")]
    pub client_secret: RealtimeSessionCreateResponseClientSecret,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeSessionCreateResponseModality>>,
    #[doc = "The default system instructions (i.e. system message) prepended to model \ncalls. This field allows the client to guide the model on desired \nresponses. The model can be instructed on response content and format, \n(e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good \nresponses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion \ninto your voice\", \"laugh frequently\"). The instructions are not guaranteed \nto be followed by the model, but they provide guidance to the model on the \ndesired behavior.\n\nNote that the server sets default instructions which will be used if this \nfield is not set and are visible in the `session.created` event at the \nstart of the session.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, \n`shimmer` and `verse`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "voice")]
    pub voice: Option<VoiceIdsShared>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<String>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_format")]
    pub output_audio_format: Option<String>,
    #[doc = "Configuration for input audio transcription, defaults to off and can be \nset to `null` to turn off once on. Input audio transcription is not native \nto the model, since the model consumes audio directly. Transcription runs \nasynchronously through Whisper and should be treated as rough guidance \nrather than the representation understood by the model.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription: Option<RealtimeSessionCreateResponseInputAudioTranscription>,
    #[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<RealtimeSessionCreateResponseTurnDetection>,
    #[doc = "Tools (functions) available to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<RealtimeSessionCreateResponseTool>>,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<String>,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_response_output_tokens")]
    pub max_response_output_tokens: Option<RealtimeSessionCreateResponseMaxResponseOutputTokens>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeTranscriptionSessionCreateRequestModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    #[default]
    #[serde(rename = "pcm16")]
    Pcm16,
    #[serde(rename = "g711_ulaw")]
    G711Ulaw,
    #[serde(rename = "g711_alaw")]
    G711Alaw,
}
#[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel {
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
    #[serde(rename = "whisper-1")]
    Whisper1,
}
#[doc = "Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioTranscription {
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel>,
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
}
#[doc = "Type of turn detection.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    #[default]
    #[serde(rename = "server_vad")]
    ServerVad,
    #[serde(rename = "semantic_vad")]
    SemanticVad,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[default]
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
    #[doc = "Type of turn detection.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionType>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "eagerness")]
    pub eagerness: Option<RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness>,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<u64>,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<u64>,
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs. Not available for transcription sessions.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "create_response")]
    pub create_response: Option<bool>,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs. Not available for transcription sessions.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "interrupt_response")]
    pub interrupt_response: Option<bool>,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType {
    #[serde(rename = "near_field")]
    NearField,
    #[serde(rename = "far_field")]
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType>,
}
#[doc = "Realtime transcription session object configuration."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateRequest {
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeTranscriptionSessionCreateRequestModality>>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<RealtimeTranscriptionSessionCreateRequestInputAudioFormat>,
    #[doc = "Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription:
        Option<RealtimeTranscriptionSessionCreateRequestInputAudioTranscription>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<RealtimeTranscriptionSessionCreateRequestTurnDetection>,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_noise_reduction")]
    pub input_audio_noise_reduction:
        Option<RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction>,
    #[doc = "The set of items to include in the transcription. Current available items are:\n- `item.input_audio_transcription.logprobs`\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "include")]
    pub include: Option<Vec<String>>,
}
#[doc = "Ephemeral key returned by the API. Only present when the session is\ncreated on the server via REST API.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateResponseClientSecret {
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    #[serde(rename = "expires_at")]
    pub expires_at: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeTranscriptionSessionCreateResponseModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
#[doc = "The model to use for transcription. Can be `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, or `whisper-1`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel {
    #[serde(rename = "gpt-4o-transcribe")]
    Gpt4oTranscribe,
    #[serde(rename = "gpt-4o-mini-transcribe")]
    Gpt4oMiniTranscribe,
    #[serde(rename = "whisper-1")]
    Whisper1,
}
#[doc = "Configuration of the transcription model.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateResponseInputAudioTranscription {
    #[doc = "The model to use for transcription. Can be `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, or `whisper-1`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel>,
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "language")]
    pub language: Option<String>,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment. The [prompt](/docs/guides/speech-to-text#prompting) should match\nthe audio language.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prompt")]
    pub prompt: Option<String>,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateResponseTurnDetection {
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "threshold")]
    pub threshold: Option<f64>,
    #[doc = "Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "prefix_padding_ms")]
    pub prefix_padding_ms: Option<u64>,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "silence_duration_ms")]
    pub silence_duration_ms: Option<u64>,
}
#[doc = "A new Realtime transcription session configuration.\n\nWhen a session is created on the server via REST API, the session object\nalso contains an ephemeral key. Default TTL for keys is one minute. This \nproperty is not present when a session is updated via the WebSocket API.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RealtimeTranscriptionSessionCreateResponse {
    #[doc = "Ephemeral key returned by the API. Only present when the session is\ncreated on the server via REST API.\n"]
    #[serde(rename = "client_secret")]
    pub client_secret: RealtimeTranscriptionSessionCreateResponseClientSecret,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "modalities")]
    pub modalities: Option<Vec<RealtimeTranscriptionSessionCreateResponseModality>>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_format")]
    pub input_audio_format: Option<String>,
    #[doc = "Configuration of the transcription model.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_transcription")]
    pub input_audio_transcription:
        Option<RealtimeTranscriptionSessionCreateResponseInputAudioTranscription>,
    #[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "turn_detection")]
    pub turn_detection: Option<RealtimeTranscriptionSessionCreateResponseTurnDetection>,
}
#[doc = "A summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ReasoningSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}
#[doc = "**Deprecated:** use `summary` instead.\n\nA summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ReasoningGenerateSummary {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "concise")]
    Concise,
    #[serde(rename = "detailed")]
    Detailed,
}
#[doc = "**o-series models only**\n\nConfiguration options for \n[reasoning models](https://platform.openai.com/docs/guides/reasoning).\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct Reasoning {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "effort")]
    pub effort: Option<ReasoningEffort>,
    #[doc = "A summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "summary")]
    pub summary: Option<ReasoningSummary>,
    #[doc = "**Deprecated:** use `summary` instead.\n\nA summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "generate_summary")]
    pub generate_summary: Option<ReasoningGenerateSummary>,
}
#[doc = "**o-series models only** \n\nConstrains effort on reasoning for \n[reasoning models](https://platform.openai.com/docs/guides/reasoning).\nCurrently supported values are `low`, `medium`, and `high`. Reducing\nreasoning effort can result in faster responses and fewer tokens used\non reasoning in a response.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ReasoningEffort {
    #[serde(rename = "low")]
    Low,
    #[default]
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
#[doc = "The type of the object. Always `reasoning`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ReasoningItemType {
    #[default]
    #[serde(rename = "reasoning")]
    Reasoning,
}
#[doc = "The type of the object. Always `summary_text`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ReasoningItemSummaryType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ReasoningItemSummary {
    #[doc = "The type of the object. Always `summary_text`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ReasoningItemSummaryType,
    #[doc = "A short summary of the reasoning used by the model when generating\nthe response.\n"]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ReasoningItemStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "A description of the chain of thought used by a reasoning model while generating\na response.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ReasoningItem {
    #[doc = "The type of the object. Always `reasoning`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ReasoningItemType,
    #[doc = "The unique identifier of the reasoning content.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Reasoning text contents.\n"]
    #[serde(rename = "summary")]
    pub summary: Vec<ReasoningItemSummary>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ReasoningItemStatus>,
}
#[doc = "The object type of this resource - always set to `response`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseObject {
    #[default]
    #[serde(rename = "response")]
    Response,
}
#[doc = "The status of the response generation. One of `completed`, `failed`, \n`in_progress`, or `incomplete`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The reason why the response is incomplete."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ResponseIncompleteDetailsReason {
    #[serde(rename = "max_output_tokens")]
    MaxOutputTokens,
    #[serde(rename = "content_filter")]
    ContentFilter,
}
#[doc = "Details about why the response is incomplete.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseIncompleteDetails {
    #[doc = "The reason why the response is incomplete."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reason")]
    pub reason: Option<ResponseIncompleteDetailsReason>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Response {
    #[builder(default)]
    #[serde(flatten)]
    pub model_response_properties: ModelResponseProperties,
    #[builder(default)]
    #[serde(flatten)]
    pub response_properties: ResponseProperties,
    #[doc = "Unique identifier for this Response.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type of this resource - always set to `response`.\n"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ResponseObject,
    #[doc = "The status of the response generation. One of `completed`, `failed`, \n`in_progress`, or `incomplete`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ResponseStatus>,
    #[doc = "Unix timestamp (in seconds) of when this Response was created.\n"]
    #[serde(rename = "created_at")]
    pub created_at: f64,
    #[serde(rename = "error")]
    pub error: ResponseError,
    #[doc = "Details about why the response is incomplete.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: Option<ResponseIncompleteDetails>,
    #[doc = "An array of content items generated by the model.\n\n- The length and order of items in the `output` array is dependent\n  on the model's response.\n- Rather than accessing the first item in the `output` array and \n  assuming it's an `assistant` message with the content generated by\n  the model, you might consider using the `output_text` property where\n  supported in SDKs.\n"]
    #[serde(rename = "output")]
    pub output: Vec<OutputItem>,
    #[doc = "SDK-only convenience property that contains the aggregated text output \nfrom all `output_text` items in the `output` array, if any are present. \nSupported in the Python and JavaScript SDKs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_text")]
    pub output_text: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "usage")]
    pub usage: Option<ResponseUsage>,
    #[doc = "Whether to allow the model to run tool calls in parallel.\n"]
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: bool,
}
#[doc = "The type of the event. Always `response.audio.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseAudioDeltaEventType {
    #[default]
    #[serde(rename = "response.audio.delta")]
    ResponseAudioDelta,
}
#[doc = "Emitted when there is a partial audio response."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseAudioDeltaEvent {
    #[doc = "The type of the event. Always `response.audio.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseAudioDeltaEventType,
    #[doc = "A chunk of Base64 encoded response audio bytes.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.audio.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseAudioDoneEventType {
    #[default]
    #[serde(rename = "response.audio.done")]
    ResponseAudioDone,
}
#[doc = "Emitted when the audio response is complete."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseAudioDoneEvent {
    #[doc = "The type of the event. Always `response.audio.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseAudioDoneEventType,
}
#[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseAudioTranscriptDeltaEventType {
    #[default]
    #[serde(rename = "response.audio.transcript.delta")]
    ResponseAudioTranscriptDelta,
}
#[doc = "Emitted when there is a partial transcript of audio."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseAudioTranscriptDeltaEvent {
    #[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseAudioTranscriptDeltaEventType,
    #[doc = "The partial transcript of the audio response.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseAudioTranscriptDoneEventType {
    #[default]
    #[serde(rename = "response.audio.transcript.done")]
    ResponseAudioTranscriptDone,
}
#[doc = "Emitted when the full audio transcript is completed."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseAudioTranscriptDoneEvent {
    #[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseAudioTranscriptDoneEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.code.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCodeInterpreterCallCodeDeltaEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.code.delta")]
    ResponseCodeInterpreterCallCodeDelta,
}
#[doc = "Emitted when a partial code snippet is added by the code interpreter."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    #[doc = "The type of the event. Always `response.code_interpreter_call.code.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCodeDeltaEventType,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The partial code snippet added by the code interpreter.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.code.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCodeInterpreterCallCodeDoneEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.code.done")]
    ResponseCodeInterpreterCallCodeDone,
}
#[doc = "Emitted when code snippet output is finalized by the code interpreter."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    #[doc = "The type of the event. Always `response.code_interpreter_call.code.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCodeDoneEventType,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The final code snippet output by the code interpreter.\n"]
    #[serde(rename = "code")]
    pub code: String,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.completed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCodeInterpreterCallCompletedEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.completed")]
    ResponseCodeInterpreterCallCompleted,
}
#[doc = "Emitted when the code interpreter call is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    #[doc = "The type of the event. Always `response.code_interpreter_call.completed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallCompletedEventType,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[serde(rename = "code_interpreter_call")]
    pub code_interpreter_call: CodeInterpreterToolCall,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCodeInterpreterCallInProgressEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.in_progress")]
    ResponseCodeInterpreterCallInProgress,
}
#[doc = "Emitted when a code interpreter call is in progress."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    #[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallInProgressEventType,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[serde(rename = "code_interpreter_call")]
    pub code_interpreter_call: CodeInterpreterToolCall,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCodeInterpreterCallInterpretingEventType {
    #[default]
    #[serde(rename = "response.code_interpreter_call.interpreting")]
    ResponseCodeInterpreterCallInterpreting,
}
#[doc = "Emitted when the code interpreter is actively interpreting the code snippet."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    #[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCodeInterpreterCallInterpretingEventType,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[serde(rename = "code_interpreter_call")]
    pub code_interpreter_call: CodeInterpreterToolCall,
}
#[doc = "The type of the event. Always `response.completed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCompletedEventType {
    #[default]
    #[serde(rename = "response.completed")]
    ResponseCompleted,
}
#[doc = "Emitted when the model response is complete."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCompletedEvent {
    #[doc = "The type of the event. Always `response.completed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCompletedEventType,
    #[doc = "Properties of the completed response.\n"]
    #[serde(rename = "response")]
    pub response: Response,
}
#[doc = "The type of the event. Always `response.content_part.added`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseContentPartAddedEventType {
    #[default]
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded,
}
#[doc = "Emitted when a new content part is added."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseContentPartAddedEvent {
    #[doc = "The type of the event. Always `response.content_part.added`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseContentPartAddedEventType,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that was added.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The content part that was added.\n"]
    #[serde(rename = "part")]
    pub part: OutputContent,
}
#[doc = "The type of the event. Always `response.content_part.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseContentPartDoneEventType {
    #[default]
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone,
}
#[doc = "Emitted when a content part is done."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseContentPartDoneEvent {
    #[doc = "The type of the event. Always `response.content_part.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseContentPartDoneEventType,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that is done.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The content part that is done.\n"]
    #[serde(rename = "part")]
    pub part: OutputContent,
}
#[doc = "The type of the event. Always `response.created`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseCreatedEventType {
    #[default]
    #[serde(rename = "response.created")]
    ResponseCreated,
}
#[doc = "An event that is emitted when a response is created.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseCreatedEvent {
    #[doc = "The type of the event. Always `response.created`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseCreatedEventType,
    #[doc = "The response that was created.\n"]
    #[serde(rename = "response")]
    pub response: Response,
}
#[doc = "An error object returned when the model fails to generate a Response.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseError {
    #[serde(rename = "code")]
    pub code: ResponseErrorCode,
    #[doc = "A human-readable description of the error.\n"]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "The error code for the response.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ResponseErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
    #[serde(rename = "vector_store_timeout")]
    VectorStoreTimeout,
    #[serde(rename = "invalid_image")]
    InvalidImage,
    #[serde(rename = "invalid_image_format")]
    InvalidImageFormat,
    #[serde(rename = "invalid_base64_image")]
    InvalidBase64Image,
    #[serde(rename = "invalid_image_url")]
    InvalidImageUrl,
    #[serde(rename = "image_too_large")]
    ImageTooLarge,
    #[serde(rename = "image_too_small")]
    ImageTooSmall,
    #[serde(rename = "image_parse_error")]
    ImageParseError,
    #[serde(rename = "image_content_policy_violation")]
    ImageContentPolicyViolation,
    #[serde(rename = "invalid_image_mode")]
    InvalidImageMode,
    #[serde(rename = "image_file_too_large")]
    ImageFileTooLarge,
    #[serde(rename = "unsupported_image_media_type")]
    UnsupportedImageMediaType,
    #[serde(rename = "empty_image_file")]
    EmptyImageFile,
    #[serde(rename = "failed_to_download_image")]
    FailedToDownloadImage,
    #[serde(rename = "image_file_not_found")]
    ImageFileNotFound,
}
#[doc = "The type of the event. Always `error`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseErrorEventType {
    #[default]
    #[serde(rename = "error")]
    Error,
}
#[doc = "Emitted when an error occurs."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseErrorEvent {
    #[doc = "The type of the event. Always `error`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseErrorEventType,
    #[doc = "The error code.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "The error message.\n"]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "The error parameter.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "param")]
    pub param: Option<String>,
}
#[doc = "The type of the event. Always `response.failed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFailedEventType {
    #[default]
    #[serde(rename = "response.failed")]
    ResponseFailed,
}
#[doc = "An event that is emitted when a response fails.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFailedEvent {
    #[doc = "The type of the event. Always `response.failed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFailedEventType,
    #[doc = "The response that failed.\n"]
    #[serde(rename = "response")]
    pub response: Response,
}
#[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFileSearchCallCompletedEventType {
    #[default]
    #[serde(rename = "response.file_search_call.completed")]
    ResponseFileSearchCallCompleted,
}
#[doc = "Emitted when a file search call is completed (results found)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFileSearchCallCompletedEvent {
    #[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallCompletedEventType,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFileSearchCallInProgressEventType {
    #[default]
    #[serde(rename = "response.file_search_call.in_progress")]
    ResponseFileSearchCallInProgress,
}
#[doc = "Emitted when a file search call is initiated."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFileSearchCallInProgressEvent {
    #[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallInProgressEventType,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFileSearchCallSearchingEventType {
    #[default]
    #[serde(rename = "response.file_search_call.searching")]
    ResponseFileSearchCallSearching,
}
#[doc = "Emitted when a file search is currently searching."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFileSearchCallSearchingEvent {
    #[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFileSearchCallSearchingEventType,
    #[doc = "The index of the output item that the file search call is searching.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The type of response format being defined. Always `json_object`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFormatJsonObjectType {
    #[default]
    #[serde(rename = "json_object")]
    JsonObject,
}
#[doc = "JSON object response format. An older method of generating JSON responses.\nUsing `json_schema` is recommended for models that support it. Note that the\nmodel will not generate JSON without a system or user message instructing it\nto do so.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseFormatJsonObject {
    #[doc = "The type of response format being defined. Always `json_object`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonObjectType,
}
#[doc = "The type of response format being defined. Always `json_schema`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFormatJsonSchemaType {
    #[default]
    #[serde(rename = "json_schema")]
    JsonSchema,
}
#[doc = "Structured Outputs configuration options, including a JSON Schema.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFormatJsonSchemaJsonSchema {
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    #[serde(rename = "name")]
    pub name: String,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "schema")]
    pub schema: Option<ResponseFormatJsonSchemaSchema>,
    #[doc = "Whether to enable strict schema adherence when generating the output.\nIf set to true, the model will always follow the exact schema defined\nin the `schema` field. Only a subset of JSON Schema is supported when\n`strict` is `true`. To learn more, read the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "strict")]
    pub strict: Option<bool>,
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](/docs/guides/structured-outputs).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFormatJsonSchema {
    #[doc = "The type of response format being defined. Always `json_schema`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFormatJsonSchemaType,
    #[doc = "Structured Outputs configuration options, including a JSON Schema.\n"]
    #[serde(rename = "json_schema")]
    pub json_schema: ResponseFormatJsonSchemaJsonSchema,
}
pub type ResponseFormatJsonSchemaSchema = std::collections::HashMap<String, serde_json::Value>;
#[doc = "The type of response format being defined. Always `text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFormatTextType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[doc = "Default response format. Used to generate text responses.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseFormatText {
    #[doc = "The type of response format being defined. Always `text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFormatTextType,
}
#[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFunctionCallArgumentsDeltaEventType {
    #[default]
    #[serde(rename = "response.function_call_arguments.delta")]
    ResponseFunctionCallArgumentsDelta,
}
#[doc = "Emitted when there is a partial function-call arguments delta."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    #[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFunctionCallArgumentsDeltaEventType,
    #[doc = "The ID of the output item that the function-call arguments delta is added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the function-call arguments delta is added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The function-call arguments delta that is added.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseFunctionCallArgumentsDoneEventType {
    #[default]
    #[serde(rename = "response.function_call_arguments.done")]
    ResponseFunctionCallArgumentsDone,
}
#[doc = "Emitted when function-call arguments are finalized."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseFunctionCallArgumentsDoneEventType,
    #[doc = "The ID of the item."]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item."]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The function-call arguments."]
    #[serde(rename = "arguments")]
    pub arguments: String,
}
#[doc = "The type of the event. Always `response.in_progress`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseInProgressEventType {
    #[default]
    #[serde(rename = "response.in_progress")]
    ResponseInProgress,
}
#[doc = "Emitted when the response is in progress."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseInProgressEvent {
    #[doc = "The type of the event. Always `response.in_progress`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseInProgressEventType,
    #[doc = "The response that is in progress.\n"]
    #[serde(rename = "response")]
    pub response: Response,
}
#[doc = "The type of the event. Always `response.incomplete`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseIncompleteEventType {
    #[default]
    #[serde(rename = "response.incomplete")]
    ResponseIncomplete,
}
#[doc = "An event that is emitted when a response finishes as incomplete.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseIncompleteEvent {
    #[doc = "The type of the event. Always `response.incomplete`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseIncompleteEventType,
    #[doc = "The response that was incomplete.\n"]
    #[serde(rename = "response")]
    pub response: Response,
}
#[doc = "The type of object returned, must be `list`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseItemListObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[doc = "A list of Response items."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseItemList {
    #[doc = "The type of object returned, must be `list`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ResponseItemListObject,
    #[doc = "A list of items used to generate this response."]
    #[serde(rename = "data")]
    pub data: Vec<ItemResource>,
    #[doc = "Whether there are more items available."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[doc = "The ID of the first item in the list."]
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[doc = "The ID of the last item in the list."]
    #[serde(rename = "last_id")]
    pub last_id: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ResponseModality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "audio")]
    Audio,
}
pub type ResponseModalities = Vec<ResponseModality>;
#[doc = "The type of the event. Always `response.output_item.added`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseOutputItemAddedEventType {
    #[default]
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded,
}
#[doc = "Emitted when a new output item is added."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseOutputItemAddedEvent {
    #[doc = "The type of the event. Always `response.output_item.added`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseOutputItemAddedEventType,
    #[doc = "The index of the output item that was added.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The output item that was added.\n"]
    #[serde(rename = "item")]
    pub item: OutputItem,
}
#[doc = "The type of the event. Always `response.output_item.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseOutputItemDoneEventType {
    #[default]
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone,
}
#[doc = "Emitted when an output item is marked done."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseOutputItemDoneEvent {
    #[doc = "The type of the event. Always `response.output_item.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseOutputItemDoneEventType,
    #[doc = "The index of the output item that was marked done.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The output item that was marked done.\n"]
    #[serde(rename = "item")]
    pub item: OutputItem,
}
#[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Structured Outputs](/docs/guides/structured-outputs)\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponsePropertiesText {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "format")]
    pub format: Option<TextResponseFormatConfiguration>,
}
#[doc = "How the model should select which tool (or tools) to use when generating\na response. See the `tools` parameter to see how to specify which tools\nthe model can call.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ResponsePropertiesToolChoice {
    ToolChoiceOptions(ToolChoiceOptions),
    ToolChoiceTypes(ToolChoiceTypes),
    ToolChoiceFunction(ToolChoiceFunction),
}
#[doc = "The truncation strategy to use for the model response.\n- `auto`: If the context of this response and previous ones exceeds\n  the model's context window size, the model will truncate the \n  response to fit the context window by dropping input items in the\n  middle of the conversation. \n- `disabled` (default): If a model response will exceed the context window \n  size for a model, the request will fail with a 400 error.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponsePropertiesTruncation {
    #[serde(rename = "auto")]
    Auto,
    #[default]
    #[serde(rename = "disabled")]
    Disabled,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ResponseProperties {
    #[doc = "The unique ID of the previous response to the model. Use this to\ncreate multi-turn conversations. Learn more about \n[conversation state](/docs/guides/conversation-state).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "previous_response_id")]
    pub previous_response_id: Option<String>,
    #[doc = "Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](/docs/models)\nto browse and compare available models.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<ModelIdsResponses>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reasoning")]
    pub reasoning: Option<Reasoning>,
    #[doc = "An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_output_tokens")]
    pub max_output_tokens: Option<u64>,
    #[doc = "Inserts a system (or developer) message as the first item in the model's context.\n\nWhen using along with `previous_response_id`, the instructions from a previous\nresponse will not be carried over to the next response. This makes it simple\nto swap out system (or developer) messages in new responses.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "instructions")]
    pub instructions: Option<String>,
    #[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Structured Outputs](/docs/guides/structured-outputs)\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<ResponsePropertiesText>,
    #[doc = "An array of tools the model may call while generating a response. You \ncan specify which tool to use by setting the `tool_choice` parameter.\n\nThe two categories of tools you can provide the model are:\n\n- **Built-in tools**: Tools that are provided by OpenAI that extend the\n  model's capabilities, like [web search](/docs/guides/tools-web-search)\n  or [file search](/docs/guides/tools-file-search). Learn more about\n  [built-in tools](/docs/guides/tools).\n- **Function calls (custom tools)**: Functions that are defined by you,\n  enabling the model to call your own code. Learn more about\n  [function calling](/docs/guides/function-calling).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tools")]
    pub tools: Option<Vec<Tool>>,
    #[doc = "How the model should select which tool (or tools) to use when generating\na response. See the `tools` parameter to see how to specify which tools\nthe model can call.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<ResponsePropertiesToolChoice>,
    #[doc = "The truncation strategy to use for the model response.\n- `auto`: If the context of this response and previous ones exceeds\n  the model's context window size, the model will truncate the \n  response to fit the context window by dropping input items in the\n  middle of the conversation. \n- `disabled` (default): If a model response will exceed the context window \n  size for a model, the request will fail with a 400 error.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "truncation")]
    pub truncation: Option<ResponsePropertiesTruncation>,
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryPartAddedEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_part.added")]
    ResponseReasoningSummaryPartAdded,
}
#[doc = "The type of the summary part. Always `summary_text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryPartAddedEventPartType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}
#[doc = "The summary part that was added.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryPartAddedEventPart {
    #[doc = "The type of the summary part. Always `summary_text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartAddedEventPartType,
    #[doc = "The text of the summary part."]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "Emitted when a new reasoning summary part is added."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryPartAddedEvent {
    #[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartAddedEventType,
    #[doc = "The ID of the item this summary part is associated with.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    #[serde(rename = "summary_index")]
    pub summary_index: u64,
    #[doc = "The summary part that was added.\n"]
    #[serde(rename = "part")]
    pub part: ResponseReasoningSummaryPartAddedEventPart,
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryPartDoneEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_part.done")]
    ResponseReasoningSummaryPartDone,
}
#[doc = "The type of the summary part. Always `summary_text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryPartDoneEventPartType {
    #[default]
    #[serde(rename = "summary_text")]
    SummaryText,
}
#[doc = "The completed summary part.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryPartDoneEventPart {
    #[doc = "The type of the summary part. Always `summary_text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartDoneEventPartType,
    #[doc = "The text of the summary part."]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "Emitted when a reasoning summary part is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryPartDoneEvent {
    #[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryPartDoneEventType,
    #[doc = "The ID of the item this summary part is associated with.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    #[serde(rename = "summary_index")]
    pub summary_index: u64,
    #[doc = "The completed summary part.\n"]
    #[serde(rename = "part")]
    pub part: ResponseReasoningSummaryPartDoneEventPart,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryTextDeltaEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_text.delta")]
    ResponseReasoningSummaryTextDelta,
}
#[doc = "Emitted when a delta is added to a reasoning summary text."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    #[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryTextDeltaEventType,
    #[doc = "The ID of the item this summary text delta is associated with.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item this summary text delta is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    #[serde(rename = "summary_index")]
    pub summary_index: u64,
    #[doc = "The text delta that was added to the summary.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseReasoningSummaryTextDoneEventType {
    #[default]
    #[serde(rename = "response.reasoning_summary_text.done")]
    ResponseReasoningSummaryTextDone,
}
#[doc = "Emitted when a reasoning summary text is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseReasoningSummaryTextDoneEvent {
    #[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseReasoningSummaryTextDoneEventType,
    #[doc = "The ID of the item this summary text is associated with.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item this summary text is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    #[serde(rename = "summary_index")]
    pub summary_index: u64,
    #[doc = "The full text of the completed reasoning summary.\n"]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The type of the event. Always `response.refusal.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseRefusalDeltaEventType {
    #[default]
    #[serde(rename = "response.refusal.delta")]
    ResponseRefusalDelta,
}
#[doc = "Emitted when there is a partial refusal text."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseRefusalDeltaEvent {
    #[doc = "The type of the event. Always `response.refusal.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseRefusalDeltaEventType,
    #[doc = "The ID of the output item that the refusal text is added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the refusal text is added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that the refusal text is added to.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The refusal text that is added.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.refusal.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseRefusalDoneEventType {
    #[default]
    #[serde(rename = "response.refusal.done")]
    ResponseRefusalDone,
}
#[doc = "Emitted when refusal text is finalized."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseRefusalDoneEvent {
    #[doc = "The type of the event. Always `response.refusal.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseRefusalDoneEventType,
    #[doc = "The ID of the output item that the refusal text is finalized.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the refusal text is finalized.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that the refusal text is finalized.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The refusal text that is finalized.\n"]
    #[serde(rename = "refusal")]
    pub refusal: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ResponseStreamEvent {
    ResponseAudioDelta(ResponseAudioDeltaEvent),
    ResponseAudioDone(ResponseAudioDoneEvent),
    ResponseAudioTranscriptDelta(ResponseAudioTranscriptDeltaEvent),
    ResponseAudioTranscriptDone(ResponseAudioTranscriptDoneEvent),
    ResponseCodeInterpreterCallCodeDelta(ResponseCodeInterpreterCallCodeDeltaEvent),
    ResponseCodeInterpreterCallCodeDone(ResponseCodeInterpreterCallCodeDoneEvent),
    ResponseCodeInterpreterCallCompleted(ResponseCodeInterpreterCallCompletedEvent),
    ResponseCodeInterpreterCallInProgress(ResponseCodeInterpreterCallInProgressEvent),
    ResponseCodeInterpreterCallInterpreting(ResponseCodeInterpreterCallInterpretingEvent),
    ResponseCompleted(ResponseCompletedEvent),
    ResponseContentPartAdded(ResponseContentPartAddedEvent),
    ResponseContentPartDone(ResponseContentPartDoneEvent),
    ResponseCreated(ResponseCreatedEvent),
    Error(ResponseErrorEvent),
    ResponseFileSearchCallCompleted(ResponseFileSearchCallCompletedEvent),
    ResponseFileSearchCallInProgress(ResponseFileSearchCallInProgressEvent),
    ResponseFileSearchCallSearching(ResponseFileSearchCallSearchingEvent),
    ResponseFunctionCallArgumentsDelta(ResponseFunctionCallArgumentsDeltaEvent),
    ResponseFunctionCallArgumentsDone(ResponseFunctionCallArgumentsDoneEvent),
    ResponseInProgress(ResponseInProgressEvent),
    ResponseFailed(ResponseFailedEvent),
    ResponseIncomplete(ResponseIncompleteEvent),
    ResponseOutputItemAdded(ResponseOutputItemAddedEvent),
    ResponseOutputItemDone(ResponseOutputItemDoneEvent),
    ResponseReasoningSummaryPartAdded(ResponseReasoningSummaryPartAddedEvent),
    ResponseReasoningSummaryPartDone(ResponseReasoningSummaryPartDoneEvent),
    ResponseReasoningSummaryTextDelta(ResponseReasoningSummaryTextDeltaEvent),
    ResponseReasoningSummaryTextDone(ResponseReasoningSummaryTextDoneEvent),
    ResponseRefusalDelta(ResponseRefusalDeltaEvent),
    ResponseRefusalDone(ResponseRefusalDoneEvent),
    ResponseOutputTextAnnotationAdded(ResponseTextAnnotationDeltaEvent),
    ResponseOutputTextDelta(ResponseTextDeltaEvent),
    ResponseOutputTextDone(ResponseTextDoneEvent),
    ResponseWebSearchCallCompleted(ResponseWebSearchCallCompletedEvent),
    ResponseWebSearchCallInProgress(ResponseWebSearchCallInProgressEvent),
    ResponseWebSearchCallSearching(ResponseWebSearchCallSearchingEvent),
}
#[doc = "The type of the event. Always `response.output_text.annotation.added`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseTextAnnotationDeltaEventType {
    #[default]
    #[serde(rename = "response.output_text.annotation.added")]
    ResponseOutputTextAnnotationAdded,
}
#[doc = "Emitted when a text annotation is added."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseTextAnnotationDeltaEvent {
    #[doc = "The type of the event. Always `response.output_text.annotation.added`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseTextAnnotationDeltaEventType,
    #[doc = "The ID of the output item that the text annotation was added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the text annotation was added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that the text annotation was added to.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The index of the annotation that was added.\n"]
    #[serde(rename = "annotation_index")]
    pub annotation_index: u64,
    #[serde(rename = "annotation")]
    pub annotation: Annotation,
}
#[doc = "The type of the event. Always `response.output_text.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseTextDeltaEventType {
    #[default]
    #[serde(rename = "response.output_text.delta")]
    ResponseOutputTextDelta,
}
#[doc = "Emitted when there is an additional text delta."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseTextDeltaEvent {
    #[doc = "The type of the event. Always `response.output_text.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseTextDeltaEventType,
    #[doc = "The ID of the output item that the text delta was added to.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the text delta was added to.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that the text delta was added to.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The text delta that was added.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
}
#[doc = "The type of the event. Always `response.output_text.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseTextDoneEventType {
    #[default]
    #[serde(rename = "response.output_text.done")]
    ResponseOutputTextDone,
}
#[doc = "Emitted when text content is finalized."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseTextDoneEvent {
    #[doc = "The type of the event. Always `response.output_text.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseTextDoneEventType,
    #[doc = "The ID of the output item that the text content is finalized.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
    #[doc = "The index of the output item that the text content is finalized.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "The index of the content part that the text content is finalized.\n"]
    #[serde(rename = "content_index")]
    pub content_index: u64,
    #[doc = "The text content that is finalized.\n"]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "A detailed breakdown of the input tokens."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseUsageInputTokensDetails {
    #[doc = "The number of tokens that were retrieved from the cache. \n[More on prompt caching](/docs/guides/prompt-caching).\n"]
    #[serde(rename = "cached_tokens")]
    pub cached_tokens: u64,
}
#[doc = "A detailed breakdown of the output tokens."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseUsageOutputTokensDetails {
    #[doc = "The number of reasoning tokens."]
    #[serde(rename = "reasoning_tokens")]
    pub reasoning_tokens: u64,
}
#[doc = "Represents token usage details including input tokens, output tokens,\na breakdown of output tokens, and the total tokens used.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseUsage {
    #[doc = "The number of input tokens."]
    #[serde(rename = "input_tokens")]
    pub input_tokens: u64,
    #[doc = "A detailed breakdown of the input tokens."]
    #[serde(rename = "input_tokens_details")]
    pub input_tokens_details: ResponseUsageInputTokensDetails,
    #[doc = "The number of output tokens."]
    #[serde(rename = "output_tokens")]
    pub output_tokens: u64,
    #[doc = "A detailed breakdown of the output tokens."]
    #[serde(rename = "output_tokens_details")]
    pub output_tokens_details: ResponseUsageOutputTokensDetails,
    #[doc = "The total number of tokens used."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
}
#[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseWebSearchCallCompletedEventType {
    #[default]
    #[serde(rename = "response.web_search_call.completed")]
    ResponseWebSearchCallCompleted,
}
#[doc = "Emitted when a web search call is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseWebSearchCallCompletedEvent {
    #[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallCompletedEventType,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseWebSearchCallInProgressEventType {
    #[default]
    #[serde(rename = "response.web_search_call.in_progress")]
    ResponseWebSearchCallInProgress,
}
#[doc = "Emitted when a web search call is initiated."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseWebSearchCallInProgressEvent {
    #[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallInProgressEventType,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ResponseWebSearchCallSearchingEventType {
    #[default]
    #[serde(rename = "response.web_search_call.searching")]
    ResponseWebSearchCallSearching,
}
#[doc = "Emitted when a web search call is executing."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ResponseWebSearchCallSearchingEvent {
    #[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ResponseWebSearchCallSearchingEventType,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    #[serde(rename = "output_index")]
    pub output_index: u64,
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    #[serde(rename = "item_id")]
    pub item_id: String,
}
#[doc = "Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunCompletionUsage {
    #[doc = "Number of completion tokens used over the course of the run."]
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[doc = "Number of prompt tokens used over the course of the run."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
}
#[doc = "The object type, which is always `thread.run`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunObjectObject {
    #[default]
    #[serde(rename = "thread.run")]
    ThreadRun,
}
#[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunObjectStatus {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "requires_action")]
    RequiresAction,
    #[serde(rename = "cancelling")]
    Cancelling,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
    #[serde(rename = "expired")]
    Expired,
}
#[doc = "For now, this is always `submit_tool_outputs`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunObjectRequiredActionType {
    #[default]
    #[serde(rename = "submit_tool_outputs")]
    SubmitToolOutputs,
}
#[doc = "Details on the tool outputs needed for this run to continue."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunObjectRequiredActionSubmitToolOutputs {
    #[doc = "A list of the relevant tool calls."]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Vec<RunToolCallObject>,
}
#[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunObjectRequiredAction {
    #[doc = "For now, this is always `submit_tool_outputs`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunObjectRequiredActionType,
    #[doc = "Details on the tool outputs needed for this run to continue."]
    #[serde(rename = "submit_tool_outputs")]
    pub submit_tool_outputs: RunObjectRequiredActionSubmitToolOutputs,
}
#[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
}
#[doc = "The last error associated with this run. Will be `null` if there are no errors."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunObjectLastError {
    #[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
    #[serde(rename = "code")]
    pub code: RunObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunObjectIncompleteDetailsReason {
    #[serde(rename = "max_completion_tokens")]
    MaxCompletionTokens,
    #[serde(rename = "max_prompt_tokens")]
    MaxPromptTokens,
}
#[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunObjectIncompleteDetails {
    #[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "reason")]
    pub reason: Option<RunObjectIncompleteDetailsReason>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunObjectTool {
    CodeInterpreter(AssistantToolsCode),
    FileSearch(AssistantToolsFileSearch),
    Function(AssistantToolsFunction),
}
#[doc = "Represents an execution run on a [thread](/docs/api-reference/threads)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread.run`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: RunObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the run was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The ID of the [thread](/docs/api-reference/threads) that was executed on as a part of this run."]
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) used for execution of this run."]
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    #[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
    #[serde(rename = "status")]
    pub status: RunObjectStatus,
    #[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "required_action")]
    pub required_action: Option<RunObjectRequiredAction>,
    #[doc = "The last error associated with this run. Will be `null` if there are no errors."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_error")]
    pub last_error: Option<RunObjectLastError>,
    #[doc = "The Unix timestamp (in seconds) for when the run will expire."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run was started."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "started_at")]
    pub started_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run was cancelled."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run failed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failed_at")]
    pub failed_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run was completed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completed_at")]
    pub completed_at: Option<u64>,
    #[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "incomplete_details")]
    pub incomplete_details: Option<RunObjectIncompleteDetails>,
    #[doc = "The model that the [assistant](/docs/api-reference/assistants) used for this run."]
    #[serde(rename = "model")]
    pub model: String,
    #[doc = "The instructions that the [assistant](/docs/api-reference/assistants) used for this run."]
    #[serde(rename = "instructions")]
    pub instructions: String,
    #[doc = "The list of tools that the [assistant](/docs/api-reference/assistants) used for this run."]
    #[serde(rename = "tools")]
    pub tools: Vec<RunObjectTool>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "usage")]
    pub usage: RunCompletionUsage,
    #[doc = "The sampling temperature used for this run. If not set, defaults to 1."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "temperature")]
    pub temperature: Option<f64>,
    #[doc = "The nucleus sampling value used for this run. If not set, defaults to 1."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "top_p")]
    pub top_p: Option<f64>,
    #[doc = "The maximum number of prompt tokens specified to have been used over the course of the run.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_prompt_tokens")]
    pub max_prompt_tokens: Option<u64>,
    #[doc = "The maximum number of completion tokens specified to have been used over the course of the run.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_completion_tokens")]
    pub max_completion_tokens: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "truncation_strategy")]
    pub truncation_strategy: Option<TruncationObject>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_choice")]
    pub tool_choice: Option<AssistantsApiToolChoiceOption>,
    #[serde(rename = "parallel_tool_calls")]
    pub parallel_tool_calls: ParallelToolCalls,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub response_format: Option<AssistantsApiResponseFormatOption>,
}
#[doc = "Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepCompletionUsage {
    #[doc = "Number of completion tokens used over the course of the run step."]
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[doc = "Number of prompt tokens used over the course of the run step."]
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
}
#[doc = "The object type, which is always `thread.run.step.delta`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaObjectObject {
    #[default]
    #[serde(rename = "thread.run.step.delta")]
    ThreadRunStepDelta,
}
#[doc = "The details of the run step."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepDeltaObjectDeltaStepDetails {
    MessageCreation(RunStepDeltaStepDetailsMessageCreationObject),
    ToolCalls(RunStepDeltaStepDetailsToolCallsObject),
}
#[doc = "The delta containing the fields that have changed on the run step."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaObjectDelta {
    #[doc = "The details of the run step."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "step_details")]
    pub step_details: Option<RunStepDeltaObjectDeltaStepDetails>,
}
#[doc = "Represents a run step delta i.e. any changed fields on a run step during streaming.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaObject {
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread.run.step.delta`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: RunStepDeltaObjectObject,
    #[doc = "The delta containing the fields that have changed on the run step."]
    #[builder(default)]
    #[serde(rename = "delta")]
    pub delta: RunStepDeltaObjectDelta,
}
#[doc = "Always `message_creation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsMessageCreationObjectType {
    #[default]
    #[serde(rename = "message_creation")]
    MessageCreation,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message_id")]
    pub message_id: Option<String>,
}
#[doc = "Details of the message creation by the run step."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    #[doc = "Always `message_creation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsMessageCreationObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message_creation")]
    pub message_creation: Option<RunStepDeltaStepDetailsMessageCreationObjectMessageCreation>,
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectType {
    #[default]
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs {
    Logs(RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject),
    Image(RunStepDeltaStepDetailsToolCallsCodeOutputImageObject),
}
#[doc = "The Code Interpreter tool call definition."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input")]
    pub input: Option<String>,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "outputs")]
    pub outputs: Option<Vec<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>>,
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
    #[doc = "The index of the tool call in the tool calls array."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "The ID of the tool call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeObjectType,
    #[doc = "The Code Interpreter tool call definition."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter>,
}
#[doc = "Always `image`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    #[default]
    #[serde(rename = "image")]
    Image,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](/docs/api-reference/files) ID of the image."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    #[doc = "The index of the output in the outputs array."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `image`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image")]
    pub image: Option<RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage>,
}
#[doc = "Always `logs`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    #[default]
    #[serde(rename = "logs")]
    Logs,
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "The index of the output in the outputs array."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "Always `logs`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
    #[doc = "The text output from the Code Interpreter tool call."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logs")]
    pub logs: Option<String>,
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    #[doc = "The index of the tool call in the tool calls array."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "The ID of the tool call object."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
    #[doc = "For now, this is always going to be an empty object."]
    #[serde(rename = "file_search")]
    pub file_search: std::collections::HashMap<String, serde_json::Value>,
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "The definition of the function that was called."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The name of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The arguments passed to the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "arguments")]
    pub arguments: Option<String>,
    #[doc = "The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    #[doc = "The index of the tool call in the tool calls array."]
    #[serde(rename = "index")]
    pub index: u64,
    #[doc = "The ID of the tool call object."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsFunctionObjectType,
    #[doc = "The definition of the function that was called."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "function")]
    pub function: Option<RunStepDeltaStepDetailsToolCallsFunctionObjectFunction>,
}
#[doc = "Always `tool_calls`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDeltaStepDetailsToolCallsObjectType {
    #[default]
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepDeltaStepDetailsToolCallsObjectToolCalls {
    CodeInterpreter(RunStepDeltaStepDetailsToolCallsCodeObject),
    FileSearch(RunStepDeltaStepDetailsToolCallsFileSearchObject),
    Function(RunStepDeltaStepDetailsToolCallsFunctionObject),
}
#[doc = "Details of the tool call."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    #[doc = "Always `tool_calls`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDeltaStepDetailsToolCallsObjectType,
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Option<Vec<RunStepDeltaStepDetailsToolCallsObjectToolCalls>>,
}
#[doc = "Always `message_creation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsMessageCreationObjectType {
    #[default]
    #[serde(rename = "message_creation")]
    MessageCreation,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    #[serde(rename = "message_id")]
    pub message_id: String,
}
#[doc = "Details of the message creation by the run step."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsMessageCreationObject {
    #[doc = "Always `message_creation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsMessageCreationObjectType,
    #[serde(rename = "message_creation")]
    pub message_creation: RunStepDetailsMessageCreationObjectMessageCreation,
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsCodeObjectType {
    #[default]
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs {
    Logs(RunStepDetailsToolCallsCodeOutputLogsObject),
    Image(RunStepDetailsToolCallsCodeOutputImageObject),
}
#[doc = "The Code Interpreter tool call definition."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    #[serde(rename = "input")]
    pub input: String,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    #[serde(rename = "outputs")]
    pub outputs: Vec<RunStepDetailsToolCallsCodeObjectCodeInterpreterOutputs>,
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsCodeObject {
    #[doc = "The ID of the tool call."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeObjectType,
    #[doc = "The Code Interpreter tool call definition."]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
}
#[doc = "Always `image`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsCodeOutputImageObjectType {
    #[default]
    #[serde(rename = "image")]
    Image,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](/docs/api-reference/files) ID of the image."]
    #[serde(rename = "file_id")]
    pub file_id: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    #[doc = "Always `image`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeOutputImageObjectType,
    #[serde(rename = "image")]
    pub image: RunStepDetailsToolCallsCodeOutputImageObjectImage,
}
#[doc = "Always `logs`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsCodeOutputLogsObjectType {
    #[default]
    #[serde(rename = "logs")]
    Logs,
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "Always `logs`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsCodeOutputLogsObjectType,
    #[doc = "The text output from the Code Interpreter tool call."]
    #[serde(rename = "logs")]
    pub logs: String,
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsFileSearchObjectType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}
#[doc = "For now, this is always going to be an empty object."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<RunStepDetailsToolCallsFileSearchRankingOptionsObject>,
    #[doc = "The results of the file search."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "results")]
    pub results: Option<Vec<RunStepDetailsToolCallsFileSearchResultObject>>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFileSearchObject {
    #[doc = "The ID of the tool call object."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsFileSearchObjectType,
    #[doc = "For now, this is always going to be an empty object."]
    #[builder(default)]
    #[serde(rename = "file_search")]
    pub file_search: RunStepDetailsToolCallsFileSearchObjectFileSearch,
}
#[doc = "The ranking options for the file search."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    #[serde(rename = "ranker")]
    pub ranker: FileSearchRanker,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    #[serde(rename = "score_threshold")]
    pub score_threshold: f64,
}
#[doc = "The type of the content."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsFileSearchResultObjectContentType {
    #[default]
    #[serde(rename = "text")]
    Text,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFileSearchResultObjectContent {
    #[doc = "The type of the content."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<RunStepDetailsToolCallsFileSearchResultObjectContentType>,
    #[doc = "The text content of the file."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
}
#[doc = "A result instance of the file search."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    #[doc = "The ID of the file that result was found in."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "The name of the file that result was found in."]
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[doc = "The score of the result. All values must be a floating point number between 0 and 1."]
    #[serde(rename = "score")]
    pub score: f64,
    #[doc = "The content of the result that was found. The content is only included if requested via the include query parameter."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "content")]
    pub content: Option<Vec<RunStepDetailsToolCallsFileSearchResultObjectContent>>,
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsFunctionObjectType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "The definition of the function that was called."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The name of the function."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The arguments passed to the function."]
    #[serde(rename = "arguments")]
    pub arguments: String,
    #[doc = "The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsFunctionObject {
    #[doc = "The ID of the tool call object."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsFunctionObjectType,
    #[doc = "The definition of the function that was called."]
    #[serde(rename = "function")]
    pub function: RunStepDetailsToolCallsFunctionObjectFunction,
}
#[doc = "Always `tool_calls`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepDetailsToolCallsObjectType {
    #[default]
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepDetailsToolCallsObjectToolCalls {
    CodeInterpreter(RunStepDetailsToolCallsCodeObject),
    FileSearch(RunStepDetailsToolCallsFileSearchObject),
    Function(RunStepDetailsToolCallsFunctionObject),
}
#[doc = "Details of the tool call."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepDetailsToolCallsObject {
    #[doc = "Always `tool_calls`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunStepDetailsToolCallsObjectType,
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    #[serde(rename = "tool_calls")]
    pub tool_calls: Vec<RunStepDetailsToolCallsObjectToolCalls>,
}
#[doc = "The object type, which is always `thread.run.step`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepObjectObject {
    #[default]
    #[serde(rename = "thread.run.step")]
    ThreadRunStep,
}
#[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunStepObjectType {
    #[serde(rename = "message_creation")]
    MessageCreation,
    #[serde(rename = "tool_calls")]
    ToolCalls,
}
#[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunStepObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "expired")]
    Expired,
}
#[doc = "The details of the run step."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepObjectStepDetails {
    MessageCreation(RunStepDetailsMessageCreationObject),
    ToolCalls(RunStepDetailsToolCallsObject),
}
#[doc = "One of `server_error` or `rate_limit_exceeded`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RunStepObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
}
#[doc = "The last error associated with this run step. Will be `null` if there are no errors."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepObjectLastError {
    #[doc = "One of `server_error` or `rate_limit_exceeded`."]
    #[serde(rename = "code")]
    pub code: RunStepObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "Represents a step in execution of a run.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepObject {
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread.run.step`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: RunStepObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the run step was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) associated with the run step."]
    #[serde(rename = "assistant_id")]
    pub assistant_id: String,
    #[doc = "The ID of the [thread](/docs/api-reference/threads) that was run."]
    #[serde(rename = "thread_id")]
    pub thread_id: String,
    #[doc = "The ID of the [run](/docs/api-reference/runs) that this run step is a part of."]
    #[serde(rename = "run_id")]
    pub run_id: String,
    #[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
    #[serde(rename = "type")]
    pub type_: RunStepObjectType,
    #[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
    #[serde(rename = "status")]
    pub status: RunStepObjectStatus,
    #[doc = "The details of the run step."]
    #[serde(rename = "step_details")]
    pub step_details: RunStepObjectStepDetails,
    #[doc = "The last error associated with this run step. Will be `null` if there are no errors."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_error")]
    pub last_error: Option<RunStepObjectLastError>,
    #[doc = "The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expired_at")]
    pub expired_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run step was cancelled."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "cancelled_at")]
    pub cancelled_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run step failed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "failed_at")]
    pub failed_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the run step completed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "completed_at")]
    pub completed_at: Option<u64>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
    #[serde(rename = "usage")]
    pub usage: RunStepCompletionUsage,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepCreatedEvent {
    #[default]
    #[serde(rename = "thread.run.step.created")]
    ThreadRunStepCreated,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is created."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepCreated {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepCreatedEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepInProgressEvent {
    #[default]
    #[serde(rename = "thread.run.step.in_progress")]
    ThreadRunStepInProgress,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) moves to an `in_progress` state."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepInProgress {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepInProgressEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepDeltaEvent {
    #[default]
    #[serde(rename = "thread.run.step.delta")]
    ThreadRunStepDelta,
}
#[doc = "Occurs when parts of a [run step](/docs/api-reference/run-steps/step-object) are being streamed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepDelta {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepDeltaEvent,
    #[serde(rename = "data")]
    pub data: RunStepDeltaObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepCompletedEvent {
    #[default]
    #[serde(rename = "thread.run.step.completed")]
    ThreadRunStepCompleted,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepCompleted {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepCompletedEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepFailedEvent {
    #[default]
    #[serde(rename = "thread.run.step.failed")]
    ThreadRunStepFailed,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) fails."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepFailed {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepFailedEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepCancelledEvent {
    #[default]
    #[serde(rename = "thread.run.step.cancelled")]
    ThreadRunStepCancelled,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is cancelled."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepCancelled {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepCancelledEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStepStreamEventThreadRunStepExpiredEvent {
    #[default]
    #[serde(rename = "thread.run.step.expired")]
    ThreadRunStepExpired,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) expires."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStepStreamEventThreadRunStepExpired {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStepStreamEventThreadRunStepExpiredEvent,
    #[serde(rename = "data")]
    pub data: RunStepObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStepStreamEvent {
    ThreadRunStepCreated(RunStepStreamEventThreadRunStepCreated),
    ThreadRunStepInProgress(RunStepStreamEventThreadRunStepInProgress),
    ThreadRunStepDelta(RunStepStreamEventThreadRunStepDelta),
    ThreadRunStepCompleted(RunStepStreamEventThreadRunStepCompleted),
    ThreadRunStepFailed(RunStepStreamEventThreadRunStepFailed),
    ThreadRunStepCancelled(RunStepStreamEventThreadRunStepCancelled),
    ThreadRunStepExpired(RunStepStreamEventThreadRunStepExpired),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunCreatedEvent {
    #[default]
    #[serde(rename = "thread.run.created")]
    ThreadRunCreated,
}
#[doc = "Occurs when a new [run](/docs/api-reference/runs/object) is created."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunCreated {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunCreatedEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunQueuedEvent {
    #[default]
    #[serde(rename = "thread.run.queued")]
    ThreadRunQueued,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `queued` status."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunQueued {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunQueuedEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunInProgressEvent {
    #[default]
    #[serde(rename = "thread.run.in_progress")]
    ThreadRunInProgress,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to an `in_progress` status."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunInProgress {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunInProgressEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunRequiresActionEvent {
    #[default]
    #[serde(rename = "thread.run.requires_action")]
    ThreadRunRequiresAction,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `requires_action` status."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunRequiresAction {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunRequiresActionEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunCompletedEvent {
    #[default]
    #[serde(rename = "thread.run.completed")]
    ThreadRunCompleted,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) is completed."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunCompleted {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunCompletedEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunIncompleteEvent {
    #[default]
    #[serde(rename = "thread.run.incomplete")]
    ThreadRunIncomplete,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) ends with status `incomplete`."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunIncomplete {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunIncompleteEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunFailedEvent {
    #[default]
    #[serde(rename = "thread.run.failed")]
    ThreadRunFailed,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) fails."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunFailed {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunFailedEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunCancellingEvent {
    #[default]
    #[serde(rename = "thread.run.cancelling")]
    ThreadRunCancelling,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `cancelling` status."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunCancelling {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunCancellingEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunCancelledEvent {
    #[default]
    #[serde(rename = "thread.run.cancelled")]
    ThreadRunCancelled,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) is cancelled."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunCancelled {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunCancelledEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunStreamEventThreadRunExpiredEvent {
    #[default]
    #[serde(rename = "thread.run.expired")]
    ThreadRunExpired,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) expires."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunStreamEventThreadRunExpired {
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: RunStreamEventThreadRunExpiredEvent,
    #[serde(rename = "data")]
    pub data: RunObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum RunStreamEvent {
    ThreadRunCreated(RunStreamEventThreadRunCreated),
    ThreadRunQueued(RunStreamEventThreadRunQueued),
    ThreadRunInProgress(RunStreamEventThreadRunInProgress),
    ThreadRunRequiresAction(RunStreamEventThreadRunRequiresAction),
    ThreadRunCompleted(RunStreamEventThreadRunCompleted),
    ThreadRunIncomplete(RunStreamEventThreadRunIncomplete),
    ThreadRunFailed(RunStreamEventThreadRunFailed),
    ThreadRunCancelling(RunStreamEventThreadRunCancelling),
    ThreadRunCancelled(RunStreamEventThreadRunCancelled),
    ThreadRunExpired(RunStreamEventThreadRunExpired),
}
#[doc = "The type of tool call the output is required for. For now, this is always `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RunToolCallObjectType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "The function definition."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunToolCallObjectFunction {
    #[doc = "The name of the function."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The arguments that the model expects you to pass to the function."]
    #[serde(rename = "arguments")]
    pub arguments: String,
}
#[doc = "Tool call objects"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RunToolCallObject {
    #[doc = "The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](/docs/api-reference/runs/submitToolOutputs) endpoint."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of tool call the output is required for. For now, this is always `function`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RunToolCallObjectType,
    #[doc = "The function definition."]
    #[serde(rename = "function")]
    pub function: RunToolCallObjectFunction,
}
#[doc = "Specifies the event type. For a screenshot action, this property is \nalways set to `screenshot`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ScreenshotType {
    #[default]
    #[serde(rename = "screenshot")]
    Screenshot,
}
#[doc = "A screenshot action.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct Screenshot {
    #[doc = "Specifies the event type. For a screenshot action, this property is \nalways set to `screenshot`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ScreenshotType,
}
#[doc = "Specifies the event type. For a scroll action, this property is \nalways set to `scroll`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ScrollType {
    #[default]
    #[serde(rename = "scroll")]
    Scroll,
}
#[doc = "A scroll action.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Scroll {
    #[doc = "Specifies the event type. For a scroll action, this property is \nalways set to `scroll`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ScrollType,
    #[doc = "The x-coordinate where the scroll occurred.\n"]
    #[serde(rename = "x")]
    pub x: u64,
    #[doc = "The y-coordinate where the scroll occurred.\n"]
    #[serde(rename = "y")]
    pub y: u64,
    #[doc = "The horizontal scroll distance.\n"]
    #[serde(rename = "scroll_x")]
    pub scroll_x: u64,
    #[doc = "The vertical scroll distance.\n"]
    #[serde(rename = "scroll_y")]
    pub scroll_y: u64,
}
#[doc = "Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:\n  - If set to 'auto', and the Project is Scale tier enabled, the system\n    will utilize scale tier credits until they are exhausted.\n  - If set to 'auto', and the Project is not Scale tier enabled, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.\n  - If set to 'default', the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.\n  - If set to 'flex', the request will be processed with the Flex Processing service tier. [Learn more](/docs/guides/flex-processing).\n  - When not set, the default behavior is 'auto'.\n\n  When this parameter is set, the response body will include the `service_tier` utilized.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ServiceTier {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "flex")]
    Flex,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct StaticChunkingStrategy {
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    #[serde(rename = "max_chunk_size_tokens")]
    pub max_chunk_size_tokens: u64,
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    #[serde(rename = "chunk_overlap_tokens")]
    pub chunk_overlap_tokens: u64,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum StaticChunkingStrategyRequestParamType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[doc = "Customize your own chunking strategy by setting chunk size and chunk overlap."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct StaticChunkingStrategyRequestParam {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: StaticChunkingStrategyRequestParamType,
    #[serde(rename = "static")]
    pub static_: StaticChunkingStrategy,
}
#[doc = "Always `static`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum StaticChunkingStrategyResponseParamType {
    #[default]
    #[serde(rename = "static")]
    Static,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct StaticChunkingStrategyResponseParam {
    #[doc = "Always `static`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: StaticChunkingStrategyResponseParamType,
    #[serde(rename = "static")]
    pub static_: StaticChunkingStrategy,
}
#[doc = "Not supported with latest reasoning models `o3` and `o4-mini`.\n\nUp to 4 sequences where the API will stop generating further tokens. The\nreturned text will not contain the stop sequence.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum StopConfiguration {
    _0(String),
    _1(Vec<String>),
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct SubmitToolOutputsRunRequestToolOutputs {
    #[doc = "The ID of the tool call in the `required_action` object within the run object the output is being submitted for."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_call_id")]
    pub tool_call_id: Option<String>,
    #[doc = "The output of the tool call to be submitted to continue the run."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output")]
    pub output: Option<String>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct SubmitToolOutputsRunRequest {
    #[doc = "A list of tools for which the outputs are being submitted."]
    #[serde(rename = "tool_outputs")]
    pub tool_outputs: Vec<SubmitToolOutputsRunRequestToolOutputs>,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "stream")]
    pub stream: Option<bool>,
}
#[doc = "An object specifying the format that the model must output.\n\nConfiguring `{ \"type\": \"json_schema\" }` enables Structured Outputs, \nwhich ensures the model will match your supplied JSON schema. Learn more in the \n[Structured Outputs guide](/docs/guides/structured-outputs).\n\nThe default format is `{ \"type\": \"text\" }` with no additional options.\n\n**Not recommended for gpt-4o and newer models:**\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum TextResponseFormatConfiguration {
    Text(ResponseFormatText),
    JsonSchema(TextResponseFormatJsonSchema),
    JsonObject(ResponseFormatJsonObject),
}
#[doc = "The type of response format being defined. Always `json_schema`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum TextResponseFormatJsonSchemaType {
    #[default]
    #[serde(rename = "json_schema")]
    JsonSchema,
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](/docs/guides/structured-outputs).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TextResponseFormatJsonSchema {
    #[doc = "The type of response format being defined. Always `json_schema`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: TextResponseFormatJsonSchemaType,
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "schema")]
    pub schema: ResponseFormatJsonSchemaSchema,
    #[doc = "Whether to enable strict schema adherence when generating the output.\nIf set to true, the model will always follow the exact schema defined\nin the `schema` field. Only a subset of JSON Schema is supported when\n`strict` is `true`. To learn more, read the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "strict")]
    pub strict: Option<bool>,
}
#[doc = "The object type, which is always `thread`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ThreadObjectObject {
    #[default]
    #[serde(rename = "thread")]
    Thread,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ThreadObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_ids")]
    pub file_ids: Option<Vec<String>>,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ThreadObjectToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Option<Vec<String>>,
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ThreadObjectToolResources {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code_interpreter")]
    pub code_interpreter: Option<ThreadObjectToolResourcesCodeInterpreter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_search")]
    pub file_search: Option<ThreadObjectToolResourcesFileSearch>,
}
#[doc = "Represents a thread that contains [messages](/docs/api-reference/messages)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ThreadObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `thread`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: ThreadObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the thread was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "tool_resources")]
    pub tool_resources: Option<ThreadObjectToolResources>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ThreadStreamEvent0Event {
    #[default]
    #[serde(rename = "thread.created")]
    ThreadCreated,
}
#[doc = "Occurs when a new [thread](/docs/api-reference/threads/object) is created."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ThreadStreamEvent0 {
    #[doc = "Whether to enable input audio transcription."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[builder(default)]
    #[serde(rename = "event")]
    pub event: ThreadStreamEvent0Event,
    #[serde(rename = "data")]
    pub data: ThreadObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum ThreadStreamEvent {
    _0(ThreadStreamEvent0),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ToggleCertificatesRequest {
    #[serde(rename = "certificate_ids")]
    pub certificate_ids: Vec<String>,
}
#[doc = "For function calling, the type is always `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ToolChoiceFunctionType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "Use this option to force the model to call a specific function.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ToolChoiceFunction {
    #[doc = "For function calling, the type is always `function`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ToolChoiceFunctionType,
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
}
#[doc = "Controls which (if any) tool is called by the model.\n\n`none` means the model will not call any tool and instead generates a message.\n\n`auto` means the model can pick between generating a message or calling one or\nmore tools.\n\n`required` means the model must call one or more tools.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ToolChoiceOptions {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "required")]
    Required,
}
#[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer_use_preview`\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ToolChoiceTypesType {
    #[serde(rename = "file_search")]
    FileSearch,
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
}
#[doc = "Indicates that the model should use a built-in tool to generate a response.\n[Learn more about built-in tools](/docs/guides/tools).\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ToolChoiceTypes {
    #[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer_use_preview`\n"]
    #[serde(rename = "type")]
    pub type_: ToolChoiceTypesType,
}
#[doc = "The type of the event. Always `transcript.text.delta`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum TranscriptTextDeltaEventType {
    #[default]
    #[serde(rename = "transcript.text.delta")]
    TranscriptTextDelta,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct TranscriptTextDeltaEventLogprob {
    #[doc = "The token that was used to generate the log probability.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[doc = "The log probability of the token.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprob")]
    pub logprob: Option<f64>,
    #[doc = "The bytes that were used to generate the log probability.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<serde_json::Value>>,
}
#[doc = "Emitted when there is an additional text delta. This is also the first event emitted when the transcription starts. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TranscriptTextDeltaEvent {
    #[doc = "The type of the event. Always `transcript.text.delta`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: TranscriptTextDeltaEventType,
    #[doc = "The text delta that was additionally transcribed.\n"]
    #[serde(rename = "delta")]
    pub delta: String,
    #[doc = "The log probabilities of the delta. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<TranscriptTextDeltaEventLogprob>>,
}
#[doc = "The type of the event. Always `transcript.text.done`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum TranscriptTextDoneEventType {
    #[default]
    #[serde(rename = "transcript.text.done")]
    TranscriptTextDone,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct TranscriptTextDoneEventLogprob {
    #[doc = "The token that was used to generate the log probability.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "token")]
    pub token: Option<String>,
    #[doc = "The log probability of the token.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprob")]
    pub logprob: Option<f64>,
    #[doc = "The bytes that were used to generate the log probability.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bytes")]
    pub bytes: Option<Vec<serde_json::Value>>,
}
#[doc = "Emitted when the transcription is complete. Contains the complete transcription text. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TranscriptTextDoneEvent {
    #[doc = "The type of the event. Always `transcript.text.done`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: TranscriptTextDoneEventType,
    #[doc = "The text that was transcribed.\n"]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "logprobs")]
    pub logprobs: Option<Vec<TranscriptTextDoneEventLogprob>>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum TranscriptionInclude {
    #[serde(rename = "logprobs")]
    Logprobs,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TranscriptionSegment {
    #[doc = "Unique identifier of the segment."]
    #[serde(rename = "id")]
    pub id: u64,
    #[doc = "Seek offset of the segment."]
    #[serde(rename = "seek")]
    pub seek: u64,
    #[doc = "Start time of the segment in seconds."]
    #[serde(rename = "start")]
    pub start: f64,
    #[doc = "End time of the segment in seconds."]
    #[serde(rename = "end")]
    pub end: f64,
    #[doc = "Text content of the segment."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Array of token IDs for the text content."]
    #[serde(rename = "tokens")]
    pub tokens: Vec<u64>,
    #[doc = "Temperature parameter used for generating the segment."]
    #[serde(rename = "temperature")]
    pub temperature: f64,
    #[doc = "Average logprob of the segment. If the value is lower than -1, consider the logprobs failed."]
    #[serde(rename = "avg_logprob")]
    pub avg_logprob: f64,
    #[doc = "Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed."]
    #[serde(rename = "compression_ratio")]
    pub compression_ratio: f64,
    #[doc = "Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent."]
    #[serde(rename = "no_speech_prob")]
    pub no_speech_prob: f64,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TranscriptionWord {
    #[doc = "The text content of the word."]
    #[serde(rename = "word")]
    pub word: String,
    #[doc = "Start time of the word in seconds."]
    #[serde(rename = "start")]
    pub start: f64,
    #[doc = "End time of the word in seconds."]
    #[serde(rename = "end")]
    pub end: f64,
}
#[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum TruncationObjectType {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "last_messages")]
    LastMessages,
}
#[doc = "Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct TruncationObject {
    #[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
    #[serde(rename = "type")]
    pub type_: TruncationObjectType,
    #[doc = "The number of most recent messages from the thread when constructing the context for the run."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_messages")]
    pub last_messages: Option<u64>,
}
#[doc = "Specifies the event type. For a type action, this property is \nalways set to `type`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum TypeType {
    #[default]
    #[serde(rename = "type")]
    Type,
}
#[doc = "An action to type in text.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Type {
    #[doc = "Specifies the event type. For a type action, this property is \nalways set to `type`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: TypeType,
    #[doc = "The text to type.\n"]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UpdateVectorStoreFileAttributesRequest {
    #[serde(rename = "attributes")]
    pub attributes: VectorStoreFileAttributes,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct UpdateVectorStoreRequest {
    #[doc = "The name of the vector store."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_after")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metadata")]
    pub metadata: Option<Metadata>,
}
#[doc = "The status of the Upload."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum UploadStatus {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "expired")]
    Expired,
}
#[doc = "The object type, which is always \"upload\"."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UploadObject {
    #[default]
    #[serde(rename = "upload")]
    Upload,
}
#[doc = "The Upload object can accept byte chunks in the form of Parts.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct Upload {
    #[doc = "The Upload unique identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the Upload was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The name of the file to be uploaded."]
    #[serde(rename = "filename")]
    pub filename: String,
    #[doc = "The intended number of bytes to be uploaded."]
    #[serde(rename = "bytes")]
    pub bytes: u64,
    #[doc = "The intended purpose of the file. [Please refer here](/docs/api-reference/files/object#files/object-purpose) for acceptable values."]
    #[serde(rename = "purpose")]
    pub purpose: String,
    #[doc = "The status of the Upload."]
    #[serde(rename = "status")]
    pub status: UploadStatus,
    #[doc = "The Unix timestamp (in seconds) for when the Upload will expire."]
    #[serde(rename = "expires_at")]
    pub expires_at: u64,
    #[doc = "The object type, which is always \"upload\"."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "object")]
    pub object: Option<UploadObject>,
    #[doc = "The ready File object after the Upload is completed."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file")]
    pub file: Option<OpenAiFile>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UploadCertificateRequest {
    #[doc = "An optional name for the certificate"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[doc = "The certificate content in PEM format"]
    #[serde(rename = "content")]
    pub content: String,
}
#[doc = "The object type, which is always `upload.part`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UploadPartObject {
    #[default]
    #[serde(rename = "upload.part")]
    UploadPart,
}
#[doc = "The upload Part represents a chunk of bytes we can add to an Upload object.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UploadPart {
    #[doc = "The upload Part unique identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the Part was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The ID of the Upload object that this Part was added to."]
    #[serde(rename = "upload_id")]
    pub upload_id: String,
    #[doc = "The object type, which is always `upload.part`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UploadPartObject,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageAudioSpeechesResultObject {
    #[default]
    #[serde(rename = "organization.usage.audio_speeches.result")]
    OrganizationUsageAudioSpeechesResult,
}
#[doc = "The aggregated audio speeches usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageAudioSpeechesResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageAudioSpeechesResultObject,
    #[doc = "The number of characters processed."]
    #[serde(rename = "characters")]
    pub characters: u64,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageAudioTranscriptionsResultObject {
    #[default]
    #[serde(rename = "organization.usage.audio_transcriptions.result")]
    OrganizationUsageAudioTranscriptionsResult,
}
#[doc = "The aggregated audio transcriptions usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageAudioTranscriptionsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageAudioTranscriptionsResultObject,
    #[doc = "The number of seconds processed."]
    #[serde(rename = "seconds")]
    pub seconds: u64,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageCodeInterpreterSessionsResultObject {
    #[default]
    #[serde(rename = "organization.usage.code_interpreter_sessions.result")]
    OrganizationUsageCodeInterpreterSessionsResult,
}
#[doc = "The aggregated code interpreter sessions usage details of the specific time bucket."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct UsageCodeInterpreterSessionsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageCodeInterpreterSessionsResultObject,
    #[doc = "The number of code interpreter sessions."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "num_sessions")]
    pub num_sessions: Option<u64>,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageCompletionsResultObject {
    #[default]
    #[serde(rename = "organization.usage.completions.result")]
    OrganizationUsageCompletionsResult,
}
#[doc = "The aggregated completions usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageCompletionsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageCompletionsResultObject,
    #[doc = "The aggregated number of text input tokens used, including cached tokens. For customers subscribe to scale tier, this includes scale tier tokens."]
    #[serde(rename = "input_tokens")]
    pub input_tokens: u64,
    #[doc = "The aggregated number of text input tokens that has been cached from previous requests. For customers subscribe to scale tier, this includes scale tier tokens."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_cached_tokens")]
    pub input_cached_tokens: Option<u64>,
    #[doc = "The aggregated number of text output tokens used. For customers subscribe to scale tier, this includes scale tier tokens."]
    #[serde(rename = "output_tokens")]
    pub output_tokens: u64,
    #[doc = "The aggregated number of audio input tokens used, including cached tokens."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "input_audio_tokens")]
    pub input_audio_tokens: Option<u64>,
    #[doc = "The aggregated number of audio output tokens used."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "output_audio_tokens")]
    pub output_audio_tokens: Option<u64>,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[doc = "When `group_by=batch`, this field tells whether the grouped usage result is batch or not."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "batch")]
    pub batch: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageEmbeddingsResultObject {
    #[default]
    #[serde(rename = "organization.usage.embeddings.result")]
    OrganizationUsageEmbeddingsResult,
}
#[doc = "The aggregated embeddings usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageEmbeddingsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageEmbeddingsResultObject,
    #[doc = "The aggregated number of input tokens used."]
    #[serde(rename = "input_tokens")]
    pub input_tokens: u64,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageImagesResultObject {
    #[default]
    #[serde(rename = "organization.usage.images.result")]
    OrganizationUsageImagesResult,
}
#[doc = "The aggregated images usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageImagesResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageImagesResultObject,
    #[doc = "The number of images processed."]
    #[serde(rename = "images")]
    pub images: u64,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=source`, this field provides the source of the grouped usage result, possible values are `image.generation`, `image.edit`, `image.variation`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[doc = "When `group_by=size`, this field provides the image size of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "size")]
    pub size: Option<String>,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageModerationsResultObject {
    #[default]
    #[serde(rename = "organization.usage.moderations.result")]
    OrganizationUsageModerationsResult,
}
#[doc = "The aggregated moderations usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageModerationsResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageModerationsResultObject,
    #[doc = "The aggregated number of input tokens used."]
    #[serde(rename = "input_tokens")]
    pub input_tokens: u64,
    #[doc = "The count of requests made to the model."]
    #[serde(rename = "num_model_requests")]
    pub num_model_requests: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_id")]
    pub user_id: Option<String>,
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "api_key_id")]
    pub api_key_id: Option<String>,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "model")]
    pub model: Option<String>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageResponseObject {
    #[default]
    #[serde(rename = "page")]
    Page,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<UsageTimeBucket>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "next_page")]
    pub next_page: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageTimeBucketObject {
    #[default]
    #[serde(rename = "bucket")]
    Bucket,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum UsageTimeBucketResult {
    OrganizationUsageCompletionsResult(UsageCompletionsResult),
    OrganizationUsageEmbeddingsResult(UsageEmbeddingsResult),
    OrganizationUsageModerationsResult(UsageModerationsResult),
    OrganizationUsageImagesResult(UsageImagesResult),
    OrganizationUsageAudioSpeechesResult(UsageAudioSpeechesResult),
    OrganizationUsageAudioTranscriptionsResult(UsageAudioTranscriptionsResult),
    OrganizationUsageVectorStoresResult(UsageVectorStoresResult),
    OrganizationUsageCodeInterpreterSessionsResult(UsageCodeInterpreterSessionsResult),
    OrganizationCostsResult(CostsResult),
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageTimeBucket {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageTimeBucketObject,
    #[serde(rename = "start_time")]
    pub start_time: u64,
    #[serde(rename = "end_time")]
    pub end_time: u64,
    #[serde(rename = "result")]
    pub result: Vec<UsageTimeBucketResult>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UsageVectorStoresResultObject {
    #[default]
    #[serde(rename = "organization.usage.vector_stores.result")]
    OrganizationUsageVectorStoresResult,
}
#[doc = "The aggregated vector stores usage details of the specific time bucket."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UsageVectorStoresResult {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UsageVectorStoresResultObject,
    #[doc = "The vector stores usage in bytes."]
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "project_id")]
    pub project_id: Option<String>,
}
#[doc = "The object type, which is always `organization.user`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UserObject {
    #[default]
    #[serde(rename = "organization.user")]
    OrganizationUser,
}
#[doc = "`owner` or `reader`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum UserRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
}
#[doc = "Represents an individual `user` within an organization."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct User {
    #[doc = "The object type, which is always `organization.user`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UserObject,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The name of the user"]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The email address of the user"]
    #[serde(rename = "email")]
    pub email: String,
    #[doc = "`owner` or `reader`"]
    #[serde(rename = "role")]
    pub role: UserRole,
    #[doc = "The Unix timestamp (in seconds) of when the user was added."]
    #[serde(rename = "added_at")]
    pub added_at: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UserDeleteResponseObject {
    #[default]
    #[serde(rename = "organization.user.deleted")]
    OrganizationUserDeleted,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UserDeleteResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UserDeleteResponseObject,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UserListResponseObject {
    #[default]
    #[serde(rename = "list")]
    List,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UserListResponse {
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: UserListResponseObject,
    #[serde(rename = "data")]
    pub data: Vec<User>,
    #[serde(rename = "first_id")]
    pub first_id: String,
    #[serde(rename = "last_id")]
    pub last_id: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
}
#[doc = "`owner` or `reader`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum UserRoleUpdateRequestRole {
    #[serde(rename = "owner")]
    Owner,
    #[serde(rename = "reader")]
    Reader,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UserRoleUpdateRequest {
    #[doc = "`owner` or `reader`"]
    #[serde(rename = "role")]
    pub role: UserRoleUpdateRequestRole,
}
#[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreExpirationAfterAnchor {
    #[default]
    #[serde(rename = "last_active_at")]
    LastActiveAt,
}
#[doc = "The expiration policy for a vector store."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
    #[builder(default)]
    #[serde(rename = "anchor")]
    pub anchor: VectorStoreExpirationAfterAnchor,
    #[doc = "The number of days after the anchor time that the vector store will expire."]
    #[serde(rename = "days")]
    pub days: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreFileAttribute {
    _0(String),
    _1(f64),
    _2(bool),
}
pub type VectorStoreFileAttributes = Vec<VectorStoreFileAttribute>;
#[doc = "The object type, which is always `vector_store.file_batch`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreFileBatchObjectObject {
    #[default]
    #[serde(rename = "vector_store.files_batch")]
    VectorStoreFilesBatch,
}
#[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum VectorStoreFileBatchObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileBatchObjectFileCounts {
    #[doc = "The number of files that are currently being processed."]
    #[serde(rename = "in_progress")]
    pub in_progress: u64,
    #[doc = "The number of files that have been processed."]
    #[serde(rename = "completed")]
    pub completed: u64,
    #[doc = "The number of files that have failed to process."]
    #[serde(rename = "failed")]
    pub failed: u64,
    #[doc = "The number of files that where cancelled."]
    #[serde(rename = "cancelled")]
    pub cancelled: u64,
    #[doc = "The total number of files."]
    #[serde(rename = "total")]
    pub total: u64,
}
#[doc = "A batch of files attached to a vector store."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileBatchObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `vector_store.file_batch`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: VectorStoreFileBatchObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the vector store files batch was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to."]
    #[serde(rename = "vector_store_id")]
    pub vector_store_id: String,
    #[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
    #[serde(rename = "status")]
    pub status: VectorStoreFileBatchObjectStatus,
    #[serde(rename = "file_counts")]
    pub file_counts: VectorStoreFileBatchObjectFileCounts,
}
#[doc = "The object type, which is always `vector_store.file_content.page`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreFileContentResponseObject {
    #[default]
    #[serde(rename = "vector_store.file_content.page")]
    VectorStoreFileContentPage,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileContentResponseDatum {
    #[doc = "The content type (currently only `\"text\"`)"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[doc = "The text content"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "text")]
    pub text: Option<String>,
}
#[doc = "Represents the parsed content of a vector store file."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileContentResponse {
    #[doc = "The object type, which is always `vector_store.file_content.page`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: VectorStoreFileContentResponseObject,
    #[doc = "Parsed content of the file."]
    #[serde(rename = "data")]
    pub data: Vec<VectorStoreFileContentResponseDatum>,
    #[doc = "Indicates if there are more content pages to fetch."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[doc = "The token for the next page, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "next_page")]
    pub next_page: Option<String>,
}
#[doc = "The object type, which is always `vector_store.file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreFileObjectObject {
    #[default]
    #[serde(rename = "vector_store.file")]
    VectorStoreFile,
}
#[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum VectorStoreFileObjectStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed,
}
#[doc = "One of `server_error` or `rate_limit_exceeded`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum VectorStoreFileObjectLastErrorCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "unsupported_file")]
    UnsupportedFile,
    #[serde(rename = "invalid_file")]
    InvalidFile,
}
#[doc = "The last error associated with this vector store file. Will be `null` if there are no errors."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileObjectLastError {
    #[doc = "One of `server_error` or `rate_limit_exceeded`."]
    #[serde(rename = "code")]
    pub code: VectorStoreFileObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "The strategy used to chunk the file."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreFileObjectChunkingStrategy {
    Static(StaticChunkingStrategyResponseParam),
    Other(OtherChunkingStrategyResponseParam),
}
#[doc = "A list of files attached to a vector store."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreFileObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `vector_store.file`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: VectorStoreFileObjectObject,
    #[doc = "The total vector store usage in bytes. Note that this may be different from the original file size."]
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: u64,
    #[doc = "The Unix timestamp (in seconds) for when the vector store file was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to."]
    #[serde(rename = "vector_store_id")]
    pub vector_store_id: String,
    #[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
    #[serde(rename = "status")]
    pub status: VectorStoreFileObjectStatus,
    #[doc = "The last error associated with this vector store file. Will be `null` if there are no errors."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_error")]
    pub last_error: Option<VectorStoreFileObjectLastError>,
    #[doc = "The strategy used to chunk the file."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "chunking_strategy")]
    pub chunking_strategy: Option<VectorStoreFileObjectChunkingStrategy>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "attributes")]
    pub attributes: Option<VectorStoreFileAttributes>,
}
#[doc = "The object type, which is always `vector_store`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreObjectObject {
    #[default]
    #[serde(rename = "vector_store")]
    VectorStore,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreObjectFileCounts {
    #[doc = "The number of files that are currently being processed."]
    #[serde(rename = "in_progress")]
    pub in_progress: u64,
    #[doc = "The number of files that have been successfully processed."]
    #[serde(rename = "completed")]
    pub completed: u64,
    #[doc = "The number of files that have failed to process."]
    #[serde(rename = "failed")]
    pub failed: u64,
    #[doc = "The number of files that were cancelled."]
    #[serde(rename = "cancelled")]
    pub cancelled: u64,
    #[doc = "The total number of files."]
    #[serde(rename = "total")]
    pub total: u64,
}
#[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum VectorStoreObjectStatus {
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}
#[doc = "A vector store is a collection of processed files can be used by the `file_search` tool."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreObject {
    #[doc = "The identifier, which can be referenced in API endpoints."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The object type, which is always `vector_store`."]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: VectorStoreObjectObject,
    #[doc = "The Unix timestamp (in seconds) for when the vector store was created."]
    #[serde(rename = "created_at")]
    pub created_at: u64,
    #[doc = "The name of the vector store."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "The total number of bytes used by the files in the vector store."]
    #[serde(rename = "usage_bytes")]
    pub usage_bytes: u64,
    #[serde(rename = "file_counts")]
    pub file_counts: VectorStoreObjectFileCounts,
    #[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
    #[serde(rename = "status")]
    pub status: VectorStoreObjectStatus,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_after")]
    pub expires_after: Option<VectorStoreExpirationAfter>,
    #[doc = "The Unix timestamp (in seconds) for when the vector store will expire."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "expires_at")]
    pub expires_at: Option<u64>,
    #[doc = "The Unix timestamp (in seconds) for when the vector store was last active."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "last_active_at")]
    pub last_active_at: Option<u64>,
    #[serde(rename = "metadata")]
    pub metadata: Metadata,
}
#[doc = "A query string for a search"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreSearchRequestQuery {
    _0(String),
    _1(Vec<String>),
}
#[doc = "A filter to apply based on file attributes."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreSearchRequestFilters {
    ComparisonFilter(ComparisonFilter),
    CompoundFilter(CompoundFilter),
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreSearchRequestRankingOptionsRanker {
    #[default]
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}
#[doc = "Ranking options for search."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct VectorStoreSearchRequestRankingOptions {
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranker")]
    pub ranker: Option<VectorStoreSearchRequestRankingOptionsRanker>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "score_threshold")]
    pub score_threshold: Option<f64>,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreSearchRequest {
    #[doc = "A query string for a search"]
    #[serde(rename = "query")]
    pub query: VectorStoreSearchRequestQuery,
    #[doc = "Whether to rewrite the natural language query for vector search."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rewrite_query")]
    pub rewrite_query: Option<bool>,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_num_results")]
    pub max_num_results: Option<u64>,
    #[doc = "A filter to apply based on file attributes."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filters")]
    pub filters: Option<VectorStoreSearchRequestFilters>,
    #[doc = "Ranking options for search."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<VectorStoreSearchRequestRankingOptions>,
}
#[doc = "The type of content."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum VectorStoreSearchResultContentObjectType {
    #[serde(rename = "text")]
    Text,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreSearchResultContentObject {
    #[doc = "The type of content."]
    #[serde(rename = "type")]
    pub type_: VectorStoreSearchResultContentObjectType,
    #[doc = "The text content returned from search."]
    #[serde(rename = "text")]
    pub text: String,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreSearchResultItem {
    #[doc = "The ID of the vector store file."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "The name of the vector store file."]
    #[serde(rename = "filename")]
    pub filename: String,
    #[doc = "The similarity score for the result."]
    #[serde(rename = "score")]
    pub score: f64,
    #[serde(rename = "attributes")]
    pub attributes: VectorStoreFileAttributes,
    #[doc = "Content chunks from the file."]
    #[serde(rename = "content")]
    pub content: Vec<VectorStoreSearchResultContentObject>,
}
#[doc = "The object type, which is always `vector_store.search_results.page`"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum VectorStoreSearchResultsPageObject {
    #[default]
    #[serde(rename = "vector_store.search_results.page")]
    VectorStoreSearchResultsPage,
}
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct VectorStoreSearchResultsPage {
    #[doc = "The object type, which is always `vector_store.search_results.page`"]
    #[builder(default)]
    #[serde(rename = "object")]
    pub object: VectorStoreSearchResultsPageObject,
    #[serde(rename = "search_query")]
    pub search_query: Vec<String>,
    #[doc = "The list of search result items."]
    #[serde(rename = "data")]
    pub data: Vec<VectorStoreSearchResultItem>,
    #[doc = "Indicates if there are more results to fetch."]
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[doc = "The token for the next page, if any."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "next_page")]
    pub next_page: Option<String>,
}
pub type VoiceIdsShared = String;
#[doc = "Specifies the event type. For a wait action, this property is \nalways set to `wait`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum WaitType {
    #[default]
    #[serde(rename = "wait")]
    Wait,
}
#[doc = "A wait action.\n"]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct Wait {
    #[doc = "Specifies the event type. For a wait action, this property is \nalways set to `wait`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: WaitType,
}
#[doc = "High level guidance for the amount of context window space to use for the \nsearch. One of `low`, `medium`, or `high`. `medium` is the default.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum WebSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[default]
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
#[doc = "Approximate location parameters for the search."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct WebSearchLocation {
    #[doc = "The two-letter \n[ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,\ne.g. `US`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[doc = "Free text input for the region of the user, e.g. `California`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[doc = "Free text input for the city of the user, e.g. `San Francisco`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[doc = "The [IANA timezone](https://timeapi.io/documentation/iana-timezones) \nof the user, e.g. `America/Los_Angeles`.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}
#[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum WebSearchToolCallType {
    #[default]
    #[serde(rename = "web_search_call")]
    WebSearchCall,
}
#[doc = "The status of the web search tool call.\n"]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum WebSearchToolCallStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "searching")]
    Searching,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "failed")]
    Failed,
}
#[doc = "The results of a web search tool call. See the \n[web search guide](/docs/guides/tools-web-search) for more information.\n"]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct WebSearchToolCall {
    #[doc = "The unique ID of the web search tool call.\n"]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: WebSearchToolCallType,
    #[doc = "The status of the web search tool call.\n"]
    #[serde(rename = "status")]
    pub status: WebSearchToolCallStatus,
}
#[doc = "The type of the input item. Always `input_text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InputTextContentType {
    #[default]
    #[serde(rename = "input_text")]
    InputText,
}
#[doc = "A text input to the model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InputTextContent {
    #[doc = "The type of the input item. Always `input_text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: InputTextContentType,
    #[doc = "The text input to the model."]
    #[serde(rename = "text")]
    pub text: String,
}
#[doc = "The type of the input item. Always `input_image`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InputImageContentType {
    #[default]
    #[serde(rename = "input_image")]
    InputImage,
}
#[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum InputImageContentDetail {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "auto")]
    Auto,
}
#[doc = "An image input to the model. Learn about [image inputs](/docs/guides/vision)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct InputImageContent {
    #[doc = "The type of the input item. Always `input_image`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: InputImageContentType,
    #[doc = "The URL of the image to be sent to the model. A fully qualified URL or base64 encoded image in a data URL."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "image_url")]
    pub image_url: Option<String>,
    #[doc = "The ID of the file to be sent to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`."]
    #[serde(rename = "detail")]
    pub detail: InputImageContentDetail,
}
#[doc = "The type of the input item. Always `input_file`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum InputFileContentType {
    #[default]
    #[serde(rename = "input_file")]
    InputFile,
}
#[doc = "A file input to the model."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct InputFileContent {
    #[doc = "The type of the input item. Always `input_file`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: InputFileContentType,
    #[doc = "The ID of the file to be sent to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_id")]
    pub file_id: Option<String>,
    #[doc = "The name of the file to be sent to the model."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filename")]
    pub filename: Option<String>,
    #[doc = "The content of the file to be sent to the model.\n"]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "file_data")]
    pub file_data: Option<String>,
}
#[doc = "The ranker to use for the file search."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum RankingOptionsRanker {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "default-2024-11-15")]
    Default20241115,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct RankingOptions {
    #[doc = "The ranker to use for the file search."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranker")]
    pub ranker: Option<RankingOptionsRanker>,
    #[doc = "The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "score_threshold")]
    pub score_threshold: Option<f64>,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Filters {
    ComparisonFilter(ComparisonFilter),
    CompoundFilter(CompoundFilter),
}
#[doc = "The type of the file search tool. Always `file_search`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FileSearchToolType {
    #[default]
    #[serde(rename = "file_search")]
    FileSearch,
}
#[doc = "A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FileSearchTool {
    #[doc = "The type of the file search tool. Always `file_search`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FileSearchToolType,
    #[doc = "The IDs of the vector stores to search."]
    #[serde(rename = "vector_store_ids")]
    pub vector_store_ids: Vec<String>,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "max_num_results")]
    pub max_num_results: Option<u64>,
    #[doc = "Ranking options for search."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "ranking_options")]
    pub ranking_options: Option<RankingOptions>,
    #[doc = "A filter to apply."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "filters")]
    pub filters: Option<Filters>,
}
#[doc = "The type of the function tool. Always `function`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FunctionToolType {
    #[default]
    #[serde(rename = "function")]
    Function,
}
#[doc = "Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionTool {
    #[doc = "The type of the function tool. Always `function`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FunctionToolType,
    #[doc = "The name of the function to call."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "A description of the function. Used by the model to determine whether or not to call the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[doc = "A JSON schema object describing the parameters of the function."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "parameters")]
    pub parameters: Option<Vec<serde_json::Value>>,
    #[doc = "Whether to enforce strict parameter validation. Default `true`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "strict")]
    pub strict: Option<bool>,
}
#[doc = "The type of location approximation. Always `approximate`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ApproximateLocationType {
    #[default]
    #[serde(rename = "approximate")]
    Approximate,
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct ApproximateLocation {
    #[doc = "The type of location approximation. Always `approximate`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ApproximateLocationType,
    #[doc = "The two-letter [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user, e.g. `US`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "country")]
    pub country: Option<String>,
    #[doc = "Free text input for the region of the user, e.g. `California`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "region")]
    pub region: Option<String>,
    #[doc = "Free text input for the city of the user, e.g. `San Francisco`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[doc = "The [IANA timezone](https://timeapi.io/documentation/iana-timezones) of the user, e.g. `America/Los_Angeles`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}
#[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum WebSearchPreviewToolType {
    #[default]
    #[serde(rename = "web_search_preview")]
    WebSearchPreview,
    #[serde(rename = "web_search_preview_2025_03_11")]
    WebSearchPreview20250311,
}
#[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum WebSearchPreviewToolSearchContextSize {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
}
#[doc = "This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search)."]
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde :: Deserialize,
    serde :: Serialize,
    Default,
    typed_builder :: TypedBuilder,
)]
pub struct WebSearchPreviewTool {
    #[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: WebSearchPreviewToolType,
    #[doc = "The user's location."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "user_location")]
    pub user_location: Option<ApproximateLocation>,
    #[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "search_context_size")]
    pub search_context_size: Option<WebSearchPreviewToolSearchContextSize>,
}
#[doc = "The type of the computer use tool. Always `computer_use_preview`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComputerUsePreviewToolType {
    #[default]
    #[serde(rename = "computer_use_preview")]
    ComputerUsePreview,
}
#[doc = "The type of computer environment to control."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ComputerUsePreviewToolEnvironment {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "mac")]
    Mac,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "ubuntu")]
    Ubuntu,
    #[serde(rename = "browser")]
    Browser,
}
#[doc = "A tool that controls a virtual computer. Learn more about the [computer tool](https://platform.openai.com/docs/guides/tools-computer-use)."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerUsePreviewTool {
    #[doc = "The type of the computer use tool. Always `computer_use_preview`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComputerUsePreviewToolType,
    #[doc = "The type of computer environment to control."]
    #[serde(rename = "environment")]
    pub environment: ComputerUsePreviewToolEnvironment,
    #[doc = "The width of the computer display."]
    #[serde(rename = "display_width")]
    pub display_width: u64,
    #[doc = "The height of the computer display."]
    #[serde(rename = "display_height")]
    pub display_height: u64,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Tool {
    FileSearch(FileSearchTool),
    Function(FunctionTool),
    WebSearchPreview(WebSearchPreviewTool),
    ComputerUsePreview(ComputerUsePreviewTool),
}
#[doc = "The type of the file citation. Always `file_citation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FileCitationBodyType {
    #[default]
    #[serde(rename = "file_citation")]
    FileCitation,
}
#[doc = "A citation to a file."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FileCitationBody {
    #[doc = "The type of the file citation. Always `file_citation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FileCitationBodyType,
    #[doc = "The ID of the file."]
    #[serde(rename = "file_id")]
    pub file_id: String,
    #[doc = "The index of the file in the list of files."]
    #[serde(rename = "index")]
    pub index: u64,
}
#[doc = "The type of the URL citation. Always `url_citation`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum UrlCitationBodyType {
    #[default]
    #[serde(rename = "url_citation")]
    UrlCitation,
}
#[doc = "A citation for a web resource used to generate a model response."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct UrlCitationBody {
    #[doc = "The type of the URL citation. Always `url_citation`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: UrlCitationBodyType,
    #[doc = "The URL of the web resource."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "The index of the first character of the URL citation in the message."]
    #[serde(rename = "start_index")]
    pub start_index: u64,
    #[doc = "The index of the last character of the URL citation in the message."]
    #[serde(rename = "end_index")]
    pub end_index: u64,
    #[doc = "The title of the web resource."]
    #[serde(rename = "title")]
    pub title: String,
}
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum Annotation {
    FileCitation(FileCitationBody),
    UrlCitation(UrlCitationBody),
    FilePath(FilePath),
}
#[doc = "The type of the output text. Always `output_text`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum OutputTextContentType {
    #[default]
    #[serde(rename = "output_text")]
    OutputText,
}
#[doc = "A text output from the model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct OutputTextContent {
    #[doc = "The type of the output text. Always `output_text`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: OutputTextContentType,
    #[doc = "The text output from the model."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "The annotations of the text output."]
    #[serde(rename = "annotations")]
    pub annotations: Vec<Annotation>,
}
#[doc = "The type of the refusal. Always `refusal`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum RefusalContentType {
    #[default]
    #[serde(rename = "refusal")]
    Refusal,
}
#[doc = "A refusal from the model."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct RefusalContent {
    #[doc = "The type of the refusal. Always `refusal`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: RefusalContentType,
    #[doc = "The refusal explanationfrom the model."]
    #[serde(rename = "refusal")]
    pub refusal: String,
}
#[doc = "A pending safety check for the computer call."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerCallSafetyCheckParam {
    #[doc = "The ID of the pending safety check."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "The type of the pending safety check."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "code")]
    pub code: Option<String>,
    #[doc = "Details about the pending safety check."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "message")]
    pub message: Option<String>,
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ComputerCallOutputItemParamType {
    #[default]
    #[serde(rename = "computer_call_output")]
    ComputerCallOutput,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum ComputerCallOutputItemParamStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The output of a computer tool call."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ComputerCallOutputItemParam {
    #[doc = "The ID of the computer tool call output."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The ID of the computer tool call that produced the output."]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: ComputerCallOutputItemParamType,
    #[builder(default)]
    #[serde(rename = "output")]
    pub output: ComputerScreenshotImage,
    #[doc = "The safety checks reported by the API that have been acknowledged by the developer."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "acknowledged_safety_checks")]
    pub acknowledged_safety_checks: Option<Vec<ComputerCallSafetyCheckParam>>,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<ComputerCallOutputItemParamStatus>,
}
#[doc = "The type of the function tool call output. Always `function_call_output`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum FunctionCallOutputItemParamType {
    #[default]
    #[serde(rename = "function_call_output")]
    FunctionCallOutput,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize)]
pub enum FunctionCallOutputItemParamStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "incomplete")]
    Incomplete,
}
#[doc = "The output of a function tool call."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct FunctionCallOutputItemParam {
    #[doc = "The unique ID of the function tool call output. Populated when this item is returned via API."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[doc = "The unique ID of the function tool call generated by the model."]
    #[serde(rename = "call_id")]
    pub call_id: String,
    #[doc = "The type of the function tool call output. Always `function_call_output`."]
    #[builder(default)]
    #[serde(rename = "type")]
    pub type_: FunctionCallOutputItemParamType,
    #[doc = "A JSON string of the output of the function tool call."]
    #[serde(rename = "output")]
    pub output: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "status")]
    pub status: Option<FunctionCallOutputItemParamStatus>,
}
#[doc = "The type of item to reference. Always `item_reference`."]
#[derive(Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, Default)]
pub enum ItemReferenceParamType {
    #[default]
    #[serde(rename = "item_reference")]
    ItemReference,
}
#[doc = "An internal identifier for an item to reference."]
#[derive(
    Clone, Debug, PartialEq, serde :: Deserialize, serde :: Serialize, typed_builder :: TypedBuilder,
)]
pub struct ItemReferenceParam {
    #[doc = "The type of item to reference. Always `item_reference`."]
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<ItemReferenceParamType>,
    #[doc = "The ID of the item to reference."]
    #[serde(rename = "id")]
    pub id: String,
}
#[cfg(test)]
mod tests;
