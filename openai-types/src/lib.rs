pub struct AddUploadPartRequest {
    #[doc = "The chunk of bytes for this Part.\n"]
    pub data: Vec<u8>,
}
#[doc = "Represents an individual Admin API key in an org."]
pub struct AdminApiKey {
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) of when the API key was last used"]
    pub last_used_at: u64,
    #[doc = "The name of the API key"]
    pub name: String,
    #[doc = "The object type, which is always `organization.admin_api_key`"]
    pub object: String,
    pub owner: AdminApiKeyOwner,
    #[doc = "The redacted value of the API key"]
    pub redacted_value: String,
    #[doc = "The value of the API key. Only shown on create."]
    pub value: String,
}
pub struct AdminApiKeyOwner {
    #[doc = "The Unix timestamp (in seconds) of when the user was created"]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the user"]
    pub name: String,
    #[doc = "The object type, which is always organization.user"]
    pub object: String,
    #[doc = "Always `owner`"]
    pub role: String,
    #[doc = "Always `user`"]
    pub type_: String,
}
#[allow(clippy::large_enum_variant)]
pub enum Annotation {
    OneOf0(FileCitationBody),
    OneOf1(UrlCitationBody),
    OneOf2(FilePath),
}
pub struct ApiKeyList {
    pub data: Vec<AdminApiKey>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ApproximateLocation {
    pub city: Vec<String>,
    pub country: Vec<String>,
    pub region: Vec<String>,
    pub timezone: Vec<String>,
    #[doc = "The type of location approximation. Always `approximate`."]
    pub type_: ApproximateLocationType,
}
#[doc = "The type of location approximation. Always `approximate`."]
pub enum ApproximateLocationType {
    Approximate,
}
#[doc = "Represents an `assistant` that can call the model and use tools."]
pub struct AssistantObject {
    #[doc = "The Unix timestamp (in seconds) for when the assistant was created."]
    pub created_at: u64,
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    pub description: String,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    pub instructions: String,
    pub metadata: Metadata,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    pub model: String,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    pub name: String,
    #[doc = "The object type, which is always `assistant`."]
    pub object: AssistantObjectObject,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    pub temperature: f64,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: AssistantObjectToolResources,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    pub tools: Vec<AssistantObjectToolsInner>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    pub top_p: f64,
}
#[doc = "The object type, which is always `assistant`."]
pub enum AssistantObjectObject {
    Assistant,
}
pub struct AssistantObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct AssistantObjectToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    pub vector_store_ids: Vec<String>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct AssistantObjectToolResources {
    pub code_interpreter: AssistantObjectToolResourcesCodeInterpreter,
    pub file_search: AssistantObjectToolResourcesFileSearch,
}
#[allow(clippy::large_enum_variant)]
pub enum AssistantObjectToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
#[doc = "Represents an event emitted when streaming a Run.\n\nEach event in a server-sent events stream has an `event` and `data` property:\n\n```\nevent: thread.created\ndata: {\"id\": \"thread_123\", \"object\": \"thread\", ...}\n```\n\nWe emit events whenever a new object is created, transitions to a new state, or is being\nstreamed in parts (deltas). For example, we emit `thread.run.created` when a new run\nis created, `thread.run.completed` when a run completes, and so on. When an Assistant chooses\nto create a message during a run, we emit a `thread.message.created event`, a\n`thread.message.in_progress` event, many `thread.message.delta` events, and finally a\n`thread.message.completed` event.\n\nWe may add additional events over time, so we recommend handling unknown events gracefully\nin your code. See the [Assistants API quickstart](/docs/assistants/overview) to learn how to\nintegrate the Assistants API with streaming.\n"]
#[allow(clippy::large_enum_variant)]
pub enum AssistantStreamEvent {
    OneOf0(ThreadStreamEvent),
    OneOf1(RunStreamEvent),
    OneOf2(RunStepStreamEvent),
    OneOf3(MessageStreamEvent),
    OneOf4(ErrorEvent),
    OneOf5(DoneEvent),
}
pub enum AssistantSupportedModels {
    Gpt41,
    Gpt41Mini,
    Gpt41Nano,
    Gpt4120250414,
    Gpt41Mini20250414,
    Gpt41Nano20250414,
    O3Mini,
    O3Mini20250131,
    O1,
    O120241217,
    Gpt4o,
    Gpt4o20241120,
    Gpt4o20240806,
    Gpt4o20240513,
    Gpt4oMini,
    Gpt4oMini20240718,
    Gpt45Preview,
    Gpt45Preview20250227,
    Gpt4Turbo,
    Gpt4Turbo20240409,
    Gpt40125Preview,
    Gpt4TurboPreview,
    Gpt41106Preview,
    Gpt4VisionPreview,
    Gpt4,
    Gpt40314,
    Gpt40613,
    Gpt432k,
    Gpt432k0314,
    Gpt432k0613,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo0613,
    Gpt35Turbo1106,
    Gpt35Turbo0125,
    Gpt35Turbo16k0613,
}
pub struct AssistantToolsCode {
    #[doc = "The type of tool being defined: `code_interpreter`"]
    pub type_: AssistantToolsCodeType,
}
#[doc = "The type of tool being defined: `code_interpreter`"]
pub enum AssistantToolsCodeType {
    CodeInterpreter,
}
pub struct AssistantToolsFileSearch {
    #[doc = "Overrides for the file search tool."]
    pub file_search: AssistantToolsFileSearchFileSearch,
    #[doc = "The type of tool being defined: `file_search`"]
    pub type_: AssistantToolsFileSearchType,
}
#[doc = "Overrides for the file search tool."]
pub struct AssistantToolsFileSearchFileSearch {
    #[doc = "The maximum number of results the file search tool should output. The default is 20 for `gpt-4*` models and 5 for `gpt-3.5-turbo`. This number should be between 1 and 50 inclusive.\n\nNote that the file search tool may output fewer than `max_num_results` results. See the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.\n"]
    pub max_num_results: u64,
    pub ranking_options: FileSearchRankingOptions,
}
#[doc = "The type of tool being defined: `file_search`"]
pub enum AssistantToolsFileSearchType {
    FileSearch,
}
pub struct AssistantToolsFileSearchTypeOnly {
    #[doc = "The type of tool being defined: `file_search`"]
    pub type_: AssistantToolsFileSearchTypeOnlyType,
}
#[doc = "The type of tool being defined: `file_search`"]
pub enum AssistantToolsFileSearchTypeOnlyType {
    FileSearch,
}
pub struct AssistantToolsFunction {
    pub function: FunctionObject,
    #[doc = "The type of tool being defined: `function`"]
    pub type_: AssistantToolsFunctionType,
}
#[doc = "The type of tool being defined: `function`"]
pub enum AssistantToolsFunctionType {
    Function,
}
#[doc = "Specifies the format that the model must output. Compatible with [GPT-4o](/docs/models#gpt-4o), [GPT-4 Turbo](/docs/models#gpt-4-turbo-and-gpt-4), and all GPT-3.5 Turbo models since `gpt-3.5-turbo-1106`.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables Structured Outputs which ensures the model will match your supplied JSON schema. Learn more in the [Structured Outputs guide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables JSON mode, which ensures the message the model generates is valid JSON.\n\n**Important:** when using JSON mode, you **must** also instruct the model to produce JSON yourself via a system or user message. Without this, the model may generate an unending stream of whitespace until the generation reaches the token limit, resulting in a long-running and seemingly \"stuck\" request. Also note that the message content may be partially cut off if `finish_reason=\"length\"`, which indicates the generation exceeded `max_tokens` or the conversation exceeded the max context length.\n"]
#[allow(clippy::large_enum_variant)]
pub enum AssistantsApiResponseFormatOption {
    OneOf0(AssistantsApiResponseFormatOption0),
    OneOf1(ResponseFormatText),
    OneOf2(ResponseFormatJsonObject),
    OneOf3(ResponseFormatJsonSchema),
}
#[doc = "`auto` is the default value\n"]
pub enum AssistantsApiResponseFormatOption0 {
    Auto,
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tools and instead generates a message.\n`auto` is the default value and means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools before responding to the user.\nSpecifying a particular tool like `{\"type\": \"file_search\"}` or `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n"]
#[allow(clippy::large_enum_variant)]
pub enum AssistantsApiToolChoiceOption {
    OneOf0(AssistantsApiToolChoiceOption0),
    OneOf1(AssistantsNamedToolChoice),
}
#[doc = "`none` means the model will not call any tools and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools before responding to the user.\n"]
pub enum AssistantsApiToolChoiceOption0 {
    None,
    Auto,
    Required,
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific tool."]
pub struct AssistantsNamedToolChoice {
    pub function: AssistantsNamedToolChoiceFunction,
    #[doc = "The type of the tool. If type is `function`, the function name must be set"]
    pub type_: AssistantsNamedToolChoiceType,
}
pub struct AssistantsNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The type of the tool. If type is `function`, the function name must be set"]
pub enum AssistantsNamedToolChoiceType {
    Function,
    CodeInterpreter,
    FileSearch,
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. For `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`, the only supported format is `json`.\n"]
pub enum AudioResponseFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}
#[doc = "A log of a user action or configuration change within this organization."]
pub struct AuditLog {
    pub actor: AuditLogActor,
    #[doc = "The details for events with this `type`."]
    pub api_key_created: AuditLogApiKeyCreated,
    #[doc = "The details for events with this `type`."]
    pub api_key_deleted: AuditLogApiKeyDeleted,
    #[doc = "The details for events with this `type`."]
    pub api_key_updated: AuditLogApiKeyUpdated,
    #[doc = "The details for events with this `type`."]
    pub certificate_created: AuditLogCertificateCreated,
    #[doc = "The details for events with this `type`."]
    pub certificate_deleted: AuditLogCertificateDeleted,
    #[doc = "The details for events with this `type`."]
    pub certificate_updated: AuditLogCertificateUpdated,
    #[doc = "The details for events with this `type`."]
    pub certificates_activated: AuditLogCertificatesActivated,
    #[doc = "The details for events with this `type`."]
    pub certificates_deactivated: AuditLogCertificatesDeactivated,
    #[doc = "The project and fine-tuned model checkpoint that the checkpoint permission was created for."]
    pub checkpoint_permission_created: AuditLogCheckpointPermissionCreated,
    #[doc = "The details for events with this `type`."]
    pub checkpoint_permission_deleted: AuditLogCheckpointPermissionDeleted,
    #[doc = "The Unix timestamp (in seconds) of the event."]
    pub effective_at: u64,
    #[doc = "The ID of this log."]
    pub id: String,
    #[doc = "The details for events with this `type`."]
    pub invite_accepted: AuditLogInviteAccepted,
    #[doc = "The details for events with this `type`."]
    pub invite_deleted: AuditLogInviteDeleted,
    #[doc = "The details for events with this `type`."]
    pub invite_sent: AuditLogInviteSent,
    #[doc = "The details for events with this `type`."]
    pub login_failed: AuditLogLoginFailed,
    #[doc = "The details for events with this `type`."]
    pub logout_failed: AuditLogLogoutFailed,
    #[doc = "The details for events with this `type`."]
    pub organization_updated: AuditLogOrganizationUpdated,
    #[doc = "The project that the action was scoped to. Absent for actions not scoped to projects."]
    pub project: AuditLogProject,
    #[doc = "The details for events with this `type`."]
    pub project_archived: AuditLogProjectArchived,
    #[doc = "The details for events with this `type`."]
    pub project_created: AuditLogProjectCreated,
    #[doc = "The details for events with this `type`."]
    pub project_updated: AuditLogProjectUpdated,
    #[doc = "The details for events with this `type`."]
    pub rate_limit_deleted: AuditLogRateLimitDeleted,
    #[doc = "The details for events with this `type`."]
    pub rate_limit_updated: AuditLogRateLimitUpdated,
    #[doc = "The details for events with this `type`."]
    pub service_account_created: AuditLogServiceAccountCreated,
    #[doc = "The details for events with this `type`."]
    pub service_account_deleted: AuditLogServiceAccountDeleted,
    #[doc = "The details for events with this `type`."]
    pub service_account_updated: AuditLogServiceAccountUpdated,
    pub type_: AuditLogEventType,
    #[doc = "The details for events with this `type`."]
    pub user_added: AuditLogUserAdded,
    #[doc = "The details for events with this `type`."]
    pub user_deleted: AuditLogUserDeleted,
    #[doc = "The details for events with this `type`."]
    pub user_updated: AuditLogUserUpdated,
}
#[doc = "The payload used to create the API key."]
pub struct AuditLogApiKeyCreatedData {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    pub scopes: Vec<String>,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogApiKeyCreated {
    #[doc = "The payload used to create the API key."]
    pub data: AuditLogApiKeyCreatedData,
    #[doc = "The tracking ID of the API key."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogApiKeyDeleted {
    #[doc = "The tracking ID of the API key."]
    pub id: String,
}
#[doc = "The payload used to update the API key."]
pub struct AuditLogApiKeyUpdatedChangesRequested {
    #[doc = "A list of scopes allowed for the API key, e.g. `[\"api.model.request\"]`"]
    pub scopes: Vec<String>,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogApiKeyUpdated {
    #[doc = "The payload used to update the API key."]
    pub changes_requested: AuditLogApiKeyUpdatedChangesRequested,
    #[doc = "The tracking ID of the API key."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCertificateCreated {
    #[doc = "The certificate ID."]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCertificateDeleted {
    #[doc = "The certificate content in PEM format."]
    pub certificate: String,
    #[doc = "The certificate ID."]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCertificateUpdated {
    #[doc = "The certificate ID."]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
}
pub struct AuditLogCertificatesActivatedCertificatesInner {
    #[doc = "The certificate ID."]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCertificatesActivated {
    pub certificates: Vec<AuditLogCertificatesActivatedCertificatesInner>,
}
pub struct AuditLogCertificatesDeactivatedCertificatesInner {
    #[doc = "The certificate ID."]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCertificatesDeactivated {
    pub certificates: Vec<AuditLogCertificatesDeactivatedCertificatesInner>,
}
#[doc = "The payload used to create the checkpoint permission."]
pub struct AuditLogCheckpointPermissionCreatedData {
    #[doc = "The ID of the fine-tuned model checkpoint."]
    pub fine_tuned_model_checkpoint: String,
    #[doc = "The ID of the project that the checkpoint permission was created for."]
    pub project_id: String,
}
#[doc = "The project and fine-tuned model checkpoint that the checkpoint permission was created for."]
pub struct AuditLogCheckpointPermissionCreated {
    #[doc = "The payload used to create the checkpoint permission."]
    pub data: AuditLogCheckpointPermissionCreatedData,
    #[doc = "The ID of the checkpoint permission."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogCheckpointPermissionDeleted {
    #[doc = "The ID of the checkpoint permission."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogInviteAccepted {
    #[doc = "The ID of the invite."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogInviteDeleted {
    #[doc = "The ID of the invite."]
    pub id: String,
}
#[doc = "The payload used to create the invite."]
pub struct AuditLogInviteSentData {
    #[doc = "The email invited to the organization."]
    pub email: String,
    #[doc = "The role the email was invited to be. Is either `owner` or `member`."]
    pub role: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogInviteSent {
    #[doc = "The payload used to create the invite."]
    pub data: AuditLogInviteSentData,
    #[doc = "The ID of the invite."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogLoginFailed {
    #[doc = "The error code of the failure."]
    pub error_code: String,
    #[doc = "The error message of the failure."]
    pub error_message: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogLogoutFailed {
    #[doc = "The error code of the failure."]
    pub error_code: String,
    #[doc = "The error message of the failure."]
    pub error_message: String,
}
pub struct AuditLogOrganizationUpdatedChangesRequestedSettings {
    #[doc = "Visibility of the threads page which shows messages created with the Assistants API and Playground. One of `ANY_ROLE`, `OWNERS`, or `NONE`."]
    pub threads_ui_visibility: String,
    #[doc = "Visibility of the usage dashboard which shows activity and costs for your organization. One of `ANY_ROLE` or `OWNERS`."]
    pub usage_dashboard_visibility: String,
}
#[doc = "The payload used to update the organization settings."]
pub struct AuditLogOrganizationUpdatedChangesRequested {
    #[doc = "The organization description."]
    pub description: String,
    #[doc = "The organization name."]
    pub name: String,
    pub settings: AuditLogOrganizationUpdatedChangesRequestedSettings,
    #[doc = "The organization title."]
    pub title: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogOrganizationUpdated {
    #[doc = "The payload used to update the organization settings."]
    pub changes_requested: AuditLogOrganizationUpdatedChangesRequested,
    #[doc = "The organization ID."]
    pub id: String,
}
#[doc = "The project that the action was scoped to. Absent for actions not scoped to projects."]
pub struct AuditLogProject {
    #[doc = "The project ID."]
    pub id: String,
    #[doc = "The project title."]
    pub name: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogProjectArchived {
    #[doc = "The project ID."]
    pub id: String,
}
#[doc = "The payload used to create the project."]
pub struct AuditLogProjectCreatedData {
    #[doc = "The project name."]
    pub name: String,
    #[doc = "The title of the project as seen on the dashboard."]
    pub title: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogProjectCreated {
    #[doc = "The payload used to create the project."]
    pub data: AuditLogProjectCreatedData,
    #[doc = "The project ID."]
    pub id: String,
}
#[doc = "The payload used to update the project."]
pub struct AuditLogProjectUpdatedChangesRequested {
    #[doc = "The title of the project as seen on the dashboard."]
    pub title: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogProjectUpdated {
    #[doc = "The payload used to update the project."]
    pub changes_requested: AuditLogProjectUpdatedChangesRequested,
    #[doc = "The project ID."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogRateLimitDeleted {
    #[doc = "The rate limit ID"]
    pub id: String,
}
#[doc = "The payload used to update the rate limits."]
pub struct AuditLogRateLimitUpdatedChangesRequested {
    #[doc = "The maximum batch input tokens per day. Only relevant for certain models."]
    pub batch_1_day_max_input_tokens: u64,
    #[doc = "The maximum audio megabytes per minute. Only relevant for certain models."]
    pub max_audio_megabytes_per_1_minute: u64,
    #[doc = "The maximum images per minute. Only relevant for certain models."]
    pub max_images_per_1_minute: u64,
    #[doc = "The maximum requests per day. Only relevant for certain models."]
    pub max_requests_per_1_day: u64,
    #[doc = "The maximum requests per minute."]
    pub max_requests_per_1_minute: u64,
    #[doc = "The maximum tokens per minute."]
    pub max_tokens_per_1_minute: u64,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogRateLimitUpdated {
    #[doc = "The payload used to update the rate limits."]
    pub changes_requested: AuditLogRateLimitUpdatedChangesRequested,
    #[doc = "The rate limit ID"]
    pub id: String,
}
#[doc = "The payload used to create the service account."]
pub struct AuditLogServiceAccountCreatedData {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    pub role: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogServiceAccountCreated {
    #[doc = "The payload used to create the service account."]
    pub data: AuditLogServiceAccountCreatedData,
    #[doc = "The service account ID."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogServiceAccountDeleted {
    #[doc = "The service account ID."]
    pub id: String,
}
#[doc = "The payload used to updated the service account."]
pub struct AuditLogServiceAccountUpdatedChangesRequested {
    #[doc = "The role of the service account. Is either `owner` or `member`."]
    pub role: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogServiceAccountUpdated {
    #[doc = "The payload used to updated the service account."]
    pub changes_requested: AuditLogServiceAccountUpdatedChangesRequested,
    #[doc = "The service account ID."]
    pub id: String,
}
#[doc = "The payload used to add the user to the project."]
pub struct AuditLogUserAddedData {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    pub role: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogUserAdded {
    #[doc = "The payload used to add the user to the project."]
    pub data: AuditLogUserAddedData,
    #[doc = "The user ID."]
    pub id: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogUserDeleted {
    #[doc = "The user ID."]
    pub id: String,
}
#[doc = "The payload used to update the user."]
pub struct AuditLogUserUpdatedChangesRequested {
    #[doc = "The role of the user. Is either `owner` or `member`."]
    pub role: String,
}
#[doc = "The details for events with this `type`."]
pub struct AuditLogUserUpdated {
    #[doc = "The payload used to update the user."]
    pub changes_requested: AuditLogUserUpdatedChangesRequested,
    #[doc = "The project ID."]
    pub id: String,
}
#[doc = "The actor who performed the audit logged action."]
pub struct AuditLogActor {
    pub api_key: AuditLogActorApiKey,
    pub session: AuditLogActorSession,
    #[doc = "The type of actor. Is either `session` or `api_key`."]
    pub type_: AuditLogActorType,
}
#[doc = "The type of actor. Is either `session` or `api_key`."]
pub enum AuditLogActorType {
    Session,
    ApiKey,
}
#[doc = "The API Key used to perform the audit logged action."]
pub struct AuditLogActorApiKey {
    #[doc = "The tracking id of the API key."]
    pub id: String,
    pub service_account: AuditLogActorServiceAccount,
    #[doc = "The type of API key. Can be either `user` or `service_account`."]
    pub type_: AuditLogActorApiKeyType,
    pub user: AuditLogActorUser,
}
#[doc = "The type of API key. Can be either `user` or `service_account`."]
pub enum AuditLogActorApiKeyType {
    User,
    ServiceAccount,
}
#[doc = "The service account that performed the audit logged action."]
pub struct AuditLogActorServiceAccount {
    #[doc = "The service account id."]
    pub id: String,
}
#[doc = "The session in which the audit logged action was performed."]
pub struct AuditLogActorSession {
    #[doc = "The IP address from which the action was performed."]
    pub ip_address: String,
    pub user: AuditLogActorUser,
}
#[doc = "The user who performed the audit logged action."]
pub struct AuditLogActorUser {
    #[doc = "The user email."]
    pub email: String,
    #[doc = "The user id."]
    pub id: String,
}
#[doc = "The event type."]
pub enum AuditLogEventType {
    ApiKeyCreated,
    ApiKeyUpdated,
    ApiKeyDeleted,
    CheckpointPermissionCreated,
    CheckpointPermissionDeleted,
    InviteSent,
    InviteAccepted,
    InviteDeleted,
    LoginSucceeded,
    LoginFailed,
    LogoutSucceeded,
    LogoutFailed,
    OrganizationUpdated,
    ProjectCreated,
    ProjectUpdated,
    ProjectArchived,
    ServiceAccountCreated,
    ServiceAccountUpdated,
    ServiceAccountDeleted,
    RateLimitUpdated,
    RateLimitDeleted,
    UserAdded,
    UserUpdated,
    UserDeleted,
}
#[doc = "The default strategy. This strategy currently uses a `max_chunk_size_tokens` of `800` and `chunk_overlap_tokens` of `400`."]
pub struct AutoChunkingStrategyRequestParam {
    #[doc = "Always `auto`."]
    pub type_: AutoChunkingStrategyRequestParamType,
}
#[doc = "Always `auto`."]
pub enum AutoChunkingStrategyRequestParamType {
    Auto,
}
pub struct Batch {
    #[doc = "The Unix timestamp (in seconds) for when the batch was cancelled."]
    pub cancelled_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch started cancelling."]
    pub cancelling_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch was completed."]
    pub completed_at: u64,
    #[doc = "The time frame within which the batch should be processed."]
    pub completion_window: String,
    #[doc = "The Unix timestamp (in seconds) for when the batch was created."]
    pub created_at: u64,
    #[doc = "The OpenAI API endpoint used by the batch."]
    pub endpoint: String,
    #[doc = "The ID of the file containing the outputs of requests with errors."]
    pub error_file_id: String,
    pub errors: BatchErrors,
    #[doc = "The Unix timestamp (in seconds) for when the batch expired."]
    pub expired_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch will expire."]
    pub expires_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch failed."]
    pub failed_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the batch started finalizing."]
    pub finalizing_at: u64,
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the batch started processing."]
    pub in_progress_at: u64,
    #[doc = "The ID of the input file for the batch."]
    pub input_file_id: String,
    pub metadata: Metadata,
    #[doc = "The object type, which is always `batch`."]
    pub object: BatchObject,
    #[doc = "The ID of the file containing the outputs of successfully executed requests."]
    pub output_file_id: String,
    #[doc = "The request counts for different statuses within the batch."]
    pub request_counts: BatchRequestCounts,
    #[doc = "The current status of the batch."]
    pub status: BatchStatus,
}
pub struct BatchErrorsDataInner {
    #[doc = "An error code identifying the error type."]
    pub code: String,
    #[doc = "The line number of the input file where the error occurred, if applicable."]
    pub line: u64,
    #[doc = "A human-readable message providing more details about the error."]
    pub message: String,
    #[doc = "The name of the parameter that caused the error, if applicable."]
    pub param: String,
}
pub struct BatchErrors {
    pub data: Vec<BatchErrorsDataInner>,
    #[doc = "The object type, which is always `list`."]
    pub object: String,
}
#[doc = "The object type, which is always `batch`."]
pub enum BatchObject {
    Batch,
}
#[doc = "The request counts for different statuses within the batch."]
pub struct BatchRequestCounts {
    #[doc = "Number of requests that have been completed successfully."]
    pub completed: u64,
    #[doc = "Number of requests that have failed."]
    pub failed: u64,
    #[doc = "Total number of requests in the batch."]
    pub total: u64,
}
#[doc = "The current status of the batch."]
pub enum BatchStatus {
    Validating,
    Failed,
    InProgress,
    Finalizing,
    Completed,
    Expired,
    Cancelling,
    Cancelled,
}
#[doc = "The per-line object of the batch input file"]
pub struct BatchRequestInput {
    #[doc = "A developer-provided per-request id that will be used to match outputs to inputs. Must be unique for each request in a batch."]
    pub custom_id: String,
    #[doc = "The HTTP method to be used for the request. Currently only `POST` is supported."]
    pub method: BatchRequestInputMethod,
    #[doc = "The OpenAI API relative URL to be used for the request. Currently `/v1/chat/completions`, `/v1/embeddings`, and `/v1/completions` are supported."]
    pub url: String,
}
#[doc = "The HTTP method to be used for the request. Currently only `POST` is supported."]
pub enum BatchRequestInputMethod {
    Post,
}
#[doc = "The per-line object of the batch output and error files"]
pub struct BatchRequestOutput {
    #[doc = "A developer-provided per-request id that will be used to match outputs to inputs."]
    pub custom_id: String,
    #[doc = "For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure."]
    pub error: BatchRequestOutputError,
    pub id: String,
    pub response: BatchRequestOutputResponse,
}
#[doc = "For requests that failed with a non-HTTP error, this will contain more information on the cause of the failure."]
pub struct BatchRequestOutputError {
    #[doc = "A machine-readable error code."]
    pub code: String,
    #[doc = "A human-readable error message."]
    pub message: String,
}
pub struct BatchRequestOutputResponse {
    #[doc = "The JSON body of the response"]
    pub body: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "An unique identifier for the OpenAI API request. Please include this request ID when contacting support."]
    pub request_id: String,
    #[doc = "The HTTP status code of the response"]
    pub status_code: u64,
}
#[doc = "Represents an individual `certificate` uploaded to the organization."]
pub struct Certificate {
    #[doc = "Whether the certificate is currently active at the specified scope. Not returned when getting details for a specific certificate."]
    pub active: Vec<bool>,
    pub certificate_details: CertificateCertificateDetails,
    #[doc = "The Unix timestamp (in seconds) of when the certificate was uploaded."]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the certificate."]
    pub name: String,
    #[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
    pub object: CertificateObject,
}
pub struct CertificateCertificateDetails {
    #[doc = "The content of the certificate in PEM format."]
    pub content: String,
    #[doc = "The Unix timestamp (in seconds) of when the certificate expires."]
    pub expires_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the certificate becomes valid."]
    pub valid_at: u64,
}
#[doc = "The object type.\n\n- If creating, updating, or getting a specific certificate, the object type is `certificate`.\n- If listing, activating, or deactivating certificates for the organization, the object type is `organization.certificate`.\n- If listing, activating, or deactivating certificates for a project, the object type is `organization.project.certificate`.\n"]
pub enum CertificateObject {
    Certificate,
    OrganizationCertificate,
    OrganizationProjectCertificate,
}
pub struct ChatCompletionDeleted {
    #[doc = "Whether the chat completion was deleted."]
    pub deleted: Vec<bool>,
    #[doc = "The ID of the chat completion that was deleted."]
    pub id: String,
    #[doc = "The type of object being deleted."]
    pub object: ChatCompletionDeletedObject,
}
#[doc = "The type of object being deleted."]
pub enum ChatCompletionDeletedObject {
    ChatCompletionDeleted,
}
#[doc = "Specifying a particular function via `{\"name\": \"my_function\"}` forces the model to call that function.\n"]
pub struct ChatCompletionFunctionCallOption {
    #[doc = "The name of the function to call."]
    pub name: String,
}
pub struct ChatCompletionFunctions {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    pub description: String,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    pub name: String,
    pub parameters: FunctionParameters,
}
#[doc = "An object representing a list of Chat Completions.\n"]
pub struct ChatCompletionList {
    #[doc = "An array of chat completion objects.\n"]
    pub data: Vec<CreateChatCompletionResponse>,
    #[doc = "The identifier of the first chat completion in the data array."]
    pub first_id: String,
    #[doc = "Indicates whether there are more Chat Completions available."]
    pub has_more: Vec<bool>,
    #[doc = "The identifier of the last chat completion in the data array."]
    pub last_id: String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: ChatCompletionListObject,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
pub enum ChatCompletionListObject {
    List,
}
#[doc = "An object representing a list of chat completion messages.\n"]
pub struct ChatCompletionMessageList {
    #[doc = "An array of chat completion message objects.\n"]
    pub data: Vec<ChatCompletionMessageListDataInner>,
    #[doc = "The identifier of the first chat message in the data array."]
    pub first_id: String,
    #[doc = "Indicates whether there are more chat messages available."]
    pub has_more: Vec<bool>,
    #[doc = "The identifier of the last chat message in the data array."]
    pub last_id: String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: ChatCompletionMessageListObject,
}
pub struct ChatCompletionMessageListDataInner1 {
    #[doc = "The identifier of the chat message."]
    pub id: String,
}
pub struct ChatCompletionMessageListDataInner {
    pub all_of_0: ChatCompletionResponseMessage,
    pub all_of_1: ChatCompletionMessageListDataInner1,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
pub enum ChatCompletionMessageListObject {
    List,
}
pub struct ChatCompletionMessageToolCall {
    #[doc = "The function that the model called."]
    pub function: ChatCompletionMessageToolCallFunction,
    #[doc = "The ID of the tool call."]
    pub id: String,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    pub type_: ChatCompletionMessageToolCallType,
}
#[doc = "The function that the model called."]
pub struct ChatCompletionMessageToolCallFunction {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
pub enum ChatCompletionMessageToolCallType {
    Function,
}
pub struct ChatCompletionMessageToolCallChunk {
    pub function: ChatCompletionMessageToolCallChunkFunction,
    #[doc = "The ID of the tool call."]
    pub id: String,
    pub index: u64,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    pub type_: ChatCompletionMessageToolCallChunkType,
}
pub struct ChatCompletionMessageToolCallChunkFunction {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
pub enum ChatCompletionMessageToolCallChunkType {
    Function,
}
#[doc = "The tool calls generated by the model, such as function calls."]
pub type ChatCompletionMessageToolCalls = Vec<ChatCompletionMessageToolCall>;
#[doc = "Output types that you would like the model to generate for this request.\nMost models are capable of generating text, which is the default:\n\n`[\"text\"]`\n\nThe `gpt-4o-audio-preview` model can also be used to [generate audio](/docs/guides/audio). To\nrequest that this model generate both text and audio responses, you can\nuse:\n\n`[\"text\", \"audio\"]`\n"]
pub type ChatCompletionModalities = Vec<ChatCompletionModalitiesInner>;
pub enum ChatCompletionModalitiesInner {
    Text,
    Audio,
}
#[doc = "Specifies a tool the model should use. Use to force the model to call a specific function."]
pub struct ChatCompletionNamedToolChoice {
    pub function: ChatCompletionNamedToolChoiceFunction,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    pub type_: ChatCompletionNamedToolChoiceType,
}
pub struct ChatCompletionNamedToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
pub enum ChatCompletionNamedToolChoiceType {
    Function,
}
#[doc = "Messages sent by the model in response to user messages.\n"]
pub struct ChatCompletionRequestAssistantMessage {
    #[doc = "Data about a previous audio response from the model. \n[Learn more](/docs/guides/audio).\n"]
    pub audio: ChatCompletionRequestAssistantMessageAudio,
    #[doc = "The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.\n"]
    pub content: ChatCompletionRequestAssistantMessageContent,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    pub function_call: ChatCompletionRequestAssistantMessageFunctionCall,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    pub name: String,
    #[doc = "The refusal message by the assistant."]
    pub refusal: String,
    #[doc = "The role of the messages author, in this case `assistant`."]
    pub role: ChatCompletionRequestAssistantMessageRole,
    pub tool_calls: ChatCompletionMessageToolCalls,
}
#[doc = "Data about a previous audio response from the model. \n[Learn more](/docs/guides/audio).\n"]
pub struct ChatCompletionRequestAssistantMessageAudio {
    #[doc = "Unique identifier for a previous audio response from the model.\n"]
    pub id: String,
}
#[doc = "The contents of the assistant message. Required unless `tool_calls` or `function_call` is specified.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestAssistantMessageContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestAssistantMessageContentPart>),
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
pub struct ChatCompletionRequestAssistantMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The role of the messages author, in this case `assistant`."]
pub enum ChatCompletionRequestAssistantMessageRole {
    Assistant,
}
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestAssistantMessageContentPart {
    OneOf0(ChatCompletionRequestMessageContentPartText),
    OneOf1(ChatCompletionRequestMessageContentPartRefusal),
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, `developer` messages\nreplace the previous `system` messages.\n"]
pub struct ChatCompletionRequestDeveloperMessage {
    #[doc = "The contents of the developer message."]
    pub content: ChatCompletionRequestDeveloperMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    pub name: String,
    #[doc = "The role of the messages author, in this case `developer`."]
    pub role: ChatCompletionRequestDeveloperMessageRole,
}
#[doc = "The contents of the developer message."]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestDeveloperMessageContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestMessageContentPartText>),
}
#[doc = "The role of the messages author, in this case `developer`."]
pub enum ChatCompletionRequestDeveloperMessageRole {
    Developer,
}
pub struct ChatCompletionRequestFunctionMessage {
    #[doc = "The contents of the function message."]
    pub content: String,
    #[doc = "The name of the function to call."]
    pub name: String,
    #[doc = "The role of the messages author, in this case `function`."]
    pub role: ChatCompletionRequestFunctionMessageRole,
}
#[doc = "The role of the messages author, in this case `function`."]
pub enum ChatCompletionRequestFunctionMessageRole {
    Function,
}
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestMessage {
    OneOf0(ChatCompletionRequestDeveloperMessage),
    OneOf1(ChatCompletionRequestSystemMessage),
    OneOf2(ChatCompletionRequestUserMessage),
    OneOf3(ChatCompletionRequestAssistantMessage),
    OneOf4(ChatCompletionRequestToolMessage),
    OneOf5(ChatCompletionRequestFunctionMessage),
}
#[doc = "Learn about [audio inputs](/docs/guides/audio).\n"]
pub struct ChatCompletionRequestMessageContentPartAudio {
    pub input_audio: ChatCompletionRequestMessageContentPartAudioInputAudio,
    #[doc = "The type of the content part. Always `input_audio`."]
    pub type_: ChatCompletionRequestMessageContentPartAudioType,
}
#[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
pub enum ChatCompletionRequestMessageContentPartAudioInputAudioFormat {
    Wav,
    Mp3,
}
pub struct ChatCompletionRequestMessageContentPartAudioInputAudio {
    #[doc = "Base64 encoded audio data."]
    pub data: String,
    #[doc = "The format of the encoded audio data. Currently supports \"wav\" and \"mp3\".\n"]
    pub format: ChatCompletionRequestMessageContentPartAudioInputAudioFormat,
}
#[doc = "The type of the content part. Always `input_audio`."]
pub enum ChatCompletionRequestMessageContentPartAudioType {
    InputAudio,
}
#[doc = "Learn about [file inputs](/docs/guides/text) for text generation.\n"]
pub struct ChatCompletionRequestMessageContentPartFile {
    pub file: ChatCompletionRequestMessageContentPartFileFile,
    #[doc = "The type of the content part. Always `file`."]
    pub type_: ChatCompletionRequestMessageContentPartFileType,
}
pub struct ChatCompletionRequestMessageContentPartFileFile {
    #[doc = "The base64 encoded file data, used when passing the file to the model \nas a string.\n"]
    pub file_data: String,
    #[doc = "The ID of an uploaded file to use as input.\n"]
    pub file_id: String,
    #[doc = "The name of the file, used when passing the file to the model as a \nstring.\n"]
    pub filename: String,
}
#[doc = "The type of the content part. Always `file`."]
pub enum ChatCompletionRequestMessageContentPartFileType {
    File,
}
#[doc = "Learn about [image inputs](/docs/guides/vision).\n"]
pub struct ChatCompletionRequestMessageContentPartImage {
    pub image_url: ChatCompletionRequestMessageContentPartImageImageUrl,
    #[doc = "The type of the content part."]
    pub type_: ChatCompletionRequestMessageContentPartImageType,
}
#[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding)."]
pub enum ChatCompletionRequestMessageContentPartImageImageUrlDetail {
    Auto,
    Low,
    High,
}
pub struct ChatCompletionRequestMessageContentPartImageImageUrl {
    #[doc = "Specifies the detail level of the image. Learn more in the [Vision guide](/docs/guides/vision#low-or-high-fidelity-image-understanding)."]
    pub detail: ChatCompletionRequestMessageContentPartImageImageUrlDetail,
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    pub url: String,
}
#[doc = "The type of the content part."]
pub enum ChatCompletionRequestMessageContentPartImageType {
    ImageUrl,
}
pub struct ChatCompletionRequestMessageContentPartRefusal {
    #[doc = "The refusal message generated by the model."]
    pub refusal: String,
    #[doc = "The type of the content part."]
    pub type_: ChatCompletionRequestMessageContentPartRefusalType,
}
#[doc = "The type of the content part."]
pub enum ChatCompletionRequestMessageContentPartRefusalType {
    Refusal,
}
#[doc = "Learn about [text inputs](/docs/guides/text-generation).\n"]
pub struct ChatCompletionRequestMessageContentPartText {
    #[doc = "The text content."]
    pub text: String,
    #[doc = "The type of the content part."]
    pub type_: ChatCompletionRequestMessageContentPartTextType,
}
#[doc = "The type of the content part."]
pub enum ChatCompletionRequestMessageContentPartTextType {
    Text,
}
#[doc = "Developer-provided instructions that the model should follow, regardless of\nmessages sent by the user. With o1 models and newer, use `developer` messages\nfor this purpose instead.\n"]
pub struct ChatCompletionRequestSystemMessage {
    #[doc = "The contents of the system message."]
    pub content: ChatCompletionRequestSystemMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    pub name: String,
    #[doc = "The role of the messages author, in this case `system`."]
    pub role: ChatCompletionRequestSystemMessageRole,
}
#[doc = "The contents of the system message."]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestSystemMessageContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestSystemMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `system`."]
pub enum ChatCompletionRequestSystemMessageRole {
    System,
}
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestSystemMessageContentPart {
    OneOf0(ChatCompletionRequestMessageContentPartText),
}
pub struct ChatCompletionRequestToolMessage {
    #[doc = "The contents of the tool message."]
    pub content: ChatCompletionRequestToolMessageContent,
    #[doc = "The role of the messages author, in this case `tool`."]
    pub role: ChatCompletionRequestToolMessageRole,
    #[doc = "Tool call that this message is responding to."]
    pub tool_call_id: String,
}
#[doc = "The contents of the tool message."]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestToolMessageContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestToolMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `tool`."]
pub enum ChatCompletionRequestToolMessageRole {
    Tool,
}
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestToolMessageContentPart {
    OneOf0(ChatCompletionRequestMessageContentPartText),
}
#[doc = "Messages sent by an end user, containing prompts or additional context\ninformation.\n"]
pub struct ChatCompletionRequestUserMessage {
    #[doc = "The contents of the user message.\n"]
    pub content: ChatCompletionRequestUserMessageContent,
    #[doc = "An optional name for the participant. Provides the model information to differentiate between participants of the same role."]
    pub name: String,
    #[doc = "The role of the messages author, in this case `user`."]
    pub role: ChatCompletionRequestUserMessageRole,
}
#[doc = "The contents of the user message.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestUserMessageContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestUserMessageContentPart>),
}
#[doc = "The role of the messages author, in this case `user`."]
pub enum ChatCompletionRequestUserMessageRole {
    User,
}
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionRequestUserMessageContentPart {
    OneOf0(ChatCompletionRequestMessageContentPartText),
    OneOf1(ChatCompletionRequestMessageContentPartImage),
    OneOf2(ChatCompletionRequestMessageContentPartAudio),
    OneOf3(ChatCompletionRequestMessageContentPartFile),
}
#[doc = "A chat completion message generated by the model."]
pub struct ChatCompletionResponseMessage {
    #[doc = "Annotations for the message, when applicable, as when using the\n[web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
    pub annotations: Vec<ChatCompletionResponseMessageAnnotationsInner>,
    #[doc = "If the audio output modality is requested, this object contains data\nabout the audio response from the model. [Learn more](/docs/guides/audio).\n"]
    pub audio: ChatCompletionResponseMessageAudio,
    #[doc = "The contents of the message."]
    pub content: String,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    pub function_call: ChatCompletionResponseMessageFunctionCall,
    #[doc = "The refusal message generated by the model."]
    pub refusal: String,
    #[doc = "The role of the author of this message."]
    pub role: ChatCompletionResponseMessageRole,
    pub tool_calls: ChatCompletionMessageToolCalls,
}
#[doc = "The type of the URL citation. Always `url_citation`."]
pub enum ChatCompletionResponseMessageAnnotationsInnerType {
    UrlCitation,
}
#[doc = "A URL citation when using web search."]
pub struct ChatCompletionResponseMessageAnnotationsInnerUrlCitation {
    #[doc = "The index of the last character of the URL citation in the message."]
    pub end_index: u64,
    #[doc = "The index of the first character of the URL citation in the message."]
    pub start_index: u64,
    #[doc = "The title of the web resource."]
    pub title: String,
    #[doc = "The URL of the web resource."]
    pub url: String,
}
#[doc = "A URL citation when using web search.\n"]
pub struct ChatCompletionResponseMessageAnnotationsInner {
    #[doc = "The type of the URL citation. Always `url_citation`."]
    pub type_: ChatCompletionResponseMessageAnnotationsInnerType,
    #[doc = "A URL citation when using web search."]
    pub url_citation: ChatCompletionResponseMessageAnnotationsInnerUrlCitation,
}
#[doc = "If the audio output modality is requested, this object contains data\nabout the audio response from the model. [Learn more](/docs/guides/audio).\n"]
pub struct ChatCompletionResponseMessageAudio {
    #[doc = "Base64 encoded audio bytes generated by the model, in the format\nspecified in the request.\n"]
    pub data: String,
    #[doc = "The Unix timestamp (in seconds) for when this audio response will\nno longer be accessible on the server for use in multi-turn\nconversations.\n"]
    pub expires_at: u64,
    #[doc = "Unique identifier for this audio response."]
    pub id: String,
    #[doc = "Transcript of the audio generated by the model."]
    pub transcript: String,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
pub struct ChatCompletionResponseMessageFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The role of the author of this message."]
pub enum ChatCompletionResponseMessageRole {
    Assistant,
}
#[doc = "The role of the author of a message"]
pub enum ChatCompletionRole {
    Developer,
    System,
    User,
    Assistant,
    Tool,
    Function,
}
#[doc = "Options for streaming response. Only set this when you set `stream: true`.\n"]
pub struct ChatCompletionStreamOptions {
    #[doc = "If set, an additional chunk will be streamed before the `data: [DONE]`\nmessage. The `usage` field on this chunk shows the token usage statistics\nfor the entire request, and the `choices` field will always be an empty\narray. \n\nAll other chunks will also include a `usage` field, but with a null\nvalue. **NOTE:** If the stream is interrupted, you may not receive the\nfinal usage chunk which contains the total token usage for the request.\n"]
    pub include_usage: Vec<bool>,
}
#[doc = "A chat completion delta generated by streamed model responses."]
pub struct ChatCompletionStreamResponseDelta {
    #[doc = "The contents of the chunk message."]
    pub content: String,
    #[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
    pub function_call: ChatCompletionStreamResponseDeltaFunctionCall,
    #[doc = "The refusal message generated by the model."]
    pub refusal: String,
    #[doc = "The role of the author of this message."]
    pub role: ChatCompletionStreamResponseDeltaRole,
    pub tool_calls: Vec<ChatCompletionMessageToolCallChunk>,
}
#[doc = "Deprecated and replaced by `tool_calls`. The name and arguments of a function that should be called, as generated by the model."]
pub struct ChatCompletionStreamResponseDeltaFunctionCall {
    #[doc = "The arguments to call the function with, as generated by the model in JSON format. Note that the model does not always generate valid JSON, and may hallucinate parameters not defined by your function schema. Validate the arguments in your code before calling your function."]
    pub arguments: String,
    #[doc = "The name of the function to call."]
    pub name: String,
}
#[doc = "The role of the author of this message."]
pub enum ChatCompletionStreamResponseDeltaRole {
    Developer,
    System,
    User,
    Assistant,
    Tool,
}
pub struct ChatCompletionTokenLogprob {
    #[doc = "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token."]
    pub bytes: Vec<u64>,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    pub logprob: f64,
    #[doc = "The token."]
    pub token: String,
    #[doc = "List of the most likely tokens and their log probability, at this token position. In rare cases, there may be fewer than the number of requested `top_logprobs` returned."]
    pub top_logprobs: Vec<ChatCompletionTokenLogprobTopLogprobsInner>,
}
pub struct ChatCompletionTokenLogprobTopLogprobsInner {
    #[doc = "A list of integers representing the UTF-8 bytes representation of the token. Useful in instances where characters are represented by multiple tokens and their byte representations must be combined to generate the correct text representation. Can be `null` if there is no bytes representation for the token."]
    pub bytes: Vec<u64>,
    #[doc = "The log probability of this token, if it is within the top 20 most likely tokens. Otherwise, the value `-9999.0` is used to signify that the token is very unlikely."]
    pub logprob: f64,
    #[doc = "The token."]
    pub token: String,
}
pub struct ChatCompletionTool {
    pub function: FunctionObject,
    #[doc = "The type of the tool. Currently, only `function` is supported."]
    pub type_: ChatCompletionToolType,
}
#[doc = "The type of the tool. Currently, only `function` is supported."]
pub enum ChatCompletionToolType {
    Function,
}
#[doc = "Controls which (if any) tool is called by the model.\n`none` means the model will not call any tool and instead generates a message.\n`auto` means the model can pick between generating a message or calling one or more tools.\n`required` means the model must call one or more tools.\nSpecifying a particular tool via `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}` forces the model to call that tool.\n\n`none` is the default when no tools are present. `auto` is the default if tools are present.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ChatCompletionToolChoiceOption {
    OneOf0(ChatCompletionToolChoiceOption0),
    OneOf1(ChatCompletionNamedToolChoice),
}
#[doc = "`none` means the model will not call any tool and instead generates a message. `auto` means the model can pick between generating a message or calling one or more tools. `required` means the model must call one or more tools.\n"]
pub enum ChatCompletionToolChoiceOption0 {
    None,
    Auto,
    Required,
}
#[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
pub type ChunkingStrategyRequestParam = std::collections::BTreeMap<String, serde_json::Value>;
#[doc = "A click action.\n"]
pub struct Click {
    #[doc = "Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.\n"]
    pub button: ClickButton,
    #[doc = "Specifies the event type. For a click action, this property is \nalways set to `click`.\n"]
    pub type_: ClickType,
    #[doc = "The x-coordinate where the click occurred.\n"]
    pub x: u64,
    #[doc = "The y-coordinate where the click occurred.\n"]
    pub y: u64,
}
#[doc = "Indicates which mouse button was pressed during the click. One of `left`, `right`, `wheel`, `back`, or `forward`.\n"]
pub enum ClickButton {
    Left,
    Right,
    Wheel,
    Back,
    Forward,
}
#[doc = "Specifies the event type. For a click action, this property is \nalways set to `click`.\n"]
pub enum ClickType {
    Click,
}
#[doc = "The output of a code interpreter tool call that is a file.\n"]
pub struct CodeInterpreterFileOutput {
    pub files: Vec<CodeInterpreterFileOutputFilesInner>,
    #[doc = "The type of the code interpreter file output. Always `files`.\n"]
    pub type_: CodeInterpreterFileOutputType,
}
pub struct CodeInterpreterFileOutputFilesInner {
    #[doc = "The ID of the file.\n"]
    pub file_id: String,
    #[doc = "The MIME type of the file.\n"]
    pub mime_type: String,
}
#[doc = "The type of the code interpreter file output. Always `files`.\n"]
pub enum CodeInterpreterFileOutputType {
    Files,
}
#[doc = "The output of a code interpreter tool call that is text.\n"]
pub struct CodeInterpreterTextOutput {
    #[doc = "The logs of the code interpreter tool call.\n"]
    pub logs: String,
    #[doc = "The type of the code interpreter text output. Always `logs`.\n"]
    pub type_: CodeInterpreterTextOutputType,
}
#[doc = "The type of the code interpreter text output. Always `logs`.\n"]
pub enum CodeInterpreterTextOutputType {
    Logs,
}
#[doc = "A tool call to run code.\n"]
pub struct CodeInterpreterToolCall {
    #[doc = "The code to run.\n"]
    pub code: String,
    #[doc = "The unique ID of the code interpreter tool call.\n"]
    pub id: String,
    #[doc = "The results of the code interpreter tool call.\n"]
    pub results: Vec<CodeInterpreterToolOutput>,
    #[doc = "The status of the code interpreter tool call.\n"]
    pub status: CodeInterpreterToolCallStatus,
    #[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
    pub type_: CodeInterpreterToolCallType,
}
#[doc = "The status of the code interpreter tool call.\n"]
pub enum CodeInterpreterToolCallStatus {
    InProgress,
    Interpreting,
    Completed,
}
#[doc = "The type of the code interpreter tool call. Always `code_interpreter_call`.\n"]
pub enum CodeInterpreterToolCallType {
    CodeInterpreterCall,
}
#[allow(clippy::large_enum_variant)]
pub enum CodeInterpreterToolOutput {
    OneOf0(CodeInterpreterTextOutput),
    OneOf1(CodeInterpreterFileOutput),
}
#[doc = "A filter used to compare a specified attribute key to a given value using a defined comparison operation.\n"]
pub struct ComparisonFilter {
    #[doc = "The key to compare against the value."]
    pub key: String,
    #[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n"]
    pub type_: ComparisonFilterType,
    #[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
    pub value: ComparisonFilterValue,
}
#[doc = "Specifies the comparison operator: `eq`, `ne`, `gt`, `gte`, `lt`, `lte`.\n- `eq`: equals\n- `ne`: not equal\n- `gt`: greater than\n- `gte`: greater than or equal\n- `lt`: less than\n- `lte`: less than or equal\n"]
pub enum ComparisonFilterType {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
}
#[doc = "The value to compare against the attribute key; supports string, number, or boolean types."]
#[allow(clippy::large_enum_variant)]
pub enum ComparisonFilterValue {
    OneOf0(String),
    OneOf1(f64),
    OneOf2(Vec<bool>),
}
pub struct CompleteUploadRequest {
    #[doc = "The optional md5 checksum for the file contents to verify if the bytes uploaded matches what you expect.\n"]
    pub md5: String,
    #[doc = "The ordered list of Part IDs.\n"]
    pub part_ids: Vec<String>,
}
#[doc = "Usage statistics for the completion request."]
pub struct CompletionUsage {
    #[doc = "Number of tokens in the generated completion."]
    pub completion_tokens: u64,
    #[doc = "Breakdown of tokens used in a completion."]
    pub completion_tokens_details: CompletionUsageCompletionTokensDetails,
    #[doc = "Number of tokens in the prompt."]
    pub prompt_tokens: u64,
    #[doc = "Breakdown of tokens used in the prompt."]
    pub prompt_tokens_details: CompletionUsagePromptTokensDetails,
    #[doc = "Total number of tokens used in the request (prompt + completion)."]
    pub total_tokens: u64,
}
#[doc = "Breakdown of tokens used in a completion."]
pub struct CompletionUsageCompletionTokensDetails {
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that appeared in the completion.\n"]
    pub accepted_prediction_tokens: u64,
    #[doc = "Audio input tokens generated by the model."]
    pub audio_tokens: u64,
    #[doc = "Tokens generated by the model for reasoning."]
    pub reasoning_tokens: u64,
    #[doc = "When using Predicted Outputs, the number of tokens in the\nprediction that did not appear in the completion. However, like\nreasoning tokens, these tokens are still counted in the total\ncompletion tokens for purposes of billing, output, and context window\nlimits.\n"]
    pub rejected_prediction_tokens: u64,
}
#[doc = "Breakdown of tokens used in the prompt."]
pub struct CompletionUsagePromptTokensDetails {
    #[doc = "Audio input tokens present in the prompt."]
    pub audio_tokens: u64,
    #[doc = "Cached tokens present in the prompt."]
    pub cached_tokens: u64,
}
#[doc = "Combine multiple filters using `and` or `or`."]
pub struct CompoundFilter {
    #[doc = "Array of filters to combine. Items can be `ComparisonFilter` or `CompoundFilter`."]
    pub filters: Vec<CompoundFilterFiltersInner>,
    #[doc = "Type of operation: `and` or `or`."]
    pub type_: CompoundFilterType,
}
#[allow(clippy::large_enum_variant)]
pub enum CompoundFilterFiltersInner {
    OneOf0(ComparisonFilter),
    OneOf1(CompoundFilter),
}
#[doc = "Type of operation: `and` or `or`."]
pub enum CompoundFilterType {
    And,
    Or,
}
#[allow(clippy::large_enum_variant)]
pub enum ComputerAction {
    OneOf0(Click),
    OneOf1(DoubleClick),
    OneOf2(Drag),
    OneOf3(KeyPress),
    OneOf4(Move),
    OneOf5(Screenshot),
    OneOf6(Scroll),
    OneOf7(Type),
    OneOf8(Wait),
}
#[doc = "The output of a computer tool call."]
pub struct ComputerCallOutputItemParam {
    pub acknowledged_safety_checks: Vec<Vec<ComputerCallSafetyCheckParam>>,
    #[doc = "The ID of the computer tool call that produced the output."]
    pub call_id: String,
    pub id: Vec<String>,
    pub output: ComputerScreenshotImage,
    pub status: Vec<ComputerCallOutputItemParamStatusInner>,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`."]
    pub type_: ComputerCallOutputItemParamType,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or `incomplete`. Populated when input items are returned via API."]
pub enum ComputerCallOutputItemParamStatusInner {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`."]
pub enum ComputerCallOutputItemParamType {
    ComputerCallOutput,
}
#[doc = "A pending safety check for the computer call."]
pub struct ComputerCallSafetyCheckParam {
    pub code: Vec<String>,
    #[doc = "The ID of the pending safety check."]
    pub id: String,
    pub message: Vec<String>,
}
#[doc = "A computer screenshot image used with the computer use tool.\n"]
pub struct ComputerScreenshotImage {
    #[doc = "The identifier of an uploaded file that contains the screenshot."]
    pub file_id: String,
    #[doc = "The URL of the screenshot image."]
    pub image_url: String,
    #[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
    pub type_: ComputerScreenshotImageType,
}
#[doc = "Specifies the event type. For a computer screenshot, this property is \nalways set to `computer_screenshot`.\n"]
pub enum ComputerScreenshotImageType {
    ComputerScreenshot,
}
#[doc = "A tool call to a computer use tool. See the \n[computer use guide](/docs/guides/tools-computer-use) for more information.\n"]
pub struct ComputerToolCall {
    pub action: ComputerAction,
    #[doc = "An identifier used when responding to the tool call with output.\n"]
    pub call_id: String,
    #[doc = "The unique ID of the computer call."]
    pub id: String,
    #[doc = "The pending safety checks for the computer call.\n"]
    pub pending_safety_checks: Vec<ComputerToolCallSafetyCheck>,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: ComputerToolCallStatus,
    #[doc = "The type of the computer call. Always `computer_call`."]
    pub type_: ComputerToolCallType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
pub enum ComputerToolCallStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the computer call. Always `computer_call`."]
pub enum ComputerToolCallType {
    ComputerCall,
}
#[doc = "The output of a computer tool call.\n"]
pub struct ComputerToolCallOutput {
    #[doc = "The safety checks reported by the API that have been acknowledged by the \ndeveloper.\n"]
    pub acknowledged_safety_checks: Vec<ComputerToolCallSafetyCheck>,
    #[doc = "The ID of the computer tool call that produced the output.\n"]
    pub call_id: String,
    #[doc = "The ID of the computer tool call output.\n"]
    pub id: String,
    pub output: ComputerScreenshotImage,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    pub status: ComputerToolCallOutputStatus,
    #[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
    pub type_: ComputerToolCallOutputType,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
pub enum ComputerToolCallOutputStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the computer tool call output. Always `computer_call_output`.\n"]
pub enum ComputerToolCallOutputType {
    ComputerCallOutput,
}
pub struct ComputerToolCallOutputResource {
    pub all_of_0: ComputerToolCallOutput,
    pub all_of_1: ComputerToolCallOutputResource1,
}
pub struct ComputerToolCallOutputResource1 {
    #[doc = "The unique ID of the computer call tool output.\n"]
    pub id: String,
}
#[doc = "A pending safety check for the computer call.\n"]
pub struct ComputerToolCallSafetyCheck {
    #[doc = "The type of the pending safety check."]
    pub code: String,
    #[doc = "The ID of the pending safety check."]
    pub id: String,
    #[doc = "Details about the pending safety check."]
    pub message: String,
}
#[doc = "A tool that controls a virtual computer. Learn more about the [computer tool](https://platform.openai.com/docs/guides/tools-computer-use)."]
pub struct ComputerUsePreviewTool {
    #[doc = "The height of the computer display."]
    pub display_height: u64,
    #[doc = "The width of the computer display."]
    pub display_width: u64,
    #[doc = "The type of computer environment to control."]
    pub environment: ComputerUsePreviewToolEnvironment,
    #[doc = "The type of the computer use tool. Always `computer_use_preview`."]
    pub type_: ComputerUsePreviewToolType,
}
#[doc = "The type of computer environment to control."]
pub enum ComputerUsePreviewToolEnvironment {
    Windows,
    Mac,
    Linux,
    Ubuntu,
    Browser,
}
#[doc = "The type of the computer use tool. Always `computer_use_preview`."]
pub enum ComputerUsePreviewToolType {
    ComputerUsePreview,
}
#[doc = "Multi-modal input and output contents.\n"]
#[allow(clippy::large_enum_variant)]
pub enum Content {
    OneOf0(InputContent),
    OneOf1(OutputContent),
}
#[doc = "An x/y coordinate pair, e.g. `{ x: 100, y: 200 }`.\n"]
pub struct Coordinate {
    #[doc = "The x-coordinate.\n"]
    pub x: u64,
    #[doc = "The y-coordinate.\n"]
    pub y: u64,
}
#[doc = "The aggregated costs details of the specific time bucket."]
pub struct CostsResult {
    #[doc = "The monetary value in its associated currency."]
    pub amount: CostsResultAmount,
    #[doc = "When `group_by=line_item`, this field provides the line item of the grouped costs result."]
    pub line_item: String,
    pub object: CostsResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped costs result."]
    pub project_id: String,
}
#[doc = "The monetary value in its associated currency."]
pub struct CostsResultAmount {
    #[doc = "Lowercase ISO-4217 currency e.g. \"usd\""]
    pub currency: String,
    #[doc = "The numeric value of the cost."]
    pub value: f64,
}
pub enum CostsResultObject {
    OrganizationCostsResult,
}
pub struct CreateAssistantRequest {
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    pub description: String,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    pub instructions: String,
    pub metadata: Metadata,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    pub model: CreateAssistantRequestModel,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    pub name: String,
    pub reasoning_effort: ReasoningEffort,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    pub temperature: f64,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: CreateAssistantRequestToolResources,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    pub tools: Vec<CreateAssistantRequestToolsInner>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    pub top_p: f64,
}
#[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestModel {
    AnyOf0(String),
    AnyOf1(AssistantSupportedModels),
}
pub struct CreateAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct CreateAssistantRequestToolResourcesFileSearchVectorStoresInner {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    pub chunking_strategy: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    pub file_ids: Vec<String>,
    pub metadata: Metadata,
}
pub struct CreateAssistantRequestToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    pub vector_store_ids: Vec<String>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    pub vector_stores: Vec<CreateAssistantRequestToolResourcesFileSearchVectorStoresInner>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct CreateAssistantRequestToolResources {
    pub code_interpreter: CreateAssistantRequestToolResourcesCodeInterpreter,
    pub file_search: CreateAssistantRequestToolResourcesFileSearch,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateAssistantRequestToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
pub struct CreateChatCompletionRequest {
    pub all_of_0: CreateModelResponseProperties,
    pub all_of_1: CreateChatCompletionRequest1,
}
#[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
pub enum CreateChatCompletionRequest1AudioFormat {
    Wav,
    Aac,
    Mp3,
    Flac,
    Opus,
    Pcm16,
}
#[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](/docs/guides/audio).\n"]
pub struct CreateChatCompletionRequest1Audio {
    #[doc = "Specifies the output audio format. Must be one of `wav`, `mp3`, `flac`,\n`opus`, or `pcm16`.\n"]
    pub format: CreateChatCompletionRequest1AudioFormat,
    #[doc = "The voice the model uses to respond. Supported voices are \n`alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `nova`, `onyx`, `sage`, and `shimmer`.\n"]
    pub voice: VoiceIdsShared,
}
#[doc = "`none` means the model will not call a function and instead generates a message. `auto` means the model can pick between generating a message or calling a function.\n"]
pub enum CreateChatCompletionRequest1FunctionCall0 {
    None,
    Auto,
}
#[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequest1FunctionCall {
    OneOf0(CreateChatCompletionRequest1FunctionCall0),
    OneOf1(ChatCompletionFunctionCallOption),
}
#[doc = "Configuration for a [Predicted Output](/docs/guides/predicted-outputs),\nwhich can greatly improve response times when large parts of the model\nresponse are known ahead of time. This is most common when you are\nregenerating a file with only minor changes to most of the content.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequest1Prediction {
    OneOf0(PredictionContent),
}
#[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateChatCompletionRequest1ResponseFormat {
    OneOf0(ResponseFormatText),
    OneOf1(ResponseFormatJsonSchema),
    OneOf2(ResponseFormatJsonObject),
}
#[doc = "The type of location approximation. Always `approximate`.\n"]
pub enum CreateChatCompletionRequest1WebSearchOptionsUserLocationType {
    Approximate,
}
#[doc = "Approximate location parameters for the search.\n"]
pub struct CreateChatCompletionRequest1WebSearchOptionsUserLocation {
    pub approximate: WebSearchLocation,
    #[doc = "The type of location approximation. Always `approximate`.\n"]
    pub type_: CreateChatCompletionRequest1WebSearchOptionsUserLocationType,
}
#[doc = "This tool searches the web for relevant results to use in a response.\nLearn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
pub struct CreateChatCompletionRequest1WebSearchOptions {
    pub search_context_size: WebSearchContextSize,
    #[doc = "Approximate location parameters for the search.\n"]
    pub user_location: CreateChatCompletionRequest1WebSearchOptionsUserLocation,
}
pub struct CreateChatCompletionRequest1 {
    #[doc = "Parameters for audio output. Required when audio output is requested with\n`modalities: [\"audio\"]`. [Learn more](/docs/guides/audio).\n"]
    pub audio: CreateChatCompletionRequest1Audio,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\ntheir existing frequency in the text so far, decreasing the model's\nlikelihood to repeat the same line verbatim.\n"]
    pub frequency_penalty: f64,
    #[doc = "Deprecated in favor of `tool_choice`.\n\nControls which (if any) function is called by the model.\n\n`none` means the model will not call a function and instead generates a\nmessage.\n\n`auto` means the model can pick between generating a message or calling a\nfunction.\n\nSpecifying a particular function via `{\"name\": \"my_function\"}` forces the\nmodel to call that function.\n\n`none` is the default when no functions are present. `auto` is the default\nif functions are present.\n"]
    pub function_call: CreateChatCompletionRequest1FunctionCall,
    #[doc = "Deprecated in favor of `tools`.\n\nA list of functions the model may generate JSON inputs for.\n"]
    pub functions: Vec<ChatCompletionFunctions>,
    #[doc = "Modify the likelihood of specified tokens appearing in the completion.\n\nAccepts a JSON object that maps tokens (specified by their token ID in the\ntokenizer) to an associated bias value from -100 to 100. Mathematically,\nthe bias is added to the logits generated by the model prior to sampling.\nThe exact effect will vary per model, but values between -1 and 1 should\ndecrease or increase likelihood of selection; values like -100 or 100\nshould result in a ban or exclusive selection of the relevant token.\n"]
    pub logit_bias: std::collections::BTreeMap<String, u64>,
    #[doc = "Whether to return log probabilities of the output tokens or not. If true,\nreturns the log probabilities of each output token returned in the\n`content` of `message`.\n"]
    pub logprobs: Vec<bool>,
    #[doc = "An upper bound for the number of tokens that can be generated for a completion, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).\n"]
    pub max_completion_tokens: u64,
    #[doc = "The maximum number of [tokens](/tokenizer) that can be generated in the\nchat completion. This value can be used to control\n[costs](https://openai.com/api/pricing/) for text generated via API.\n\nThis value is now deprecated in favor of `max_completion_tokens`, and is\nnot compatible with [o-series models](/docs/guides/reasoning).\n"]
    pub max_tokens: u64,
    #[doc = "A list of messages comprising the conversation so far. Depending on the\n[model](/docs/models) you use, different message types (modalities) are\nsupported, like [text](/docs/guides/text-generation),\n[images](/docs/guides/vision), and [audio](/docs/guides/audio).\n"]
    pub messages: Vec<ChatCompletionRequestMessage>,
    pub modalities: ResponseModalities,
    #[doc = "Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](/docs/models)\nto browse and compare available models.\n"]
    pub model: ModelIdsShared,
    #[doc = "How many chat completion choices to generate for each input message. Note that you will be charged based on the number of generated tokens across all of the choices. Keep `n` as `1` to minimize costs."]
    pub n: u64,
    pub parallel_tool_calls: ParallelToolCalls,
    #[doc = "Configuration for a [Predicted Output](/docs/guides/predicted-outputs),\nwhich can greatly improve response times when large parts of the model\nresponse are known ahead of time. This is most common when you are\nregenerating a file with only minor changes to most of the content.\n"]
    pub prediction: CreateChatCompletionRequest1Prediction,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on\nwhether they appear in the text so far, increasing the model's likelihood\nto talk about new topics.\n"]
    pub presence_penalty: f64,
    pub reasoning_effort: ReasoningEffort,
    #[doc = "An object specifying the format that the model must output.\n\nSetting to `{ \"type\": \"json_schema\", \"json_schema\": {...} }` enables\nStructured Outputs which ensures the model will match your supplied JSON\nschema. Learn more in the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
    pub response_format: CreateChatCompletionRequest1ResponseFormat,
    #[doc = "This feature is in Beta.\nIf specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.\nDeterminism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n"]
    pub seed: u64,
    pub stop: StopConfiguration,
    #[doc = "Whether or not to store the output of this chat completion request for \nuse in our [model distillation](/docs/guides/distillation) or\n[evals](/docs/guides/evals) products.\n"]
    pub store: Vec<bool>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\nSee the [Streaming section below](/docs/api-reference/chat/streaming)\nfor more information, along with the [streaming responses](/docs/guides/streaming-responses)\nguide for more information on how to handle the streaming events.\n"]
    pub stream: Vec<bool>,
    pub stream_options: ChatCompletionStreamOptions,
    pub tool_choice: ChatCompletionToolChoiceOption,
    #[doc = "A list of tools the model may call. Currently, only functions are supported as a tool. Use this to provide a list of functions the model may generate JSON inputs for. A max of 128 functions are supported.\n"]
    pub tools: Vec<ChatCompletionTool>,
    #[doc = "An integer between 0 and 20 specifying the number of most likely tokens to\nreturn at each token position, each with an associated log probability.\n`logprobs` must be set to `true` if this parameter is used.\n"]
    pub top_logprobs: u64,
    #[doc = "This tool searches the web for relevant results to use in a response.\nLearn more about the [web search tool](/docs/guides/tools-web-search?api-mode=chat).\n"]
    pub web_search_options: CreateChatCompletionRequest1WebSearchOptions,
}
#[doc = "Represents a chat completion response returned by model, based on the provided input."]
pub struct CreateChatCompletionResponse {
    #[doc = "A list of chat completion choices. Can be more than one if `n` is greater than 1."]
    pub choices: Vec<CreateChatCompletionResponseChoicesInner>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created."]
    pub created: u64,
    #[doc = "A unique identifier for the chat completion."]
    pub id: String,
    #[doc = "The model used for the chat completion."]
    pub model: String,
    #[doc = "The object type, which is always `chat.completion`."]
    pub object: CreateChatCompletionResponseObject,
    pub service_tier: ServiceTier,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\n\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    pub system_fingerprint: String,
    pub usage: CompletionUsage,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
pub enum CreateChatCompletionResponseChoicesInnerFinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    FunctionCall,
}
#[doc = "Log probability information for the choice."]
pub struct CreateChatCompletionResponseChoicesInnerLogprobs {
    #[doc = "A list of message content tokens with log probability information."]
    pub content: Vec<ChatCompletionTokenLogprob>,
    #[doc = "A list of message refusal tokens with log probability information."]
    pub refusal: Vec<ChatCompletionTokenLogprob>,
}
pub struct CreateChatCompletionResponseChoicesInner {
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
    pub finish_reason: CreateChatCompletionResponseChoicesInnerFinishReason,
    #[doc = "The index of the choice in the list of choices."]
    pub index: u64,
    #[doc = "Log probability information for the choice."]
    pub logprobs: CreateChatCompletionResponseChoicesInnerLogprobs,
    pub message: ChatCompletionResponseMessage,
}
#[doc = "The object type, which is always `chat.completion`."]
pub enum CreateChatCompletionResponseObject {
    ChatCompletion,
}
#[doc = "Represents a streamed chunk of a chat completion response returned\nby the model, based on the provided input. \n[Learn more](/docs/guides/streaming-responses).\n"]
pub struct CreateChatCompletionStreamResponse {
    #[doc = "A list of chat completion choices. Can contain more than one elements if `n` is greater than 1. Can also be empty for the\nlast chunk if you set `stream_options: {\"include_usage\": true}`.\n"]
    pub choices: Vec<CreateChatCompletionStreamResponseChoicesInner>,
    #[doc = "The Unix timestamp (in seconds) of when the chat completion was created. Each chunk has the same timestamp."]
    pub created: u64,
    #[doc = "A unique identifier for the chat completion. Each chunk has the same ID."]
    pub id: String,
    #[doc = "The model to generate the completion."]
    pub model: String,
    #[doc = "The object type, which is always `chat.completion.chunk`."]
    pub object: CreateChatCompletionStreamResponseObject,
    pub service_tier: ServiceTier,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    pub system_fingerprint: String,
    #[doc = "An optional field that will only be present when you set\n`stream_options: {\"include_usage\": true}` in your request. When present, it\ncontains a null value **except for the last chunk** which contains the\ntoken usage statistics for the entire request.\n\n**NOTE:** If the stream is interrupted or cancelled, you may not\nreceive the final usage chunk which contains the total token usage for\nthe request.\n"]
    pub usage: CompletionUsage,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
pub enum CreateChatCompletionStreamResponseChoicesInnerFinishReason {
    Stop,
    Length,
    ToolCalls,
    ContentFilter,
    FunctionCall,
}
#[doc = "Log probability information for the choice."]
pub struct CreateChatCompletionStreamResponseChoicesInnerLogprobs {
    #[doc = "A list of message content tokens with log probability information."]
    pub content: Vec<ChatCompletionTokenLogprob>,
    #[doc = "A list of message refusal tokens with log probability information."]
    pub refusal: Vec<ChatCompletionTokenLogprob>,
}
pub struct CreateChatCompletionStreamResponseChoicesInner {
    pub delta: ChatCompletionStreamResponseDelta,
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\n`content_filter` if content was omitted due to a flag from our content filters,\n`tool_calls` if the model called a tool, or `function_call` (deprecated) if the model called a function.\n"]
    pub finish_reason: CreateChatCompletionStreamResponseChoicesInnerFinishReason,
    #[doc = "The index of the choice in the list of choices."]
    pub index: u64,
    #[doc = "Log probability information for the choice."]
    pub logprobs: CreateChatCompletionStreamResponseChoicesInnerLogprobs,
}
#[doc = "The object type, which is always `chat.completion.chunk`."]
pub enum CreateChatCompletionStreamResponseObject {
    ChatCompletionChunk,
}
pub struct CreateCompletionRequest {
    #[doc = "Generates `best_of` completions server-side and returns the \"best\" (the one with the highest log probability per token). Results cannot be streamed.\n\nWhen used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return – `best_of` must be greater than `n`.\n\n**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.\n"]
    pub best_of: u64,
    #[doc = "Echo back the prompt in addition to the completion\n"]
    pub echo: Vec<bool>,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.\n\n[See more information about frequency and presence penalties.](/docs/guides/text-generation)\n"]
    pub frequency_penalty: f64,
    #[doc = "Modify the likelihood of specified tokens appearing in the completion.\n\nAccepts a JSON object that maps tokens (specified by their token ID in the GPT tokenizer) to an associated bias value from -100 to 100. You can use this [tokenizer tool](/tokenizer?view=bpe) to convert text to token IDs. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.\n\nAs an example, you can pass `{\"50256\": -100}` to prevent the <|endoftext|> token from being generated.\n"]
    pub logit_bias: std::collections::BTreeMap<String, u64>,
    #[doc = "Include the log probabilities on the `logprobs` most likely output tokens, as well the chosen tokens. For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens. The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.\n\nThe maximum value for `logprobs` is 5.\n"]
    pub logprobs: u64,
    #[doc = "The maximum number of [tokens](/tokenizer) that can be generated in the completion.\n\nThe token count of your prompt plus `max_tokens` cannot exceed the model's context length. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens.\n"]
    pub max_tokens: u64,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    pub model: CreateCompletionRequestModel,
    #[doc = "How many completions to generate for each prompt.\n\n**Note:** Because this parameter generates many completions, it can quickly consume your token quota. Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.\n"]
    pub n: u64,
    #[doc = "Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.\n\n[See more information about frequency and presence penalties.](/docs/guides/text-generation)\n"]
    pub presence_penalty: f64,
    #[doc = "The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.\n\nNote that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.\n"]
    pub prompt: CreateCompletionRequestPrompt,
    #[doc = "If specified, our system will make a best effort to sample deterministically, such that repeated requests with the same `seed` and parameters should return the same result.\n\nDeterminism is not guaranteed, and you should refer to the `system_fingerprint` response parameter to monitor changes in the backend.\n"]
    pub seed: u64,
    pub stop: StopConfiguration,
    #[doc = "Whether to stream back partial progress. If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format) as they become available, with the stream terminated by a `data: [DONE]` message. [Example Python code](https://cookbook.openai.com/examples/how_to_stream_completions).\n"]
    pub stream: Vec<bool>,
    pub stream_options: ChatCompletionStreamOptions,
    #[doc = "The suffix that comes after a completion of inserted text.\n\nThis parameter is only supported for `gpt-3.5-turbo-instruct`.\n"]
    pub suffix: String,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n\nWe generally recommend altering this or `top_p` but not both.\n"]
    pub temperature: f64,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or `temperature` but not both.\n"]
    pub top_p: f64,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
pub enum CreateCompletionRequestModel1 {
    Gpt35TurboInstruct,
    Davinci002,
    Babbage002,
}
#[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateCompletionRequestModel {
    AnyOf0(String),
    AnyOf1(CreateCompletionRequestModel1),
}
#[doc = "The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.\n\nNote that <|endoftext|> is the document separator that the model sees during training, so if a prompt is not specified the model will generate as if from the beginning of a new document.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateCompletionRequestPrompt {
    OneOf0(String),
    OneOf1(Vec<String>),
    OneOf2(Vec<u64>),
    OneOf3(Vec<Vec<u64>>),
}
#[doc = "Represents a completion response from the API. Note: both the streamed and non-streamed response objects share the same shape (unlike the chat endpoint).\n"]
pub struct CreateCompletionResponse {
    #[doc = "The list of completion choices the model generated for the input prompt."]
    pub choices: Vec<CreateCompletionResponseChoicesInner>,
    #[doc = "The Unix timestamp (in seconds) of when the completion was created."]
    pub created: u64,
    #[doc = "A unique identifier for the completion."]
    pub id: String,
    #[doc = "The model used for completion."]
    pub model: String,
    #[doc = "The object type, which is always \"text_completion\""]
    pub object: CreateCompletionResponseObject,
    #[doc = "This fingerprint represents the backend configuration that the model runs with.\n\nCan be used in conjunction with the `seed` request parameter to understand when backend changes have been made that might impact determinism.\n"]
    pub system_fingerprint: String,
    pub usage: CompletionUsage,
}
#[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\nor `content_filter` if content was omitted due to a flag from our content filters.\n"]
pub enum CreateCompletionResponseChoicesInnerFinishReason {
    Stop,
    Length,
    ContentFilter,
}
pub struct CreateCompletionResponseChoicesInnerLogprobs {
    pub text_offset: Vec<u64>,
    pub token_logprobs: Vec<f64>,
    pub tokens: Vec<String>,
    pub top_logprobs: Vec<std::collections::BTreeMap<String, f64>>,
}
pub struct CreateCompletionResponseChoicesInner {
    #[doc = "The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence,\n`length` if the maximum number of tokens specified in the request was reached,\nor `content_filter` if content was omitted due to a flag from our content filters.\n"]
    pub finish_reason: CreateCompletionResponseChoicesInnerFinishReason,
    pub index: u64,
    pub logprobs: CreateCompletionResponseChoicesInnerLogprobs,
    pub text: String,
}
#[doc = "The object type, which is always \"text_completion\""]
pub enum CreateCompletionResponseObject {
    TextCompletion,
}
pub struct CreateEmbeddingRequest {
    #[doc = "The number of dimensions the resulting output embeddings should have. Only supported in `text-embedding-3` and later models.\n"]
    pub dimensions: u64,
    #[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
    pub encoding_format: CreateEmbeddingRequestEncodingFormat,
    #[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.\n"]
    pub input: CreateEmbeddingRequestInput,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    pub model: CreateEmbeddingRequestModel,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
#[doc = "The format to return the embeddings in. Can be either `float` or [`base64`](https://pypi.org/project/pybase64/)."]
pub enum CreateEmbeddingRequestEncodingFormat {
    Float,
    Base64,
}
#[doc = "Input text to embed, encoded as a string or array of tokens. To embed multiple inputs in a single request, pass an array of strings or array of token arrays. The input must not exceed the max input tokens for the model (8192 tokens for `text-embedding-ada-002`), cannot be an empty string, and any array must be 2048 dimensions or less. [Example Python code](https://cookbook.openai.com/examples/how_to_count_tokens_with_tiktoken) for counting tokens. Some models may also impose a limit on total number of tokens summed across inputs.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateEmbeddingRequestInput {
    OneOf0(String),
    OneOf1(Vec<String>),
    OneOf2(Vec<u64>),
    OneOf3(Vec<Vec<u64>>),
}
pub enum CreateEmbeddingRequestModel1 {
    TextEmbeddingAda002,
    TextEmbedding3Small,
    TextEmbedding3Large,
}
#[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateEmbeddingRequestModel {
    AnyOf0(String),
    AnyOf1(CreateEmbeddingRequestModel1),
}
pub struct CreateEmbeddingResponse {
    #[doc = "The list of embeddings generated by the model."]
    pub data: Vec<Embedding>,
    #[doc = "The name of the model used to generate the embedding."]
    pub model: String,
    #[doc = "The object type, which is always \"list\"."]
    pub object: CreateEmbeddingResponseObject,
    #[doc = "The usage information for the request."]
    pub usage: CreateEmbeddingResponseUsage,
}
#[doc = "The object type, which is always \"list\"."]
pub enum CreateEmbeddingResponseObject {
    List,
}
#[doc = "The usage information for the request."]
pub struct CreateEmbeddingResponseUsage {
    #[doc = "The number of tokens used by the prompt."]
    pub prompt_tokens: u64,
    #[doc = "The total number of tokens used by the request."]
    pub total_tokens: u64,
}
#[doc = "A CompletionsRunDataSource object describing a model sampling configuration.\n"]
pub struct CreateEvalCompletionsRunDataSource {
    pub input_messages: CreateEvalCompletionsRunDataSourceInputMessages,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    pub model: String,
    pub sampling_params: CreateEvalCompletionsRunDataSourceSamplingParams,
    pub source: CreateEvalCompletionsRunDataSourceSource,
    #[doc = "The type of run data source. Always `completions`."]
    pub type_: CreateEvalCompletionsRunDataSourceType,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages0TemplateInner {
    OneOf0(EasyInputMessage),
    OneOf1(EvalItem),
}
#[doc = "The type of input messages. Always `template`."]
pub enum CreateEvalCompletionsRunDataSourceInputMessages0Type {
    Template,
}
pub struct CreateEvalCompletionsRunDataSourceInputMessages0 {
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    pub template: Vec<CreateEvalCompletionsRunDataSourceInputMessages0TemplateInner>,
    #[doc = "The type of input messages. Always `template`."]
    pub type_: CreateEvalCompletionsRunDataSourceInputMessages0Type,
}
#[doc = "The type of input messages. Always `item_reference`."]
pub enum CreateEvalCompletionsRunDataSourceInputMessages1Type {
    ItemReference,
}
pub struct CreateEvalCompletionsRunDataSourceInputMessages1 {
    #[doc = "A reference to a variable in the \"item\" namespace. Ie, \"item.name\""]
    pub item_reference: String,
    #[doc = "The type of input messages. Always `item_reference`."]
    pub type_: CreateEvalCompletionsRunDataSourceInputMessages1Type,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceInputMessages {
    OneOf0(CreateEvalCompletionsRunDataSourceInputMessages0),
    OneOf1(CreateEvalCompletionsRunDataSourceInputMessages1),
}
pub struct CreateEvalCompletionsRunDataSourceSamplingParams {
    #[doc = "The maximum number of tokens in the generated output."]
    pub max_completion_tokens: u64,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    pub seed: u64,
    #[doc = "A higher temperature increases randomness in the outputs."]
    pub temperature: f64,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    pub top_p: f64,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalCompletionsRunDataSourceSource {
    OneOf0(EvalJsonlFileContentSource),
    OneOf1(EvalJsonlFileIdSource),
    OneOf2(EvalStoredCompletionsSource),
}
#[doc = "The type of run data source. Always `completions`."]
pub enum CreateEvalCompletionsRunDataSourceType {
    Completions,
}
#[doc = "A CustomDataSourceConfig object that defines the schema for the data source used for the evaluation runs.\nThis schema is used to define the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
pub struct CreateEvalCustomDataSourceConfig {
    #[doc = "Whether the eval should expect you to populate the sample namespace (ie, by generating responses off of your data source)"]
    pub include_sample_schema: Vec<bool>,
    #[doc = "The json schema for each row in the data source."]
    pub item_schema: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of data source. Always `custom`."]
    pub type_: CreateEvalCustomDataSourceConfigType,
}
#[doc = "The type of data source. Always `custom`."]
pub enum CreateEvalCustomDataSourceConfigType {
    Custom,
}
#[doc = "A chat message that makes up the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
pub type CreateEvalItem = std::collections::BTreeMap<String, serde_json::Value>;
#[doc = "A JsonlRunDataSource object with that specifies a JSONL file that matches the eval \n"]
pub struct CreateEvalJsonlRunDataSource {
    pub source: CreateEvalJsonlRunDataSourceSource,
    #[doc = "The type of data source. Always `jsonl`."]
    pub type_: CreateEvalJsonlRunDataSourceType,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalJsonlRunDataSourceSource {
    OneOf0(EvalJsonlFileContentSource),
    OneOf1(EvalJsonlFileIdSource),
}
#[doc = "The type of data source. Always `jsonl`."]
pub enum CreateEvalJsonlRunDataSourceType {
    Jsonl,
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
pub struct CreateEvalLabelModelGrader {
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    pub input: Vec<CreateEvalItem>,
    #[doc = "The labels to classify to each item in the evaluation."]
    pub labels: Vec<String>,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    pub model: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    pub passing_labels: Vec<String>,
    #[doc = "The object type, which is always `label_model`."]
    pub type_: CreateEvalLabelModelGraderType,
}
#[doc = "The object type, which is always `label_model`."]
pub enum CreateEvalLabelModelGraderType {
    LabelModel,
}
#[doc = "A data source config which specifies the metadata property of your stored completions query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\n"]
pub struct CreateEvalLogsDataSourceConfig {
    #[doc = "Metadata filters for the logs data source."]
    pub metadata: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of data source. Always `logs`."]
    pub type_: CreateEvalLogsDataSourceConfigType,
}
#[doc = "The type of data source. Always `logs`."]
pub enum CreateEvalLogsDataSourceConfigType {
    Logs,
}
pub struct CreateEvalRequest {
    #[doc = "The configuration for the data source used for the evaluation runs."]
    pub data_source_config: std::collections::BTreeMap<String, serde_json::Value>,
    pub metadata: Metadata,
    #[doc = "The name of the evaluation."]
    pub name: String,
    #[doc = "A list of graders for all eval runs in this group."]
    pub testing_criteria: Vec<CreateEvalRequestTestingCriteriaInner>,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalRequestTestingCriteriaInner {
    OneOf0(CreateEvalLabelModelGrader),
    OneOf1(EvalStringCheckGrader),
    OneOf2(EvalTextSimilarityGrader),
    OneOf3(EvalPythonGrader),
    OneOf4(EvalScoreModelGrader),
}
#[doc = "A ResponsesRunDataSource object describing a model sampling configuration.\n"]
pub struct CreateEvalResponsesRunDataSource {
    pub input_messages: CreateEvalResponsesRunDataSourceInputMessages,
    #[doc = "The name of the model to use for generating completions (e.g. \"o3-mini\")."]
    pub model: String,
    pub sampling_params: CreateEvalResponsesRunDataSourceSamplingParams,
    pub source: CreateEvalResponsesRunDataSourceSource,
    #[doc = "The type of run data source. Always `completions`."]
    pub type_: CreateEvalResponsesRunDataSourceType,
}
pub struct CreateEvalResponsesRunDataSourceInputMessages0TemplateInner0 {
    #[doc = "The content of the message."]
    pub content: String,
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    pub role: String,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceInputMessages0TemplateInner {
    OneOf0(CreateEvalResponsesRunDataSourceInputMessages0TemplateInner0),
    OneOf1(EvalItem),
}
#[doc = "The type of input messages. Always `template`."]
pub enum CreateEvalResponsesRunDataSourceInputMessages0Type {
    Template,
}
pub struct CreateEvalResponsesRunDataSourceInputMessages0 {
    #[doc = "A list of chat messages forming the prompt or context. May include variable references to the \"item\" namespace, ie {{item.name}}."]
    pub template: Vec<CreateEvalResponsesRunDataSourceInputMessages0TemplateInner>,
    #[doc = "The type of input messages. Always `template`."]
    pub type_: CreateEvalResponsesRunDataSourceInputMessages0Type,
}
#[doc = "The type of input messages. Always `item_reference`."]
pub enum CreateEvalResponsesRunDataSourceInputMessages1Type {
    ItemReference,
}
pub struct CreateEvalResponsesRunDataSourceInputMessages1 {
    #[doc = "A reference to a variable in the \"item\" namespace. Ie, \"item.name\""]
    pub item_reference: String,
    #[doc = "The type of input messages. Always `item_reference`."]
    pub type_: CreateEvalResponsesRunDataSourceInputMessages1Type,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceInputMessages {
    OneOf0(CreateEvalResponsesRunDataSourceInputMessages0),
    OneOf1(CreateEvalResponsesRunDataSourceInputMessages1),
}
pub struct CreateEvalResponsesRunDataSourceSamplingParams {
    #[doc = "The maximum number of tokens in the generated output."]
    pub max_completion_tokens: u64,
    #[doc = "A seed value to initialize the randomness, during sampling."]
    pub seed: u64,
    #[doc = "A higher temperature increases randomness in the outputs."]
    pub temperature: f64,
    #[doc = "An alternative to temperature for nucleus sampling; 1.0 includes all tokens."]
    pub top_p: f64,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateEvalResponsesRunDataSourceSource {
    OneOf0(EvalJsonlFileContentSource),
    OneOf1(EvalJsonlFileIdSource),
    OneOf2(EvalResponsesSource),
}
#[doc = "The type of run data source. Always `completions`."]
pub enum CreateEvalResponsesRunDataSourceType {
    Completions,
}
pub struct CreateEvalRunRequest {
    #[doc = "Details about the run's data source."]
    pub data_source: std::collections::BTreeMap<String, serde_json::Value>,
    pub metadata: Metadata,
    #[doc = "The name of the run."]
    pub name: String,
}
pub struct CreateFileRequest {
    #[doc = "The File object (not file name) to be uploaded.\n"]
    pub file: Vec<u8>,
    #[doc = "The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets\n"]
    pub purpose: CreateFileRequestPurpose,
}
#[doc = "The intended purpose of the uploaded file. One of: - `assistants`: Used in the Assistants API - `batch`: Used in the Batch API - `fine-tune`: Used for fine-tuning - `vision`: Images used for vision fine-tuning - `user_data`: Flexible file type for any purpose - `evals`: Used for eval data sets\n"]
pub enum CreateFileRequestPurpose {
    Assistants,
    Batch,
    FineTune,
    Vision,
    UserData,
    Evals,
}
pub struct CreateFineTuningCheckpointPermissionRequest {
    #[doc = "The project identifiers to grant access to."]
    pub project_ids: Vec<String>,
}
pub struct CreateFineTuningJobRequest {
    #[doc = "The hyperparameters used for the fine-tuning job.\nThis value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.\n"]
    pub hyperparameters: CreateFineTuningJobRequestHyperparameters,
    #[doc = "A list of integrations to enable for your fine-tuning job."]
    pub integrations: Vec<CreateFineTuningJobRequestIntegrationsInner>,
    pub metadata: Metadata,
    pub method: FineTuneMethod,
    #[doc = "The name of the model to fine-tune. You can select one of the\n[supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned).\n"]
    pub model: CreateFineTuningJobRequestModel,
    #[doc = "The seed controls the reproducibility of the job. Passing in the same seed and job parameters should produce the same results, but may differ in rare cases.\nIf a seed is not specified, one will be generated for you.\n"]
    pub seed: u64,
    #[doc = "A string of up to 64 characters that will be added to your fine-tuned model name.\n\nFor example, a `suffix` of \"custom-model-name\" would produce a model name like `ft:gpt-4o-mini:openai:custom-model-name:7p4lURel`.\n"]
    pub suffix: String,
    #[doc = "The ID of an uploaded file that contains training data.\n\nSee [upload file](/docs/api-reference/files/create) for how to upload a file.\n\nYour dataset must be formatted as a JSONL file. Additionally, you must upload your file with the purpose `fine-tune`.\n\nThe contents of the file should differ depending on if the model uses the [chat](/docs/api-reference/fine-tuning/chat-input), [completions](/docs/api-reference/fine-tuning/completions-input) format, or if the fine-tuning method uses the [preference](/docs/api-reference/fine-tuning/preference-input) format.\n\nSee the [fine-tuning guide](/docs/guides/fine-tuning) for more details.\n"]
    pub training_file: String,
    #[doc = "The ID of an uploaded file that contains validation data.\n\nIf you provide this file, the data is used to generate validation\nmetrics periodically during fine-tuning. These metrics can be viewed in\nthe fine-tuning results file.\nThe same data should not be present in both train and validation files.\n\nYour dataset must be formatted as a JSONL file. You must upload your file with the purpose `fine-tune`.\n\nSee the [fine-tuning guide](/docs/guides/fine-tuning) for more details.\n"]
    pub validation_file: String,
}
pub enum CreateFineTuningJobRequestHyperparametersBatchSize0 {
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersBatchSize {
    OneOf0(CreateFineTuningJobRequestHyperparametersBatchSize0),
    OneOf1(u64),
}
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier0 {
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersLearningRateMultiplier {
    OneOf0(CreateFineTuningJobRequestHyperparametersLearningRateMultiplier0),
    OneOf1(f64),
}
pub enum CreateFineTuningJobRequestHyperparametersNEpochs0 {
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestHyperparametersNEpochs {
    OneOf0(CreateFineTuningJobRequestHyperparametersNEpochs0),
    OneOf1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job.\nThis value is now deprecated in favor of `method`, and should be passed in under the `method` parameter.\n"]
pub struct CreateFineTuningJobRequestHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
    pub batch_size: CreateFineTuningJobRequestHyperparametersBatchSize,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    pub learning_rate_multiplier: CreateFineTuningJobRequestHyperparametersLearningRateMultiplier,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    pub n_epochs: CreateFineTuningJobRequestHyperparametersNEpochs,
}
pub enum CreateFineTuningJobRequestIntegrationsInnerType0 {
    Wandb,
}
#[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestIntegrationsInnerType {
    OneOf0(CreateFineTuningJobRequestIntegrationsInnerType0),
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
pub struct CreateFineTuningJobRequestIntegrationsInnerWandb {
    #[doc = "The entity to use for the run. This allows you to set the team or username of the WandB user that you would\nlike associated with the run. If not set, the default entity for the registered WandB API key is used.\n"]
    pub entity: String,
    #[doc = "A display name to set for the run. If not set, we will use the Job ID as the name.\n"]
    pub name: String,
    #[doc = "The name of the project that the new run will be created under.\n"]
    pub project: String,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    pub tags: Vec<String>,
}
pub struct CreateFineTuningJobRequestIntegrationsInner {
    #[doc = "The type of integration to enable. Currently, only \"wandb\" (Weights and Biases) is supported.\n"]
    pub type_: CreateFineTuningJobRequestIntegrationsInnerType,
    #[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
    pub wandb: CreateFineTuningJobRequestIntegrationsInnerWandb,
}
pub enum CreateFineTuningJobRequestModel1 {
    Babbage002,
    Davinci002,
    Gpt35Turbo,
    Gpt4oMini,
}
#[doc = "The name of the model to fine-tune. You can select one of the\n[supported models](/docs/guides/fine-tuning#which-models-can-be-fine-tuned).\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateFineTuningJobRequestModel {
    AnyOf0(String),
    AnyOf1(CreateFineTuningJobRequestModel1),
}
pub struct CreateImageEditRequest {
    #[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less \nthan 25MB. You can provide up to 16 images.\n\nFor `dall-e-2`, you can only provide one image, and it should be a square \n`png` file less than 4MB.\n"]
    pub image: CreateImageEditRequestImage,
    #[doc = "An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited. If there are multiple images provided, the mask will be applied on the first image. Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`."]
    pub mask: Vec<u8>,
    #[doc = "The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
    pub model: CreateImageEditRequestModel,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    pub n: u64,
    #[doc = "A text description of the desired image(s). The maximum length is 1000 characters for `dall-e-2`, and 32000 characters for `gpt-image-1`."]
    pub prompt: String,
    #[doc = "The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.\n"]
    pub quality: CreateImageEditRequestQuality,
    #[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images."]
    pub response_format: CreateImageEditRequestResponseFormat,
    #[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`."]
    pub size: CreateImageEditRequestSize,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
#[doc = "The image(s) to edit. Must be a supported image file or an array of images.\n\nFor `gpt-image-1`, each image should be a `png`, `webp`, or `jpg` file less \nthan 25MB. You can provide up to 16 images.\n\nFor `dall-e-2`, you can only provide one image, and it should be a square \n`png` file less than 4MB.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateImageEditRequestImage {
    AnyOf0(Vec<u8>),
    AnyOf1(Vec<Vec<u8>>),
}
pub enum CreateImageEditRequestModel1 {
    DallE2,
    GptImage1,
}
#[doc = "The model to use for image generation. Only `dall-e-2` and `gpt-image-1` are supported. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
#[allow(clippy::large_enum_variant)]
pub enum CreateImageEditRequestModel {
    AnyOf0(String),
    AnyOf1(CreateImageEditRequestModel1),
}
#[doc = "The quality of the image that will be generated. `high`, `medium` and `low` are only supported for `gpt-image-1`. `dall-e-2` only supports `standard` quality. Defaults to `auto`.\n"]
pub enum CreateImageEditRequestQuality {
    Standard,
    Low,
    Medium,
    High,
    Auto,
}
#[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter is only supported for `dall-e-2`, as `gpt-image-1` will always return base64-encoded images."]
pub enum CreateImageEditRequestResponseFormat {
    Url,
    B64Json,
}
#[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, and one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`."]
pub enum CreateImageEditRequestSize {
    _256x256,
    _512x512,
    _1024x1024,
    _1536x1024,
    _1024x1536,
    Auto,
}
pub struct CreateImageRequest {
    #[doc = "Allows to set transparency for the background of the generated image(s). \nThis parameter is only supported for `gpt-image-1`. Must be one of \n`transparent`, `opaque` or `auto` (default value). When `auto` is used, the \nmodel will automatically determine the best background for the image.\n\nIf `transparent`, the output format needs to support transparency, so it \nshould be set to either `png` (default value) or `webp`.\n"]
    pub background: CreateImageRequestBackground,
    #[doc = "The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
    pub model: CreateImageRequestModel,
    #[doc = "Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value)."]
    pub moderation: CreateImageRequestModeration,
    #[doc = "The number of images to generate. Must be between 1 and 10. For `dall-e-3`, only `n=1` is supported."]
    pub n: u64,
    #[doc = "The compression level (0-100%) for the generated images. This parameter is only supported for `gpt-image-1` with the `webp` or `jpeg` output formats, and defaults to 100."]
    pub output_compression: u64,
    #[doc = "The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`."]
    pub output_format: CreateImageRequestOutputFormat,
    #[doc = "A text description of the desired image(s). The maximum length is 32000 characters for `gpt-image-1`, 1000 characters for `dall-e-2` and 4000 characters for `dall-e-3`."]
    pub prompt: String,
    #[doc = "The quality of the image that will be generated. \n\n- `auto` (default value) will automatically select the best quality for the given model.\n- `high`, `medium` and `low` are supported for `gpt-image-1`.\n- `hd` and `standard` are supported for `dall-e-3`.\n- `standard` is the only option for `dall-e-2`.\n"]
    pub quality: CreateImageRequestQuality,
    #[doc = "The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images."]
    pub response_format: CreateImageRequestResponseFormat,
    #[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`."]
    pub size: CreateImageRequestSize,
    #[doc = "The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images."]
    pub style: CreateImageRequestStyle,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
#[doc = "Allows to set transparency for the background of the generated image(s). \nThis parameter is only supported for `gpt-image-1`. Must be one of \n`transparent`, `opaque` or `auto` (default value). When `auto` is used, the \nmodel will automatically determine the best background for the image.\n\nIf `transparent`, the output format needs to support transparency, so it \nshould be set to either `png` (default value) or `webp`.\n"]
pub enum CreateImageRequestBackground {
    Transparent,
    Opaque,
    Auto,
}
pub enum CreateImageRequestModel1 {
    DallE2,
    DallE3,
    GptImage1,
}
#[doc = "The model to use for image generation. One of `dall-e-2`, `dall-e-3`, or `gpt-image-1`. Defaults to `dall-e-2` unless a parameter specific to `gpt-image-1` is used."]
#[allow(clippy::large_enum_variant)]
pub enum CreateImageRequestModel {
    AnyOf0(String),
    AnyOf1(CreateImageRequestModel1),
}
#[doc = "Control the content-moderation level for images generated by `gpt-image-1`. Must be either `low` for less restrictive filtering or `auto` (default value)."]
pub enum CreateImageRequestModeration {
    Low,
    Auto,
}
#[doc = "The format in which the generated images are returned. This parameter is only supported for `gpt-image-1`. Must be one of `png`, `jpeg`, or `webp`."]
pub enum CreateImageRequestOutputFormat {
    Png,
    Jpeg,
    Webp,
}
#[doc = "The quality of the image that will be generated. \n\n- `auto` (default value) will automatically select the best quality for the given model.\n- `high`, `medium` and `low` are supported for `gpt-image-1`.\n- `hd` and `standard` are supported for `dall-e-3`.\n- `standard` is the only option for `dall-e-2`.\n"]
pub enum CreateImageRequestQuality {
    Standard,
    Hd,
    Low,
    Medium,
    High,
    Auto,
}
#[doc = "The format in which generated images with `dall-e-2` and `dall-e-3` are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated. This parameter isn't supported for `gpt-image-1` which will always return base64-encoded images."]
pub enum CreateImageRequestResponseFormat {
    Url,
    B64Json,
}
#[doc = "The size of the generated images. Must be one of `1024x1024`, `1536x1024` (landscape), `1024x1536` (portrait), or `auto` (default value) for `gpt-image-1`, one of `256x256`, `512x512`, or `1024x1024` for `dall-e-2`, and one of `1024x1024`, `1792x1024`, or `1024x1792` for `dall-e-3`."]
pub enum CreateImageRequestSize {
    Auto,
    _1024x1024,
    _1536x1024,
    _1024x1536,
    _256x256,
    _512x512,
    _1792x1024,
    _1024x1792,
}
#[doc = "The style of the generated images. This parameter is only supported for `dall-e-3`. Must be one of `vivid` or `natural`. Vivid causes the model to lean towards generating hyper-real and dramatic images. Natural causes the model to produce more natural, less hyper-real looking images."]
pub enum CreateImageRequestStyle {
    Vivid,
    Natural,
}
pub struct CreateImageVariationRequest {
    #[doc = "The image to use as the basis for the variation(s). Must be a valid PNG file, less than 4MB, and square."]
    pub image: Vec<u8>,
    #[doc = "The model to use for image generation. Only `dall-e-2` is supported at this time."]
    pub model: CreateImageVariationRequestModel,
    #[doc = "The number of images to generate. Must be between 1 and 10."]
    pub n: u64,
    #[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated."]
    pub response_format: CreateImageVariationRequestResponseFormat,
    #[doc = "The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`."]
    pub size: CreateImageVariationRequestSize,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
pub enum CreateImageVariationRequestModel1 {
    DallE2,
}
#[doc = "The model to use for image generation. Only `dall-e-2` is supported at this time."]
#[allow(clippy::large_enum_variant)]
pub enum CreateImageVariationRequestModel {
    AnyOf0(String),
    AnyOf1(CreateImageVariationRequestModel1),
}
#[doc = "The format in which the generated images are returned. Must be one of `url` or `b64_json`. URLs are only valid for 60 minutes after the image has been generated."]
pub enum CreateImageVariationRequestResponseFormat {
    Url,
    B64Json,
}
#[doc = "The size of the generated images. Must be one of `256x256`, `512x512`, or `1024x1024`."]
pub enum CreateImageVariationRequestSize {
    _256x256,
    _512x512,
    _1024x1024,
}
pub struct CreateMessageRequest {
    #[doc = "A list of files attached to the message, and the tools they should be added to."]
    pub attachments: Vec<CreateMessageRequestAttachmentsInner>,
    pub content: CreateMessageRequestContent,
    pub metadata: Metadata,
    #[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
    pub role: CreateMessageRequestRole,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestAttachmentsInnerToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearchTypeOnly),
}
pub struct CreateMessageRequestAttachmentsInner {
    #[doc = "The ID of the file to attach to the message."]
    pub file_id: String,
    #[doc = "The tools to add this file to."]
    pub tools: Vec<CreateMessageRequestAttachmentsInnerToolsInner>,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestContent1Inner {
    OneOf0(MessageContentImageFileObject),
    OneOf1(MessageContentImageUrlObject),
    OneOf2(MessageRequestContentTextObject),
}
#[allow(clippy::large_enum_variant)]
pub enum CreateMessageRequestContent {
    OneOf0(String),
    OneOf1(Vec<CreateMessageRequestContent1Inner>),
}
#[doc = "The role of the entity that is creating the message. Allowed values include:\n- `user`: Indicates the message is sent by an actual user and should be used in most cases to represent user-generated messages.\n- `assistant`: Indicates the message is generated by the assistant. Use this value to insert messages from the assistant into the conversation.\n"]
pub enum CreateMessageRequestRole {
    User,
    Assistant,
}
pub struct CreateModelResponseProperties {
    pub all_of_0: ModelResponseProperties,
}
pub struct CreateModerationRequest {
    #[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
    pub input: CreateModerationRequestInput,
    #[doc = "The content moderation model you would like to use. Learn more in\n[the moderation guide](/docs/guides/moderation), and learn about\navailable models [here](/docs/models#moderation).\n"]
    pub model: CreateModerationRequestModel,
}
#[doc = "Contains either an image URL or a data URL for a base64 encoded image."]
pub struct CreateModerationRequestInput2Inner0ImageUrl {
    #[doc = "Either a URL of the image or the base64 encoded image data."]
    pub url: String,
}
#[doc = "Always `image_url`."]
pub enum CreateModerationRequestInput2Inner0Type {
    ImageUrl,
}
#[doc = "An object describing an image to classify."]
pub struct CreateModerationRequestInput2Inner0 {
    #[doc = "Contains either an image URL or a data URL for a base64 encoded image."]
    pub image_url: CreateModerationRequestInput2Inner0ImageUrl,
    #[doc = "Always `image_url`."]
    pub type_: CreateModerationRequestInput2Inner0Type,
}
#[doc = "Always `text`."]
pub enum CreateModerationRequestInput2Inner1Type {
    Text,
}
#[doc = "An object describing text to classify."]
pub struct CreateModerationRequestInput2Inner1 {
    #[doc = "A string of text to classify."]
    pub text: String,
    #[doc = "Always `text`."]
    pub type_: CreateModerationRequestInput2Inner1Type,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateModerationRequestInput2Inner {
    OneOf0(CreateModerationRequestInput2Inner0),
    OneOf1(CreateModerationRequestInput2Inner1),
}
#[doc = "Input (or inputs) to classify. Can be a single string, an array of strings, or\nan array of multi-modal input objects similar to other models.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateModerationRequestInput {
    OneOf0(String),
    OneOf1(Vec<String>),
    OneOf2(Vec<CreateModerationRequestInput2Inner>),
}
pub enum CreateModerationRequestModel1 {
    OmniModerationLatest,
    OmniModeration20240926,
    TextModerationLatest,
    TextModerationStable,
}
#[doc = "The content moderation model you would like to use. Learn more in\n[the moderation guide](/docs/guides/moderation), and learn about\navailable models [here](/docs/models#moderation).\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateModerationRequestModel {
    AnyOf0(String),
    AnyOf1(CreateModerationRequestModel1),
}
#[doc = "Represents if a given text input is potentially harmful."]
pub struct CreateModerationResponse {
    #[doc = "The unique identifier for the moderation request."]
    pub id: String,
    #[doc = "The model used to generate the moderation results."]
    pub model: String,
    #[doc = "A list of moderation objects."]
    pub results: Vec<CreateModerationResponseResultsInner>,
}
#[doc = "A list of the categories, and whether they are flagged or not."]
pub struct CreateModerationResponseResultsInnerCategories {
    #[doc = "Content that expresses, incites, or promotes harassing language towards any target."]
    pub harassment: Vec<bool>,
    #[doc = "Harassment content that also includes violence or serious harm towards any target."]
    pub harassment_threatening: Vec<bool>,
    #[doc = "Content that expresses, incites, or promotes hate based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste. Hateful content aimed at non-protected groups (e.g., chess players) is harassment."]
    pub hate: Vec<bool>,
    #[doc = "Hateful content that also includes violence or serious harm towards the targeted group based on race, gender, ethnicity, religion, nationality, sexual orientation, disability status, or caste."]
    pub hate_threatening: Vec<bool>,
    #[doc = "Content that includes instructions or advice that facilitate the planning or execution of wrongdoing, or that gives advice or instruction on how to commit illicit acts. For example, \"how to shoplift\" would fit this category."]
    pub illicit: Vec<bool>,
    #[doc = "Content that includes instructions or advice that facilitate the planning or execution of wrongdoing that also includes violence, or that gives advice or instruction on the procurement of any weapon."]
    pub illicit_violent: Vec<bool>,
    #[doc = "Content that promotes, encourages, or depicts acts of self-harm, such as suicide, cutting, and eating disorders."]
    pub self_harm: Vec<bool>,
    #[doc = "Content that encourages performing acts of self-harm, such as suicide, cutting, and eating disorders, or that gives instructions or advice on how to commit such acts."]
    pub self_harm_instructions: Vec<bool>,
    #[doc = "Content where the speaker expresses that they are engaging or intend to engage in acts of self-harm, such as suicide, cutting, and eating disorders."]
    pub self_harm_intent: Vec<bool>,
    #[doc = "Content meant to arouse sexual excitement, such as the description of sexual activity, or that promotes sexual services (excluding sex education and wellness)."]
    pub sexual: Vec<bool>,
    #[doc = "Sexual content that includes an individual who is under 18 years old."]
    pub sexual_minors: Vec<bool>,
    #[doc = "Content that depicts death, violence, or physical injury."]
    pub violence: Vec<bool>,
    #[doc = "Content that depicts death, violence, or physical injury in graphic detail."]
    pub violence_graphic: Vec<bool>,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesHarassmentInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesHarassmentThreateningInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesHateInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesHateThreateningInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesIllicitInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesIllicitViolentInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmInner {
    Text,
    Image,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmInstructionsInner {
    Text,
    Image,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmIntentInner {
    Text,
    Image,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesSexualInner {
    Text,
    Image,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesSexualMinorsInner {
    Text,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesViolenceInner {
    Text,
    Image,
}
pub enum CreateModerationResponseResultsInnerCategoryAppliedInputTypesViolenceGraphicInner {
    Text,
    Image,
}
#[doc = "A list of the categories along with the input type(s) that the score applies to."]
pub struct CreateModerationResponseResultsInnerCategoryAppliedInputTypes {
    #[doc = "The applied input type(s) for the category 'harassment'."]
    pub harassment:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesHarassmentInner>,
    #[doc = "The applied input type(s) for the category 'harassment/threatening'."]
    pub harassment_threatening: Vec<
        CreateModerationResponseResultsInnerCategoryAppliedInputTypesHarassmentThreateningInner,
    >,
    #[doc = "The applied input type(s) for the category 'hate'."]
    pub hate: Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesHateInner>,
    #[doc = "The applied input type(s) for the category 'hate/threatening'."]
    pub hate_threatening:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesHateThreateningInner>,
    #[doc = "The applied input type(s) for the category 'illicit'."]
    pub illicit: Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesIllicitInner>,
    #[doc = "The applied input type(s) for the category 'illicit/violent'."]
    pub illicit_violent:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesIllicitViolentInner>,
    #[doc = "The applied input type(s) for the category 'self-harm'."]
    pub self_harm: Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmInner>,
    #[doc = "The applied input type(s) for the category 'self-harm/instructions'."]
    pub self_harm_instructions:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmInstructionsInner>,
    #[doc = "The applied input type(s) for the category 'self-harm/intent'."]
    pub self_harm_intent:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesSelfHarmIntentInner>,
    #[doc = "The applied input type(s) for the category 'sexual'."]
    pub sexual: Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesSexualInner>,
    #[doc = "The applied input type(s) for the category 'sexual/minors'."]
    pub sexual_minors:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesSexualMinorsInner>,
    #[doc = "The applied input type(s) for the category 'violence'."]
    pub violence: Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesViolenceInner>,
    #[doc = "The applied input type(s) for the category 'violence/graphic'."]
    pub violence_graphic:
        Vec<CreateModerationResponseResultsInnerCategoryAppliedInputTypesViolenceGraphicInner>,
}
#[doc = "A list of the categories along with their scores as predicted by model."]
pub struct CreateModerationResponseResultsInnerCategoryScores {
    #[doc = "The score for the category 'harassment'."]
    pub harassment: f64,
    #[doc = "The score for the category 'harassment/threatening'."]
    pub harassment_threatening: f64,
    #[doc = "The score for the category 'hate'."]
    pub hate: f64,
    #[doc = "The score for the category 'hate/threatening'."]
    pub hate_threatening: f64,
    #[doc = "The score for the category 'illicit'."]
    pub illicit: f64,
    #[doc = "The score for the category 'illicit/violent'."]
    pub illicit_violent: f64,
    #[doc = "The score for the category 'self-harm'."]
    pub self_harm: f64,
    #[doc = "The score for the category 'self-harm/instructions'."]
    pub self_harm_instructions: f64,
    #[doc = "The score for the category 'self-harm/intent'."]
    pub self_harm_intent: f64,
    #[doc = "The score for the category 'sexual'."]
    pub sexual: f64,
    #[doc = "The score for the category 'sexual/minors'."]
    pub sexual_minors: f64,
    #[doc = "The score for the category 'violence'."]
    pub violence: f64,
    #[doc = "The score for the category 'violence/graphic'."]
    pub violence_graphic: f64,
}
pub struct CreateModerationResponseResultsInner {
    #[doc = "A list of the categories, and whether they are flagged or not."]
    pub categories: CreateModerationResponseResultsInnerCategories,
    #[doc = "A list of the categories along with the input type(s) that the score applies to."]
    pub category_applied_input_types: CreateModerationResponseResultsInnerCategoryAppliedInputTypes,
    #[doc = "A list of the categories along with their scores as predicted by model."]
    pub category_scores: CreateModerationResponseResultsInnerCategoryScores,
    #[doc = "Whether any of the below categories are flagged."]
    pub flagged: Vec<bool>,
}
pub struct CreateResponse {
    pub all_of_0: CreateModelResponseProperties,
    pub all_of_1: ResponseProperties,
    pub all_of_2: CreateResponse2,
}
#[doc = "Text, image, or file inputs to the model, used to generate a response.\n\nLearn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Image inputs](/docs/guides/images)\n- [File inputs](/docs/guides/pdf-files)\n- [Conversation state](/docs/guides/conversation-state)\n- [Function calling](/docs/guides/function-calling)\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateResponse2Input {
    OneOf0(String),
    OneOf1(Vec<InputItem>),
}
pub struct CreateResponse2 {
    #[doc = "Specify additional output data to include in the model response. Currently\nsupported values are:\n- `file_search_call.results`: Include the search results of\n  the file search tool call.\n- `message.input_image.image_url`: Include image urls from the input message.\n- `computer_call_output.output.image_url`: Include image urls from the computer call output.\n"]
    pub include: Vec<Includable>,
    #[doc = "Text, image, or file inputs to the model, used to generate a response.\n\nLearn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Image inputs](/docs/guides/images)\n- [File inputs](/docs/guides/pdf-files)\n- [Conversation state](/docs/guides/conversation-state)\n- [Function calling](/docs/guides/function-calling)\n"]
    pub input: CreateResponse2Input,
    #[doc = "Whether to allow the model to run tool calls in parallel.\n"]
    pub parallel_tool_calls: Vec<bool>,
    #[doc = "Whether to store the generated model response for later retrieval via\nAPI.\n"]
    pub store: Vec<bool>,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format).\nSee the [Streaming section below](/docs/api-reference/responses-streaming)\nfor more information.\n"]
    pub stream: Vec<bool>,
}
pub struct CreateRunRequest {
    #[doc = "Appends additional instructions at the end of the instructions for the run. This is useful for modifying the behavior on a per-run basis without overriding other instructions."]
    pub additional_instructions: String,
    #[doc = "Adds additional messages to the thread before creating the run."]
    pub additional_messages: Vec<CreateMessageRequest>,
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run."]
    pub assistant_id: String,
    #[doc = "Overrides the [instructions](/docs/api-reference/assistants/createAssistant) of the assistant. This is useful for modifying the behavior on a per-run basis."]
    pub instructions: String,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    pub max_completion_tokens: u64,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    pub max_prompt_tokens: u64,
    pub metadata: Metadata,
    #[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    pub model: CreateRunRequestModel,
    pub parallel_tool_calls: ParallelToolCalls,
    pub reasoning_effort: ReasoningEffort,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    pub stream: Vec<bool>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    pub temperature: f64,
    pub tool_choice: CreateRunRequestToolChoice,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    pub tools: Vec<CreateRunRequestToolsInner>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    pub top_p: f64,
    pub truncation_strategy: CreateRunRequestTruncationStrategy,
}
#[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
#[allow(clippy::large_enum_variant)]
pub enum CreateRunRequestModel {
    AnyOf0(String),
    AnyOf1(AssistantSupportedModels),
}
pub struct CreateRunRequestToolChoice {
    pub all_of_0: AssistantsApiToolChoiceOption,
    pub all_of_1: serde_json::Value,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateRunRequestToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
pub struct CreateRunRequestTruncationStrategy {
    pub all_of_0: TruncationObject,
    pub all_of_1: serde_json::Value,
}
pub struct CreateSpeechRequest {
    #[doc = "The text to generate audio for. The maximum length is 4096 characters."]
    pub input: String,
    #[doc = "Control the voice of your generated audio with additional instructions. Does not work with `tts-1` or `tts-1-hd`."]
    pub instructions: String,
    #[doc = "One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.\n"]
    pub model: CreateSpeechRequestModel,
    #[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
    pub response_format: CreateSpeechRequestResponseFormat,
    #[doc = "The speed of the generated audio. Select a value from `0.25` to `4.0`. `1.0` is the default."]
    pub speed: f64,
    #[doc = "The voice to use when generating the audio. Supported voices are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`, `onyx`, `nova`, `sage`, `shimmer`, and `verse`. Previews of the voices are available in the [Text to speech guide](/docs/guides/text-to-speech#voice-options)."]
    pub voice: VoiceIdsShared,
}
pub enum CreateSpeechRequestModel1 {
    Tts1,
    Tts1Hd,
    Gpt4oMiniTts,
}
#[doc = "One of the available [TTS models](/docs/models#tts): `tts-1`, `tts-1-hd` or `gpt-4o-mini-tts`.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateSpeechRequestModel {
    AnyOf0(String),
    AnyOf1(CreateSpeechRequestModel1),
}
#[doc = "The format to audio in. Supported formats are `mp3`, `opus`, `aac`, `flac`, `wav`, and `pcm`."]
pub enum CreateSpeechRequestResponseFormat {
    Mp3,
    Opus,
    Aac,
    Flac,
    Wav,
    Pcm,
}
pub struct CreateThreadAndRunRequest {
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) to use to execute this run."]
    pub assistant_id: String,
    #[doc = "Override the default system message of the assistant. This is useful for modifying the behavior on a per-run basis."]
    pub instructions: String,
    #[doc = "The maximum number of completion tokens that may be used over the course of the run. The run will make a best effort to use only the number of completion tokens specified, across multiple turns of the run. If the run exceeds the number of completion tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    pub max_completion_tokens: u64,
    #[doc = "The maximum number of prompt tokens that may be used over the course of the run. The run will make a best effort to use only the number of prompt tokens specified, across multiple turns of the run. If the run exceeds the number of prompt tokens specified, the run will end with status `incomplete`. See `incomplete_details` for more info.\n"]
    pub max_prompt_tokens: u64,
    pub metadata: Metadata,
    #[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
    pub model: CreateThreadAndRunRequestModel,
    pub parallel_tool_calls: ParallelToolCalls,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    pub stream: Vec<bool>,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    pub temperature: f64,
    pub thread: CreateThreadRequest,
    pub tool_choice: CreateThreadAndRunRequestToolChoice,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: CreateThreadAndRunRequestToolResources,
    #[doc = "Override the tools the assistant can use for this run. This is useful for modifying the behavior on a per-run basis."]
    pub tools: Vec<CreateThreadAndRunRequestToolsInner>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    pub top_p: f64,
    pub truncation_strategy: CreateThreadAndRunRequestTruncationStrategy,
}
pub enum CreateThreadAndRunRequestModel1 {
    Gpt41,
    Gpt41Mini,
    Gpt41Nano,
    Gpt4120250414,
    Gpt41Mini20250414,
    Gpt41Nano20250414,
    Gpt4o,
    Gpt4o20241120,
    Gpt4o20240806,
    Gpt4o20240513,
    Gpt4oMini,
    Gpt4oMini20240718,
    Gpt45Preview,
    Gpt45Preview20250227,
    Gpt4Turbo,
    Gpt4Turbo20240409,
    Gpt40125Preview,
    Gpt4TurboPreview,
    Gpt41106Preview,
    Gpt4VisionPreview,
    Gpt4,
    Gpt40314,
    Gpt40613,
    Gpt432k,
    Gpt432k0314,
    Gpt432k0613,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo0613,
    Gpt35Turbo1106,
    Gpt35Turbo0125,
    Gpt35Turbo16k0613,
}
#[doc = "The ID of the [Model](/docs/api-reference/models) to be used to execute this run. If a value is provided here, it will override the model associated with the assistant. If not, the model associated with the assistant will be used."]
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadAndRunRequestModel {
    AnyOf0(String),
    AnyOf1(CreateThreadAndRunRequestModel1),
}
pub struct CreateThreadAndRunRequestToolChoice {
    pub all_of_0: AssistantsApiToolChoiceOption,
    pub all_of_1: serde_json::Value,
}
pub struct CreateThreadAndRunRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct CreateThreadAndRunRequestToolResourcesFileSearch {
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    pub vector_store_ids: Vec<String>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct CreateThreadAndRunRequestToolResources {
    pub code_interpreter: CreateThreadAndRunRequestToolResourcesCodeInterpreter,
    pub file_search: CreateThreadAndRunRequestToolResourcesFileSearch,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateThreadAndRunRequestToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
pub struct CreateThreadAndRunRequestTruncationStrategy {
    pub all_of_0: TruncationObject,
    pub all_of_1: serde_json::Value,
}
#[doc = "Options to create a new thread. If no thread is provided when running a \nrequest, an empty thread will be created.\n"]
pub struct CreateThreadRequest {
    #[doc = "A list of [messages](/docs/api-reference/messages) to start the thread with."]
    pub messages: Vec<CreateMessageRequest>,
    pub metadata: Metadata,
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: CreateThreadRequestToolResources,
}
pub struct CreateThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct CreateThreadRequestToolResourcesFileSearchVectorStoresInner {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy."]
    pub chunking_strategy: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "A list of [file](/docs/api-reference/files) IDs to add to the vector store. There can be a maximum of 10000 files in a vector store.\n"]
    pub file_ids: Vec<String>,
    pub metadata: Metadata,
}
pub struct CreateThreadRequestToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    pub vector_store_ids: Vec<String>,
    #[doc = "A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    pub vector_stores: Vec<CreateThreadRequestToolResourcesFileSearchVectorStoresInner>,
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct CreateThreadRequestToolResources {
    pub code_interpreter: CreateThreadRequestToolResourcesCodeInterpreter,
    pub file_search: CreateThreadRequestToolResourcesFileSearch,
}
pub struct CreateTranscriptionRequest {
    #[doc = "The audio file object (not file name) to transcribe, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.\n"]
    pub file: Vec<u8>,
    #[doc = "Additional information to include in the transcription response. \n`logprobs` will return the log probabilities of the tokens in the \nresponse to understand the model's confidence in the transcription. \n`logprobs` only works with response_format set to `json` and only with \nthe models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe`.\n"]
    pub include: Vec<TranscriptionInclude>,
    #[doc = "The language of the input audio. Supplying the input language in [ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format will improve accuracy and latency.\n"]
    pub language: String,
    #[doc = "ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model).\n"]
    pub model: CreateTranscriptionRequestModel,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should match the audio language.\n"]
    pub prompt: String,
    pub response_format: AudioResponseFormat,
    #[doc = "If set to true, the model response data will be streamed to the client\nas it is generated using [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format). \nSee the [Streaming section of the Speech-to-Text guide](/docs/guides/speech-to-text?lang=curl#streaming-transcriptions)\nfor more information.\n\nNote: Streaming is not supported for the `whisper-1` model and will be ignored.\n"]
    pub stream: Vec<bool>,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    pub temperature: f64,
    #[doc = "The timestamp granularities to populate for this transcription. `response_format` must be set `verbose_json` to use timestamp granularities. Either or both of these options are supported: `word`, or `segment`. Note: There is no additional latency for segment timestamps, but generating word timestamps incurs additional latency.\n"]
    pub timestamp_granularities: Vec<CreateTranscriptionRequestTimestampGranularitiesInner>,
}
pub enum CreateTranscriptionRequestModel1 {
    Whisper1,
    Gpt4oTranscribe,
    Gpt4oMiniTranscribe,
}
#[doc = "ID of the model to use. The options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1` (which is powered by our open source Whisper V2 model).\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateTranscriptionRequestModel {
    AnyOf0(String),
    AnyOf1(CreateTranscriptionRequestModel1),
}
pub enum CreateTranscriptionRequestTimestampGranularitiesInner {
    Word,
    Segment,
}
#[doc = "Represents a transcription response returned by model, based on the provided input."]
pub struct CreateTranscriptionResponseJson {
    #[doc = "The log probabilities of the tokens in the transcription. Only returned with the models `gpt-4o-transcribe` and `gpt-4o-mini-transcribe` if `logprobs` is added to the `include` array.\n"]
    pub logprobs: Vec<CreateTranscriptionResponseJsonLogprobsInner>,
    #[doc = "The transcribed text."]
    pub text: String,
}
pub struct CreateTranscriptionResponseJsonLogprobsInner {
    #[doc = "The bytes of the token."]
    pub bytes: Vec<f64>,
    #[doc = "The log probability of the token."]
    pub logprob: f64,
    #[doc = "The token in the transcription."]
    pub token: String,
}
#[allow(clippy::large_enum_variant)]
pub enum CreateTranscriptionResponseStreamEvent {
    AnyOf0(TranscriptTextDeltaEvent),
    AnyOf1(TranscriptTextDoneEvent),
}
#[doc = "Represents a verbose json transcription response returned by model, based on the provided input."]
pub struct CreateTranscriptionResponseVerboseJson {
    #[doc = "The duration of the input audio."]
    pub duration: f64,
    #[doc = "The language of the input audio."]
    pub language: String,
    #[doc = "Segments of the transcribed text and their corresponding details."]
    pub segments: Vec<TranscriptionSegment>,
    #[doc = "The transcribed text."]
    pub text: String,
    #[doc = "Extracted words and their corresponding timestamps."]
    pub words: Vec<TranscriptionWord>,
}
pub struct CreateTranslationRequest {
    #[doc = "The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.\n"]
    pub file: Vec<u8>,
    #[doc = "ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.\n"]
    pub model: CreateTranslationRequestModel,
    #[doc = "An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text#prompting) should be in English.\n"]
    pub prompt: String,
    #[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
    pub response_format: CreateTranslationRequestResponseFormat,
    #[doc = "The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the temperature until certain thresholds are hit.\n"]
    pub temperature: f64,
}
pub enum CreateTranslationRequestModel1 {
    Whisper1,
}
#[doc = "ID of the model to use. Only `whisper-1` (which is powered by our open source Whisper V2 model) is currently available.\n"]
#[allow(clippy::large_enum_variant)]
pub enum CreateTranslationRequestModel {
    AnyOf0(String),
    AnyOf1(CreateTranslationRequestModel1),
}
#[doc = "The format of the output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.\n"]
pub enum CreateTranslationRequestResponseFormat {
    Json,
    Text,
    Srt,
    VerboseJson,
    Vtt,
}
pub struct CreateTranslationResponseJson {
    pub text: String,
}
pub struct CreateTranslationResponseVerboseJson {
    #[doc = "The duration of the input audio."]
    pub duration: f64,
    #[doc = "The language of the output translation (always `english`)."]
    pub language: String,
    #[doc = "Segments of the translated text and their corresponding details."]
    pub segments: Vec<TranscriptionSegment>,
    #[doc = "The translated text."]
    pub text: String,
}
pub struct CreateUploadRequest {
    #[doc = "The number of bytes in the file you are uploading.\n"]
    pub bytes: u64,
    #[doc = "The name of the file to upload.\n"]
    pub filename: String,
    #[doc = "The MIME type of the file.\n\nThis must fall within the supported MIME types for your file purpose. See the supported MIME types for assistants and vision.\n"]
    pub mime_type: String,
    #[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose).\n"]
    pub purpose: CreateUploadRequestPurpose,
}
#[doc = "The intended purpose of the uploaded file.\n\nSee the [documentation on File purposes](/docs/api-reference/files/create#files-create-purpose).\n"]
pub enum CreateUploadRequestPurpose {
    Assistants,
    Batch,
    FineTune,
    Vision,
}
pub struct CreateVectorStoreFileBatchRequest {
    pub attributes: VectorStoreFileAttributes,
    pub chunking_strategy: ChunkingStrategyRequestParam,
    #[doc = "A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files."]
    pub file_ids: Vec<String>,
}
pub struct CreateVectorStoreFileRequest {
    pub attributes: VectorStoreFileAttributes,
    pub chunking_strategy: ChunkingStrategyRequestParam,
    #[doc = "A [File](/docs/api-reference/files) ID that the vector store should use. Useful for tools like `file_search` that can access files."]
    pub file_id: String,
}
pub struct CreateVectorStoreRequest {
    #[doc = "The chunking strategy used to chunk the file(s). If not set, will use the `auto` strategy. Only applicable if `file_ids` is non-empty."]
    pub chunking_strategy: std::collections::BTreeMap<String, serde_json::Value>,
    pub expires_after: VectorStoreExpirationAfter,
    #[doc = "A list of [File](/docs/api-reference/files) IDs that the vector store should use. Useful for tools like `file_search` that can access files."]
    pub file_ids: Vec<String>,
    pub metadata: Metadata,
    #[doc = "The name of the vector store."]
    pub name: String,
}
pub struct DeleteAssistantResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteAssistantResponseObject,
}
pub enum DeleteAssistantResponseObject {
    AssistantDeleted,
}
pub struct DeleteCertificateResponse {
    #[doc = "The ID of the certificate that was deleted."]
    pub id: String,
    #[doc = "The object type, must be `certificate.deleted`."]
    pub object: DeleteCertificateResponseObject,
}
#[doc = "The object type, must be `certificate.deleted`."]
pub enum DeleteCertificateResponseObject {
    CertificateDeleted,
}
pub struct DeleteFileResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteFileResponseObject,
}
pub enum DeleteFileResponseObject {
    File,
}
pub struct DeleteFineTuningCheckpointPermissionResponse {
    #[doc = "Whether the fine-tuned model checkpoint permission was successfully deleted."]
    pub deleted: Vec<bool>,
    #[doc = "The ID of the fine-tuned model checkpoint permission that was deleted."]
    pub id: String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    pub object: DeleteFineTuningCheckpointPermissionResponseObject,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
pub enum DeleteFineTuningCheckpointPermissionResponseObject {
    CheckpointPermission,
}
pub struct DeleteMessageResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteMessageResponseObject,
}
pub enum DeleteMessageResponseObject {
    ThreadMessageDeleted,
}
pub struct DeleteModelResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: String,
}
pub struct DeleteThreadResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteThreadResponseObject,
}
pub enum DeleteThreadResponseObject {
    ThreadDeleted,
}
pub struct DeleteVectorStoreFileResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteVectorStoreFileResponseObject,
}
pub enum DeleteVectorStoreFileResponseObject {
    VectorStoreFileDeleted,
}
pub struct DeleteVectorStoreResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: DeleteVectorStoreResponseObject,
}
pub enum DeleteVectorStoreResponseObject {
    VectorStoreDeleted,
}
#[doc = "Occurs when a stream ends."]
pub struct DoneEvent {
    pub data: DoneEventData,
    pub event: DoneEventEvent,
}
pub enum DoneEventData {
    Done,
}
pub enum DoneEventEvent {
    Done,
}
#[doc = "A double click action.\n"]
pub struct DoubleClick {
    #[doc = "Specifies the event type. For a double click action, this property is \nalways set to `double_click`.\n"]
    pub type_: DoubleClickType,
    #[doc = "The x-coordinate where the double click occurred.\n"]
    pub x: u64,
    #[doc = "The y-coordinate where the double click occurred.\n"]
    pub y: u64,
}
#[doc = "Specifies the event type. For a double click action, this property is \nalways set to `double_click`.\n"]
pub enum DoubleClickType {
    DoubleClick,
}
#[doc = "A drag action.\n"]
pub struct Drag {
    #[doc = "An array of coordinates representing the path of the drag action. Coordinates will appear as an array\nof objects, eg\n```\n[\n  { x: 100, y: 200 },\n  { x: 200, y: 300 }\n]\n```\n"]
    pub path: Vec<Coordinate>,
    #[doc = "Specifies the event type. For a drag action, this property is \nalways set to `drag`.\n"]
    pub type_: DragType,
}
#[doc = "Specifies the event type. For a drag action, this property is \nalways set to `drag`.\n"]
pub enum DragType {
    Drag,
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
pub struct EasyInputMessage {
    #[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
    pub content: EasyInputMessageContent,
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    pub role: EasyInputMessageRole,
    #[doc = "The type of the message input. Always `message`.\n"]
    pub type_: EasyInputMessageType,
}
#[doc = "Text, image, or audio input to the model, used to generate a response.\nCan also contain previous assistant responses.\n"]
#[allow(clippy::large_enum_variant)]
pub enum EasyInputMessageContent {
    OneOf0(String),
    OneOf1(InputMessageContentList),
}
#[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
pub enum EasyInputMessageRole {
    User,
    Assistant,
    System,
    Developer,
}
#[doc = "The type of the message input. Always `message`.\n"]
pub enum EasyInputMessageType {
    Message,
}
#[doc = "Represents an embedding vector returned by embedding endpoint.\n"]
pub struct Embedding {
    #[doc = "The embedding vector, which is a list of floats. The length of vector depends on the model as listed in the [embedding guide](/docs/guides/embeddings).\n"]
    pub embedding: Vec<f64>,
    #[doc = "The index of the embedding in the list of embeddings."]
    pub index: u64,
    #[doc = "The object type, which is always \"embedding\"."]
    pub object: EmbeddingObject,
}
#[doc = "The object type, which is always \"embedding\"."]
pub enum EmbeddingObject {
    Embedding,
}
pub struct Error {
    pub code: String,
    pub message: String,
    pub param: String,
    pub type_: String,
}
#[doc = "Occurs when an [error](/docs/guides/error-codes#api-errors) occurs. This can happen due to an internal server error or a timeout."]
pub struct ErrorEvent {
    pub data: Error,
    pub event: ErrorEventEvent,
}
pub enum ErrorEventEvent {
    Error,
}
pub struct ErrorResponse {
    pub error: Error,
}
#[doc = "An Eval object with a data source config and testing criteria.\nAn Eval represents a task to be done for your LLM integration.\nLike:\n - Improve the quality of my chatbot\n - See how well my chatbot handles customer support\n - Check if o3-mini is better at my usecase than gpt-4o\n"]
pub struct Eval {
    #[doc = "The Unix timestamp (in seconds) for when the eval was created."]
    pub created_at: u64,
    #[doc = "Configuration of data sources used in runs of the evaluation."]
    pub data_source_config: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "Unique identifier for the evaluation."]
    pub id: String,
    pub metadata: Metadata,
    #[doc = "The name of the evaluation."]
    pub name: String,
    #[doc = "The object type."]
    pub object: EvalObject,
    #[doc = "A list of testing criteria."]
    pub testing_criteria: Vec<EvalTestingCriteriaInner>,
}
#[doc = "The object type."]
pub enum EvalObject {
    Eval,
}
#[allow(clippy::large_enum_variant)]
pub enum EvalTestingCriteriaInner {
    OneOf0(EvalLabelModelGrader),
    OneOf1(EvalStringCheckGrader),
    OneOf2(EvalTextSimilarityGrader),
    OneOf3(EvalPythonGrader),
    OneOf4(EvalScoreModelGrader),
}
#[doc = "An object representing an error response from the Eval API.\n"]
pub struct EvalApiError {
    #[doc = "The error code."]
    pub code: String,
    #[doc = "The error message."]
    pub message: String,
}
#[doc = "A CustomDataSourceConfig which specifies the schema of your `item` and optionally `sample` namespaces.\nThe response schema defines the shape of the data that will be:\n- Used to define your testing criteria and\n- What data is required when creating a run\n"]
pub struct EvalCustomDataSourceConfig {
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    pub schema: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of data source. Always `custom`."]
    pub type_: EvalCustomDataSourceConfigType,
}
#[doc = "The type of data source. Always `custom`."]
pub enum EvalCustomDataSourceConfigType {
    Custom,
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role. Messages with the\n`assistant` role are presumed to have been generated by the model in previous\ninteractions.\n"]
pub struct EvalItem {
    #[doc = "Text inputs to the model - can contain template strings.\n"]
    pub content: EvalItemContent,
    #[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
    pub role: EvalItemRole,
    #[doc = "The type of the message input. Always `message`.\n"]
    pub type_: EvalItemType,
}
#[doc = "The type of the output text. Always `output_text`.\n"]
pub enum EvalItemContent2Type {
    OutputText,
}
#[doc = "A text output from the model.\n"]
pub struct EvalItemContent2 {
    #[doc = "The text output from the model.\n"]
    pub text: String,
    #[doc = "The type of the output text. Always `output_text`.\n"]
    pub type_: EvalItemContent2Type,
}
#[doc = "Text inputs to the model - can contain template strings.\n"]
#[allow(clippy::large_enum_variant)]
pub enum EvalItemContent {
    OneOf0(String),
    OneOf1(InputTextContent),
    OneOf2(EvalItemContent2),
}
#[doc = "The role of the message input. One of `user`, `assistant`, `system`, or\n`developer`.\n"]
pub enum EvalItemRole {
    User,
    Assistant,
    System,
    Developer,
}
#[doc = "The type of the message input. Always `message`.\n"]
pub enum EvalItemType {
    Message,
}
pub struct EvalJsonlFileContentSource {
    #[doc = "The content of the jsonl file."]
    pub content: Vec<EvalJsonlFileContentSourceContentInner>,
    #[doc = "The type of jsonl source. Always `file_content`."]
    pub type_: EvalJsonlFileContentSourceType,
}
pub struct EvalJsonlFileContentSourceContentInner {
    pub item: std::collections::BTreeMap<String, serde_json::Value>,
    pub sample: std::collections::BTreeMap<String, serde_json::Value>,
}
#[doc = "The type of jsonl source. Always `file_content`."]
pub enum EvalJsonlFileContentSourceType {
    FileContent,
}
pub struct EvalJsonlFileIdSource {
    #[doc = "The identifier of the file."]
    pub id: String,
    #[doc = "The type of jsonl source. Always `file_id`."]
    pub type_: EvalJsonlFileIdSourceType,
}
#[doc = "The type of jsonl source. Always `file_id`."]
pub enum EvalJsonlFileIdSourceType {
    FileId,
}
#[doc = "A LabelModelGrader object which uses a model to assign labels to each item\nin the evaluation.\n"]
pub struct EvalLabelModelGrader {
    pub input: Vec<EvalItem>,
    #[doc = "The labels to assign to each item in the evaluation."]
    pub labels: Vec<String>,
    #[doc = "The model to use for the evaluation. Must support structured outputs."]
    pub model: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "The labels that indicate a passing result. Must be a subset of labels."]
    pub passing_labels: Vec<String>,
    #[doc = "The object type, which is always `label_model`."]
    pub type_: EvalLabelModelGraderType,
}
#[doc = "The object type, which is always `label_model`."]
pub enum EvalLabelModelGraderType {
    LabelModel,
}
#[doc = "An object representing a list of evals.\n"]
pub struct EvalList {
    #[doc = "An array of eval objects.\n"]
    pub data: Vec<Eval>,
    #[doc = "The identifier of the first eval in the data array."]
    pub first_id: String,
    #[doc = "Indicates whether there are more evals available."]
    pub has_more: Vec<bool>,
    #[doc = "The identifier of the last eval in the data array."]
    pub last_id: String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalListObject,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
pub enum EvalListObject {
    List,
}
#[doc = "A PythonGrader object that runs a python script on the input.\n"]
pub struct EvalPythonGrader {
    #[doc = "The image tag to use for the python script."]
    pub image_tag: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "The threshold for the score."]
    pub pass_threshold: f64,
    #[doc = "The source code of the python script."]
    pub source: String,
    #[doc = "The object type, which is always `python`."]
    pub type_: EvalPythonGraderType,
}
#[doc = "The object type, which is always `python`."]
pub enum EvalPythonGraderType {
    Python,
}
#[doc = "A EvalResponsesSource object describing a run data source configuration.\n"]
pub struct EvalResponsesSource {
    #[doc = "Whether to allow parallel tool calls. This is a query parameter used to select responses."]
    pub allow_parallel_tool_calls: Vec<bool>,
    #[doc = "Only include items created after this timestamp (inclusive). This is a query parameter used to select responses."]
    pub created_after: u64,
    #[doc = "Only include items created before this timestamp (inclusive). This is a query parameter used to select responses."]
    pub created_before: u64,
    #[doc = "Whether the response has tool calls. This is a query parameter used to select responses."]
    pub has_tool_calls: Vec<bool>,
    #[doc = "Optional search string for instructions. This is a query parameter used to select responses."]
    pub instructions_search: String,
    #[doc = "Metadata filter for the responses. This is a query parameter used to select responses."]
    pub metadata: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The name of the model to find responses for. This is a query parameter used to select responses."]
    pub model: String,
    #[doc = "Optional reasoning effort parameter. This is a query parameter used to select responses."]
    pub reasoning_effort: ReasoningEffort,
    #[doc = "Sampling temperature. This is a query parameter used to select responses."]
    pub temperature: f64,
    #[doc = "Nucleus sampling parameter. This is a query parameter used to select responses."]
    pub top_p: f64,
    #[doc = "The type of run data source. Always `responses`."]
    pub type_: EvalResponsesSourceType,
    #[doc = "List of user identifiers. This is a query parameter used to select responses."]
    pub users: Vec<String>,
}
#[doc = "The type of run data source. Always `responses`."]
pub enum EvalResponsesSourceType {
    Responses,
}
#[doc = "A schema representing an evaluation run.\n"]
pub struct EvalRun {
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    pub created_at: u64,
    #[doc = "Information about the run's data source."]
    pub data_source: std::collections::BTreeMap<String, serde_json::Value>,
    pub error: EvalApiError,
    #[doc = "The identifier of the associated evaluation."]
    pub eval_id: String,
    #[doc = "Unique identifier for the evaluation run."]
    pub id: String,
    pub metadata: Metadata,
    #[doc = "The model that is evaluated, if applicable."]
    pub model: String,
    #[doc = "The name of the evaluation run."]
    pub name: String,
    #[doc = "The type of the object. Always \"eval.run\"."]
    pub object: EvalRunObject,
    #[doc = "Usage statistics for each model during the evaluation run."]
    pub per_model_usage: Vec<EvalRunPerModelUsageInner>,
    #[doc = "Results per testing criteria applied during the evaluation run."]
    pub per_testing_criteria_results: Vec<EvalRunPerTestingCriteriaResultsInner>,
    #[doc = "The URL to the rendered evaluation run report on the UI dashboard."]
    pub report_url: String,
    #[doc = "Counters summarizing the outcomes of the evaluation run."]
    pub result_counts: EvalRunResultCounts,
    #[doc = "The status of the evaluation run."]
    pub status: String,
}
#[doc = "The type of the object. Always \"eval.run\"."]
pub enum EvalRunObject {
    EvalRun,
}
pub struct EvalRunPerModelUsageInner {
    #[doc = "The number of tokens retrieved from cache."]
    pub cached_tokens: u64,
    #[doc = "The number of completion tokens generated."]
    pub completion_tokens: u64,
    #[doc = "The number of invocations."]
    pub invocation_count: u64,
    #[doc = "The name of the model."]
    pub model_name: String,
    #[doc = "The number of prompt tokens used."]
    pub prompt_tokens: u64,
    #[doc = "The total number of tokens used."]
    pub total_tokens: u64,
}
pub struct EvalRunPerTestingCriteriaResultsInner {
    #[doc = "Number of tests failed for this criteria."]
    pub failed: u64,
    #[doc = "Number of tests passed for this criteria."]
    pub passed: u64,
    #[doc = "A description of the testing criteria."]
    pub testing_criteria: String,
}
#[doc = "Counters summarizing the outcomes of the evaluation run."]
pub struct EvalRunResultCounts {
    #[doc = "Number of output items that resulted in an error."]
    pub errored: u64,
    #[doc = "Number of output items that failed to pass the evaluation."]
    pub failed: u64,
    #[doc = "Number of output items that passed the evaluation."]
    pub passed: u64,
    #[doc = "Total number of executed output items."]
    pub total: u64,
}
#[doc = "An object representing a list of runs for an evaluation.\n"]
pub struct EvalRunList {
    #[doc = "An array of eval run objects.\n"]
    pub data: Vec<EvalRun>,
    #[doc = "The identifier of the first eval run in the data array."]
    pub first_id: String,
    #[doc = "Indicates whether there are more evals available."]
    pub has_more: Vec<bool>,
    #[doc = "The identifier of the last eval run in the data array."]
    pub last_id: String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalRunListObject,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
pub enum EvalRunListObject {
    List,
}
#[doc = "A schema representing an evaluation run output item.\n"]
pub struct EvalRunOutputItem {
    #[doc = "Unix timestamp (in seconds) when the evaluation run was created."]
    pub created_at: u64,
    #[doc = "Details of the input data source item."]
    pub datasource_item: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The identifier for the data source item."]
    pub datasource_item_id: u64,
    #[doc = "The identifier of the evaluation group."]
    pub eval_id: String,
    #[doc = "Unique identifier for the evaluation run output item."]
    pub id: String,
    #[doc = "The type of the object. Always \"eval.run.output_item\"."]
    pub object: EvalRunOutputItemObject,
    #[doc = "A list of results from the evaluation run."]
    pub results: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    #[doc = "The identifier of the evaluation run associated with this output item."]
    pub run_id: String,
    #[doc = "A sample containing the input and output of the evaluation run."]
    pub sample: EvalRunOutputItemSample,
    #[doc = "The status of the evaluation run."]
    pub status: String,
}
#[doc = "The type of the object. Always \"eval.run.output_item\"."]
pub enum EvalRunOutputItemObject {
    EvalRunOutputItem,
}
#[doc = "An input message."]
pub struct EvalRunOutputItemSampleInputInner {
    #[doc = "The content of the message."]
    pub content: String,
    #[doc = "The role of the message sender (e.g., system, user, developer)."]
    pub role: String,
}
pub struct EvalRunOutputItemSampleOutputInner {
    #[doc = "The content of the message."]
    pub content: String,
    #[doc = "The role of the message (e.g. \"system\", \"assistant\", \"user\")."]
    pub role: String,
}
#[doc = "Token usage details for the sample."]
pub struct EvalRunOutputItemSampleUsage {
    #[doc = "The number of tokens retrieved from cache."]
    pub cached_tokens: u64,
    #[doc = "The number of completion tokens generated."]
    pub completion_tokens: u64,
    #[doc = "The number of prompt tokens used."]
    pub prompt_tokens: u64,
    #[doc = "The total number of tokens used."]
    pub total_tokens: u64,
}
#[doc = "A sample containing the input and output of the evaluation run."]
pub struct EvalRunOutputItemSample {
    pub error: EvalApiError,
    #[doc = "The reason why the sample generation was finished."]
    pub finish_reason: String,
    #[doc = "An array of input messages."]
    pub input: Vec<EvalRunOutputItemSampleInputInner>,
    #[doc = "The maximum number of tokens allowed for completion."]
    pub max_completion_tokens: u64,
    #[doc = "The model used for generating the sample."]
    pub model: String,
    #[doc = "An array of output messages."]
    pub output: Vec<EvalRunOutputItemSampleOutputInner>,
    #[doc = "The seed used for generating the sample."]
    pub seed: u64,
    #[doc = "The sampling temperature used."]
    pub temperature: f64,
    #[doc = "The top_p value used for sampling."]
    pub top_p: f64,
    #[doc = "Token usage details for the sample."]
    pub usage: EvalRunOutputItemSampleUsage,
}
#[doc = "An object representing a list of output items for an evaluation run.\n"]
pub struct EvalRunOutputItemList {
    #[doc = "An array of eval run output item objects.\n"]
    pub data: Vec<EvalRunOutputItem>,
    #[doc = "The identifier of the first eval run output item in the data array."]
    pub first_id: String,
    #[doc = "Indicates whether there are more eval run output items available."]
    pub has_more: Vec<bool>,
    #[doc = "The identifier of the last eval run output item in the data array."]
    pub last_id: String,
    #[doc = "The type of this object. It is always set to \"list\".\n"]
    pub object: EvalRunOutputItemListObject,
}
#[doc = "The type of this object. It is always set to \"list\".\n"]
pub enum EvalRunOutputItemListObject {
    List,
}
#[doc = "A ScoreModelGrader object that uses a model to assign a score to the input.\n"]
pub struct EvalScoreModelGrader {
    #[doc = "The input text. This may include template strings."]
    pub input: Vec<EvalItem>,
    #[doc = "The model to use for the evaluation."]
    pub model: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "The threshold for the score."]
    pub pass_threshold: f64,
    #[doc = "The range of the score. Defaults to `[0, 1]`."]
    pub range: Vec<f64>,
    #[doc = "The sampling parameters for the model."]
    pub sampling_params: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The object type, which is always `score_model`."]
    pub type_: EvalScoreModelGraderType,
}
#[doc = "The object type, which is always `score_model`."]
pub enum EvalScoreModelGraderType {
    ScoreModel,
}
#[doc = "A StoredCompletionsDataSourceConfig which specifies the metadata property of your stored completions query.\nThis is usually metadata like `usecase=chatbot` or `prompt-version=v2`, etc.\nThe schema returned by this data source config is used to defined what variables are available in your evals.\n`item` and `sample` are both defined when using this data source config.\n"]
pub struct EvalStoredCompletionsDataSourceConfig {
    pub metadata: Metadata,
    #[doc = "The json schema for the run data source items.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
    pub schema: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of data source. Always `stored_completions`."]
    pub type_: EvalStoredCompletionsDataSourceConfigType,
}
#[doc = "The type of data source. Always `stored_completions`."]
pub enum EvalStoredCompletionsDataSourceConfigType {
    StoredCompletions,
}
#[doc = "A StoredCompletionsRunDataSource configuration describing a set of filters\n"]
pub struct EvalStoredCompletionsSource {
    #[doc = "An optional Unix timestamp to filter items created after this time."]
    pub created_after: u64,
    #[doc = "An optional Unix timestamp to filter items created before this time."]
    pub created_before: u64,
    #[doc = "An optional maximum number of items to return."]
    pub limit: u64,
    pub metadata: Metadata,
    #[doc = "An optional model to filter by (e.g., 'gpt-4o')."]
    pub model: String,
    #[doc = "The type of source. Always `stored_completions`."]
    pub type_: EvalStoredCompletionsSourceType,
}
#[doc = "The type of source. Always `stored_completions`."]
pub enum EvalStoredCompletionsSourceType {
    StoredCompletions,
}
#[doc = "A StringCheckGrader object that performs a string comparison between input and reference using a specified operation.\n"]
pub struct EvalStringCheckGrader {
    #[doc = "The input text. This may include template strings."]
    pub input: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
    pub operation: EvalStringCheckGraderOperation,
    #[doc = "The reference text. This may include template strings."]
    pub reference: String,
    #[doc = "The object type, which is always `string_check`."]
    pub type_: EvalStringCheckGraderType,
}
#[doc = "The string check operation to perform. One of `eq`, `ne`, `like`, or `ilike`."]
pub enum EvalStringCheckGraderOperation {
    Eq,
    Ne,
    Like,
    Ilike,
}
#[doc = "The object type, which is always `string_check`."]
pub enum EvalStringCheckGraderType {
    StringCheck,
}
#[doc = "A TextSimilarityGrader object which grades text based on similarity metrics.\n"]
pub struct EvalTextSimilarityGrader {
    #[doc = "The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`."]
    pub evaluation_metric: EvalTextSimilarityGraderEvaluationMetric,
    #[doc = "The text being graded."]
    pub input: String,
    #[doc = "The name of the grader."]
    pub name: String,
    #[doc = "A float score where a value greater than or equal indicates a passing grade."]
    pub pass_threshold: f64,
    #[doc = "The text being graded against."]
    pub reference: String,
    #[doc = "The type of grader."]
    pub type_: EvalTextSimilarityGraderType,
}
#[doc = "The evaluation metric to use. One of `fuzzy_match`, `bleu`, `gleu`, `meteor`, `rouge_1`, `rouge_2`, `rouge_3`, `rouge_4`, `rouge_5`, or `rouge_l`."]
pub enum EvalTextSimilarityGraderEvaluationMetric {
    FuzzyMatch,
    Bleu,
    Gleu,
    Meteor,
    Rouge1,
    Rouge2,
    Rouge3,
    Rouge4,
    Rouge5,
    RougeL,
}
#[doc = "The type of grader."]
pub enum EvalTextSimilarityGraderType {
    TextSimilarity,
}
#[doc = "A citation to a file."]
pub struct FileCitationBody {
    #[doc = "The ID of the file."]
    pub file_id: String,
    #[doc = "The index of the file in the list of files."]
    pub index: u64,
    #[doc = "The type of the file citation. Always `file_citation`."]
    pub type_: FileCitationBodyType,
}
#[doc = "The type of the file citation. Always `file_citation`."]
pub enum FileCitationBodyType {
    FileCitation,
}
#[doc = "A path to a file.\n"]
pub struct FilePath {
    #[doc = "The ID of the file.\n"]
    pub file_id: String,
    #[doc = "The index of the file in the list of files.\n"]
    pub index: u64,
    #[doc = "The type of the file path. Always `file_path`.\n"]
    pub type_: FilePathType,
}
#[doc = "The type of the file path. Always `file_path`.\n"]
pub enum FilePathType {
    FilePath,
}
#[doc = "The ranker to use for the file search. If not specified will use the `auto` ranker."]
pub enum FileSearchRanker {
    Auto,
    Default20240821,
}
#[doc = "The ranking options for the file search. If not specified, the file search tool will use the `auto` ranker and a score_threshold of 0.\n\nSee the [file search tool documentation](/docs/assistants/tools/file-search#customizing-file-search-settings) for more information.\n"]
pub struct FileSearchRankingOptions {
    pub ranker: FileSearchRanker,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    pub score_threshold: f64,
}
#[doc = "A tool that searches for relevant content from uploaded files. Learn more about the [file search tool](https://platform.openai.com/docs/guides/tools-file-search)."]
pub struct FileSearchTool {
    pub filters: Vec<Filters>,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    pub max_num_results: u64,
    #[doc = "Ranking options for search."]
    pub ranking_options: RankingOptions,
    #[doc = "The type of the file search tool. Always `file_search`."]
    pub type_: FileSearchToolType,
    #[doc = "The IDs of the vector stores to search."]
    pub vector_store_ids: Vec<String>,
}
#[doc = "The type of the file search tool. Always `file_search`."]
pub enum FileSearchToolType {
    FileSearch,
}
#[doc = "The results of a file search tool call. See the \n[file search guide](/docs/guides/tools-file-search) for more information.\n"]
pub struct FileSearchToolCall {
    #[doc = "The unique ID of the file search tool call.\n"]
    pub id: String,
    #[doc = "The queries used to search for files.\n"]
    pub queries: Vec<String>,
    #[doc = "The results of the file search tool call.\n"]
    pub results: Vec<FileSearchToolCallResultsInner>,
    #[doc = "The status of the file search tool call. One of `in_progress`, \n`searching`, `incomplete` or `failed`,\n"]
    pub status: FileSearchToolCallStatus,
    #[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
    pub type_: FileSearchToolCallType,
}
pub struct FileSearchToolCallResultsInner {
    pub attributes: VectorStoreFileAttributes,
    #[doc = "The unique ID of the file.\n"]
    pub file_id: String,
    #[doc = "The name of the file.\n"]
    pub filename: String,
    #[doc = "The relevance score of the file - a value between 0 and 1.\n"]
    pub score: f64,
    #[doc = "The text that was retrieved from the file.\n"]
    pub text: String,
}
#[doc = "The status of the file search tool call. One of `in_progress`, \n`searching`, `incomplete` or `failed`,\n"]
pub enum FileSearchToolCallStatus {
    InProgress,
    Searching,
    Completed,
    Incomplete,
    Failed,
}
#[doc = "The type of the file search tool call. Always `file_search_call`.\n"]
pub enum FileSearchToolCallType {
    FileSearchCall,
}
#[allow(clippy::large_enum_variant)]
pub enum Filters {
    AnyOf0(ComparisonFilter),
    AnyOf1(CompoundFilter),
}
pub struct FineTuneChatCompletionRequestAssistantMessage {
    pub all_of_0: FineTuneChatCompletionRequestAssistantMessage0,
    pub all_of_1: ChatCompletionRequestAssistantMessage,
}
pub struct FineTuneChatCompletionRequestAssistantMessage0 {
    #[doc = "Controls whether the assistant message is trained against (0 or 1)"]
    pub weight: u64,
}
#[doc = "The per-line training example of a fine-tuning input file for chat models using the supervised method."]
pub struct FineTuneChatRequestInput {
    #[doc = "A list of functions the model may generate JSON inputs for."]
    pub functions: Vec<ChatCompletionFunctions>,
    pub messages: Vec<FineTuneChatRequestInputMessagesInner>,
    pub parallel_tool_calls: ParallelToolCalls,
    #[doc = "A list of tools the model may generate JSON inputs for."]
    pub tools: Vec<ChatCompletionTool>,
}
#[allow(clippy::large_enum_variant)]
pub enum FineTuneChatRequestInputMessagesInner {
    OneOf0(ChatCompletionRequestSystemMessage),
    OneOf1(ChatCompletionRequestUserMessage),
    OneOf2(FineTuneChatCompletionRequestAssistantMessage),
    OneOf3(ChatCompletionRequestToolMessage),
    OneOf4(ChatCompletionRequestFunctionMessage),
}
#[doc = "The per-line training example of a fine-tuning input file for completions models"]
pub struct FineTuneCompletionRequestInput {
    #[doc = "The desired completion for this training example."]
    pub completion: String,
    #[doc = "The input prompt for this training example."]
    pub prompt: String,
}
#[doc = "Configuration for the DPO fine-tuning method."]
pub struct FineTuneDpoMethod {
    #[doc = "The hyperparameters used for the fine-tuning job."]
    pub hyperparameters: FineTuneDpoMethodHyperparameters,
}
pub enum FineTuneDpoMethodHyperparametersBatchSize0 {
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersBatchSize {
    OneOf0(FineTuneDpoMethodHyperparametersBatchSize0),
    OneOf1(u64),
}
pub enum FineTuneDpoMethodHyperparametersBeta0 {
    Auto,
}
#[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersBeta {
    OneOf0(FineTuneDpoMethodHyperparametersBeta0),
    OneOf1(f64),
}
pub enum FineTuneDpoMethodHyperparametersLearningRateMultiplier0 {
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersLearningRateMultiplier {
    OneOf0(FineTuneDpoMethodHyperparametersLearningRateMultiplier0),
    OneOf1(f64),
}
pub enum FineTuneDpoMethodHyperparametersNEpochs0 {
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneDpoMethodHyperparametersNEpochs {
    OneOf0(FineTuneDpoMethodHyperparametersNEpochs0),
    OneOf1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job."]
pub struct FineTuneDpoMethodHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    pub batch_size: FineTuneDpoMethodHyperparametersBatchSize,
    #[doc = "The beta value for the DPO method. A higher beta value will increase the weight of the penalty between the policy and reference model.\n"]
    pub beta: FineTuneDpoMethodHyperparametersBeta,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    pub learning_rate_multiplier: FineTuneDpoMethodHyperparametersLearningRateMultiplier,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    pub n_epochs: FineTuneDpoMethodHyperparametersNEpochs,
}
#[doc = "The method used for fine-tuning."]
pub struct FineTuneMethod {
    pub dpo: FineTuneDpoMethod,
    pub supervised: FineTuneSupervisedMethod,
    #[doc = "The type of method. Is either `supervised` or `dpo`."]
    pub type_: FineTuneMethodType,
}
#[doc = "The type of method. Is either `supervised` or `dpo`."]
pub enum FineTuneMethodType {
    Supervised,
    Dpo,
}
#[doc = "The per-line training example of a fine-tuning input file for chat models using the dpo method."]
pub struct FineTunePreferenceRequestInput {
    pub input: FineTunePreferenceRequestInputInput,
    #[doc = "The non-preferred completion message for the output."]
    pub non_preferred_completion: Vec<FineTunePreferenceRequestInputNonPreferredCompletionInner>,
    #[doc = "The preferred completion message for the output."]
    pub preferred_completion: Vec<FineTunePreferenceRequestInputPreferredCompletionInner>,
}
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputInputMessagesInner {
    OneOf0(ChatCompletionRequestSystemMessage),
    OneOf1(ChatCompletionRequestUserMessage),
    OneOf2(FineTuneChatCompletionRequestAssistantMessage),
    OneOf3(ChatCompletionRequestToolMessage),
    OneOf4(ChatCompletionRequestFunctionMessage),
}
pub struct FineTunePreferenceRequestInputInput {
    pub messages: Vec<FineTunePreferenceRequestInputInputMessagesInner>,
    pub parallel_tool_calls: ParallelToolCalls,
    #[doc = "A list of tools the model may generate JSON inputs for."]
    pub tools: Vec<ChatCompletionTool>,
}
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputNonPreferredCompletionInner {
    OneOf0(ChatCompletionRequestAssistantMessage),
}
#[allow(clippy::large_enum_variant)]
pub enum FineTunePreferenceRequestInputPreferredCompletionInner {
    OneOf0(ChatCompletionRequestAssistantMessage),
}
#[doc = "Configuration for the supervised fine-tuning method."]
pub struct FineTuneSupervisedMethod {
    #[doc = "The hyperparameters used for the fine-tuning job."]
    pub hyperparameters: FineTuneSupervisedMethodHyperparameters,
}
pub enum FineTuneSupervisedMethodHyperparametersBatchSize0 {
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersBatchSize {
    OneOf0(FineTuneSupervisedMethodHyperparametersBatchSize0),
    OneOf1(u64),
}
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier0 {
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersLearningRateMultiplier {
    OneOf0(FineTuneSupervisedMethodHyperparametersLearningRateMultiplier0),
    OneOf1(f64),
}
pub enum FineTuneSupervisedMethodHyperparametersNEpochs0 {
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuneSupervisedMethodHyperparametersNEpochs {
    OneOf0(FineTuneSupervisedMethodHyperparametersNEpochs0),
    OneOf1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job."]
pub struct FineTuneSupervisedMethodHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters are updated less frequently, but with lower variance.\n"]
    pub batch_size: FineTuneSupervisedMethodHyperparametersBatchSize,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid overfitting.\n"]
    pub learning_rate_multiplier: FineTuneSupervisedMethodHyperparametersLearningRateMultiplier,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle through the training dataset.\n"]
    pub n_epochs: FineTuneSupervisedMethodHyperparametersNEpochs,
}
#[doc = "The `checkpoint.permission` object represents a permission for a fine-tuned model checkpoint.\n"]
pub struct FineTuningCheckpointPermission {
    #[doc = "The Unix timestamp (in seconds) for when the permission was created."]
    pub created_at: u64,
    #[doc = "The permission identifier, which can be referenced in the API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always \"checkpoint.permission\"."]
    pub object: FineTuningCheckpointPermissionObject,
    #[doc = "The project identifier that the permission is for."]
    pub project_id: String,
}
#[doc = "The object type, which is always \"checkpoint.permission\"."]
pub enum FineTuningCheckpointPermissionObject {
    CheckpointPermission,
}
pub struct FineTuningIntegration {
    #[doc = "The type of the integration being enabled for the fine-tuning job"]
    pub type_: FineTuningIntegrationType,
    #[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
    pub wandb: FineTuningIntegrationWandb,
}
#[doc = "The type of the integration being enabled for the fine-tuning job"]
pub enum FineTuningIntegrationType {
    Wandb,
}
#[doc = "The settings for your integration with Weights and Biases. This payload specifies the project that\nmetrics will be sent to. Optionally, you can set an explicit display name for your run, add tags\nto your run, and set a default entity (team, username, etc) to be associated with your run.\n"]
pub struct FineTuningIntegrationWandb {
    #[doc = "The entity to use for the run. This allows you to set the team or username of the WandB user that you would\nlike associated with the run. If not set, the default entity for the registered WandB API key is used.\n"]
    pub entity: String,
    #[doc = "A display name to set for the run. If not set, we will use the Job ID as the name.\n"]
    pub name: String,
    #[doc = "The name of the project that the new run will be created under.\n"]
    pub project: String,
    #[doc = "A list of tags to be attached to the newly created run. These tags are passed through directly to WandB. Some\ndefault tags are generated by OpenAI: \"openai/finetune\", \"openai/{base-model}\", \"openai/{ftjob-abcdef}\".\n"]
    pub tags: Vec<String>,
}
#[doc = "The `fine_tuning.job` object represents a fine-tuning job that has been created through the API.\n"]
pub struct FineTuningJob {
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    pub created_at: u64,
    #[doc = "For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure."]
    pub error: FineTuningJobError,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job is estimated to finish. The value will be null if the fine-tuning job is not running."]
    pub estimated_finish: u64,
    #[doc = "The name of the fine-tuned model that is being created. The value will be null if the fine-tuning job is still running."]
    pub fine_tuned_model: String,
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was finished. The value will be null if the fine-tuning job is still running."]
    pub finished_at: u64,
    #[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
    pub hyperparameters: FineTuningJobHyperparameters,
    #[doc = "The object identifier, which can be referenced in the API endpoints."]
    pub id: String,
    #[doc = "A list of integrations to enable for this fine-tuning job."]
    pub integrations: Vec<FineTuningJobIntegrationsInner>,
    pub metadata: Metadata,
    pub method: FineTuneMethod,
    #[doc = "The base model that is being fine-tuned."]
    pub model: String,
    #[doc = "The object type, which is always \"fine_tuning.job\"."]
    pub object: FineTuningJobObject,
    #[doc = "The organization that owns the fine-tuning job."]
    pub organization_id: String,
    #[doc = "The compiled results file ID(s) for the fine-tuning job. You can retrieve the results with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    pub result_files: Vec<String>,
    #[doc = "The seed used for the fine-tuning job."]
    pub seed: u64,
    #[doc = "The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`."]
    pub status: FineTuningJobStatus,
    #[doc = "The total number of billable tokens processed by this fine-tuning job. The value will be null if the fine-tuning job is still running."]
    pub trained_tokens: u64,
    #[doc = "The file ID used for training. You can retrieve the training data with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    pub training_file: String,
    #[doc = "The file ID used for validation. You can retrieve the validation results with the [Files API](/docs/api-reference/files/retrieve-contents)."]
    pub validation_file: String,
}
#[doc = "For fine-tuning jobs that have `failed`, this will contain more information on the cause of the failure."]
pub struct FineTuningJobError {
    #[doc = "A machine-readable error code."]
    pub code: String,
    #[doc = "A human-readable error message."]
    pub message: String,
    #[doc = "The parameter that was invalid, usually `training_file` or `validation_file`. This field will be null if the failure was not parameter-specific."]
    pub param: String,
}
pub enum FineTuningJobHyperparametersBatchSize0 {
    Auto,
}
#[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersBatchSize {
    OneOf0(FineTuningJobHyperparametersBatchSize0),
    OneOf1(u64),
}
pub enum FineTuningJobHyperparametersLearningRateMultiplier0 {
    Auto,
}
#[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersLearningRateMultiplier {
    OneOf0(FineTuningJobHyperparametersLearningRateMultiplier0),
    OneOf1(f64),
}
pub enum FineTuningJobHyperparametersNEpochs0 {
    Auto,
}
#[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobHyperparametersNEpochs {
    OneOf0(FineTuningJobHyperparametersNEpochs0),
    OneOf1(u64),
}
#[doc = "The hyperparameters used for the fine-tuning job. This value will only be returned when running `supervised` jobs."]
pub struct FineTuningJobHyperparameters {
    #[doc = "Number of examples in each batch. A larger batch size means that model parameters\nare updated less frequently, but with lower variance.\n"]
    pub batch_size: FineTuningJobHyperparametersBatchSize,
    #[doc = "Scaling factor for the learning rate. A smaller learning rate may be useful to avoid\noverfitting.\n"]
    pub learning_rate_multiplier: FineTuningJobHyperparametersLearningRateMultiplier,
    #[doc = "The number of epochs to train the model for. An epoch refers to one full cycle\nthrough the training dataset.\n"]
    pub n_epochs: FineTuningJobHyperparametersNEpochs,
}
#[allow(clippy::large_enum_variant)]
pub enum FineTuningJobIntegrationsInner {
    OneOf0(FineTuningIntegration),
}
#[doc = "The object type, which is always \"fine_tuning.job\"."]
pub enum FineTuningJobObject {
    FineTuningJob,
}
#[doc = "The current status of the fine-tuning job, which can be either `validating_files`, `queued`, `running`, `succeeded`, `failed`, or `cancelled`."]
pub enum FineTuningJobStatus {
    ValidatingFiles,
    Queued,
    Running,
    Succeeded,
    Failed,
    Cancelled,
}
#[doc = "The `fine_tuning.job.checkpoint` object represents a model checkpoint for a fine-tuning job that is ready to use.\n"]
pub struct FineTuningJobCheckpoint {
    #[doc = "The Unix timestamp (in seconds) for when the checkpoint was created."]
    pub created_at: u64,
    #[doc = "The name of the fine-tuned checkpoint model that is created."]
    pub fine_tuned_model_checkpoint: String,
    #[doc = "The name of the fine-tuning job that this checkpoint was created from."]
    pub fine_tuning_job_id: String,
    #[doc = "The checkpoint identifier, which can be referenced in the API endpoints."]
    pub id: String,
    #[doc = "Metrics at the step number during the fine-tuning job."]
    pub metrics: FineTuningJobCheckpointMetrics,
    #[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
    pub object: FineTuningJobCheckpointObject,
    #[doc = "The step number that the checkpoint was created at."]
    pub step_number: u64,
}
#[doc = "Metrics at the step number during the fine-tuning job."]
pub struct FineTuningJobCheckpointMetrics {
    pub full_valid_loss: f64,
    pub full_valid_mean_token_accuracy: f64,
    pub step: f64,
    pub train_loss: f64,
    pub train_mean_token_accuracy: f64,
    pub valid_loss: f64,
    pub valid_mean_token_accuracy: f64,
}
#[doc = "The object type, which is always \"fine_tuning.job.checkpoint\"."]
pub enum FineTuningJobCheckpointObject {
    FineTuningJobCheckpoint,
}
#[doc = "Fine-tuning job event object"]
pub struct FineTuningJobEvent {
    #[doc = "The Unix timestamp (in seconds) for when the fine-tuning job was created."]
    pub created_at: u64,
    #[doc = "The data associated with the event."]
    pub data: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The object identifier."]
    pub id: String,
    #[doc = "The log level of the event."]
    pub level: FineTuningJobEventLevel,
    #[doc = "The message of the event."]
    pub message: String,
    #[doc = "The object type, which is always \"fine_tuning.job.event\"."]
    pub object: FineTuningJobEventObject,
    #[doc = "The type of event."]
    pub type_: FineTuningJobEventType,
}
#[doc = "The log level of the event."]
pub enum FineTuningJobEventLevel {
    Info,
    Warn,
    Error,
}
#[doc = "The object type, which is always \"fine_tuning.job.event\"."]
pub enum FineTuningJobEventObject {
    FineTuningJobEvent,
}
#[doc = "The type of event."]
pub enum FineTuningJobEventType {
    Message,
    Metrics,
}
#[doc = "The output of a function tool call."]
pub struct FunctionCallOutputItemParam {
    #[doc = "The unique ID of the function tool call generated by the model."]
    pub call_id: String,
    pub id: Vec<String>,
    #[doc = "A JSON string of the output of the function tool call."]
    pub output: String,
    pub status: Vec<FunctionCallOutputItemParamStatusInner>,
    #[doc = "The type of the function tool call output. Always `function_call_output`."]
    pub type_: FunctionCallOutputItemParamType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or `incomplete`. Populated when items are returned via API."]
pub enum FunctionCallOutputItemParamStatusInner {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the function tool call output. Always `function_call_output`."]
pub enum FunctionCallOutputItemParamType {
    FunctionCallOutput,
}
pub struct FunctionObject {
    #[doc = "A description of what the function does, used by the model to choose when and how to call the function."]
    pub description: String,
    #[doc = "The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64."]
    pub name: String,
    pub parameters: FunctionParameters,
    #[doc = "Whether to enable strict schema adherence when generating the function call. If set to true, the model will follow the exact schema defined in the `parameters` field. Only a subset of JSON Schema is supported when `strict` is `true`. Learn more about Structured Outputs in the [function calling guide](docs/guides/function-calling)."]
    pub strict: Vec<bool>,
}
#[doc = "The parameters the functions accepts, described as a JSON Schema object. See the [guide](/docs/guides/function-calling) for examples, and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/) for documentation about the format. \n\nOmitting `parameters` defines a function with an empty parameter list."]
pub type FunctionParameters = std::collections::BTreeMap<String, serde_json::Value>;
#[doc = "Defines a function in your own code the model can choose to call. Learn more about [function calling](https://platform.openai.com/docs/guides/function-calling)."]
pub struct FunctionTool {
    pub description: Vec<String>,
    #[doc = "The name of the function to call."]
    pub name: String,
    pub parameters: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
    pub strict: Vec<Vec<bool>>,
    #[doc = "The type of the function tool. Always `function`."]
    pub type_: FunctionToolType,
}
#[doc = "The type of the function tool. Always `function`."]
pub enum FunctionToolType {
    Function,
}
#[doc = "A tool call to run a function. See the \n[function calling guide](/docs/guides/function-calling) for more information.\n"]
pub struct FunctionToolCall {
    #[doc = "A JSON string of the arguments to pass to the function.\n"]
    pub arguments: String,
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    pub call_id: String,
    #[doc = "The unique ID of the function tool call.\n"]
    pub id: String,
    #[doc = "The name of the function to run.\n"]
    pub name: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionToolCallStatus,
    #[doc = "The type of the function tool call. Always `function_call`.\n"]
    pub type_: FunctionToolCallType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
pub enum FunctionToolCallStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the function tool call. Always `function_call`.\n"]
pub enum FunctionToolCallType {
    FunctionCall,
}
#[doc = "The output of a function tool call.\n"]
pub struct FunctionToolCallOutput {
    #[doc = "The unique ID of the function tool call generated by the model.\n"]
    pub call_id: String,
    #[doc = "The unique ID of the function tool call output. Populated when this item\nis returned via API.\n"]
    pub id: String,
    #[doc = "A JSON string of the output of the function tool call.\n"]
    pub output: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: FunctionToolCallOutputStatus,
    #[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
    pub type_: FunctionToolCallOutputType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
pub enum FunctionToolCallOutputStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the function tool call output. Always `function_call_output`.\n"]
pub enum FunctionToolCallOutputType {
    FunctionCallOutput,
}
pub struct FunctionToolCallOutputResource {
    pub all_of_0: FunctionToolCallOutput,
    pub all_of_1: FunctionToolCallOutputResource1,
}
pub struct FunctionToolCallOutputResource1 {
    #[doc = "The unique ID of the function call tool output.\n"]
    pub id: String,
}
pub struct FunctionToolCallResource {
    pub all_of_0: FunctionToolCall,
    pub all_of_1: FunctionToolCallResource1,
}
pub struct FunctionToolCallResource1 {
    #[doc = "The unique ID of the function tool call.\n"]
    pub id: String,
}
#[doc = "Represents the content or the URL of an image generated by the OpenAI API."]
pub struct Image {
    #[doc = "The base64-encoded JSON of the generated image. Default value for `gpt-image-1`, and only present if `response_format` is set to `b64_json` for `dall-e-2` and `dall-e-3`."]
    pub b64_json: String,
    #[doc = "For `dall-e-3` only, the revised prompt that was used to generate the image."]
    pub revised_prompt: String,
    #[doc = "When using `dall-e-2` or `dall-e-3`, the URL of the generated image if `response_format` is set to `url` (default value). Unsupported for `gpt-image-1`."]
    pub url: String,
}
#[doc = "The response from the image generation endpoint."]
pub struct ImagesResponse {
    #[doc = "The Unix timestamp (in seconds) of when the image was created."]
    pub created: u64,
    #[doc = "The list of generated images."]
    pub data: Vec<Image>,
    #[doc = "For `gpt-image-1` only, the token usage information for the image generation.\n"]
    pub usage: ImagesResponseUsage,
}
#[doc = "The input tokens detailed information for the image generation."]
pub struct ImagesResponseUsageInputTokensDetails {
    #[doc = "The number of image tokens in the input prompt."]
    pub image_tokens: u64,
    #[doc = "The number of text tokens in the input prompt."]
    pub text_tokens: u64,
}
#[doc = "For `gpt-image-1` only, the token usage information for the image generation.\n"]
pub struct ImagesResponseUsage {
    #[doc = "The number of tokens (images and text) in the input prompt."]
    pub input_tokens: u64,
    #[doc = "The input tokens detailed information for the image generation."]
    pub input_tokens_details: ImagesResponseUsageInputTokensDetails,
    #[doc = "The number of image tokens in the output image."]
    pub output_tokens: u64,
    #[doc = "The total number of tokens (images and text) used for the image generation."]
    pub total_tokens: u64,
}
#[doc = "Specify additional output data to include in the model response. Currently\nsupported values are:\n- `file_search_call.results`: Include the search results of\n  the file search tool call.\n- `message.input_image.image_url`: Include image urls from the input message.\n- `computer_call_output.output.image_url`: Include image urls from the computer call output.\n"]
pub enum Includable {
    FileSearchCallResults,
    MessageInputImageImageUrl,
    ComputerCallOutputOutputImageUrl,
}
#[doc = "An audio input to the model.\n"]
pub struct InputAudio {
    #[doc = "Base64-encoded audio data.\n"]
    pub data: String,
    #[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
    pub format: InputAudioFormat,
    #[doc = "The type of the input item. Always `input_audio`.\n"]
    pub type_: InputAudioType,
}
#[doc = "The format of the audio data. Currently supported formats are `mp3` and\n`wav`.\n"]
pub enum InputAudioFormat {
    Mp3,
    Wav,
}
#[doc = "The type of the input item. Always `input_audio`.\n"]
pub enum InputAudioType {
    InputAudio,
}
#[allow(clippy::large_enum_variant)]
pub enum InputContent {
    OneOf0(InputTextContent),
    OneOf1(InputImageContent),
    OneOf2(InputFileContent),
}
#[doc = "A file input to the model."]
pub struct InputFileContent {
    #[doc = "The content of the file to be sent to the model.\n"]
    pub file_data: String,
    pub file_id: Vec<String>,
    #[doc = "The name of the file to be sent to the model."]
    pub filename: String,
    #[doc = "The type of the input item. Always `input_file`."]
    pub type_: InputFileContentType,
}
#[doc = "The type of the input item. Always `input_file`."]
pub enum InputFileContentType {
    InputFile,
}
#[doc = "An image input to the model. Learn about [image inputs](/docs/guides/vision)."]
pub struct InputImageContent {
    #[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`."]
    pub detail: InputImageContentDetail,
    pub file_id: Vec<String>,
    pub image_url: Vec<String>,
    #[doc = "The type of the input item. Always `input_image`."]
    pub type_: InputImageContentType,
}
#[doc = "The detail level of the image to be sent to the model. One of `high`, `low`, or `auto`. Defaults to `auto`."]
pub enum InputImageContentDetail {
    Low,
    High,
    Auto,
}
#[doc = "The type of the input item. Always `input_image`."]
pub enum InputImageContentType {
    InputImage,
}
#[allow(clippy::large_enum_variant)]
pub enum InputItem {
    OneOf0(EasyInputMessage),
    OneOf1(std::collections::BTreeMap<String, serde_json::Value>),
    OneOf2(ItemReferenceParam),
}
#[doc = "A message input to the model with a role indicating instruction following\nhierarchy. Instructions given with the `developer` or `system` role take\nprecedence over instructions given with the `user` role.\n"]
pub struct InputMessage {
    pub content: InputMessageContentList,
    #[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
    pub role: InputMessageRole,
    #[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: InputMessageStatus,
    #[doc = "The type of the message input. Always set to `message`.\n"]
    pub type_: InputMessageType,
}
#[doc = "The role of the message input. One of `user`, `system`, or `developer`.\n"]
pub enum InputMessageRole {
    User,
    System,
    Developer,
}
#[doc = "The status of item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
pub enum InputMessageStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the message input. Always set to `message`.\n"]
pub enum InputMessageType {
    Message,
}
#[doc = "A list of one or many input items to the model, containing different content \ntypes.\n"]
pub type InputMessageContentList = Vec<InputContent>;
pub struct InputMessageResource {
    pub all_of_0: InputMessage,
    pub all_of_1: InputMessageResource1,
}
pub struct InputMessageResource1 {
    #[doc = "The unique ID of the message input.\n"]
    pub id: String,
}
#[doc = "A text input to the model."]
pub struct InputTextContent {
    #[doc = "The text input to the model."]
    pub text: String,
    #[doc = "The type of the input item. Always `input_text`."]
    pub type_: InputTextContentType,
}
#[doc = "The type of the input item. Always `input_text`."]
pub enum InputTextContentType {
    InputText,
}
#[doc = "Represents an individual `invite` to the organization."]
pub struct Invite {
    #[doc = "The Unix timestamp (in seconds) of when the invite was accepted."]
    pub accepted_at: u64,
    #[doc = "The email address of the individual to whom the invite was sent"]
    pub email: String,
    #[doc = "The Unix timestamp (in seconds) of when the invite expires."]
    pub expires_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) of when the invite was sent."]
    pub invited_at: u64,
    #[doc = "The object type, which is always `organization.invite`"]
    pub object: InviteObject,
    #[doc = "The projects that were granted membership upon acceptance of the invite."]
    pub projects: Vec<InviteProjectsInner>,
    #[doc = "`owner` or `reader`"]
    pub role: InviteRole,
    #[doc = "`accepted`,`expired`, or `pending`"]
    pub status: InviteStatus,
}
#[doc = "The object type, which is always `organization.invite`"]
pub enum InviteObject {
    OrganizationInvite,
}
#[doc = "Project membership role"]
pub enum InviteProjectsInnerRole {
    Member,
    Owner,
}
pub struct InviteProjectsInner {
    #[doc = "Project's public ID"]
    pub id: String,
    #[doc = "Project membership role"]
    pub role: InviteProjectsInnerRole,
}
#[doc = "`owner` or `reader`"]
pub enum InviteRole {
    Owner,
    Reader,
}
#[doc = "`accepted`,`expired`, or `pending`"]
pub enum InviteStatus {
    Accepted,
    Expired,
    Pending,
}
pub struct InviteDeleteResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    #[doc = "The object type, which is always `organization.invite.deleted`"]
    pub object: InviteDeleteResponseObject,
}
#[doc = "The object type, which is always `organization.invite.deleted`"]
pub enum InviteDeleteResponseObject {
    OrganizationInviteDeleted,
}
pub struct InviteListResponse {
    pub data: Vec<Invite>,
    #[doc = "The first `invite_id` in the retrieved `list`"]
    pub first_id: String,
    #[doc = "The `has_more` property is used for pagination to indicate there are additional results."]
    pub has_more: Vec<bool>,
    #[doc = "The last `invite_id` in the retrieved `list`"]
    pub last_id: String,
    #[doc = "The object type, which is always `list`"]
    pub object: InviteListResponseObject,
}
#[doc = "The object type, which is always `list`"]
pub enum InviteListResponseObject {
    List,
}
pub struct InviteRequest {
    #[doc = "Send an email to this address"]
    pub email: String,
    #[doc = "An array of projects to which membership is granted at the same time the org invite is accepted. If omitted, the user will be invited to the default project for compatibility with legacy behavior."]
    pub projects: Vec<InviteRequestProjectsInner>,
    #[doc = "`owner` or `reader`"]
    pub role: InviteRequestRole,
}
#[doc = "Project membership role"]
pub enum InviteRequestProjectsInnerRole {
    Member,
    Owner,
}
pub struct InviteRequestProjectsInner {
    #[doc = "Project's public ID"]
    pub id: String,
    #[doc = "Project membership role"]
    pub role: InviteRequestProjectsInnerRole,
}
#[doc = "`owner` or `reader`"]
pub enum InviteRequestRole {
    Reader,
    Owner,
}
#[doc = "Content item used to generate a response.\n"]
pub type Item = std::collections::BTreeMap<String, serde_json::Value>;
#[doc = "An internal identifier for an item to reference."]
pub struct ItemReferenceParam {
    #[doc = "The ID of the item to reference."]
    pub id: String,
    pub type_: Vec<ItemReferenceParamTypeInner>,
}
#[doc = "The type of item to reference. Always `item_reference`."]
pub enum ItemReferenceParamTypeInner {
    ItemReference,
}
#[doc = "Content item used to generate a response.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ItemResource {
    OneOf0(InputMessageResource),
    OneOf1(OutputMessage),
    OneOf2(FileSearchToolCall),
    OneOf3(ComputerToolCall),
    OneOf4(ComputerToolCallOutputResource),
    OneOf5(WebSearchToolCall),
    OneOf6(FunctionToolCallResource),
    OneOf7(FunctionToolCallOutputResource),
}
#[doc = "A collection of keypresses the model would like to perform.\n"]
pub struct KeyPress {
    #[doc = "The combination of keys the model is requesting to be pressed. This is an\narray of strings, each representing a key.\n"]
    pub keys: Vec<String>,
    #[doc = "Specifies the event type. For a keypress action, this property is \nalways set to `keypress`.\n"]
    pub type_: KeyPressType,
}
#[doc = "Specifies the event type. For a keypress action, this property is \nalways set to `keypress`.\n"]
pub enum KeyPressType {
    Keypress,
}
pub struct ListAssistantsResponse {
    pub data: Vec<AssistantObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListAuditLogsResponse {
    pub data: Vec<AuditLog>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ListAuditLogsResponseObject,
}
pub enum ListAuditLogsResponseObject {
    List,
}
pub struct ListBatchesResponse {
    pub data: Vec<Batch>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ListBatchesResponseObject,
}
pub enum ListBatchesResponseObject {
    List,
}
pub struct ListCertificatesResponse {
    pub data: Vec<Certificate>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ListCertificatesResponseObject,
}
pub enum ListCertificatesResponseObject {
    List,
}
pub struct ListFilesResponse {
    pub data: Vec<OpenAiFile>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListFineTuningCheckpointPermissionResponse {
    pub data: Vec<FineTuningCheckpointPermission>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ListFineTuningCheckpointPermissionResponseObject,
}
pub enum ListFineTuningCheckpointPermissionResponseObject {
    List,
}
pub struct ListFineTuningJobCheckpointsResponse {
    pub data: Vec<FineTuningJobCheckpoint>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ListFineTuningJobCheckpointsResponseObject,
}
pub enum ListFineTuningJobCheckpointsResponseObject {
    List,
}
pub struct ListFineTuningJobEventsResponse {
    pub data: Vec<FineTuningJobEvent>,
    pub has_more: Vec<bool>,
    pub object: ListFineTuningJobEventsResponseObject,
}
pub enum ListFineTuningJobEventsResponseObject {
    List,
}
pub struct ListMessagesResponse {
    pub data: Vec<MessageObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListModelsResponse {
    pub data: Vec<Model>,
    pub object: ListModelsResponseObject,
}
pub enum ListModelsResponseObject {
    List,
}
pub struct ListPaginatedFineTuningJobsResponse {
    pub data: Vec<FineTuningJob>,
    pub has_more: Vec<bool>,
    pub object: ListPaginatedFineTuningJobsResponseObject,
}
pub enum ListPaginatedFineTuningJobsResponseObject {
    List,
}
pub struct ListRunStepsResponse {
    pub data: Vec<RunStepObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListRunsResponse {
    pub data: Vec<RunObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListVectorStoreFilesResponse {
    pub data: Vec<VectorStoreFileObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ListVectorStoresResponse {
    pub data: Vec<VectorStoreObject>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
#[doc = "A log probability object.\n"]
pub struct LogProbProperties {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    pub bytes: Vec<u64>,
    #[doc = "The log probability of the token.\n"]
    pub logprob: f64,
    #[doc = "The token that was used to generate the log probability.\n"]
    pub token: String,
}
#[doc = "References an image [File](/docs/api-reference/files) in the content of a message."]
pub struct MessageContentImageFileObject {
    pub image_file: MessageContentImageFileObjectImageFile,
    #[doc = "Always `image_file`."]
    pub type_: MessageContentImageFileObjectType,
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
pub enum MessageContentImageFileObjectImageFileDetail {
    Auto,
    Low,
    High,
}
pub struct MessageContentImageFileObjectImageFile {
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    pub detail: MessageContentImageFileObjectImageFileDetail,
    #[doc = "The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    pub file_id: String,
}
#[doc = "Always `image_file`."]
pub enum MessageContentImageFileObjectType {
    ImageFile,
}
#[doc = "References an image URL in the content of a message."]
pub struct MessageContentImageUrlObject {
    pub image_url: MessageContentImageUrlObjectImageUrl,
    #[doc = "The type of the content part."]
    pub type_: MessageContentImageUrlObjectType,
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
pub enum MessageContentImageUrlObjectImageUrlDetail {
    Auto,
    Low,
    High,
}
pub struct MessageContentImageUrlObjectImageUrl {
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`. Default value is `auto`"]
    pub detail: MessageContentImageUrlObjectImageUrlDetail,
    #[doc = "The external URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    pub url: String,
}
#[doc = "The type of the content part."]
pub enum MessageContentImageUrlObjectType {
    ImageUrl,
}
#[doc = "The refusal content generated by the assistant."]
pub struct MessageContentRefusalObject {
    pub refusal: String,
    #[doc = "Always `refusal`."]
    pub type_: MessageContentRefusalObjectType,
}
#[doc = "Always `refusal`."]
pub enum MessageContentRefusalObjectType {
    Refusal,
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
pub struct MessageContentTextAnnotationsFileCitationObject {
    pub end_index: u64,
    pub file_citation: MessageContentTextAnnotationsFileCitationObjectFileCitation,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: String,
    #[doc = "Always `file_citation`."]
    pub type_: MessageContentTextAnnotationsFileCitationObjectType,
}
pub struct MessageContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    pub file_id: String,
}
#[doc = "Always `file_citation`."]
pub enum MessageContentTextAnnotationsFileCitationObjectType {
    FileCitation,
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
pub struct MessageContentTextAnnotationsFilePathObject {
    pub end_index: u64,
    pub file_path: MessageContentTextAnnotationsFilePathObjectFilePath,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: String,
    #[doc = "Always `file_path`."]
    pub type_: MessageContentTextAnnotationsFilePathObjectType,
}
pub struct MessageContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    pub file_id: String,
}
#[doc = "Always `file_path`."]
pub enum MessageContentTextAnnotationsFilePathObjectType {
    FilePath,
}
#[doc = "The text content that is part of a message."]
pub struct MessageContentTextObject {
    pub text: MessageContentTextObjectText,
    #[doc = "Always `text`."]
    pub type_: MessageContentTextObjectType,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageContentTextObjectTextAnnotationsInner {
    OneOf0(MessageContentTextAnnotationsFileCitationObject),
    OneOf1(MessageContentTextAnnotationsFilePathObject),
}
pub struct MessageContentTextObjectText {
    pub annotations: Vec<MessageContentTextObjectTextAnnotationsInner>,
    #[doc = "The data that makes up the text."]
    pub value: String,
}
#[doc = "Always `text`."]
pub enum MessageContentTextObjectType {
    Text,
}
#[doc = "References an image [File](/docs/api-reference/files) in the content of a message."]
pub struct MessageDeltaContentImageFileObject {
    pub image_file: MessageDeltaContentImageFileObjectImageFile,
    #[doc = "The index of the content part in the message."]
    pub index: u64,
    #[doc = "Always `image_file`."]
    pub type_: MessageDeltaContentImageFileObjectType,
}
#[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
pub enum MessageDeltaContentImageFileObjectImageFileDetail {
    Auto,
    Low,
    High,
}
pub struct MessageDeltaContentImageFileObjectImageFile {
    #[doc = "Specifies the detail level of the image if specified by the user. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    pub detail: MessageDeltaContentImageFileObjectImageFileDetail,
    #[doc = "The [File](/docs/api-reference/files) ID of the image in the message content. Set `purpose=\"vision\"` when uploading the File if you need to later display the file content."]
    pub file_id: String,
}
#[doc = "Always `image_file`."]
pub enum MessageDeltaContentImageFileObjectType {
    ImageFile,
}
#[doc = "References an image URL in the content of a message."]
pub struct MessageDeltaContentImageUrlObject {
    pub image_url: MessageDeltaContentImageUrlObjectImageUrl,
    #[doc = "The index of the content part in the message."]
    pub index: u64,
    #[doc = "Always `image_url`."]
    pub type_: MessageDeltaContentImageUrlObjectType,
}
#[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
pub enum MessageDeltaContentImageUrlObjectImageUrlDetail {
    Auto,
    Low,
    High,
}
pub struct MessageDeltaContentImageUrlObjectImageUrl {
    #[doc = "Specifies the detail level of the image. `low` uses fewer tokens, you can opt in to high resolution using `high`."]
    pub detail: MessageDeltaContentImageUrlObjectImageUrlDetail,
    #[doc = "The URL of the image, must be a supported image types: jpeg, jpg, png, gif, webp."]
    pub url: String,
}
#[doc = "Always `image_url`."]
pub enum MessageDeltaContentImageUrlObjectType {
    ImageUrl,
}
#[doc = "The refusal content that is part of a message."]
pub struct MessageDeltaContentRefusalObject {
    #[doc = "The index of the refusal part in the message."]
    pub index: u64,
    pub refusal: String,
    #[doc = "Always `refusal`."]
    pub type_: MessageDeltaContentRefusalObjectType,
}
#[doc = "Always `refusal`."]
pub enum MessageDeltaContentRefusalObjectType {
    Refusal,
}
#[doc = "A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files."]
pub struct MessageDeltaContentTextAnnotationsFileCitationObject {
    pub end_index: u64,
    pub file_citation: MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation,
    #[doc = "The index of the annotation in the text content part."]
    pub index: u64,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: String,
    #[doc = "Always `file_citation`."]
    pub type_: MessageDeltaContentTextAnnotationsFileCitationObjectType,
}
pub struct MessageDeltaContentTextAnnotationsFileCitationObjectFileCitation {
    #[doc = "The ID of the specific File the citation is from."]
    pub file_id: String,
    #[doc = "The specific quote in the file."]
    pub quote: String,
}
#[doc = "Always `file_citation`."]
pub enum MessageDeltaContentTextAnnotationsFileCitationObjectType {
    FileCitation,
}
#[doc = "A URL for the file that's generated when the assistant used the `code_interpreter` tool to generate a file."]
pub struct MessageDeltaContentTextAnnotationsFilePathObject {
    pub end_index: u64,
    pub file_path: MessageDeltaContentTextAnnotationsFilePathObjectFilePath,
    #[doc = "The index of the annotation in the text content part."]
    pub index: u64,
    pub start_index: u64,
    #[doc = "The text in the message content that needs to be replaced."]
    pub text: String,
    #[doc = "Always `file_path`."]
    pub type_: MessageDeltaContentTextAnnotationsFilePathObjectType,
}
pub struct MessageDeltaContentTextAnnotationsFilePathObjectFilePath {
    #[doc = "The ID of the file that was generated."]
    pub file_id: String,
}
#[doc = "Always `file_path`."]
pub enum MessageDeltaContentTextAnnotationsFilePathObjectType {
    FilePath,
}
#[doc = "The text content that is part of a message."]
pub struct MessageDeltaContentTextObject {
    #[doc = "The index of the content part in the message."]
    pub index: u64,
    pub text: MessageDeltaContentTextObjectText,
    #[doc = "Always `text`."]
    pub type_: MessageDeltaContentTextObjectType,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageDeltaContentTextObjectTextAnnotationsInner {
    OneOf0(MessageDeltaContentTextAnnotationsFileCitationObject),
    OneOf1(MessageDeltaContentTextAnnotationsFilePathObject),
}
pub struct MessageDeltaContentTextObjectText {
    pub annotations: Vec<MessageDeltaContentTextObjectTextAnnotationsInner>,
    #[doc = "The data that makes up the text."]
    pub value: String,
}
#[doc = "Always `text`."]
pub enum MessageDeltaContentTextObjectType {
    Text,
}
#[doc = "Represents a message delta i.e. any changed fields on a message during streaming.\n"]
pub struct MessageDeltaObject {
    #[doc = "The delta containing the fields that have changed on the Message."]
    pub delta: MessageDeltaObjectDelta,
    #[doc = "The identifier of the message, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always `thread.message.delta`."]
    pub object: MessageDeltaObjectObject,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageDeltaObjectDeltaContentInner {
    OneOf0(MessageDeltaContentImageFileObject),
    OneOf1(MessageDeltaContentTextObject),
    OneOf2(MessageDeltaContentRefusalObject),
    OneOf3(MessageDeltaContentImageUrlObject),
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
pub enum MessageDeltaObjectDeltaRole {
    User,
    Assistant,
}
#[doc = "The delta containing the fields that have changed on the Message."]
pub struct MessageDeltaObjectDelta {
    #[doc = "The content of the message in array of text and/or images."]
    pub content: Vec<MessageDeltaObjectDeltaContentInner>,
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    pub role: MessageDeltaObjectDeltaRole,
}
#[doc = "The object type, which is always `thread.message.delta`."]
pub enum MessageDeltaObjectObject {
    ThreadMessageDelta,
}
#[doc = "Represents a message within a [thread](/docs/api-reference/threads)."]
pub struct MessageObject {
    #[doc = "If applicable, the ID of the [assistant](/docs/api-reference/assistants) that authored this message."]
    pub assistant_id: String,
    #[doc = "A list of files attached to the message, and the tools they were added to."]
    pub attachments: Vec<MessageObjectAttachmentsInner>,
    #[doc = "The Unix timestamp (in seconds) for when the message was completed."]
    pub completed_at: u64,
    #[doc = "The content of the message in array of text and/or images."]
    pub content: Vec<MessageObjectContentInner>,
    #[doc = "The Unix timestamp (in seconds) for when the message was created."]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the message was marked as incomplete."]
    pub incomplete_at: u64,
    #[doc = "On an incomplete message, details about why the message is incomplete."]
    pub incomplete_details: MessageObjectIncompleteDetails,
    pub metadata: Metadata,
    #[doc = "The object type, which is always `thread.message`."]
    pub object: MessageObjectObject,
    #[doc = "The entity that produced the message. One of `user` or `assistant`."]
    pub role: MessageObjectRole,
    #[doc = "The ID of the [run](/docs/api-reference/runs) associated with the creation of this message. Value is `null` when messages are created manually using the create message or create thread endpoints."]
    pub run_id: String,
    #[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
    pub status: MessageObjectStatus,
    #[doc = "The [thread](/docs/api-reference/threads) ID that this message belongs to."]
    pub thread_id: String,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageObjectAttachmentsInnerToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearchTypeOnly),
}
pub struct MessageObjectAttachmentsInner {
    #[doc = "The ID of the file to attach to the message."]
    pub file_id: String,
    #[doc = "The tools to add this file to."]
    pub tools: Vec<MessageObjectAttachmentsInnerToolsInner>,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageObjectContentInner {
    OneOf0(MessageContentImageFileObject),
    OneOf1(MessageContentImageUrlObject),
    OneOf2(MessageContentTextObject),
    OneOf3(MessageContentRefusalObject),
}
#[doc = "The reason the message is incomplete."]
pub enum MessageObjectIncompleteDetailsReason {
    ContentFilter,
    MaxTokens,
    RunCancelled,
    RunExpired,
    RunFailed,
}
#[doc = "On an incomplete message, details about why the message is incomplete."]
pub struct MessageObjectIncompleteDetails {
    #[doc = "The reason the message is incomplete."]
    pub reason: MessageObjectIncompleteDetailsReason,
}
#[doc = "The object type, which is always `thread.message`."]
pub enum MessageObjectObject {
    ThreadMessage,
}
#[doc = "The entity that produced the message. One of `user` or `assistant`."]
pub enum MessageObjectRole {
    User,
    Assistant,
}
#[doc = "The status of the message, which can be either `in_progress`, `incomplete`, or `completed`."]
pub enum MessageObjectStatus {
    InProgress,
    Incomplete,
    Completed,
}
#[doc = "The text content that is part of a message."]
pub struct MessageRequestContentTextObject {
    #[doc = "Text content to be sent to the model"]
    pub text: String,
    #[doc = "Always `text`."]
    pub type_: MessageRequestContentTextObjectType,
}
#[doc = "Always `text`."]
pub enum MessageRequestContentTextObjectType {
    Text,
}
#[allow(clippy::large_enum_variant)]
pub enum MessageStreamEvent {
    OneOf0(MessageStreamEvent0),
    OneOf1(MessageStreamEvent1),
    OneOf2(MessageStreamEvent2),
    OneOf3(MessageStreamEvent3),
    OneOf4(MessageStreamEvent4),
}
pub enum MessageStreamEvent0Event {
    ThreadMessageCreated,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) is created."]
pub struct MessageStreamEvent0 {
    pub data: MessageObject,
    pub event: MessageStreamEvent0Event,
}
pub enum MessageStreamEvent1Event {
    ThreadMessageInProgress,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) moves to an `in_progress` state."]
pub struct MessageStreamEvent1 {
    pub data: MessageObject,
    pub event: MessageStreamEvent1Event,
}
pub enum MessageStreamEvent2Event {
    ThreadMessageDelta,
}
#[doc = "Occurs when parts of a [Message](/docs/api-reference/messages/object) are being streamed."]
pub struct MessageStreamEvent2 {
    pub data: MessageDeltaObject,
    pub event: MessageStreamEvent2Event,
}
pub enum MessageStreamEvent3Event {
    ThreadMessageCompleted,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) is completed."]
pub struct MessageStreamEvent3 {
    pub data: MessageObject,
    pub event: MessageStreamEvent3Event,
}
pub enum MessageStreamEvent4Event {
    ThreadMessageIncomplete,
}
#[doc = "Occurs when a [message](/docs/api-reference/messages/object) ends before it is completed."]
pub struct MessageStreamEvent4 {
    pub data: MessageObject,
    pub event: MessageStreamEvent4Event,
}
#[doc = "Set of 16 key-value pairs that can be attached to an object. This can be\nuseful for storing additional information about the object in a structured\nformat, and querying for objects via API or the dashboard. \n\nKeys are strings with a maximum length of 64 characters. Values are strings\nwith a maximum length of 512 characters.\n"]
pub type Metadata = std::collections::BTreeMap<String, String>;
#[doc = "Describes an OpenAI model offering that can be used with the API."]
pub struct Model {
    #[doc = "The Unix timestamp (in seconds) when the model was created."]
    pub created: u64,
    #[doc = "The model identifier, which can be referenced in the API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always \"model\"."]
    pub object: ModelObject,
    #[doc = "The organization that owns the model."]
    pub owned_by: String,
}
#[doc = "The object type, which is always \"model\"."]
pub enum ModelObject {
    Model,
}
#[allow(clippy::large_enum_variant)]
pub enum ModelIds {
    AnyOf0(ModelIdsShared),
    AnyOf1(ModelIdsResponses),
}
#[allow(clippy::large_enum_variant)]
pub enum ModelIdsResponses {
    AnyOf0(ModelIdsShared),
    AnyOf1(ModelIdsResponses1),
}
pub enum ModelIdsResponses1 {
    O1Pro,
    O1Pro20250319,
    ComputerUsePreview,
    ComputerUsePreview20250311,
}
#[allow(clippy::large_enum_variant)]
pub enum ModelIdsShared {
    AnyOf0(String),
    AnyOf1(ModelIdsShared1),
}
pub enum ModelIdsShared1 {
    Gpt41,
    Gpt41Mini,
    Gpt41Nano,
    Gpt4120250414,
    Gpt41Mini20250414,
    Gpt41Nano20250414,
    O4Mini,
    O4Mini20250416,
    O3,
    O320250416,
    O3Mini,
    O3Mini20250131,
    O1,
    O120241217,
    O1Preview,
    O1Preview20240912,
    O1Mini,
    O1Mini20240912,
    Gpt4o,
    Gpt4o20241120,
    Gpt4o20240806,
    Gpt4o20240513,
    Gpt4oAudioPreview,
    Gpt4oAudioPreview20241001,
    Gpt4oAudioPreview20241217,
    Gpt4oMiniAudioPreview,
    Gpt4oMiniAudioPreview20241217,
    Gpt4oSearchPreview,
    Gpt4oMiniSearchPreview,
    Gpt4oSearchPreview20250311,
    Gpt4oMiniSearchPreview20250311,
    Chatgpt4oLatest,
    Gpt4oMini,
    Gpt4oMini20240718,
    Gpt4Turbo,
    Gpt4Turbo20240409,
    Gpt40125Preview,
    Gpt4TurboPreview,
    Gpt41106Preview,
    Gpt4VisionPreview,
    Gpt4,
    Gpt40314,
    Gpt40613,
    Gpt432k,
    Gpt432k0314,
    Gpt432k0613,
    Gpt35Turbo,
    Gpt35Turbo16k,
    Gpt35Turbo0301,
    Gpt35Turbo0613,
    Gpt35Turbo1106,
    Gpt35Turbo0125,
    Gpt35Turbo16k0613,
}
pub struct ModelResponseProperties {
    pub metadata: Metadata,
    pub service_tier: ServiceTier,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\nWe generally recommend altering this or `top_p` but not both.\n"]
    pub temperature: f64,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling,\nwhere the model considers the results of the tokens with top_p probability\nmass. So 0.1 means only the tokens comprising the top 10% probability mass\nare considered.\n\nWe generally recommend altering this or `temperature` but not both.\n"]
    pub top_p: f64,
    #[doc = "A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](/docs/guides/safety-best-practices#end-user-ids).\n"]
    pub user: String,
}
pub struct ModifyAssistantRequest {
    #[doc = "The description of the assistant. The maximum length is 512 characters.\n"]
    pub description: String,
    #[doc = "The system instructions that the assistant uses. The maximum length is 256,000 characters.\n"]
    pub instructions: String,
    pub metadata: Metadata,
    #[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
    pub model: ModifyAssistantRequestModel,
    #[doc = "The name of the assistant. The maximum length is 256 characters.\n"]
    pub name: String,
    pub reasoning_effort: ReasoningEffort,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.\n"]
    pub temperature: f64,
    #[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: ModifyAssistantRequestToolResources,
    #[doc = "A list of tool enabled on the assistant. There can be a maximum of 128 tools per assistant. Tools can be of types `code_interpreter`, `file_search`, or `function`.\n"]
    pub tools: Vec<ModifyAssistantRequestToolsInner>,
    #[doc = "An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.\n\nWe generally recommend altering this or temperature but not both.\n"]
    pub top_p: f64,
}
#[doc = "ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models) for descriptions of them.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ModifyAssistantRequestModel {
    AnyOf0(String),
    AnyOf1(AssistantSupportedModels),
}
pub struct ModifyAssistantRequestToolResourcesCodeInterpreter {
    #[doc = "Overrides the list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct ModifyAssistantRequestToolResourcesFileSearch {
    #[doc = "Overrides the [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.\n"]
    pub vector_store_ids: Vec<String>,
}
#[doc = "A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct ModifyAssistantRequestToolResources {
    pub code_interpreter: ModifyAssistantRequestToolResourcesCodeInterpreter,
    pub file_search: ModifyAssistantRequestToolResourcesFileSearch,
}
#[allow(clippy::large_enum_variant)]
pub enum ModifyAssistantRequestToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
pub struct ModifyCertificateRequest {
    #[doc = "The updated name for the certificate"]
    pub name: String,
}
pub struct ModifyMessageRequest {
    pub metadata: Metadata,
}
pub struct ModifyRunRequest {
    pub metadata: Metadata,
}
pub struct ModifyThreadRequest {
    pub metadata: Metadata,
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: ModifyThreadRequestToolResources,
}
pub struct ModifyThreadRequestToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct ModifyThreadRequestToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    pub vector_store_ids: Vec<String>,
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct ModifyThreadRequestToolResources {
    pub code_interpreter: ModifyThreadRequestToolResourcesCodeInterpreter,
    pub file_search: ModifyThreadRequestToolResourcesFileSearch,
}
#[doc = "A mouse move action.\n"]
pub struct Move {
    #[doc = "Specifies the event type. For a move action, this property is \nalways set to `move`.\n"]
    pub type_: MoveType,
    #[doc = "The x-coordinate to move to.\n"]
    pub x: u64,
    #[doc = "The y-coordinate to move to.\n"]
    pub y: u64,
}
#[doc = "Specifies the event type. For a move action, this property is \nalways set to `move`.\n"]
pub enum MoveType {
    Move,
}
#[doc = "The `File` object represents a document that has been uploaded to OpenAI."]
pub struct OpenAiFile {
    #[doc = "The size of the file, in bytes."]
    pub bytes: u64,
    #[doc = "The Unix timestamp (in seconds) for when the file was created."]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the file will expire."]
    pub expires_at: u64,
    #[doc = "The name of the file."]
    pub filename: String,
    #[doc = "The file identifier, which can be referenced in the API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always `file`."]
    pub object: OpenAiFileObject,
    #[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`."]
    pub purpose: OpenAiFilePurpose,
    #[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
    pub status: OpenAiFileStatus,
    #[doc = "Deprecated. For details on why a fine-tuning training file failed validation, see the `error` field on `fine_tuning.job`."]
    pub status_details: String,
}
#[doc = "The object type, which is always `file`."]
pub enum OpenAiFileObject {
    File,
}
#[doc = "The intended purpose of the file. Supported values are `assistants`, `assistants_output`, `batch`, `batch_output`, `fine-tune`, `fine-tune-results` and `vision`."]
pub enum OpenAiFilePurpose {
    Assistants,
    AssistantsOutput,
    Batch,
    BatchOutput,
    FineTune,
    FineTuneResults,
    Vision,
}
#[doc = "Deprecated. The current status of the file, which can be either `uploaded`, `processed`, or `error`."]
pub enum OpenAiFileStatus {
    Uploaded,
    Processed,
    Error,
}
#[doc = "This is returned when the chunking strategy is unknown. Typically, this is because the file was indexed before the `chunking_strategy` concept was introduced in the API."]
pub struct OtherChunkingStrategyResponseParam {
    #[doc = "Always `other`."]
    pub type_: OtherChunkingStrategyResponseParamType,
}
#[doc = "Always `other`."]
pub enum OtherChunkingStrategyResponseParamType {
    Other,
}
#[doc = "An audio output from the model.\n"]
pub struct OutputAudio {
    #[doc = "Base64-encoded audio data from the model.\n"]
    pub data: String,
    #[doc = "The transcript of the audio data from the model.\n"]
    pub transcript: String,
    #[doc = "The type of the output audio. Always `output_audio`.\n"]
    pub type_: OutputAudioType,
}
#[doc = "The type of the output audio. Always `output_audio`.\n"]
pub enum OutputAudioType {
    OutputAudio,
}
#[allow(clippy::large_enum_variant)]
pub enum OutputContent {
    OneOf0(OutputTextContent),
    OneOf1(RefusalContent),
}
#[allow(clippy::large_enum_variant)]
pub enum OutputItem {
    AnyOf0(OutputMessage),
    AnyOf1(FileSearchToolCall),
    AnyOf2(FunctionToolCall),
    AnyOf3(WebSearchToolCall),
    AnyOf4(ComputerToolCall),
    AnyOf5(ReasoningItem),
}
#[doc = "An output message from the model.\n"]
pub struct OutputMessage {
    #[doc = "The content of the output message.\n"]
    pub content: Vec<OutputContent>,
    #[doc = "The unique ID of the output message.\n"]
    pub id: String,
    #[doc = "The role of the output message. Always `assistant`.\n"]
    pub role: OutputMessageRole,
    #[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
    pub status: OutputMessageStatus,
    #[doc = "The type of the output message. Always `message`.\n"]
    pub type_: OutputMessageType,
}
#[doc = "The role of the output message. Always `assistant`.\n"]
pub enum OutputMessageRole {
    Assistant,
}
#[doc = "The status of the message input. One of `in_progress`, `completed`, or\n`incomplete`. Populated when input items are returned via API.\n"]
pub enum OutputMessageStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the output message. Always `message`.\n"]
pub enum OutputMessageType {
    Message,
}
#[doc = "A text output from the model."]
pub struct OutputTextContent {
    #[doc = "The annotations of the text output."]
    pub annotations: Vec<Annotation>,
    #[doc = "The text output from the model."]
    pub text: String,
    #[doc = "The type of the output text. Always `output_text`."]
    pub type_: OutputTextContentType,
}
#[doc = "The type of the output text. Always `output_text`."]
pub enum OutputTextContentType {
    OutputText,
}
#[doc = "Whether to enable [parallel function calling](/docs/guides/function-calling#configuring-parallel-function-calling) during tool use."]
pub type ParallelToolCalls = Vec<bool>;
#[doc = "Static predicted output content, such as the content of a text file that is\nbeing regenerated.\n"]
pub struct PredictionContent {
    #[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
    pub content: PredictionContentContent,
    #[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
    pub type_: PredictionContentType,
}
#[doc = "The content that should be matched when generating a model response.\nIf generated tokens would match this content, the entire model response\ncan be returned much more quickly.\n"]
#[allow(clippy::large_enum_variant)]
pub enum PredictionContentContent {
    OneOf0(String),
    OneOf1(Vec<ChatCompletionRequestMessageContentPartText>),
}
#[doc = "The type of the predicted content you want to provide. This type is\ncurrently always `content`.\n"]
pub enum PredictionContentType {
    Content,
}
#[doc = "Represents an individual project."]
pub struct Project {
    #[doc = "The Unix timestamp (in seconds) of when the project was archived or `null`."]
    pub archived_at: u64,
    #[doc = "The Unix timestamp (in seconds) of when the project was created."]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the project. This appears in reporting."]
    pub name: String,
    #[doc = "The object type, which is always `organization.project`"]
    pub object: ProjectObject,
    #[doc = "`active` or `archived`"]
    pub status: ProjectStatus,
}
#[doc = "The object type, which is always `organization.project`"]
pub enum ProjectObject {
    OrganizationProject,
}
#[doc = "`active` or `archived`"]
pub enum ProjectStatus {
    Active,
    Archived,
}
#[doc = "Represents an individual API key in a project."]
pub struct ProjectApiKey {
    #[doc = "The Unix timestamp (in seconds) of when the API key was created"]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) of when the API key was last used."]
    pub last_used_at: u64,
    #[doc = "The name of the API key"]
    pub name: String,
    #[doc = "The object type, which is always `organization.project.api_key`"]
    pub object: ProjectApiKeyObject,
    pub owner: ProjectApiKeyOwner,
    #[doc = "The redacted value of the API key"]
    pub redacted_value: String,
}
#[doc = "The object type, which is always `organization.project.api_key`"]
pub enum ProjectApiKeyObject {
    OrganizationProjectApiKey,
}
#[doc = "`user` or `service_account`"]
pub enum ProjectApiKeyOwnerType {
    User,
    ServiceAccount,
}
pub struct ProjectApiKeyOwner {
    pub service_account: ProjectServiceAccount,
    #[doc = "`user` or `service_account`"]
    pub type_: ProjectApiKeyOwnerType,
    pub user: ProjectUser,
}
pub struct ProjectApiKeyDeleteResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: ProjectApiKeyDeleteResponseObject,
}
pub enum ProjectApiKeyDeleteResponseObject {
    OrganizationProjectApiKeyDeleted,
}
pub struct ProjectApiKeyListResponse {
    pub data: Vec<ProjectApiKey>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ProjectApiKeyListResponseObject,
}
pub enum ProjectApiKeyListResponseObject {
    List,
}
pub struct ProjectCreateRequest {
    #[doc = "The friendly name of the project, this name appears in reports."]
    pub name: String,
}
pub struct ProjectListResponse {
    pub data: Vec<Project>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ProjectListResponseObject,
}
pub enum ProjectListResponseObject {
    List,
}
#[doc = "Represents a project rate limit config."]
pub struct ProjectRateLimit {
    #[doc = "The maximum batch input tokens per day. Only present for relevant models."]
    pub batch_1_day_max_input_tokens: u64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The maximum audio megabytes per minute. Only present for relevant models."]
    pub max_audio_megabytes_per_1_minute: u64,
    #[doc = "The maximum images per minute. Only present for relevant models."]
    pub max_images_per_1_minute: u64,
    #[doc = "The maximum requests per day. Only present for relevant models."]
    pub max_requests_per_1_day: u64,
    #[doc = "The maximum requests per minute."]
    pub max_requests_per_1_minute: u64,
    #[doc = "The maximum tokens per minute."]
    pub max_tokens_per_1_minute: u64,
    #[doc = "The model this rate limit applies to."]
    pub model: String,
    #[doc = "The object type, which is always `project.rate_limit`"]
    pub object: ProjectRateLimitObject,
}
#[doc = "The object type, which is always `project.rate_limit`"]
pub enum ProjectRateLimitObject {
    ProjectRateLimit,
}
pub struct ProjectRateLimitListResponse {
    pub data: Vec<ProjectRateLimit>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ProjectRateLimitListResponseObject,
}
pub enum ProjectRateLimitListResponseObject {
    List,
}
pub struct ProjectRateLimitUpdateRequest {
    #[doc = "The maximum batch input tokens per day. Only relevant for certain models."]
    pub batch_1_day_max_input_tokens: u64,
    #[doc = "The maximum audio megabytes per minute. Only relevant for certain models."]
    pub max_audio_megabytes_per_1_minute: u64,
    #[doc = "The maximum images per minute. Only relevant for certain models."]
    pub max_images_per_1_minute: u64,
    #[doc = "The maximum requests per day. Only relevant for certain models."]
    pub max_requests_per_1_day: u64,
    #[doc = "The maximum requests per minute."]
    pub max_requests_per_1_minute: u64,
    #[doc = "The maximum tokens per minute."]
    pub max_tokens_per_1_minute: u64,
}
#[doc = "Represents an individual service account in a project."]
pub struct ProjectServiceAccount {
    #[doc = "The Unix timestamp (in seconds) of when the service account was created"]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the service account"]
    pub name: String,
    #[doc = "The object type, which is always `organization.project.service_account`"]
    pub object: ProjectServiceAccountObject,
    #[doc = "`owner` or `member`"]
    pub role: ProjectServiceAccountRole,
}
#[doc = "The object type, which is always `organization.project.service_account`"]
pub enum ProjectServiceAccountObject {
    OrganizationProjectServiceAccount,
}
#[doc = "`owner` or `member`"]
pub enum ProjectServiceAccountRole {
    Owner,
    Member,
}
pub struct ProjectServiceAccountApiKey {
    pub created_at: u64,
    pub id: String,
    pub name: String,
    #[doc = "The object type, which is always `organization.project.service_account.api_key`"]
    pub object: ProjectServiceAccountApiKeyObject,
    pub value: String,
}
#[doc = "The object type, which is always `organization.project.service_account.api_key`"]
pub enum ProjectServiceAccountApiKeyObject {
    OrganizationProjectServiceAccountApiKey,
}
pub struct ProjectServiceAccountCreateRequest {
    #[doc = "The name of the service account being created."]
    pub name: String,
}
pub struct ProjectServiceAccountCreateResponse {
    pub api_key: ProjectServiceAccountApiKey,
    pub created_at: u64,
    pub id: String,
    pub name: String,
    pub object: ProjectServiceAccountCreateResponseObject,
    #[doc = "Service accounts can only have one role of type `member`"]
    pub role: ProjectServiceAccountCreateResponseRole,
}
pub enum ProjectServiceAccountCreateResponseObject {
    OrganizationProjectServiceAccount,
}
#[doc = "Service accounts can only have one role of type `member`"]
pub enum ProjectServiceAccountCreateResponseRole {
    Member,
}
pub struct ProjectServiceAccountDeleteResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: ProjectServiceAccountDeleteResponseObject,
}
pub enum ProjectServiceAccountDeleteResponseObject {
    OrganizationProjectServiceAccountDeleted,
}
pub struct ProjectServiceAccountListResponse {
    pub data: Vec<ProjectServiceAccount>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: ProjectServiceAccountListResponseObject,
}
pub enum ProjectServiceAccountListResponseObject {
    List,
}
pub struct ProjectUpdateRequest {
    #[doc = "The updated name of the project, this name appears in reports."]
    pub name: String,
}
#[doc = "Represents an individual user in a project."]
pub struct ProjectUser {
    #[doc = "The Unix timestamp (in seconds) of when the project was added."]
    pub added_at: u64,
    #[doc = "The email address of the user"]
    pub email: String,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the user"]
    pub name: String,
    #[doc = "The object type, which is always `organization.project.user`"]
    pub object: ProjectUserObject,
    #[doc = "`owner` or `member`"]
    pub role: ProjectUserRole,
}
#[doc = "The object type, which is always `organization.project.user`"]
pub enum ProjectUserObject {
    OrganizationProjectUser,
}
#[doc = "`owner` or `member`"]
pub enum ProjectUserRole {
    Owner,
    Member,
}
pub struct ProjectUserCreateRequest {
    #[doc = "`owner` or `member`"]
    pub role: ProjectUserCreateRequestRole,
    #[doc = "The ID of the user."]
    pub user_id: String,
}
#[doc = "`owner` or `member`"]
pub enum ProjectUserCreateRequestRole {
    Owner,
    Member,
}
pub struct ProjectUserDeleteResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: ProjectUserDeleteResponseObject,
}
pub enum ProjectUserDeleteResponseObject {
    OrganizationProjectUserDeleted,
}
pub struct ProjectUserListResponse {
    pub data: Vec<ProjectUser>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: String,
}
pub struct ProjectUserUpdateRequest {
    #[doc = "`owner` or `member`"]
    pub role: ProjectUserUpdateRequestRole,
}
#[doc = "`owner` or `member`"]
pub enum ProjectUserUpdateRequestRole {
    Owner,
    Member,
}
pub struct RankingOptions {
    #[doc = "The ranker to use for the file search."]
    pub ranker: RankingOptionsRanker,
    #[doc = "The score threshold for the file search, a number between 0 and 1. Numbers closer to 1 will attempt to return only the most relevant results, but may return fewer results."]
    pub score_threshold: f64,
}
#[doc = "The ranker to use for the file search."]
pub enum RankingOptionsRanker {
    Auto,
    Default20241115,
}
#[doc = "A realtime client event.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeClientEvent {
    AnyOf0(RealtimeClientEventConversationItemCreate),
    AnyOf1(RealtimeClientEventConversationItemDelete),
    AnyOf2(RealtimeClientEventConversationItemRetrieve),
    AnyOf3(RealtimeClientEventConversationItemTruncate),
    AnyOf4(RealtimeClientEventInputAudioBufferAppend),
    AnyOf5(RealtimeClientEventInputAudioBufferClear),
    AnyOf6(RealtimeClientEventOutputAudioBufferClear),
    AnyOf7(RealtimeClientEventInputAudioBufferCommit),
    AnyOf8(RealtimeClientEventResponseCancel),
    AnyOf9(RealtimeClientEventResponseCreate),
    AnyOf10(RealtimeClientEventSessionUpdate),
    AnyOf11(RealtimeClientEventTranscriptionSessionUpdate),
}
#[doc = "Add a new Item to the Conversation's context, including messages, function \ncalls, and function call responses. This event can be used both to populate a \n\"history\" of the conversation and to add new items mid-stream, but has the \ncurrent limitation that it cannot populate assistant audio messages.\n\nIf successful, the server will respond with a `conversation.item.created` \nevent, otherwise an `error` event will be sent.\n"]
pub struct RealtimeClientEventConversationItemCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[doc = "The ID of the preceding item after which the new item will be inserted. \nIf not set, the new item will be appended to the end of the conversation.\nIf set to `root`, the new item will be added to the beginning of the conversation.\nIf set to an existing ID, it allows an item to be inserted mid-conversation. If the\nID cannot be found, an error will be returned and the item will not be added.\n"]
    pub previous_item_id: String,
    #[doc = "The event type, must be `conversation.item.create`."]
    pub type_: RealtimeClientEventConversationItemCreateType,
}
#[doc = "The event type, must be `conversation.item.create`."]
pub enum RealtimeClientEventConversationItemCreateType {
    ConversationItemCreate,
}
#[doc = "Send this event when you want to remove any item from the conversation \nhistory. The server will respond with a `conversation.item.deleted` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
pub struct RealtimeClientEventConversationItemDelete {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The ID of the item to delete."]
    pub item_id: String,
    #[doc = "The event type, must be `conversation.item.delete`."]
    pub type_: RealtimeClientEventConversationItemDeleteType,
}
#[doc = "The event type, must be `conversation.item.delete`."]
pub enum RealtimeClientEventConversationItemDeleteType {
    ConversationItemDelete,
}
#[doc = "Send this event when you want to retrieve the server's representation of a specific item in the conversation history. This is useful, for example, to inspect user audio after noise cancellation and VAD.\nThe server will respond with a `conversation.item.retrieved` event, \nunless the item does not exist in the conversation history, in which case the \nserver will respond with an error.\n"]
pub struct RealtimeClientEventConversationItemRetrieve {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The ID of the item to retrieve."]
    pub item_id: String,
    #[doc = "The event type, must be `conversation.item.retrieve`."]
    pub type_: RealtimeClientEventConversationItemRetrieveType,
}
#[doc = "The event type, must be `conversation.item.retrieve`."]
pub enum RealtimeClientEventConversationItemRetrieveType {
    ConversationItemRetrieve,
}
#[doc = "Send this event to truncate a previous assistant message’s audio. The server \nwill produce audio faster than realtime, so this event is useful when the user \ninterrupts to truncate audio that has already been sent to the client but not \nyet played. This will synchronize the server's understanding of the audio with \nthe client's playback.\n\nTruncating audio will delete the server-side text transcript to ensure there \nis not text in the context that hasn't been heard by the user.\n\nIf successful, the server will respond with a `conversation.item.truncated` \nevent. \n"]
pub struct RealtimeClientEventConversationItemTruncate {
    #[doc = "Inclusive duration up to which audio is truncated, in milliseconds. If \nthe audio_end_ms is greater than the actual audio duration, the server \nwill respond with an error.\n"]
    pub audio_end_ms: u64,
    #[doc = "The index of the content part to truncate. Set this to 0."]
    pub content_index: u64,
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The ID of the assistant message item to truncate. Only assistant message \nitems can be truncated.\n"]
    pub item_id: String,
    #[doc = "The event type, must be `conversation.item.truncate`."]
    pub type_: RealtimeClientEventConversationItemTruncateType,
}
#[doc = "The event type, must be `conversation.item.truncate`."]
pub enum RealtimeClientEventConversationItemTruncateType {
    ConversationItemTruncate,
}
#[doc = "Send this event to append audio bytes to the input audio buffer. The audio \nbuffer is temporary storage you can write to and later commit. In Server VAD \nmode, the audio buffer is used to detect speech and the server will decide \nwhen to commit. When Server VAD is disabled, you must commit the audio buffer\nmanually.\n\nThe client may choose how much audio to place in each event up to a maximum \nof 15 MiB, for example streaming smaller chunks from the client may allow the \nVAD to be more responsive. Unlike made other client events, the server will \nnot send a confirmation response to this event.\n"]
pub struct RealtimeClientEventInputAudioBufferAppend {
    #[doc = "Base64-encoded audio bytes. This must be in the format specified by the \n`input_audio_format` field in the session configuration.\n"]
    pub audio: String,
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.append`."]
    pub type_: RealtimeClientEventInputAudioBufferAppendType,
}
#[doc = "The event type, must be `input_audio_buffer.append`."]
pub enum RealtimeClientEventInputAudioBufferAppendType {
    InputAudioBufferAppend,
}
#[doc = "Send this event to clear the audio bytes in the buffer. The server will \nrespond with an `input_audio_buffer.cleared` event.\n"]
pub struct RealtimeClientEventInputAudioBufferClear {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.clear`."]
    pub type_: RealtimeClientEventInputAudioBufferClearType,
}
#[doc = "The event type, must be `input_audio_buffer.clear`."]
pub enum RealtimeClientEventInputAudioBufferClearType {
    InputAudioBufferClear,
}
#[doc = "Send this event to commit the user input audio buffer, which will create a \nnew user message item in the conversation. This event will produce an error \nif the input audio buffer is empty. When in Server VAD mode, the client does \nnot need to send this event, the server will commit the audio buffer \nautomatically.\n\nCommitting the input audio buffer will trigger input audio transcription \n(if enabled in session configuration), but it will not create a response \nfrom the model. The server will respond with an `input_audio_buffer.committed` \nevent.\n"]
pub struct RealtimeClientEventInputAudioBufferCommit {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.commit`."]
    pub type_: RealtimeClientEventInputAudioBufferCommitType,
}
#[doc = "The event type, must be `input_audio_buffer.commit`."]
pub enum RealtimeClientEventInputAudioBufferCommitType {
    InputAudioBufferCommit,
}
#[doc = "**WebRTC Only:** Emit to cut off the current audio response. This will trigger the server to\nstop generating audio and emit a `output_audio_buffer.cleared` event. This \nevent should be preceded by a `response.cancel` client event to stop the \ngeneration of the current response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
pub struct RealtimeClientEventOutputAudioBufferClear {
    #[doc = "The unique ID of the client event used for error handling."]
    pub event_id: String,
    #[doc = "The event type, must be `output_audio_buffer.clear`."]
    pub type_: RealtimeClientEventOutputAudioBufferClearType,
}
#[doc = "The event type, must be `output_audio_buffer.clear`."]
pub enum RealtimeClientEventOutputAudioBufferClearType {
    OutputAudioBufferClear,
}
#[doc = "Send this event to cancel an in-progress response. The server will respond \nwith a `response.cancelled` event or an error if there is no response to \ncancel.\n"]
pub struct RealtimeClientEventResponseCancel {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    #[doc = "A specific response ID to cancel - if not provided, will cancel an \nin-progress response in the default conversation.\n"]
    pub response_id: String,
    #[doc = "The event type, must be `response.cancel`."]
    pub type_: RealtimeClientEventResponseCancelType,
}
#[doc = "The event type, must be `response.cancel`."]
pub enum RealtimeClientEventResponseCancelType {
    ResponseCancel,
}
#[doc = "This event instructs the server to create a Response, which means triggering \nmodel inference. When in Server VAD mode, the server will create Responses \nautomatically.\n\nA Response will include at least one Item, and may have two, in which case \nthe second will be a function call. These Items will be appended to the \nconversation history.\n\nThe server will respond with a `response.created` event, events for Items \nand content created, and finally a `response.done` event to indicate the \nResponse is complete.\n\nThe `response.create` event includes inference configuration like \n`instructions`, and `temperature`. These fields will override the Session's \nconfiguration for this Response only.\n"]
pub struct RealtimeClientEventResponseCreate {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    pub response: RealtimeResponseCreateParams,
    #[doc = "The event type, must be `response.create`."]
    pub type_: RealtimeClientEventResponseCreateType,
}
#[doc = "The event type, must be `response.create`."]
pub enum RealtimeClientEventResponseCreateType {
    ResponseCreate,
}
#[doc = "Send this event to update the session’s default configuration.\nThe client may send this event at any time to update any field,\nexcept for `voice`. However, note that once a session has been\ninitialized with a particular `model`, it can’t be changed to\nanother model using `session.update`.\n\nWhen the server receives a `session.update`, it will respond\nwith a `session.updated` event showing the full, effective configuration.\nOnly the fields that are present are updated. To clear a field like\n`instructions`, pass an empty string.\n"]
pub struct RealtimeClientEventSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    pub session: RealtimeSessionCreateRequest,
    #[doc = "The event type, must be `session.update`."]
    pub type_: RealtimeClientEventSessionUpdateType,
}
#[doc = "The event type, must be `session.update`."]
pub enum RealtimeClientEventSessionUpdateType {
    SessionUpdate,
}
#[doc = "Send this event to update a transcription session.\n"]
pub struct RealtimeClientEventTranscriptionSessionUpdate {
    #[doc = "Optional client-generated ID used to identify this event."]
    pub event_id: String,
    pub session: RealtimeTranscriptionSessionCreateRequest,
    #[doc = "The event type, must be `transcription_session.update`."]
    pub type_: RealtimeClientEventTranscriptionSessionUpdateType,
}
#[doc = "The event type, must be `transcription_session.update`."]
pub enum RealtimeClientEventTranscriptionSessionUpdateType {
    TranscriptionSessionUpdate,
}
#[doc = "The item to add to the conversation."]
pub struct RealtimeConversationItem {
    #[doc = "The arguments of the function call (for `function_call` items).\n"]
    pub arguments: String,
    #[doc = "The ID of the function call (for `function_call` and \n`function_call_output` items). If passed on a `function_call_output` \nitem, the server will check that a `function_call` item with the same \nID exists in the conversation history.\n"]
    pub call_id: String,
    #[doc = "The content of the message, applicable for `message` items. \n- Message items of role `system` support only `input_text` content\n- Message items of role `user` support `input_text` and `input_audio` \n  content\n- Message items of role `assistant` support `text` content.\n"]
    pub content: Vec<RealtimeConversationItemContentInner>,
    #[doc = "The unique ID of the item, this can be generated by the client to help \nmanage server-side context, but is not required because the server will \ngenerate one if not provided.\n"]
    pub id: String,
    #[doc = "The name of the function being called (for `function_call` items).\n"]
    pub name: String,
    #[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
    pub object: RealtimeConversationItemObject,
    #[doc = "The output of the function call (for `function_call_output` items).\n"]
    pub output: String,
    #[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
    pub role: RealtimeConversationItemRole,
    #[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
    pub status: RealtimeConversationItemStatus,
    #[doc = "The type of the item (`message`, `function_call`, `function_call_output`).\n"]
    pub type_: RealtimeConversationItemType,
}
#[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
pub enum RealtimeConversationItemContentInnerType {
    InputAudio,
    InputText,
    ItemReference,
    Text,
}
pub struct RealtimeConversationItemContentInner {
    #[doc = "Base64-encoded audio bytes, used for `input_audio` content type.\n"]
    pub audio: String,
    #[doc = "ID of a previous conversation item to reference (for `item_reference`\ncontent types in `response.create` events). These can reference both\nclient and server created items.\n"]
    pub id: String,
    #[doc = "The text content, used for `input_text` and `text` content types.\n"]
    pub text: String,
    #[doc = "The transcript of the audio, used for `input_audio` content type.\n"]
    pub transcript: String,
    #[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
    pub type_: RealtimeConversationItemContentInnerType,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
pub enum RealtimeConversationItemObject {
    RealtimeItem,
}
#[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
pub enum RealtimeConversationItemRole {
    User,
    Assistant,
    System,
}
#[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
pub enum RealtimeConversationItemStatus {
    Completed,
    Incomplete,
}
#[doc = "The type of the item (`message`, `function_call`, `function_call_output`).\n"]
pub enum RealtimeConversationItemType {
    Message,
    FunctionCall,
    FunctionCallOutput,
}
#[doc = "The item to add to the conversation."]
pub struct RealtimeConversationItemWithReference {
    #[doc = "The arguments of the function call (for `function_call` items).\n"]
    pub arguments: String,
    #[doc = "The ID of the function call (for `function_call` and \n`function_call_output` items). If passed on a `function_call_output` \nitem, the server will check that a `function_call` item with the same \nID exists in the conversation history.\n"]
    pub call_id: String,
    #[doc = "The content of the message, applicable for `message` items. \n- Message items of role `system` support only `input_text` content\n- Message items of role `user` support `input_text` and `input_audio` \n  content\n- Message items of role `assistant` support `text` content.\n"]
    pub content: Vec<RealtimeConversationItemWithReferenceContentInner>,
    #[doc = "For an item of type (`message` | `function_call` | `function_call_output`)\nthis field allows the client to assign the unique ID of the item. It is\nnot required because the server will generate one if not provided.\n\nFor an item of type `item_reference`, this field is required and is a\nreference to any item that has previously existed in the conversation.\n"]
    pub id: String,
    #[doc = "The name of the function being called (for `function_call` items).\n"]
    pub name: String,
    #[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
    pub object: RealtimeConversationItemWithReferenceObject,
    #[doc = "The output of the function call (for `function_call_output` items).\n"]
    pub output: String,
    #[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
    pub role: RealtimeConversationItemWithReferenceRole,
    #[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
    pub status: RealtimeConversationItemWithReferenceStatus,
    #[doc = "The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).\n"]
    pub type_: RealtimeConversationItemWithReferenceType,
}
#[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
pub enum RealtimeConversationItemWithReferenceContentInnerType {
    InputAudio,
    InputText,
    ItemReference,
    Text,
}
pub struct RealtimeConversationItemWithReferenceContentInner {
    #[doc = "Base64-encoded audio bytes, used for `input_audio` content type.\n"]
    pub audio: String,
    #[doc = "ID of a previous conversation item to reference (for `item_reference`\ncontent types in `response.create` events). These can reference both\nclient and server created items.\n"]
    pub id: String,
    #[doc = "The text content, used for `input_text` and `text` content types.\n"]
    pub text: String,
    #[doc = "The transcript of the audio, used for `input_audio` content type.\n"]
    pub transcript: String,
    #[doc = "The content type (`input_text`, `input_audio`, `item_reference`, `text`).\n"]
    pub type_: RealtimeConversationItemWithReferenceContentInnerType,
}
#[doc = "Identifier for the API object being returned - always `realtime.item`.\n"]
pub enum RealtimeConversationItemWithReferenceObject {
    RealtimeItem,
}
#[doc = "The role of the message sender (`user`, `assistant`, `system`), only \napplicable for `message` items.\n"]
pub enum RealtimeConversationItemWithReferenceRole {
    User,
    Assistant,
    System,
}
#[doc = "The status of the item (`completed`, `incomplete`). These have no effect \non the conversation, but are accepted for consistency with the \n`conversation.item.created` event.\n"]
pub enum RealtimeConversationItemWithReferenceStatus {
    Completed,
    Incomplete,
}
#[doc = "The type of the item (`message`, `function_call`, `function_call_output`, `item_reference`).\n"]
pub enum RealtimeConversationItemWithReferenceType {
    Message,
    FunctionCall,
    FunctionCallOutput,
}
#[doc = "The response resource."]
pub struct RealtimeResponse {
    #[doc = "Which conversation the response is added to, determined by the `conversation`\nfield in the `response.create` event. If `auto`, the response will be added to\nthe default conversation and the value of `conversation_id` will be an id like\n`conv_1234`. If `none`, the response will not be added to any conversation and\nthe value of `conversation_id` will be `null`. If responses are being triggered\nby server VAD, the response will be added to the default conversation, thus\nthe `conversation_id` will be an id like `conv_1234`.\n"]
    pub conversation_id: String,
    #[doc = "The unique ID of the response."]
    pub id: String,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
    pub max_output_tokens: RealtimeResponseMaxOutputTokens,
    pub metadata: Metadata,
    #[doc = "The set of modalities the model used to respond. If there are multiple modalities,\nthe model will pick one, for example if `modalities` is `[\"text\", \"audio\"]`, the model\ncould be responding in either text or audio.\n"]
    pub modalities: Vec<RealtimeResponseModalitiesInner>,
    #[doc = "The object type, must be `realtime.response`."]
    pub object: RealtimeResponseObject,
    #[doc = "The list of output items generated by the response."]
    pub output: Vec<RealtimeConversationItem>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    pub output_audio_format: RealtimeResponseOutputAudioFormat,
    #[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`).\n"]
    pub status: RealtimeResponseStatus,
    #[doc = "Additional details about the status."]
    pub status_details: RealtimeResponseStatusDetails,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    pub temperature: f64,
    #[doc = "Usage statistics for the Response, this will correspond to billing. A \nRealtime API session will maintain a conversation context and append new \nItems to the Conversation, thus output from previous turns (text and \naudio tokens) will become the input for later turns.\n"]
    pub usage: RealtimeResponseUsage,
    #[doc = "The voice the model used to respond.\nCurrent voice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    pub voice: VoiceIdsShared,
}
pub enum RealtimeResponseMaxOutputTokens1 {
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls, that was used in this response.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseMaxOutputTokens {
    OneOf0(u64),
    OneOf1(RealtimeResponseMaxOutputTokens1),
}
pub enum RealtimeResponseModalitiesInner {
    Text,
    Audio,
}
#[doc = "The object type, must be `realtime.response`."]
pub enum RealtimeResponseObject {
    RealtimeResponse,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
pub enum RealtimeResponseOutputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "The final status of the response (`completed`, `cancelled`, `failed`, or \n`incomplete`).\n"]
pub enum RealtimeResponseStatus {
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}
#[doc = "A description of the error that caused the response to fail, \npopulated when the `status` is `failed`.\n"]
pub struct RealtimeResponseStatusDetailsError {
    #[doc = "Error code, if any."]
    pub code: String,
    #[doc = "The type of error."]
    pub type_: String,
}
#[doc = "The reason the Response did not complete. For a `cancelled` Response, \none of `turn_detected` (the server VAD detected a new start of speech) \nor `client_cancelled` (the client sent a cancel event). For an \n`incomplete` Response, one of `max_output_tokens` or `content_filter` \n(the server-side safety filter activated and cut off the response).\n"]
pub enum RealtimeResponseStatusDetailsReason {
    TurnDetected,
    ClientCancelled,
    MaxOutputTokens,
    ContentFilter,
}
#[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
pub enum RealtimeResponseStatusDetailsType {
    Completed,
    Cancelled,
    Failed,
    Incomplete,
}
#[doc = "Additional details about the status."]
pub struct RealtimeResponseStatusDetails {
    #[doc = "A description of the error that caused the response to fail, \npopulated when the `status` is `failed`.\n"]
    pub error: RealtimeResponseStatusDetailsError,
    #[doc = "The reason the Response did not complete. For a `cancelled` Response, \none of `turn_detected` (the server VAD detected a new start of speech) \nor `client_cancelled` (the client sent a cancel event). For an \n`incomplete` Response, one of `max_output_tokens` or `content_filter` \n(the server-side safety filter activated and cut off the response).\n"]
    pub reason: RealtimeResponseStatusDetailsReason,
    #[doc = "The type of error that caused the response to fail, corresponding \nwith the `status` field (`completed`, `cancelled`, `incomplete`, \n`failed`).\n"]
    pub type_: RealtimeResponseStatusDetailsType,
}
#[doc = "Details about the input tokens used in the Response."]
pub struct RealtimeResponseUsageInputTokenDetails {
    #[doc = "The number of audio tokens used in the Response."]
    pub audio_tokens: u64,
    #[doc = "The number of cached tokens used in the Response."]
    pub cached_tokens: u64,
    #[doc = "The number of text tokens used in the Response."]
    pub text_tokens: u64,
}
#[doc = "Details about the output tokens used in the Response."]
pub struct RealtimeResponseUsageOutputTokenDetails {
    #[doc = "The number of audio tokens used in the Response."]
    pub audio_tokens: u64,
    #[doc = "The number of text tokens used in the Response."]
    pub text_tokens: u64,
}
#[doc = "Usage statistics for the Response, this will correspond to billing. A \nRealtime API session will maintain a conversation context and append new \nItems to the Conversation, thus output from previous turns (text and \naudio tokens) will become the input for later turns.\n"]
pub struct RealtimeResponseUsage {
    #[doc = "Details about the input tokens used in the Response."]
    pub input_token_details: RealtimeResponseUsageInputTokenDetails,
    #[doc = "The number of input tokens used in the Response, including text and \naudio tokens.\n"]
    pub input_tokens: u64,
    #[doc = "Details about the output tokens used in the Response."]
    pub output_token_details: RealtimeResponseUsageOutputTokenDetails,
    #[doc = "The number of output tokens sent in the Response, including text and \naudio tokens.\n"]
    pub output_tokens: u64,
    #[doc = "The total number of tokens in the Response including input and output \ntext and audio tokens.\n"]
    pub total_tokens: u64,
}
#[doc = "Create a new Realtime response with these parameters"]
pub struct RealtimeResponseCreateParams {
    #[doc = "Controls which conversation the response is added to. Currently supports\n`auto` and `none`, with `auto` as the default value. The `auto` value\nmeans that the contents of the response will be added to the default\nconversation. Set this to `none` to create an out-of-band response which \nwill not add items to default conversation.\n"]
    pub conversation: RealtimeResponseCreateParamsConversation,
    #[doc = "Input items to include in the prompt for the model. Using this field\ncreates a new context for this Response instead of using the default\nconversation. An empty array `[]` will clear the context for this Response.\nNote that this can include references to items from the default conversation.\n"]
    pub input: Vec<RealtimeConversationItemWithReference>,
    #[doc = "The default system instructions (i.e. system message) prepended to model \ncalls. This field allows the client to guide the model on desired \nresponses. The model can be instructed on response content and format, \n(e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good \nresponses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion \ninto your voice\", \"laugh frequently\"). The instructions are not guaranteed \nto be followed by the model, but they provide guidance to the model on the \ndesired behavior.\n\nNote that the server sets default instructions which will be used if this \nfield is not set and are visible in the `session.created` event at the \nstart of the session.\n"]
    pub instructions: String,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    pub max_response_output_tokens: RealtimeResponseCreateParamsMaxResponseOutputTokens,
    pub metadata: Metadata,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeResponseCreateParamsModalitiesInner>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    pub output_audio_format: RealtimeResponseCreateParamsOutputAudioFormat,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    pub temperature: f64,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function, like `{\"type\": \"function\", \"function\": {\"name\": \"my_function\"}}`.\n"]
    pub tool_choice: String,
    #[doc = "Tools (functions) available to the model."]
    pub tools: Vec<RealtimeResponseCreateParamsToolsInner>,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    pub voice: VoiceIdsShared,
}
pub enum RealtimeResponseCreateParamsConversation1 {
    Auto,
    None,
}
#[doc = "Controls which conversation the response is added to. Currently supports\n`auto` and `none`, with `auto` as the default value. The `auto` value\nmeans that the contents of the response will be added to the default\nconversation. Set this to `none` to create an out-of-band response which \nwill not add items to default conversation.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseCreateParamsConversation {
    OneOf0(String),
    OneOf1(RealtimeResponseCreateParamsConversation1),
}
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens1 {
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeResponseCreateParamsMaxResponseOutputTokens {
    OneOf0(u64),
    OneOf1(RealtimeResponseCreateParamsMaxResponseOutputTokens1),
}
pub enum RealtimeResponseCreateParamsModalitiesInner {
    Text,
    Audio,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
pub enum RealtimeResponseCreateParamsOutputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "The type of the tool, i.e. `function`."]
pub enum RealtimeResponseCreateParamsToolsInnerType {
    Function,
}
pub struct RealtimeResponseCreateParamsToolsInner {
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    pub description: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "Parameters of the function in JSON Schema."]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    pub type_: RealtimeResponseCreateParamsToolsInnerType,
}
#[doc = "A realtime server event.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeServerEvent {
    AnyOf0(RealtimeServerEventConversationCreated),
    AnyOf1(RealtimeServerEventConversationItemCreated),
    AnyOf2(RealtimeServerEventConversationItemDeleted),
    AnyOf3(RealtimeServerEventConversationItemInputAudioTranscriptionCompleted),
    AnyOf4(RealtimeServerEventConversationItemInputAudioTranscriptionDelta),
    AnyOf5(RealtimeServerEventConversationItemInputAudioTranscriptionFailed),
    AnyOf6(RealtimeServerEventConversationItemRetrieved),
    AnyOf7(RealtimeServerEventConversationItemTruncated),
    AnyOf8(RealtimeServerEventError),
    AnyOf9(RealtimeServerEventInputAudioBufferCleared),
    AnyOf10(RealtimeServerEventInputAudioBufferCommitted),
    AnyOf11(RealtimeServerEventInputAudioBufferSpeechStarted),
    AnyOf12(RealtimeServerEventInputAudioBufferSpeechStopped),
    AnyOf13(RealtimeServerEventRateLimitsUpdated),
    AnyOf14(RealtimeServerEventResponseAudioDelta),
    AnyOf15(RealtimeServerEventResponseAudioDone),
    AnyOf16(RealtimeServerEventResponseAudioTranscriptDelta),
    AnyOf17(RealtimeServerEventResponseAudioTranscriptDone),
    AnyOf18(RealtimeServerEventResponseContentPartAdded),
    AnyOf19(RealtimeServerEventResponseContentPartDone),
    AnyOf20(RealtimeServerEventResponseCreated),
    AnyOf21(RealtimeServerEventResponseDone),
    AnyOf22(RealtimeServerEventResponseFunctionCallArgumentsDelta),
    AnyOf23(RealtimeServerEventResponseFunctionCallArgumentsDone),
    AnyOf24(RealtimeServerEventResponseOutputItemAdded),
    AnyOf25(RealtimeServerEventResponseOutputItemDone),
    AnyOf26(RealtimeServerEventResponseTextDelta),
    AnyOf27(RealtimeServerEventResponseTextDone),
    AnyOf28(RealtimeServerEventSessionCreated),
    AnyOf29(RealtimeServerEventSessionUpdated),
    AnyOf30(RealtimeServerEventTranscriptionSessionUpdated),
    AnyOf31(RealtimeServerEventOutputAudioBufferStarted),
    AnyOf32(RealtimeServerEventOutputAudioBufferStopped),
    AnyOf33(RealtimeServerEventOutputAudioBufferCleared),
}
#[doc = "Returned when a conversation is created. Emitted right after session creation.\n"]
pub struct RealtimeServerEventConversationCreated {
    #[doc = "The conversation resource."]
    pub conversation: RealtimeServerEventConversationCreatedConversation,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The event type, must be `conversation.created`."]
    pub type_: RealtimeServerEventConversationCreatedType,
}
#[doc = "The conversation resource."]
pub struct RealtimeServerEventConversationCreatedConversation {
    #[doc = "The unique ID of the conversation."]
    pub id: String,
    #[doc = "The object type, must be `realtime.conversation`."]
    pub object: String,
}
#[doc = "The event type, must be `conversation.created`."]
pub enum RealtimeServerEventConversationCreatedType {
    ConversationCreated,
}
#[doc = "Returned when a conversation item is created. There are several scenarios that produce this event:\n  - The server is generating a Response, which if successful will produce \n    either one or two Items, which will be of type `message` \n    (role `assistant`) or type `function_call`.\n  - The input audio buffer has been committed, either by the client or the \n    server (in `server_vad` mode). The server will take the content of the \n    input audio buffer and add it to a new user message Item.\n  - The client has sent a `conversation.item.create` event to add a new Item \n    to the Conversation.\n"]
pub struct RealtimeServerEventConversationItemCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[doc = "The ID of the preceding item in the Conversation context, allows the \nclient to understand the order of the conversation.\n"]
    pub previous_item_id: String,
    #[doc = "The event type, must be `conversation.item.created`."]
    pub type_: RealtimeServerEventConversationItemCreatedType,
}
#[doc = "The event type, must be `conversation.item.created`."]
pub enum RealtimeServerEventConversationItemCreatedType {
    ConversationItemCreated,
}
#[doc = "Returned when an item in the conversation is deleted by the client with a \n`conversation.item.delete` event. This event is used to synchronize the \nserver's understanding of the conversation history with the client's view.\n"]
pub struct RealtimeServerEventConversationItemDeleted {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item that was deleted."]
    pub item_id: String,
    #[doc = "The event type, must be `conversation.item.deleted`."]
    pub type_: RealtimeServerEventConversationItemDeletedType,
}
#[doc = "The event type, must be `conversation.item.deleted`."]
pub enum RealtimeServerEventConversationItemDeletedType {
    ConversationItemDeleted,
}
#[doc = "This event is the output of audio transcription for user audio written to the \nuser audio buffer. Transcription begins when the input audio buffer is \ncommitted by the client or server (in `server_vad` mode). Transcription runs \nasynchronously with Response creation, so this event may come before or after \nthe Response events.\n\nRealtime API models accept audio natively, and thus input transcription is a \nseparate process run on a separate ASR (Automatic Speech Recognition) model, \ncurrently always `whisper-1`. Thus the transcript may diverge somewhat from \nthe model's interpretation, and should be treated as a rough guide.\n"]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionCompleted {
    #[doc = "The index of the content part containing the audio."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the user message item containing the audio."]
    pub item_id: String,
    #[doc = "The log probabilities of the transcription."]
    pub logprobs: Vec<LogProbProperties>,
    #[doc = "The transcribed text."]
    pub transcript: String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.completed`.\n"]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionCompletedType {
    ConversationItemInputAudioTranscriptionCompleted,
}
#[doc = "Returned when the text value of an input audio transcription content part is updated.\n"]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The text delta."]
    pub delta: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The log probabilities of the transcription."]
    pub logprobs: Vec<LogProbProperties>,
    #[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType,
}
#[doc = "The event type, must be `conversation.item.input_audio_transcription.delta`."]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionDeltaType {
    ConversationItemInputAudioTranscriptionDelta,
}
#[doc = "Returned when input audio transcription is configured, and a transcription \nrequest for a user message failed. These events are separate from other \n`error` events so that the client can identify the related Item.\n"]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailed {
    #[doc = "The index of the content part containing the audio."]
    pub content_index: u64,
    #[doc = "Details of the transcription error."]
    pub error: RealtimeServerEventConversationItemInputAudioTranscriptionFailedError,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the user message item."]
    pub item_id: String,
    #[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
    pub type_: RealtimeServerEventConversationItemInputAudioTranscriptionFailedType,
}
#[doc = "Details of the transcription error."]
pub struct RealtimeServerEventConversationItemInputAudioTranscriptionFailedError {
    #[doc = "Error code, if any."]
    pub code: String,
    #[doc = "A human-readable error message."]
    pub message: String,
    #[doc = "Parameter related to the error, if any."]
    pub param: String,
    #[doc = "The type of error."]
    pub type_: String,
}
#[doc = "The event type, must be\n`conversation.item.input_audio_transcription.failed`.\n"]
pub enum RealtimeServerEventConversationItemInputAudioTranscriptionFailedType {
    ConversationItemInputAudioTranscriptionFailed,
}
#[doc = "Returned when a conversation item is retrieved with `conversation.item.retrieve`.\n"]
pub struct RealtimeServerEventConversationItemRetrieved {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[doc = "The event type, must be `conversation.item.retrieved`."]
    pub type_: RealtimeServerEventConversationItemRetrievedType,
}
#[doc = "The event type, must be `conversation.item.retrieved`."]
pub enum RealtimeServerEventConversationItemRetrievedType {
    ConversationItemRetrieved,
}
#[doc = "Returned when an earlier assistant audio message item is truncated by the \nclient with a `conversation.item.truncate` event. This event is used to \nsynchronize the server's understanding of the audio with the client's playback.\n\nThis action will truncate the audio and remove the server-side text transcript \nto ensure there is no text in the context that hasn't been heard by the user.\n"]
pub struct RealtimeServerEventConversationItemTruncated {
    #[doc = "The duration up to which the audio was truncated, in milliseconds.\n"]
    pub audio_end_ms: u64,
    #[doc = "The index of the content part that was truncated."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the assistant message item that was truncated."]
    pub item_id: String,
    #[doc = "The event type, must be `conversation.item.truncated`."]
    pub type_: RealtimeServerEventConversationItemTruncatedType,
}
#[doc = "The event type, must be `conversation.item.truncated`."]
pub enum RealtimeServerEventConversationItemTruncatedType {
    ConversationItemTruncated,
}
#[doc = "Returned when an error occurs, which could be a client problem or a server \nproblem. Most errors are recoverable and the session will stay open, we \nrecommend to implementors to monitor and log error messages by default.\n"]
pub struct RealtimeServerEventError {
    #[doc = "Details of the error."]
    pub error: RealtimeServerEventErrorError,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The event type, must be `error`."]
    pub type_: RealtimeServerEventErrorType,
}
#[doc = "Details of the error."]
pub struct RealtimeServerEventErrorError {
    #[doc = "Error code, if any."]
    pub code: String,
    #[doc = "The event_id of the client event that caused the error, if applicable.\n"]
    pub event_id: String,
    #[doc = "A human-readable error message."]
    pub message: String,
    #[doc = "Parameter related to the error, if any."]
    pub param: String,
    #[doc = "The type of error (e.g., \"invalid_request_error\", \"server_error\").\n"]
    pub type_: String,
}
#[doc = "The event type, must be `error`."]
pub enum RealtimeServerEventErrorType {
    Error,
}
#[doc = "Returned when the input audio buffer is cleared by the client with a \n`input_audio_buffer.clear` event.\n"]
pub struct RealtimeServerEventInputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The event type, must be `input_audio_buffer.cleared`."]
    pub type_: RealtimeServerEventInputAudioBufferClearedType,
}
#[doc = "The event type, must be `input_audio_buffer.cleared`."]
pub enum RealtimeServerEventInputAudioBufferClearedType {
    InputAudioBufferCleared,
}
#[doc = "Returned when an input audio buffer is committed, either by the client or \nautomatically in server VAD mode. The `item_id` property is the ID of the user\nmessage item that will be created, thus a `conversation.item.created` event \nwill also be sent to the client.\n"]
pub struct RealtimeServerEventInputAudioBufferCommitted {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the user message item that will be created."]
    pub item_id: String,
    #[doc = "The ID of the preceding item after which the new item will be inserted.\n"]
    pub previous_item_id: String,
    #[doc = "The event type, must be `input_audio_buffer.committed`."]
    pub type_: RealtimeServerEventInputAudioBufferCommittedType,
}
#[doc = "The event type, must be `input_audio_buffer.committed`."]
pub enum RealtimeServerEventInputAudioBufferCommittedType {
    InputAudioBufferCommitted,
}
#[doc = "Sent by the server when in `server_vad` mode to indicate that speech has been \ndetected in the audio buffer. This can happen any time audio is added to the \nbuffer (unless speech is already detected). The client may want to use this \nevent to interrupt audio playback or provide visual feedback to the user. \n\nThe client should expect to receive a `input_audio_buffer.speech_stopped` event \nwhen speech stops. The `item_id` property is the ID of the user message item \nthat will be created when speech stops and will also be included in the \n`input_audio_buffer.speech_stopped` event (unless the client manually commits \nthe audio buffer during VAD activation).\n"]
pub struct RealtimeServerEventInputAudioBufferSpeechStarted {
    #[doc = "Milliseconds from the start of all audio written to the buffer during the \nsession when speech was first detected. This will correspond to the \nbeginning of audio sent to the model, and thus includes the \n`prefix_padding_ms` configured in the Session.\n"]
    pub audio_start_ms: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the user message item that will be created when speech stops.\n"]
    pub item_id: String,
    #[doc = "The event type, must be `input_audio_buffer.speech_started`."]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStartedType,
}
#[doc = "The event type, must be `input_audio_buffer.speech_started`."]
pub enum RealtimeServerEventInputAudioBufferSpeechStartedType {
    InputAudioBufferSpeechStarted,
}
#[doc = "Returned in `server_vad` mode when the server detects the end of speech in \nthe audio buffer. The server will also send an `conversation.item.created` \nevent with the user message item that is created from the audio buffer.\n"]
pub struct RealtimeServerEventInputAudioBufferSpeechStopped {
    #[doc = "Milliseconds since the session started when speech stopped. This will \ncorrespond to the end of audio sent to the model, and thus includes the \n`min_silence_duration_ms` configured in the Session.\n"]
    pub audio_end_ms: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the user message item that will be created."]
    pub item_id: String,
    #[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
    pub type_: RealtimeServerEventInputAudioBufferSpeechStoppedType,
}
#[doc = "The event type, must be `input_audio_buffer.speech_stopped`."]
pub enum RealtimeServerEventInputAudioBufferSpeechStoppedType {
    InputAudioBufferSpeechStopped,
}
#[doc = "**WebRTC Only:** Emitted when the output audio buffer is cleared. This happens either in VAD\nmode when the user has interrupted (`input_audio_buffer.speech_started`),\nor when the client has emitted the `output_audio_buffer.clear` event to manually\ncut off the current audio response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
pub struct RealtimeServerEventOutputAudioBufferCleared {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: String,
    #[doc = "The event type, must be `output_audio_buffer.cleared`."]
    pub type_: RealtimeServerEventOutputAudioBufferClearedType,
}
#[doc = "The event type, must be `output_audio_buffer.cleared`."]
pub enum RealtimeServerEventOutputAudioBufferClearedType {
    OutputAudioBufferCleared,
}
#[doc = "**WebRTC Only:** Emitted when the server begins streaming audio to the client. This event is\nemitted after an audio content part has been added (`response.content_part.added`)\nto the response.\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
pub struct RealtimeServerEventOutputAudioBufferStarted {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: String,
    #[doc = "The event type, must be `output_audio_buffer.started`."]
    pub type_: RealtimeServerEventOutputAudioBufferStartedType,
}
#[doc = "The event type, must be `output_audio_buffer.started`."]
pub enum RealtimeServerEventOutputAudioBufferStartedType {
    OutputAudioBufferStarted,
}
#[doc = "**WebRTC Only:** Emitted when the output audio buffer has been completely drained on the server,\nand no more audio is forthcoming. This event is emitted after the full response\ndata has been sent to the client (`response.done`).\n[Learn more](/docs/guides/realtime-model-capabilities#client-and-server-events-for-audio-in-webrtc).\n"]
pub struct RealtimeServerEventOutputAudioBufferStopped {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The unique ID of the response that produced the audio."]
    pub response_id: String,
    #[doc = "The event type, must be `output_audio_buffer.stopped`."]
    pub type_: RealtimeServerEventOutputAudioBufferStoppedType,
}
#[doc = "The event type, must be `output_audio_buffer.stopped`."]
pub enum RealtimeServerEventOutputAudioBufferStoppedType {
    OutputAudioBufferStopped,
}
#[doc = "Emitted at the beginning of a Response to indicate the updated rate limits. \nWhen a Response is created some tokens will be \"reserved\" for the output \ntokens, the rate limits shown here reflect that reservation, which is then \nadjusted accordingly once the Response is completed.\n"]
pub struct RealtimeServerEventRateLimitsUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "List of rate limit information."]
    pub rate_limits: Vec<RealtimeServerEventRateLimitsUpdatedRateLimitsInner>,
    #[doc = "The event type, must be `rate_limits.updated`."]
    pub type_: RealtimeServerEventRateLimitsUpdatedType,
}
#[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
pub enum RealtimeServerEventRateLimitsUpdatedRateLimitsInnerName {
    Requests,
    Tokens,
}
pub struct RealtimeServerEventRateLimitsUpdatedRateLimitsInner {
    #[doc = "The maximum allowed value for the rate limit."]
    pub limit: u64,
    #[doc = "The name of the rate limit (`requests`, `tokens`).\n"]
    pub name: RealtimeServerEventRateLimitsUpdatedRateLimitsInnerName,
    #[doc = "The remaining value before the limit is reached."]
    pub remaining: u64,
    #[doc = "Seconds until the rate limit resets."]
    pub reset_seconds: f64,
}
#[doc = "The event type, must be `rate_limits.updated`."]
pub enum RealtimeServerEventRateLimitsUpdatedType {
    RateLimitsUpdated,
}
#[doc = "Returned when the model-generated audio is updated."]
pub struct RealtimeServerEventResponseAudioDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "Base64-encoded audio data delta."]
    pub delta: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.audio.delta`."]
    pub type_: RealtimeServerEventResponseAudioDeltaType,
}
#[doc = "The event type, must be `response.audio.delta`."]
pub enum RealtimeServerEventResponseAudioDeltaType {
    ResponseAudioDelta,
}
#[doc = "Returned when the model-generated audio is done. Also emitted when a Response\nis interrupted, incomplete, or cancelled.\n"]
pub struct RealtimeServerEventResponseAudioDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.audio.done`."]
    pub type_: RealtimeServerEventResponseAudioDoneType,
}
#[doc = "The event type, must be `response.audio.done`."]
pub enum RealtimeServerEventResponseAudioDoneType {
    ResponseAudioDone,
}
#[doc = "Returned when the model-generated transcription of audio output is updated.\n"]
pub struct RealtimeServerEventResponseAudioTranscriptDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The transcript delta."]
    pub delta: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.audio_transcript.delta`."]
    pub type_: RealtimeServerEventResponseAudioTranscriptDeltaType,
}
#[doc = "The event type, must be `response.audio_transcript.delta`."]
pub enum RealtimeServerEventResponseAudioTranscriptDeltaType {
    ResponseAudioTranscriptDelta,
}
#[doc = "Returned when the model-generated transcription of audio output is done\nstreaming. Also emitted when a Response is interrupted, incomplete, or\ncancelled.\n"]
pub struct RealtimeServerEventResponseAudioTranscriptDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The final transcript of the audio."]
    pub transcript: String,
    #[doc = "The event type, must be `response.audio_transcript.done`."]
    pub type_: RealtimeServerEventResponseAudioTranscriptDoneType,
}
#[doc = "The event type, must be `response.audio_transcript.done`."]
pub enum RealtimeServerEventResponseAudioTranscriptDoneType {
    ResponseAudioTranscriptDone,
}
#[doc = "Returned when a new content part is added to an assistant message item during\nresponse generation.\n"]
pub struct RealtimeServerEventResponseContentPartAdded {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item to which the content part was added."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The content part that was added."]
    pub part: RealtimeServerEventResponseContentPartAddedPart,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.content_part.added`."]
    pub type_: RealtimeServerEventResponseContentPartAddedType,
}
#[doc = "The content type (\"text\", \"audio\")."]
pub enum RealtimeServerEventResponseContentPartAddedPartType {
    Audio,
    Text,
}
#[doc = "The content part that was added."]
pub struct RealtimeServerEventResponseContentPartAddedPart {
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    pub audio: String,
    #[doc = "The text content (if type is \"text\")."]
    pub text: String,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    pub transcript: String,
    #[doc = "The content type (\"text\", \"audio\")."]
    pub type_: RealtimeServerEventResponseContentPartAddedPartType,
}
#[doc = "The event type, must be `response.content_part.added`."]
pub enum RealtimeServerEventResponseContentPartAddedType {
    ResponseContentPartAdded,
}
#[doc = "Returned when a content part is done streaming in an assistant message item.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
pub struct RealtimeServerEventResponseContentPartDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The content part that is done."]
    pub part: RealtimeServerEventResponseContentPartDonePart,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.content_part.done`."]
    pub type_: RealtimeServerEventResponseContentPartDoneType,
}
#[doc = "The content type (\"text\", \"audio\")."]
pub enum RealtimeServerEventResponseContentPartDonePartType {
    Audio,
    Text,
}
#[doc = "The content part that is done."]
pub struct RealtimeServerEventResponseContentPartDonePart {
    #[doc = "Base64-encoded audio data (if type is \"audio\")."]
    pub audio: String,
    #[doc = "The text content (if type is \"text\")."]
    pub text: String,
    #[doc = "The transcript of the audio (if type is \"audio\")."]
    pub transcript: String,
    #[doc = "The content type (\"text\", \"audio\")."]
    pub type_: RealtimeServerEventResponseContentPartDonePartType,
}
#[doc = "The event type, must be `response.content_part.done`."]
pub enum RealtimeServerEventResponseContentPartDoneType {
    ResponseContentPartDone,
}
#[doc = "Returned when a new Response is created. The first event of response creation,\nwhere the response is in an initial state of `in_progress`.\n"]
pub struct RealtimeServerEventResponseCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub response: RealtimeResponse,
    #[doc = "The event type, must be `response.created`."]
    pub type_: RealtimeServerEventResponseCreatedType,
}
#[doc = "The event type, must be `response.created`."]
pub enum RealtimeServerEventResponseCreatedType {
    ResponseCreated,
}
#[doc = "Returned when a Response is done streaming. Always emitted, no matter the \nfinal state. The Response object included in the `response.done` event will \ninclude all output Items in the Response but will omit the raw audio data.\n"]
pub struct RealtimeServerEventResponseDone {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub response: RealtimeResponse,
    #[doc = "The event type, must be `response.done`."]
    pub type_: RealtimeServerEventResponseDoneType,
}
#[doc = "The event type, must be `response.done`."]
pub enum RealtimeServerEventResponseDoneType {
    ResponseDone,
}
#[doc = "Returned when the model-generated function call arguments are updated.\n"]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDelta {
    #[doc = "The ID of the function call."]
    pub call_id: String,
    #[doc = "The arguments delta as a JSON string."]
    pub delta: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the function call item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDeltaType,
}
#[doc = "The event type, must be `response.function_call_arguments.delta`.\n"]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDeltaType {
    ResponseFunctionCallArgumentsDelta,
}
#[doc = "Returned when the model-generated function call arguments are done streaming.\nAlso emitted when a Response is interrupted, incomplete, or cancelled.\n"]
pub struct RealtimeServerEventResponseFunctionCallArgumentsDone {
    #[doc = "The final arguments as a JSON string."]
    pub arguments: String,
    #[doc = "The ID of the function call."]
    pub call_id: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the function call item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
    pub type_: RealtimeServerEventResponseFunctionCallArgumentsDoneType,
}
#[doc = "The event type, must be `response.function_call_arguments.done`.\n"]
pub enum RealtimeServerEventResponseFunctionCallArgumentsDoneType {
    ResponseFunctionCallArgumentsDone,
}
#[doc = "Returned when a new Item is created during Response generation."]
pub struct RealtimeServerEventResponseOutputItemAdded {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[doc = "The index of the output item in the Response."]
    pub output_index: u64,
    #[doc = "The ID of the Response to which the item belongs."]
    pub response_id: String,
    #[doc = "The event type, must be `response.output_item.added`."]
    pub type_: RealtimeServerEventResponseOutputItemAddedType,
}
#[doc = "The event type, must be `response.output_item.added`."]
pub enum RealtimeServerEventResponseOutputItemAddedType {
    ResponseOutputItemAdded,
}
#[doc = "Returned when an Item is done streaming. Also emitted when a Response is \ninterrupted, incomplete, or cancelled.\n"]
pub struct RealtimeServerEventResponseOutputItemDone {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub item: RealtimeConversationItem,
    #[doc = "The index of the output item in the Response."]
    pub output_index: u64,
    #[doc = "The ID of the Response to which the item belongs."]
    pub response_id: String,
    #[doc = "The event type, must be `response.output_item.done`."]
    pub type_: RealtimeServerEventResponseOutputItemDoneType,
}
#[doc = "The event type, must be `response.output_item.done`."]
pub enum RealtimeServerEventResponseOutputItemDoneType {
    ResponseOutputItemDone,
}
#[doc = "Returned when the text value of a \"text\" content part is updated."]
pub struct RealtimeServerEventResponseTextDelta {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The text delta."]
    pub delta: String,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The event type, must be `response.text.delta`."]
    pub type_: RealtimeServerEventResponseTextDeltaType,
}
#[doc = "The event type, must be `response.text.delta`."]
pub enum RealtimeServerEventResponseTextDeltaType {
    ResponseTextDelta,
}
#[doc = "Returned when the text value of a \"text\" content part is done streaming. Also\nemitted when a Response is interrupted, incomplete, or cancelled.\n"]
pub struct RealtimeServerEventResponseTextDone {
    #[doc = "The index of the content part in the item's content array."]
    pub content_index: u64,
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item in the response."]
    pub output_index: u64,
    #[doc = "The ID of the response."]
    pub response_id: String,
    #[doc = "The final text content."]
    pub text: String,
    #[doc = "The event type, must be `response.text.done`."]
    pub type_: RealtimeServerEventResponseTextDoneType,
}
#[doc = "The event type, must be `response.text.done`."]
pub enum RealtimeServerEventResponseTextDoneType {
    ResponseTextDone,
}
#[doc = "Returned when a Session is created. Emitted automatically when a new \nconnection is established as the first server event. This event will contain \nthe default Session configuration.\n"]
pub struct RealtimeServerEventSessionCreated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub session: RealtimeSession,
    #[doc = "The event type, must be `session.created`."]
    pub type_: RealtimeServerEventSessionCreatedType,
}
#[doc = "The event type, must be `session.created`."]
pub enum RealtimeServerEventSessionCreatedType {
    SessionCreated,
}
#[doc = "Returned when a session is updated with a `session.update` event, unless \nthere is an error.\n"]
pub struct RealtimeServerEventSessionUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub session: RealtimeSession,
    #[doc = "The event type, must be `session.updated`."]
    pub type_: RealtimeServerEventSessionUpdatedType,
}
#[doc = "The event type, must be `session.updated`."]
pub enum RealtimeServerEventSessionUpdatedType {
    SessionUpdated,
}
#[doc = "Returned when a transcription session is updated with a `transcription_session.update` event, unless \nthere is an error.\n"]
pub struct RealtimeServerEventTranscriptionSessionUpdated {
    #[doc = "The unique ID of the server event."]
    pub event_id: String,
    pub session: RealtimeTranscriptionSessionCreateResponse,
    #[doc = "The event type, must be `transcription_session.updated`."]
    pub type_: RealtimeServerEventTranscriptionSessionUpdatedType,
}
#[doc = "The event type, must be `transcription_session.updated`."]
pub enum RealtimeServerEventTranscriptionSessionUpdatedType {
    TranscriptionSessionUpdated,
}
#[doc = "Realtime session object configuration."]
pub struct RealtimeSession {
    #[doc = "Unique identifier for the session that looks like `sess_1234567890abcdef`.\n"]
    pub id: String,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    pub input_audio_format: RealtimeSessionInputAudioFormat,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    pub input_audio_noise_reduction: RealtimeSessionInputAudioNoiseReduction,
    #[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    pub input_audio_transcription: RealtimeSessionInputAudioTranscription,
    #[doc = "The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good  responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion  into your voice\", \"laugh frequently\"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.\n"]
    pub instructions: String,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    pub max_response_output_tokens: RealtimeSessionMaxResponseOutputTokens,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeSessionModalitiesInner>,
    #[doc = "The Realtime model used for this session.\n"]
    pub model: RealtimeSessionModel,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
    pub output_audio_format: RealtimeSessionOutputAudioFormat,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.\n"]
    pub temperature: f64,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    pub tool_choice: String,
    #[doc = "Tools (functions) available to the model."]
    pub tools: Vec<RealtimeSessionToolsInner>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    pub turn_detection: RealtimeSessionTurnDetection,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, \n`shimmer` and `verse`.\n"]
    pub voice: VoiceIdsShared,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
pub enum RealtimeSessionInputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
pub enum RealtimeSessionInputAudioNoiseReductionType {
    NearField,
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
pub struct RealtimeSessionInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    pub type_: RealtimeSessionInputAudioNoiseReductionType,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
pub struct RealtimeSessionInputAudioTranscription {
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    pub language: String,
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    pub model: String,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    pub prompt: String,
}
pub enum RealtimeSessionMaxResponseOutputTokens1 {
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionMaxResponseOutputTokens {
    OneOf0(u64),
    OneOf1(RealtimeSessionMaxResponseOutputTokens1),
}
pub enum RealtimeSessionModalitiesInner {
    Text,
    Audio,
}
#[doc = "The Realtime model used for this session.\n"]
pub enum RealtimeSessionModel {
    Gpt4oRealtimePreview,
    Gpt4oRealtimePreview20241001,
    Gpt4oRealtimePreview20241217,
    Gpt4oMiniRealtimePreview,
    Gpt4oMiniRealtimePreview20241217,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
pub enum RealtimeSessionOutputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "The type of the tool, i.e. `function`."]
pub enum RealtimeSessionToolsInnerType {
    Function,
}
pub struct RealtimeSessionToolsInner {
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    pub description: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "Parameters of the function in JSON Schema."]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    pub type_: RealtimeSessionToolsInnerType,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
pub enum RealtimeSessionTurnDetectionEagerness {
    Low,
    Medium,
    High,
    Auto,
}
#[doc = "Type of turn detection.\n"]
pub enum RealtimeSessionTurnDetectionType {
    ServerVad,
    SemanticVad,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
pub struct RealtimeSessionTurnDetection {
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs.\n"]
    pub create_response: Vec<bool>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    pub eagerness: RealtimeSessionTurnDetectionEagerness,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs.\n"]
    pub interrupt_response: Vec<bool>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    pub prefix_padding_ms: u64,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    pub silence_duration_ms: u64,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    pub threshold: f64,
    #[doc = "Type of turn detection.\n"]
    pub type_: RealtimeSessionTurnDetectionType,
}
#[doc = "Realtime session object configuration."]
pub struct RealtimeSessionCreateRequest {
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    pub input_audio_format: RealtimeSessionCreateRequestInputAudioFormat,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    pub input_audio_noise_reduction: RealtimeSessionCreateRequestInputAudioNoiseReduction,
    #[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    pub input_audio_transcription: RealtimeSessionCreateRequestInputAudioTranscription,
    #[doc = "The default system instructions (i.e. system message) prepended to model  calls. This field allows the client to guide the model on desired  responses. The model can be instructed on response content and format,  (e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good  responses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion  into your voice\", \"laugh frequently\"). The instructions are not guaranteed  to be followed by the model, but they provide guidance to the model on the desired behavior.\n\nNote that the server sets default instructions which will be used if this  field is not set and are visible in the `session.created` event at the  start of the session.\n"]
    pub instructions: String,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    pub max_response_output_tokens: RealtimeSessionCreateRequestMaxResponseOutputTokens,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeSessionCreateRequestModalitiesInner>,
    #[doc = "The Realtime model used for this session.\n"]
    pub model: RealtimeSessionCreateRequestModel,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
    pub output_audio_format: RealtimeSessionCreateRequestOutputAudioFormat,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. For audio models a temperature of 0.8 is highly recommended for best performance.\n"]
    pub temperature: f64,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    pub tool_choice: String,
    #[doc = "Tools (functions) available to the model."]
    pub tools: Vec<RealtimeSessionCreateRequestToolsInner>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    pub turn_detection: RealtimeSessionCreateRequestTurnDetection,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo`, `fable`,\n`onyx`, `nova`, `sage`, `shimmer`, and `verse`.\n"]
    pub voice: VoiceIdsShared,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
pub enum RealtimeSessionCreateRequestInputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
pub enum RealtimeSessionCreateRequestInputAudioNoiseReductionType {
    NearField,
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
pub struct RealtimeSessionCreateRequestInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    pub type_: RealtimeSessionCreateRequestInputAudioNoiseReductionType,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be  set to `null` to turn off once on. Input audio transcription is not native to the model, since the model consumes audio directly. Transcription runs  asynchronously through [the /audio/transcriptions endpoint](https://platform.openai.com/docs/api-reference/audio/createTranscription) and should be treated as guidance of input audio content rather than precisely what the model heard. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
pub struct RealtimeSessionCreateRequestInputAudioTranscription {
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    pub language: String,
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    pub model: String,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    pub prompt: String,
}
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens1 {
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionCreateRequestMaxResponseOutputTokens {
    OneOf0(u64),
    OneOf1(RealtimeSessionCreateRequestMaxResponseOutputTokens1),
}
pub enum RealtimeSessionCreateRequestModalitiesInner {
    Text,
    Audio,
}
#[doc = "The Realtime model used for this session.\n"]
pub enum RealtimeSessionCreateRequestModel {
    Gpt4oRealtimePreview,
    Gpt4oRealtimePreview20241001,
    Gpt4oRealtimePreview20241217,
    Gpt4oMiniRealtimePreview,
    Gpt4oMiniRealtimePreview20241217,
}
#[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, output audio is sampled at a rate of 24kHz.\n"]
pub enum RealtimeSessionCreateRequestOutputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "The type of the tool, i.e. `function`."]
pub enum RealtimeSessionCreateRequestToolsInnerType {
    Function,
}
pub struct RealtimeSessionCreateRequestToolsInner {
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    pub description: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "Parameters of the function in JSON Schema."]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    pub type_: RealtimeSessionCreateRequestToolsInnerType,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
pub enum RealtimeSessionCreateRequestTurnDetectionEagerness {
    Low,
    Medium,
    High,
    Auto,
}
#[doc = "Type of turn detection.\n"]
pub enum RealtimeSessionCreateRequestTurnDetectionType {
    ServerVad,
    SemanticVad,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
pub struct RealtimeSessionCreateRequestTurnDetection {
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs.\n"]
    pub create_response: Vec<bool>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    pub eagerness: RealtimeSessionCreateRequestTurnDetectionEagerness,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs.\n"]
    pub interrupt_response: Vec<bool>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    pub prefix_padding_ms: u64,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    pub silence_duration_ms: u64,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    pub threshold: f64,
    #[doc = "Type of turn detection.\n"]
    pub type_: RealtimeSessionCreateRequestTurnDetectionType,
}
#[doc = "A new Realtime session configuration, with an ephermeral key. Default TTL\nfor keys is one minute.\n"]
pub struct RealtimeSessionCreateResponse {
    #[doc = "Ephemeral key returned by the API."]
    pub client_secret: RealtimeSessionCreateResponseClientSecret,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    pub input_audio_format: String,
    #[doc = "Configuration for input audio transcription, defaults to off and can be \nset to `null` to turn off once on. Input audio transcription is not native \nto the model, since the model consumes audio directly. Transcription runs \nasynchronously through Whisper and should be treated as rough guidance \nrather than the representation understood by the model.\n"]
    pub input_audio_transcription: RealtimeSessionCreateResponseInputAudioTranscription,
    #[doc = "The default system instructions (i.e. system message) prepended to model \ncalls. This field allows the client to guide the model on desired \nresponses. The model can be instructed on response content and format, \n(e.g. \"be extremely succinct\", \"act friendly\", \"here are examples of good \nresponses\") and on audio behavior (e.g. \"talk quickly\", \"inject emotion \ninto your voice\", \"laugh frequently\"). The instructions are not guaranteed \nto be followed by the model, but they provide guidance to the model on the \ndesired behavior.\n\nNote that the server sets default instructions which will be used if this \nfield is not set and are visible in the `session.created` event at the \nstart of the session.\n"]
    pub instructions: String,
    #[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
    pub max_response_output_tokens: RealtimeSessionCreateResponseMaxResponseOutputTokens,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeSessionCreateResponseModalitiesInner>,
    #[doc = "The format of output audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    pub output_audio_format: String,
    #[doc = "Sampling temperature for the model, limited to [0.6, 1.2]. Defaults to 0.8.\n"]
    pub temperature: f64,
    #[doc = "How the model chooses tools. Options are `auto`, `none`, `required`, or \nspecify a function.\n"]
    pub tool_choice: String,
    #[doc = "Tools (functions) available to the model."]
    pub tools: Vec<RealtimeSessionCreateResponseToolsInner>,
    #[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
    pub turn_detection: RealtimeSessionCreateResponseTurnDetection,
    #[doc = "The voice the model uses to respond. Voice cannot be changed during the \nsession once the model has responded with audio at least once. Current \nvoice options are `alloy`, `ash`, `ballad`, `coral`, `echo` `sage`, \n`shimmer` and `verse`.\n"]
    pub voice: VoiceIdsShared,
}
#[doc = "Ephemeral key returned by the API."]
pub struct RealtimeSessionCreateResponseClientSecret {
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    pub expires_at: u64,
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    pub value: String,
}
#[doc = "Configuration for input audio transcription, defaults to off and can be \nset to `null` to turn off once on. Input audio transcription is not native \nto the model, since the model consumes audio directly. Transcription runs \nasynchronously through Whisper and should be treated as rough guidance \nrather than the representation understood by the model.\n"]
pub struct RealtimeSessionCreateResponseInputAudioTranscription {
    #[doc = "The model to use for transcription, `whisper-1` is the only currently \nsupported model.\n"]
    pub model: String,
}
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens1 {
    Inf,
}
#[doc = "Maximum number of output tokens for a single assistant response,\ninclusive of tool calls. Provide an integer between 1 and 4096 to\nlimit output tokens, or `inf` for the maximum available tokens for a\ngiven model. Defaults to `inf`.\n"]
#[allow(clippy::large_enum_variant)]
pub enum RealtimeSessionCreateResponseMaxResponseOutputTokens {
    OneOf0(u64),
    OneOf1(RealtimeSessionCreateResponseMaxResponseOutputTokens1),
}
pub enum RealtimeSessionCreateResponseModalitiesInner {
    Text,
    Audio,
}
#[doc = "The type of the tool, i.e. `function`."]
pub enum RealtimeSessionCreateResponseToolsInnerType {
    Function,
}
pub struct RealtimeSessionCreateResponseToolsInner {
    #[doc = "The description of the function, including guidance on when and how \nto call it, and guidance about what to tell the user when calling \n(if anything).\n"]
    pub description: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "Parameters of the function in JSON Schema."]
    pub parameters: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The type of the tool, i.e. `function`."]
    pub type_: RealtimeSessionCreateResponseToolsInnerType,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
pub struct RealtimeSessionCreateResponseTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    pub prefix_padding_ms: u64,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    pub silence_duration_ms: u64,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    pub threshold: f64,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    pub type_: String,
}
#[doc = "Realtime transcription session object configuration."]
pub struct RealtimeTranscriptionSessionCreateRequest {
    #[doc = "The set of items to include in the transcription. Current available items are:\n- `item.input_audio_transcription.logprobs`\n"]
    pub include: Vec<String>,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
    pub input_audio_format: RealtimeTranscriptionSessionCreateRequestInputAudioFormat,
    #[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
    pub input_audio_noise_reduction:
        RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction,
    #[doc = "Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
    pub input_audio_transcription: RealtimeTranscriptionSessionCreateRequestInputAudioTranscription,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeTranscriptionSessionCreateRequestModalitiesInner>,
    #[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
    pub turn_detection: RealtimeTranscriptionSessionCreateRequestTurnDetection,
}
#[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\nFor `pcm16`, input audio must be 16-bit PCM at a 24kHz sample rate, \nsingle channel (mono), and little-endian byte order.\n"]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioFormat {
    Pcm16,
    G711Ulaw,
    G711Alaw,
}
#[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType {
    NearField,
    FarField,
}
#[doc = "Configuration for input audio noise reduction. This can be set to `null` to turn off.\nNoise reduction filters audio added to the input audio buffer before it is sent to VAD and the model.\nFiltering the audio can improve VAD and turn detection accuracy (reducing false positives) and model performance by improving perception of the input audio.\n"]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReduction {
    #[doc = "Type of noise reduction. `near_field` is for close-talking microphones such as headphones, `far_field` is for far-field microphones such as laptop or conference room microphones.\n"]
    pub type_: RealtimeTranscriptionSessionCreateRequestInputAudioNoiseReductionType,
}
#[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
pub enum RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel {
    Gpt4oTranscribe,
    Gpt4oMiniTranscribe,
    Whisper1,
}
#[doc = "Configuration for input audio transcription. The client can optionally set the language and prompt for transcription, these offer additional guidance to the transcription service.\n"]
pub struct RealtimeTranscriptionSessionCreateRequestInputAudioTranscription {
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    pub language: String,
    #[doc = "The model to use for transcription, current options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.\n"]
    pub model: RealtimeTranscriptionSessionCreateRequestInputAudioTranscriptionModel,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment.\nFor `whisper-1`, the [prompt is a list of keywords](/docs/guides/speech-to-text#prompting).\nFor `gpt-4o-transcribe` models, the prompt is a free text string, for example \"expect words related to technology\".\n"]
    pub prompt: String,
}
pub enum RealtimeTranscriptionSessionCreateRequestModalitiesInner {
    Text,
    Audio,
}
#[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness {
    Low,
    Medium,
    High,
    Auto,
}
#[doc = "Type of turn detection.\n"]
pub enum RealtimeTranscriptionSessionCreateRequestTurnDetectionType {
    ServerVad,
    SemanticVad,
}
#[doc = "Configuration for turn detection, ether Server VAD or Semantic VAD. This can be set to `null` to turn off, in which case the client must manually trigger model response.\nServer VAD means that the model will detect the start and end of speech based on audio volume and respond at the end of user speech.\nSemantic VAD is more advanced and uses a turn detection model (in conjuction with VAD) to semantically estimate whether the user has finished speaking, then dynamically sets a timeout based on this probability. For example, if user audio trails off with \"uhhm\", the model will score a low probability of turn end and wait longer for the user to continue speaking. This can be useful for more natural conversations, but may have a higher latency.\n"]
pub struct RealtimeTranscriptionSessionCreateRequestTurnDetection {
    #[doc = "Whether or not to automatically generate a response when a VAD stop event occurs. Not available for transcription sessions.\n"]
    pub create_response: Vec<bool>,
    #[doc = "Used only for `semantic_vad` mode. The eagerness of the model to respond. `low` will wait longer for the user to continue speaking, `high` will respond more quickly. `auto` is the default and is equivalent to `medium`.\n"]
    pub eagerness: RealtimeTranscriptionSessionCreateRequestTurnDetectionEagerness,
    #[doc = "Whether or not to automatically interrupt any ongoing response with output to the default\nconversation (i.e. `conversation` of `auto`) when a VAD start event occurs. Not available for transcription sessions.\n"]
    pub interrupt_response: Vec<bool>,
    #[doc = "Used only for `server_vad` mode. Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    pub prefix_padding_ms: u64,
    #[doc = "Used only for `server_vad` mode. Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    pub silence_duration_ms: u64,
    #[doc = "Used only for `server_vad` mode. Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    pub threshold: f64,
    #[doc = "Type of turn detection.\n"]
    pub type_: RealtimeTranscriptionSessionCreateRequestTurnDetectionType,
}
#[doc = "A new Realtime transcription session configuration.\n\nWhen a session is created on the server via REST API, the session object\nalso contains an ephemeral key. Default TTL for keys is one minute. This \nproperty is not present when a session is updated via the WebSocket API.\n"]
pub struct RealtimeTranscriptionSessionCreateResponse {
    #[doc = "Ephemeral key returned by the API. Only present when the session is\ncreated on the server via REST API.\n"]
    pub client_secret: RealtimeTranscriptionSessionCreateResponseClientSecret,
    #[doc = "The format of input audio. Options are `pcm16`, `g711_ulaw`, or `g711_alaw`.\n"]
    pub input_audio_format: String,
    #[doc = "Configuration of the transcription model.\n"]
    pub input_audio_transcription:
        RealtimeTranscriptionSessionCreateResponseInputAudioTranscription,
    #[doc = "The set of modalities the model can respond with. To disable audio,\nset this to [\"text\"].\n"]
    pub modalities: Vec<RealtimeTranscriptionSessionCreateResponseModalitiesInner>,
    #[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
    pub turn_detection: RealtimeTranscriptionSessionCreateResponseTurnDetection,
}
#[doc = "Ephemeral key returned by the API. Only present when the session is\ncreated on the server via REST API.\n"]
pub struct RealtimeTranscriptionSessionCreateResponseClientSecret {
    #[doc = "Timestamp for when the token expires. Currently, all tokens expire\nafter one minute.\n"]
    pub expires_at: u64,
    #[doc = "Ephemeral key usable in client environments to authenticate connections\nto the Realtime API. Use this in client-side environments rather than\na standard API token, which should only be used server-side.\n"]
    pub value: String,
}
#[doc = "The model to use for transcription. Can be `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, or `whisper-1`.\n"]
pub enum RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel {
    Gpt4oTranscribe,
    Gpt4oMiniTranscribe,
    Whisper1,
}
#[doc = "Configuration of the transcription model.\n"]
pub struct RealtimeTranscriptionSessionCreateResponseInputAudioTranscription {
    #[doc = "The language of the input audio. Supplying the input language in\n[ISO-639-1](https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes) (e.g. `en`) format\nwill improve accuracy and latency.\n"]
    pub language: String,
    #[doc = "The model to use for transcription. Can be `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, or `whisper-1`.\n"]
    pub model: RealtimeTranscriptionSessionCreateResponseInputAudioTranscriptionModel,
    #[doc = "An optional text to guide the model's style or continue a previous audio\nsegment. The [prompt](/docs/guides/speech-to-text#prompting) should match\nthe audio language.\n"]
    pub prompt: String,
}
pub enum RealtimeTranscriptionSessionCreateResponseModalitiesInner {
    Text,
    Audio,
}
#[doc = "Configuration for turn detection. Can be set to `null` to turn off. Server \nVAD means that the model will detect the start and end of speech based on \naudio volume and respond at the end of user speech.\n"]
pub struct RealtimeTranscriptionSessionCreateResponseTurnDetection {
    #[doc = "Amount of audio to include before the VAD detected speech (in \nmilliseconds). Defaults to 300ms.\n"]
    pub prefix_padding_ms: u64,
    #[doc = "Duration of silence to detect speech stop (in milliseconds). Defaults \nto 500ms. With shorter values the model will respond more quickly, \nbut may jump in on short pauses from the user.\n"]
    pub silence_duration_ms: u64,
    #[doc = "Activation threshold for VAD (0.0 to 1.0), this defaults to 0.5. A \nhigher threshold will require louder audio to activate the model, and \nthus might perform better in noisy environments.\n"]
    pub threshold: f64,
    #[doc = "Type of turn detection, only `server_vad` is currently supported.\n"]
    pub type_: String,
}
#[doc = "**o-series models only**\n\nConfiguration options for \n[reasoning models](https://platform.openai.com/docs/guides/reasoning).\n"]
pub struct Reasoning {
    pub effort: ReasoningEffort,
    #[doc = "**Deprecated:** use `summary` instead.\n\nA summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
    pub generate_summary: ReasoningGenerateSummary,
    #[doc = "A summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
    pub summary: ReasoningSummary,
}
#[doc = "**Deprecated:** use `summary` instead.\n\nA summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
pub enum ReasoningGenerateSummary {
    Auto,
    Concise,
    Detailed,
}
#[doc = "A summary of the reasoning performed by the model. This can be\nuseful for debugging and understanding the model's reasoning process.\nOne of `auto`, `concise`, or `detailed`.\n"]
pub enum ReasoningSummary {
    Auto,
    Concise,
    Detailed,
}
#[doc = "**o-series models only** \n\nConstrains effort on reasoning for \n[reasoning models](https://platform.openai.com/docs/guides/reasoning).\nCurrently supported values are `low`, `medium`, and `high`. Reducing\nreasoning effort can result in faster responses and fewer tokens used\non reasoning in a response.\n"]
pub enum ReasoningEffort {
    Low,
    Medium,
    High,
}
#[doc = "A description of the chain of thought used by a reasoning model while generating\na response.\n"]
pub struct ReasoningItem {
    #[doc = "The unique identifier of the reasoning content.\n"]
    pub id: String,
    #[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
    pub status: ReasoningItemStatus,
    #[doc = "Reasoning text contents.\n"]
    pub summary: Vec<ReasoningItemSummaryInner>,
    #[doc = "The type of the object. Always `reasoning`.\n"]
    pub type_: ReasoningItemType,
}
#[doc = "The status of the item. One of `in_progress`, `completed`, or\n`incomplete`. Populated when items are returned via API.\n"]
pub enum ReasoningItemStatus {
    InProgress,
    Completed,
    Incomplete,
}
#[doc = "The type of the object. Always `summary_text`.\n"]
pub enum ReasoningItemSummaryInnerType {
    SummaryText,
}
pub struct ReasoningItemSummaryInner {
    #[doc = "A short summary of the reasoning used by the model when generating\nthe response.\n"]
    pub text: String,
    #[doc = "The type of the object. Always `summary_text`.\n"]
    pub type_: ReasoningItemSummaryInnerType,
}
#[doc = "The type of the object. Always `reasoning`.\n"]
pub enum ReasoningItemType {
    Reasoning,
}
#[doc = "A refusal from the model."]
pub struct RefusalContent {
    #[doc = "The refusal explanationfrom the model."]
    pub refusal: String,
    #[doc = "The type of the refusal. Always `refusal`."]
    pub type_: RefusalContentType,
}
#[doc = "The type of the refusal. Always `refusal`."]
pub enum RefusalContentType {
    Refusal,
}
pub struct Response {
    pub all_of_0: ModelResponseProperties,
    pub all_of_1: ResponseProperties,
    pub all_of_2: Response2,
}
#[doc = "The reason why the response is incomplete."]
pub enum Response2IncompleteDetailsReason {
    MaxOutputTokens,
    ContentFilter,
}
#[doc = "Details about why the response is incomplete.\n"]
pub struct Response2IncompleteDetails {
    #[doc = "The reason why the response is incomplete."]
    pub reason: Response2IncompleteDetailsReason,
}
#[doc = "The object type of this resource - always set to `response`.\n"]
pub enum Response2Object {
    Response,
}
#[doc = "The status of the response generation. One of `completed`, `failed`, \n`in_progress`, or `incomplete`.\n"]
pub enum Response2Status {
    Completed,
    Failed,
    InProgress,
    Incomplete,
}
pub struct Response2 {
    #[doc = "Unix timestamp (in seconds) of when this Response was created.\n"]
    pub created_at: f64,
    pub error: ResponseError,
    #[doc = "Unique identifier for this Response.\n"]
    pub id: String,
    #[doc = "Details about why the response is incomplete.\n"]
    pub incomplete_details: Response2IncompleteDetails,
    #[doc = "The object type of this resource - always set to `response`.\n"]
    pub object: Response2Object,
    #[doc = "An array of content items generated by the model.\n\n- The length and order of items in the `output` array is dependent\n  on the model's response.\n- Rather than accessing the first item in the `output` array and \n  assuming it's an `assistant` message with the content generated by\n  the model, you might consider using the `output_text` property where\n  supported in SDKs.\n"]
    pub output: Vec<OutputItem>,
    #[doc = "SDK-only convenience property that contains the aggregated text output \nfrom all `output_text` items in the `output` array, if any are present. \nSupported in the Python and JavaScript SDKs.\n"]
    pub output_text: String,
    #[doc = "Whether to allow the model to run tool calls in parallel.\n"]
    pub parallel_tool_calls: Vec<bool>,
    #[doc = "The status of the response generation. One of `completed`, `failed`, \n`in_progress`, or `incomplete`.\n"]
    pub status: Response2Status,
    pub usage: ResponseUsage,
}
#[doc = "Emitted when there is a partial audio response."]
pub struct ResponseAudioDeltaEvent {
    #[doc = "A chunk of Base64 encoded response audio bytes.\n"]
    pub delta: String,
    #[doc = "The type of the event. Always `response.audio.delta`.\n"]
    pub type_: ResponseAudioDeltaEventType,
}
#[doc = "The type of the event. Always `response.audio.delta`.\n"]
pub enum ResponseAudioDeltaEventType {
    ResponseAudioDelta,
}
#[doc = "Emitted when the audio response is complete."]
pub struct ResponseAudioDoneEvent {
    #[doc = "The type of the event. Always `response.audio.done`.\n"]
    pub type_: ResponseAudioDoneEventType,
}
#[doc = "The type of the event. Always `response.audio.done`.\n"]
pub enum ResponseAudioDoneEventType {
    ResponseAudioDone,
}
#[doc = "Emitted when there is a partial transcript of audio."]
pub struct ResponseAudioTranscriptDeltaEvent {
    #[doc = "The partial transcript of the audio response.\n"]
    pub delta: String,
    #[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
    pub type_: ResponseAudioTranscriptDeltaEventType,
}
#[doc = "The type of the event. Always `response.audio.transcript.delta`.\n"]
pub enum ResponseAudioTranscriptDeltaEventType {
    ResponseAudioTranscriptDelta,
}
#[doc = "Emitted when the full audio transcript is completed."]
pub struct ResponseAudioTranscriptDoneEvent {
    #[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
    pub type_: ResponseAudioTranscriptDoneEventType,
}
#[doc = "The type of the event. Always `response.audio.transcript.done`.\n"]
pub enum ResponseAudioTranscriptDoneEventType {
    ResponseAudioTranscriptDone,
}
#[doc = "Emitted when a partial code snippet is added by the code interpreter."]
pub struct ResponseCodeInterpreterCallCodeDeltaEvent {
    #[doc = "The partial code snippet added by the code interpreter.\n"]
    pub delta: String,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.code.delta`.\n"]
    pub type_: ResponseCodeInterpreterCallCodeDeltaEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.code.delta`.\n"]
pub enum ResponseCodeInterpreterCallCodeDeltaEventType {
    ResponseCodeInterpreterCallCodeDelta,
}
#[doc = "Emitted when code snippet output is finalized by the code interpreter."]
pub struct ResponseCodeInterpreterCallCodeDoneEvent {
    #[doc = "The final code snippet output by the code interpreter.\n"]
    pub code: String,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.code.done`.\n"]
    pub type_: ResponseCodeInterpreterCallCodeDoneEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.code.done`.\n"]
pub enum ResponseCodeInterpreterCallCodeDoneEventType {
    ResponseCodeInterpreterCallCodeDone,
}
#[doc = "Emitted when the code interpreter call is completed."]
pub struct ResponseCodeInterpreterCallCompletedEvent {
    pub code_interpreter_call: CodeInterpreterToolCall,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.completed`.\n"]
    pub type_: ResponseCodeInterpreterCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.completed`.\n"]
pub enum ResponseCodeInterpreterCallCompletedEventType {
    ResponseCodeInterpreterCallCompleted,
}
#[doc = "Emitted when a code interpreter call is in progress."]
pub struct ResponseCodeInterpreterCallInProgressEvent {
    pub code_interpreter_call: CodeInterpreterToolCall,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`.\n"]
    pub type_: ResponseCodeInterpreterCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.in_progress`.\n"]
pub enum ResponseCodeInterpreterCallInProgressEventType {
    ResponseCodeInterpreterCallInProgress,
}
#[doc = "Emitted when the code interpreter is actively interpreting the code snippet."]
pub struct ResponseCodeInterpreterCallInterpretingEvent {
    pub code_interpreter_call: CodeInterpreterToolCall,
    #[doc = "The index of the output item that the code interpreter call is in progress.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`.\n"]
    pub type_: ResponseCodeInterpreterCallInterpretingEventType,
}
#[doc = "The type of the event. Always `response.code_interpreter_call.interpreting`.\n"]
pub enum ResponseCodeInterpreterCallInterpretingEventType {
    ResponseCodeInterpreterCallInterpreting,
}
#[doc = "Emitted when the model response is complete."]
pub struct ResponseCompletedEvent {
    #[doc = "Properties of the completed response.\n"]
    pub response: Response,
    #[doc = "The type of the event. Always `response.completed`.\n"]
    pub type_: ResponseCompletedEventType,
}
#[doc = "The type of the event. Always `response.completed`.\n"]
pub enum ResponseCompletedEventType {
    ResponseCompleted,
}
#[doc = "Emitted when a new content part is added."]
pub struct ResponseContentPartAddedEvent {
    #[doc = "The index of the content part that was added.\n"]
    pub content_index: u64,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    pub output_index: u64,
    #[doc = "The content part that was added.\n"]
    pub part: OutputContent,
    #[doc = "The type of the event. Always `response.content_part.added`.\n"]
    pub type_: ResponseContentPartAddedEventType,
}
#[doc = "The type of the event. Always `response.content_part.added`.\n"]
pub enum ResponseContentPartAddedEventType {
    ResponseContentPartAdded,
}
#[doc = "Emitted when a content part is done."]
pub struct ResponseContentPartDoneEvent {
    #[doc = "The index of the content part that is done.\n"]
    pub content_index: u64,
    #[doc = "The ID of the output item that the content part was added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the content part was added to.\n"]
    pub output_index: u64,
    #[doc = "The content part that is done.\n"]
    pub part: OutputContent,
    #[doc = "The type of the event. Always `response.content_part.done`.\n"]
    pub type_: ResponseContentPartDoneEventType,
}
#[doc = "The type of the event. Always `response.content_part.done`.\n"]
pub enum ResponseContentPartDoneEventType {
    ResponseContentPartDone,
}
#[doc = "An event that is emitted when a response is created.\n"]
pub struct ResponseCreatedEvent {
    #[doc = "The response that was created.\n"]
    pub response: Response,
    #[doc = "The type of the event. Always `response.created`.\n"]
    pub type_: ResponseCreatedEventType,
}
#[doc = "The type of the event. Always `response.created`.\n"]
pub enum ResponseCreatedEventType {
    ResponseCreated,
}
#[doc = "An error object returned when the model fails to generate a Response.\n"]
pub struct ResponseError {
    pub code: ResponseErrorCode,
    #[doc = "A human-readable description of the error.\n"]
    pub message: String,
}
#[doc = "The error code for the response.\n"]
pub enum ResponseErrorCode {
    ServerError,
    RateLimitExceeded,
    InvalidPrompt,
    VectorStoreTimeout,
    InvalidImage,
    InvalidImageFormat,
    InvalidBase64Image,
    InvalidImageUrl,
    ImageTooLarge,
    ImageTooSmall,
    ImageParseError,
    ImageContentPolicyViolation,
    InvalidImageMode,
    ImageFileTooLarge,
    UnsupportedImageMediaType,
    EmptyImageFile,
    FailedToDownloadImage,
    ImageFileNotFound,
}
#[doc = "Emitted when an error occurs."]
pub struct ResponseErrorEvent {
    #[doc = "The error code.\n"]
    pub code: String,
    #[doc = "The error message.\n"]
    pub message: String,
    #[doc = "The error parameter.\n"]
    pub param: String,
    #[doc = "The type of the event. Always `error`.\n"]
    pub type_: ResponseErrorEventType,
}
#[doc = "The type of the event. Always `error`.\n"]
pub enum ResponseErrorEventType {
    Error,
}
#[doc = "An event that is emitted when a response fails.\n"]
pub struct ResponseFailedEvent {
    #[doc = "The response that failed.\n"]
    pub response: Response,
    #[doc = "The type of the event. Always `response.failed`.\n"]
    pub type_: ResponseFailedEventType,
}
#[doc = "The type of the event. Always `response.failed`.\n"]
pub enum ResponseFailedEventType {
    ResponseFailed,
}
#[doc = "Emitted when a file search call is completed (results found)."]
pub struct ResponseFileSearchCallCompletedEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
    pub type_: ResponseFileSearchCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.completed`.\n"]
pub enum ResponseFileSearchCallCompletedEventType {
    ResponseFileSearchCallCompleted,
}
#[doc = "Emitted when a file search call is initiated."]
pub struct ResponseFileSearchCallInProgressEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the file search call is initiated.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
    pub type_: ResponseFileSearchCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.in_progress`.\n"]
pub enum ResponseFileSearchCallInProgressEventType {
    ResponseFileSearchCallInProgress,
}
#[doc = "Emitted when a file search is currently searching."]
pub struct ResponseFileSearchCallSearchingEvent {
    #[doc = "The ID of the output item that the file search call is initiated.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the file search call is searching.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
    pub type_: ResponseFileSearchCallSearchingEventType,
}
#[doc = "The type of the event. Always `response.file_search_call.searching`.\n"]
pub enum ResponseFileSearchCallSearchingEventType {
    ResponseFileSearchCallSearching,
}
#[doc = "JSON object response format. An older method of generating JSON responses.\nUsing `json_schema` is recommended for models that support it. Note that the\nmodel will not generate JSON without a system or user message instructing it\nto do so.\n"]
pub struct ResponseFormatJsonObject {
    #[doc = "The type of response format being defined. Always `json_object`."]
    pub type_: ResponseFormatJsonObjectType,
}
#[doc = "The type of response format being defined. Always `json_object`."]
pub enum ResponseFormatJsonObjectType {
    JsonObject,
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](/docs/guides/structured-outputs).\n"]
pub struct ResponseFormatJsonSchema {
    #[doc = "Structured Outputs configuration options, including a JSON Schema.\n"]
    pub json_schema: ResponseFormatJsonSchemaJsonSchema,
    #[doc = "The type of response format being defined. Always `json_schema`."]
    pub type_: ResponseFormatJsonSchemaType,
}
#[doc = "Structured Outputs configuration options, including a JSON Schema.\n"]
pub struct ResponseFormatJsonSchemaJsonSchema {
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    pub description: String,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    pub name: String,
    pub schema: ResponseFormatJsonSchemaSchema,
    #[doc = "Whether to enable strict schema adherence when generating the output.\nIf set to true, the model will always follow the exact schema defined\nin the `schema` field. Only a subset of JSON Schema is supported when\n`strict` is `true`. To learn more, read the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n"]
    pub strict: Vec<bool>,
}
#[doc = "The type of response format being defined. Always `json_schema`."]
pub enum ResponseFormatJsonSchemaType {
    JsonSchema,
}
#[doc = "The schema for the response format, described as a JSON Schema object.\nLearn how to build JSON schemas [here](https://json-schema.org/).\n"]
pub type ResponseFormatJsonSchemaSchema = std::collections::BTreeMap<String, serde_json::Value>;
#[doc = "Default response format. Used to generate text responses.\n"]
pub struct ResponseFormatText {
    #[doc = "The type of response format being defined. Always `text`."]
    pub type_: ResponseFormatTextType,
}
#[doc = "The type of response format being defined. Always `text`."]
pub enum ResponseFormatTextType {
    Text,
}
#[doc = "Emitted when there is a partial function-call arguments delta."]
pub struct ResponseFunctionCallArgumentsDeltaEvent {
    #[doc = "The function-call arguments delta that is added.\n"]
    pub delta: String,
    #[doc = "The ID of the output item that the function-call arguments delta is added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the function-call arguments delta is added to.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
    pub type_: ResponseFunctionCallArgumentsDeltaEventType,
}
#[doc = "The type of the event. Always `response.function_call_arguments.delta`.\n"]
pub enum ResponseFunctionCallArgumentsDeltaEventType {
    ResponseFunctionCallArgumentsDelta,
}
#[doc = "Emitted when function-call arguments are finalized."]
pub struct ResponseFunctionCallArgumentsDoneEvent {
    #[doc = "The function-call arguments."]
    pub arguments: String,
    #[doc = "The ID of the item."]
    pub item_id: String,
    #[doc = "The index of the output item."]
    pub output_index: u64,
    pub type_: ResponseFunctionCallArgumentsDoneEventType,
}
pub enum ResponseFunctionCallArgumentsDoneEventType {
    ResponseFunctionCallArgumentsDone,
}
#[doc = "Emitted when the response is in progress."]
pub struct ResponseInProgressEvent {
    #[doc = "The response that is in progress.\n"]
    pub response: Response,
    #[doc = "The type of the event. Always `response.in_progress`.\n"]
    pub type_: ResponseInProgressEventType,
}
#[doc = "The type of the event. Always `response.in_progress`.\n"]
pub enum ResponseInProgressEventType {
    ResponseInProgress,
}
#[doc = "An event that is emitted when a response finishes as incomplete.\n"]
pub struct ResponseIncompleteEvent {
    #[doc = "The response that was incomplete.\n"]
    pub response: Response,
    #[doc = "The type of the event. Always `response.incomplete`.\n"]
    pub type_: ResponseIncompleteEventType,
}
#[doc = "The type of the event. Always `response.incomplete`.\n"]
pub enum ResponseIncompleteEventType {
    ResponseIncomplete,
}
#[doc = "A list of Response items."]
pub struct ResponseItemList {
    #[doc = "A list of items used to generate this response."]
    pub data: Vec<ItemResource>,
    #[doc = "The ID of the first item in the list."]
    pub first_id: String,
    #[doc = "Whether there are more items available."]
    pub has_more: Vec<bool>,
    #[doc = "The ID of the last item in the list."]
    pub last_id: String,
    #[doc = "The type of object returned, must be `list`."]
    pub object: ResponseItemListObject,
}
#[doc = "The type of object returned, must be `list`."]
pub enum ResponseItemListObject {
    List,
}
#[doc = "Output types that you would like the model to generate.\nMost models are capable of generating text, which is the default:\n\n`[\"text\"]`\n\nThe `gpt-4o-audio-preview` model can also be used to \n[generate audio](/docs/guides/audio). To request that this model generate \nboth text and audio responses, you can use:\n\n`[\"text\", \"audio\"]`\n"]
pub type ResponseModalities = Vec<ResponseModalitiesInner>;
pub enum ResponseModalitiesInner {
    Text,
    Audio,
}
#[doc = "Emitted when a new output item is added."]
pub struct ResponseOutputItemAddedEvent {
    #[doc = "The output item that was added.\n"]
    pub item: OutputItem,
    #[doc = "The index of the output item that was added.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.output_item.added`.\n"]
    pub type_: ResponseOutputItemAddedEventType,
}
#[doc = "The type of the event. Always `response.output_item.added`.\n"]
pub enum ResponseOutputItemAddedEventType {
    ResponseOutputItemAdded,
}
#[doc = "Emitted when an output item is marked done."]
pub struct ResponseOutputItemDoneEvent {
    #[doc = "The output item that was marked done.\n"]
    pub item: OutputItem,
    #[doc = "The index of the output item that was marked done.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.output_item.done`.\n"]
    pub type_: ResponseOutputItemDoneEventType,
}
#[doc = "The type of the event. Always `response.output_item.done`.\n"]
pub enum ResponseOutputItemDoneEventType {
    ResponseOutputItemDone,
}
pub struct ResponseProperties {
    #[doc = "Inserts a system (or developer) message as the first item in the model's context.\n\nWhen using along with `previous_response_id`, the instructions from a previous\nresponse will not be carried over to the next response. This makes it simple\nto swap out system (or developer) messages in new responses.\n"]
    pub instructions: String,
    #[doc = "An upper bound for the number of tokens that can be generated for a response, including visible output tokens and [reasoning tokens](/docs/guides/reasoning).\n"]
    pub max_output_tokens: u64,
    #[doc = "Model ID used to generate the response, like `gpt-4o` or `o3`. OpenAI\noffers a wide range of models with different capabilities, performance\ncharacteristics, and price points. Refer to the [model guide](/docs/models)\nto browse and compare available models.\n"]
    pub model: ModelIdsResponses,
    #[doc = "The unique ID of the previous response to the model. Use this to\ncreate multi-turn conversations. Learn more about \n[conversation state](/docs/guides/conversation-state).\n"]
    pub previous_response_id: String,
    pub reasoning: Reasoning,
    #[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Structured Outputs](/docs/guides/structured-outputs)\n"]
    pub text: ResponsePropertiesText,
    #[doc = "How the model should select which tool (or tools) to use when generating\na response. See the `tools` parameter to see how to specify which tools\nthe model can call.\n"]
    pub tool_choice: ResponsePropertiesToolChoice,
    #[doc = "An array of tools the model may call while generating a response. You \ncan specify which tool to use by setting the `tool_choice` parameter.\n\nThe two categories of tools you can provide the model are:\n\n- **Built-in tools**: Tools that are provided by OpenAI that extend the\n  model's capabilities, like [web search](/docs/guides/tools-web-search)\n  or [file search](/docs/guides/tools-file-search). Learn more about\n  [built-in tools](/docs/guides/tools).\n- **Function calls (custom tools)**: Functions that are defined by you,\n  enabling the model to call your own code. Learn more about\n  [function calling](/docs/guides/function-calling).\n"]
    pub tools: Vec<Tool>,
    #[doc = "The truncation strategy to use for the model response.\n- `auto`: If the context of this response and previous ones exceeds\n  the model's context window size, the model will truncate the \n  response to fit the context window by dropping input items in the\n  middle of the conversation. \n- `disabled` (default): If a model response will exceed the context window \n  size for a model, the request will fail with a 400 error.\n"]
    pub truncation: ResponsePropertiesTruncation,
}
#[doc = "Configuration options for a text response from the model. Can be plain\ntext or structured JSON data. Learn more:\n- [Text inputs and outputs](/docs/guides/text)\n- [Structured Outputs](/docs/guides/structured-outputs)\n"]
pub struct ResponsePropertiesText {
    pub format: TextResponseFormatConfiguration,
}
#[doc = "How the model should select which tool (or tools) to use when generating\na response. See the `tools` parameter to see how to specify which tools\nthe model can call.\n"]
#[allow(clippy::large_enum_variant)]
pub enum ResponsePropertiesToolChoice {
    OneOf0(ToolChoiceOptions),
    OneOf1(ToolChoiceTypes),
    OneOf2(ToolChoiceFunction),
}
#[doc = "The truncation strategy to use for the model response.\n- `auto`: If the context of this response and previous ones exceeds\n  the model's context window size, the model will truncate the \n  response to fit the context window by dropping input items in the\n  middle of the conversation. \n- `disabled` (default): If a model response will exceed the context window \n  size for a model, the request will fail with a 400 error.\n"]
pub enum ResponsePropertiesTruncation {
    Auto,
    Disabled,
}
#[doc = "Emitted when a new reasoning summary part is added."]
pub struct ResponseReasoningSummaryPartAddedEvent {
    #[doc = "The ID of the item this summary part is associated with.\n"]
    pub item_id: String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    pub output_index: u64,
    #[doc = "The summary part that was added.\n"]
    pub part: ResponseReasoningSummaryPartAddedEventPart,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: u64,
    #[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
    pub type_: ResponseReasoningSummaryPartAddedEventType,
}
#[doc = "The type of the summary part. Always `summary_text`."]
pub enum ResponseReasoningSummaryPartAddedEventPartType {
    SummaryText,
}
#[doc = "The summary part that was added.\n"]
pub struct ResponseReasoningSummaryPartAddedEventPart {
    #[doc = "The text of the summary part."]
    pub text: String,
    #[doc = "The type of the summary part. Always `summary_text`."]
    pub type_: ResponseReasoningSummaryPartAddedEventPartType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.added`.\n"]
pub enum ResponseReasoningSummaryPartAddedEventType {
    ResponseReasoningSummaryPartAdded,
}
#[doc = "Emitted when a reasoning summary part is completed."]
pub struct ResponseReasoningSummaryPartDoneEvent {
    #[doc = "The ID of the item this summary part is associated with.\n"]
    pub item_id: String,
    #[doc = "The index of the output item this summary part is associated with.\n"]
    pub output_index: u64,
    #[doc = "The completed summary part.\n"]
    pub part: ResponseReasoningSummaryPartDoneEventPart,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: u64,
    #[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
    pub type_: ResponseReasoningSummaryPartDoneEventType,
}
#[doc = "The type of the summary part. Always `summary_text`."]
pub enum ResponseReasoningSummaryPartDoneEventPartType {
    SummaryText,
}
#[doc = "The completed summary part.\n"]
pub struct ResponseReasoningSummaryPartDoneEventPart {
    #[doc = "The text of the summary part."]
    pub text: String,
    #[doc = "The type of the summary part. Always `summary_text`."]
    pub type_: ResponseReasoningSummaryPartDoneEventPartType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_part.done`.\n"]
pub enum ResponseReasoningSummaryPartDoneEventType {
    ResponseReasoningSummaryPartDone,
}
#[doc = "Emitted when a delta is added to a reasoning summary text."]
pub struct ResponseReasoningSummaryTextDeltaEvent {
    #[doc = "The text delta that was added to the summary.\n"]
    pub delta: String,
    #[doc = "The ID of the item this summary text delta is associated with.\n"]
    pub item_id: String,
    #[doc = "The index of the output item this summary text delta is associated with.\n"]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: u64,
    #[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
    pub type_: ResponseReasoningSummaryTextDeltaEventType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.delta`.\n"]
pub enum ResponseReasoningSummaryTextDeltaEventType {
    ResponseReasoningSummaryTextDelta,
}
#[doc = "Emitted when a reasoning summary text is completed."]
pub struct ResponseReasoningSummaryTextDoneEvent {
    #[doc = "The ID of the item this summary text is associated with.\n"]
    pub item_id: String,
    #[doc = "The index of the output item this summary text is associated with.\n"]
    pub output_index: u64,
    #[doc = "The index of the summary part within the reasoning summary.\n"]
    pub summary_index: u64,
    #[doc = "The full text of the completed reasoning summary.\n"]
    pub text: String,
    #[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
    pub type_: ResponseReasoningSummaryTextDoneEventType,
}
#[doc = "The type of the event. Always `response.reasoning_summary_text.done`.\n"]
pub enum ResponseReasoningSummaryTextDoneEventType {
    ResponseReasoningSummaryTextDone,
}
#[doc = "Emitted when there is a partial refusal text."]
pub struct ResponseRefusalDeltaEvent {
    #[doc = "The index of the content part that the refusal text is added to.\n"]
    pub content_index: u64,
    #[doc = "The refusal text that is added.\n"]
    pub delta: String,
    #[doc = "The ID of the output item that the refusal text is added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the refusal text is added to.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.refusal.delta`.\n"]
    pub type_: ResponseRefusalDeltaEventType,
}
#[doc = "The type of the event. Always `response.refusal.delta`.\n"]
pub enum ResponseRefusalDeltaEventType {
    ResponseRefusalDelta,
}
#[doc = "Emitted when refusal text is finalized."]
pub struct ResponseRefusalDoneEvent {
    #[doc = "The index of the content part that the refusal text is finalized.\n"]
    pub content_index: u64,
    #[doc = "The ID of the output item that the refusal text is finalized.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the refusal text is finalized.\n"]
    pub output_index: u64,
    #[doc = "The refusal text that is finalized.\n"]
    pub refusal: String,
    #[doc = "The type of the event. Always `response.refusal.done`.\n"]
    pub type_: ResponseRefusalDoneEventType,
}
#[doc = "The type of the event. Always `response.refusal.done`.\n"]
pub enum ResponseRefusalDoneEventType {
    ResponseRefusalDone,
}
#[allow(clippy::large_enum_variant)]
pub enum ResponseStreamEvent {
    AnyOf0(ResponseAudioDeltaEvent),
    AnyOf1(ResponseAudioDoneEvent),
    AnyOf2(ResponseAudioTranscriptDeltaEvent),
    AnyOf3(ResponseAudioTranscriptDoneEvent),
    AnyOf4(ResponseCodeInterpreterCallCodeDeltaEvent),
    AnyOf5(ResponseCodeInterpreterCallCodeDoneEvent),
    AnyOf6(ResponseCodeInterpreterCallCompletedEvent),
    AnyOf7(ResponseCodeInterpreterCallInProgressEvent),
    AnyOf8(ResponseCodeInterpreterCallInterpretingEvent),
    AnyOf9(ResponseCompletedEvent),
    AnyOf10(ResponseContentPartAddedEvent),
    AnyOf11(ResponseContentPartDoneEvent),
    AnyOf12(ResponseCreatedEvent),
    AnyOf13(ResponseErrorEvent),
    AnyOf14(ResponseFileSearchCallCompletedEvent),
    AnyOf15(ResponseFileSearchCallInProgressEvent),
    AnyOf16(ResponseFileSearchCallSearchingEvent),
    AnyOf17(ResponseFunctionCallArgumentsDeltaEvent),
    AnyOf18(ResponseFunctionCallArgumentsDoneEvent),
    AnyOf19(ResponseInProgressEvent),
    AnyOf20(ResponseFailedEvent),
    AnyOf21(ResponseIncompleteEvent),
    AnyOf22(ResponseOutputItemAddedEvent),
    AnyOf23(ResponseOutputItemDoneEvent),
    AnyOf24(ResponseReasoningSummaryPartAddedEvent),
    AnyOf25(ResponseReasoningSummaryPartDoneEvent),
    AnyOf26(ResponseReasoningSummaryTextDeltaEvent),
    AnyOf27(ResponseReasoningSummaryTextDoneEvent),
    AnyOf28(ResponseRefusalDeltaEvent),
    AnyOf29(ResponseRefusalDoneEvent),
    AnyOf30(ResponseTextAnnotationDeltaEvent),
    AnyOf31(ResponseTextDeltaEvent),
    AnyOf32(ResponseTextDoneEvent),
    AnyOf33(ResponseWebSearchCallCompletedEvent),
    AnyOf34(ResponseWebSearchCallInProgressEvent),
    AnyOf35(ResponseWebSearchCallSearchingEvent),
}
#[doc = "Emitted when a text annotation is added."]
pub struct ResponseTextAnnotationDeltaEvent {
    pub annotation: Annotation,
    #[doc = "The index of the annotation that was added.\n"]
    pub annotation_index: u64,
    #[doc = "The index of the content part that the text annotation was added to.\n"]
    pub content_index: u64,
    #[doc = "The ID of the output item that the text annotation was added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the text annotation was added to.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.output_text.annotation.added`.\n"]
    pub type_: ResponseTextAnnotationDeltaEventType,
}
#[doc = "The type of the event. Always `response.output_text.annotation.added`.\n"]
pub enum ResponseTextAnnotationDeltaEventType {
    ResponseOutputTextAnnotationAdded,
}
#[doc = "Emitted when there is an additional text delta."]
pub struct ResponseTextDeltaEvent {
    #[doc = "The index of the content part that the text delta was added to.\n"]
    pub content_index: u64,
    #[doc = "The text delta that was added.\n"]
    pub delta: String,
    #[doc = "The ID of the output item that the text delta was added to.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the text delta was added to.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.output_text.delta`.\n"]
    pub type_: ResponseTextDeltaEventType,
}
#[doc = "The type of the event. Always `response.output_text.delta`.\n"]
pub enum ResponseTextDeltaEventType {
    ResponseOutputTextDelta,
}
#[doc = "Emitted when text content is finalized."]
pub struct ResponseTextDoneEvent {
    #[doc = "The index of the content part that the text content is finalized.\n"]
    pub content_index: u64,
    #[doc = "The ID of the output item that the text content is finalized.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the text content is finalized.\n"]
    pub output_index: u64,
    #[doc = "The text content that is finalized.\n"]
    pub text: String,
    #[doc = "The type of the event. Always `response.output_text.done`.\n"]
    pub type_: ResponseTextDoneEventType,
}
#[doc = "The type of the event. Always `response.output_text.done`.\n"]
pub enum ResponseTextDoneEventType {
    ResponseOutputTextDone,
}
#[doc = "Represents token usage details including input tokens, output tokens,\na breakdown of output tokens, and the total tokens used.\n"]
pub struct ResponseUsage {
    #[doc = "The number of input tokens."]
    pub input_tokens: u64,
    #[doc = "A detailed breakdown of the input tokens."]
    pub input_tokens_details: ResponseUsageInputTokensDetails,
    #[doc = "The number of output tokens."]
    pub output_tokens: u64,
    #[doc = "A detailed breakdown of the output tokens."]
    pub output_tokens_details: ResponseUsageOutputTokensDetails,
    #[doc = "The total number of tokens used."]
    pub total_tokens: u64,
}
#[doc = "A detailed breakdown of the input tokens."]
pub struct ResponseUsageInputTokensDetails {
    #[doc = "The number of tokens that were retrieved from the cache. \n[More on prompt caching](/docs/guides/prompt-caching).\n"]
    pub cached_tokens: u64,
}
#[doc = "A detailed breakdown of the output tokens."]
pub struct ResponseUsageOutputTokensDetails {
    #[doc = "The number of reasoning tokens."]
    pub reasoning_tokens: u64,
}
#[doc = "Emitted when a web search call is completed."]
pub struct ResponseWebSearchCallCompletedEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
    pub type_: ResponseWebSearchCallCompletedEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.completed`.\n"]
pub enum ResponseWebSearchCallCompletedEventType {
    ResponseWebSearchCallCompleted,
}
#[doc = "Emitted when a web search call is initiated."]
pub struct ResponseWebSearchCallInProgressEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
    pub type_: ResponseWebSearchCallInProgressEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.in_progress`.\n"]
pub enum ResponseWebSearchCallInProgressEventType {
    ResponseWebSearchCallInProgress,
}
#[doc = "Emitted when a web search call is executing."]
pub struct ResponseWebSearchCallSearchingEvent {
    #[doc = "Unique ID for the output item associated with the web search call.\n"]
    pub item_id: String,
    #[doc = "The index of the output item that the web search call is associated with.\n"]
    pub output_index: u64,
    #[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
    pub type_: ResponseWebSearchCallSearchingEventType,
}
#[doc = "The type of the event. Always `response.web_search_call.searching`.\n"]
pub enum ResponseWebSearchCallSearchingEventType {
    ResponseWebSearchCallSearching,
}
#[doc = "Usage statistics related to the run. This value will be `null` if the run is not in a terminal state (i.e. `in_progress`, `queued`, etc.)."]
pub struct RunCompletionUsage {
    #[doc = "Number of completion tokens used over the course of the run."]
    pub completion_tokens: u64,
    #[doc = "Number of prompt tokens used over the course of the run."]
    pub prompt_tokens: u64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    pub total_tokens: u64,
}
#[doc = "Represents an execution run on a [thread](/docs/api-reference/threads)."]
pub struct RunObject {
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) used for execution of this run."]
    pub assistant_id: String,
    #[doc = "The Unix timestamp (in seconds) for when the run was cancelled."]
    pub cancelled_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run was completed."]
    pub completed_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run was created."]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run will expire."]
    pub expires_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run failed."]
    pub failed_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
    pub incomplete_details: RunObjectIncompleteDetails,
    #[doc = "The instructions that the [assistant](/docs/api-reference/assistants) used for this run."]
    pub instructions: String,
    #[doc = "The last error associated with this run. Will be `null` if there are no errors."]
    pub last_error: RunObjectLastError,
    #[doc = "The maximum number of completion tokens specified to have been used over the course of the run.\n"]
    pub max_completion_tokens: u64,
    #[doc = "The maximum number of prompt tokens specified to have been used over the course of the run.\n"]
    pub max_prompt_tokens: u64,
    pub metadata: Metadata,
    #[doc = "The model that the [assistant](/docs/api-reference/assistants) used for this run."]
    pub model: String,
    #[doc = "The object type, which is always `thread.run`."]
    pub object: RunObjectObject,
    pub parallel_tool_calls: ParallelToolCalls,
    #[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
    pub required_action: RunObjectRequiredAction,
    pub response_format: AssistantsApiResponseFormatOption,
    #[doc = "The Unix timestamp (in seconds) for when the run was started."]
    pub started_at: u64,
    #[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
    pub status: RunObjectStatus,
    #[doc = "The sampling temperature used for this run. If not set, defaults to 1."]
    pub temperature: f64,
    #[doc = "The ID of the [thread](/docs/api-reference/threads) that was executed on as a part of this run."]
    pub thread_id: String,
    pub tool_choice: RunObjectToolChoice,
    #[doc = "The list of tools that the [assistant](/docs/api-reference/assistants) used for this run."]
    pub tools: Vec<RunObjectToolsInner>,
    #[doc = "The nucleus sampling value used for this run. If not set, defaults to 1."]
    pub top_p: f64,
    pub truncation_strategy: RunObjectTruncationStrategy,
    pub usage: RunCompletionUsage,
}
#[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
pub enum RunObjectIncompleteDetailsReason {
    MaxCompletionTokens,
    MaxPromptTokens,
}
#[doc = "Details on why the run is incomplete. Will be `null` if the run is not incomplete."]
pub struct RunObjectIncompleteDetails {
    #[doc = "The reason why the run is incomplete. This will point to which specific token limit was reached over the course of the run."]
    pub reason: RunObjectIncompleteDetailsReason,
}
#[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
pub enum RunObjectLastErrorCode {
    ServerError,
    RateLimitExceeded,
    InvalidPrompt,
}
#[doc = "The last error associated with this run. Will be `null` if there are no errors."]
pub struct RunObjectLastError {
    #[doc = "One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`."]
    pub code: RunObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: String,
}
#[doc = "The object type, which is always `thread.run`."]
pub enum RunObjectObject {
    ThreadRun,
}
#[doc = "Details on the tool outputs needed for this run to continue."]
pub struct RunObjectRequiredActionSubmitToolOutputs {
    #[doc = "A list of the relevant tool calls."]
    pub tool_calls: Vec<RunToolCallObject>,
}
#[doc = "For now, this is always `submit_tool_outputs`."]
pub enum RunObjectRequiredActionType {
    SubmitToolOutputs,
}
#[doc = "Details on the action required to continue the run. Will be `null` if no action is required."]
pub struct RunObjectRequiredAction {
    #[doc = "Details on the tool outputs needed for this run to continue."]
    pub submit_tool_outputs: RunObjectRequiredActionSubmitToolOutputs,
    #[doc = "For now, this is always `submit_tool_outputs`."]
    pub type_: RunObjectRequiredActionType,
}
#[doc = "The status of the run, which can be either `queued`, `in_progress`, `requires_action`, `cancelling`, `cancelled`, `failed`, `completed`, `incomplete`, or `expired`."]
pub enum RunObjectStatus {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
    Incomplete,
    Expired,
}
pub struct RunObjectToolChoice {
    pub all_of_0: AssistantsApiToolChoiceOption,
    pub all_of_1: serde_json::Value,
}
#[allow(clippy::large_enum_variant)]
pub enum RunObjectToolsInner {
    OneOf0(AssistantToolsCode),
    OneOf1(AssistantToolsFileSearch),
    OneOf2(AssistantToolsFunction),
}
pub struct RunObjectTruncationStrategy {
    pub all_of_0: TruncationObject,
    pub all_of_1: serde_json::Value,
}
#[doc = "Usage statistics related to the run step. This value will be `null` while the run step's status is `in_progress`."]
pub struct RunStepCompletionUsage {
    #[doc = "Number of completion tokens used over the course of the run step."]
    pub completion_tokens: u64,
    #[doc = "Number of prompt tokens used over the course of the run step."]
    pub prompt_tokens: u64,
    #[doc = "Total number of tokens used (prompt + completion)."]
    pub total_tokens: u64,
}
#[doc = "Represents a run step delta i.e. any changed fields on a run step during streaming.\n"]
pub struct RunStepDeltaObject {
    #[doc = "The delta containing the fields that have changed on the run step."]
    pub delta: RunStepDeltaObjectDelta,
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always `thread.run.step.delta`."]
    pub object: RunStepDeltaObjectObject,
}
#[doc = "The delta containing the fields that have changed on the run step."]
pub struct RunStepDeltaObjectDelta {
    #[doc = "The details of the run step."]
    pub step_details: std::collections::BTreeMap<String, serde_json::Value>,
}
#[doc = "The object type, which is always `thread.run.step.delta`."]
pub enum RunStepDeltaObjectObject {
    ThreadRunStepDelta,
}
#[doc = "Details of the message creation by the run step."]
pub struct RunStepDeltaStepDetailsMessageCreationObject {
    pub message_creation: RunStepDeltaStepDetailsMessageCreationObjectMessageCreation,
    #[doc = "Always `message_creation`."]
    pub type_: RunStepDeltaStepDetailsMessageCreationObjectType,
}
pub struct RunStepDeltaStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    pub message_id: String,
}
#[doc = "Always `message_creation`."]
pub enum RunStepDeltaStepDetailsMessageCreationObjectType {
    MessageCreation,
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
pub struct RunStepDeltaStepDetailsToolCallsCodeObject {
    #[doc = "The Code Interpreter tool call definition."]
    pub code_interpreter: RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter,
    #[doc = "The ID of the tool call."]
    pub id: String,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: u64,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeObjectType,
}
#[doc = "The Code Interpreter tool call definition."]
pub struct RunStepDeltaStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    pub input: String,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    pub outputs: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
pub enum RunStepDeltaStepDetailsToolCallsCodeObjectType {
    CodeInterpreter,
}
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObject {
    pub image: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage,
    #[doc = "The index of the output in the outputs array."]
    pub index: u64,
    #[doc = "Always `image`."]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType,
}
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](/docs/api-reference/files) ID of the image."]
    pub file_id: String,
}
#[doc = "Always `image`."]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputImageObjectType {
    Image,
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
pub struct RunStepDeltaStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "The index of the output in the outputs array."]
    pub index: u64,
    #[doc = "The text output from the Code Interpreter tool call."]
    pub logs: String,
    #[doc = "Always `logs`."]
    pub type_: RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType,
}
#[doc = "Always `logs`."]
pub enum RunStepDeltaStepDetailsToolCallsCodeOutputLogsObjectType {
    Logs,
}
pub struct RunStepDeltaStepDetailsToolCallsFileSearchObject {
    #[doc = "For now, this is always going to be an empty object."]
    pub file_search: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The ID of the tool call object."]
    pub id: String,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: u64,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    pub type_: RunStepDeltaStepDetailsToolCallsFileSearchObjectType,
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
pub enum RunStepDeltaStepDetailsToolCallsFileSearchObjectType {
    FileSearch,
}
pub struct RunStepDeltaStepDetailsToolCallsFunctionObject {
    #[doc = "The definition of the function that was called."]
    pub function: RunStepDeltaStepDetailsToolCallsFunctionObjectFunction,
    #[doc = "The ID of the tool call object."]
    pub id: String,
    #[doc = "The index of the tool call in the tool calls array."]
    pub index: u64,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    pub type_: RunStepDeltaStepDetailsToolCallsFunctionObjectType,
}
#[doc = "The definition of the function that was called."]
pub struct RunStepDeltaStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The arguments passed to the function."]
    pub arguments: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet."]
    pub output: String,
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
pub enum RunStepDeltaStepDetailsToolCallsFunctionObjectType {
    Function,
}
#[doc = "Details of the tool call."]
pub struct RunStepDeltaStepDetailsToolCallsObject {
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    pub tool_calls: Vec<RunStepDeltaStepDetailsToolCallsObjectToolCallsInner>,
    #[doc = "Always `tool_calls`."]
    pub type_: RunStepDeltaStepDetailsToolCallsObjectType,
}
#[allow(clippy::large_enum_variant)]
pub enum RunStepDeltaStepDetailsToolCallsObjectToolCallsInner {
    OneOf0(RunStepDeltaStepDetailsToolCallsCodeObject),
    OneOf1(RunStepDeltaStepDetailsToolCallsFileSearchObject),
    OneOf2(RunStepDeltaStepDetailsToolCallsFunctionObject),
}
#[doc = "Always `tool_calls`."]
pub enum RunStepDeltaStepDetailsToolCallsObjectType {
    ToolCalls,
}
#[doc = "Details of the message creation by the run step."]
pub struct RunStepDetailsMessageCreationObject {
    pub message_creation: RunStepDetailsMessageCreationObjectMessageCreation,
    #[doc = "Always `message_creation`."]
    pub type_: RunStepDetailsMessageCreationObjectType,
}
pub struct RunStepDetailsMessageCreationObjectMessageCreation {
    #[doc = "The ID of the message that was created by this run step."]
    pub message_id: String,
}
#[doc = "Always `message_creation`."]
pub enum RunStepDetailsMessageCreationObjectType {
    MessageCreation,
}
#[doc = "Details of the Code Interpreter tool call the run step was involved in."]
pub struct RunStepDetailsToolCallsCodeObject {
    #[doc = "The Code Interpreter tool call definition."]
    pub code_interpreter: RunStepDetailsToolCallsCodeObjectCodeInterpreter,
    #[doc = "The ID of the tool call."]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
    pub type_: RunStepDetailsToolCallsCodeObjectType,
}
#[doc = "The Code Interpreter tool call definition."]
pub struct RunStepDetailsToolCallsCodeObjectCodeInterpreter {
    #[doc = "The input to the Code Interpreter tool call."]
    pub input: String,
    #[doc = "The outputs from the Code Interpreter tool call. Code Interpreter can output one or more items, including text (`logs`) or images (`image`). Each of these are represented by a different object type."]
    pub outputs: Vec<std::collections::BTreeMap<String, serde_json::Value>>,
}
#[doc = "The type of tool call. This is always going to be `code_interpreter` for this type of tool call."]
pub enum RunStepDetailsToolCallsCodeObjectType {
    CodeInterpreter,
}
pub struct RunStepDetailsToolCallsCodeOutputImageObject {
    pub image: RunStepDetailsToolCallsCodeOutputImageObjectImage,
    #[doc = "Always `image`."]
    pub type_: RunStepDetailsToolCallsCodeOutputImageObjectType,
}
pub struct RunStepDetailsToolCallsCodeOutputImageObjectImage {
    #[doc = "The [file](/docs/api-reference/files) ID of the image."]
    pub file_id: String,
}
#[doc = "Always `image`."]
pub enum RunStepDetailsToolCallsCodeOutputImageObjectType {
    Image,
}
#[doc = "Text output from the Code Interpreter tool call as part of a run step."]
pub struct RunStepDetailsToolCallsCodeOutputLogsObject {
    #[doc = "The text output from the Code Interpreter tool call."]
    pub logs: String,
    #[doc = "Always `logs`."]
    pub type_: RunStepDetailsToolCallsCodeOutputLogsObjectType,
}
#[doc = "Always `logs`."]
pub enum RunStepDetailsToolCallsCodeOutputLogsObjectType {
    Logs,
}
pub struct RunStepDetailsToolCallsFileSearchObject {
    #[doc = "For now, this is always going to be an empty object."]
    pub file_search: RunStepDetailsToolCallsFileSearchObjectFileSearch,
    #[doc = "The ID of the tool call object."]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
    pub type_: RunStepDetailsToolCallsFileSearchObjectType,
}
#[doc = "For now, this is always going to be an empty object."]
pub struct RunStepDetailsToolCallsFileSearchObjectFileSearch {
    pub ranking_options: RunStepDetailsToolCallsFileSearchRankingOptionsObject,
    #[doc = "The results of the file search."]
    pub results: Vec<RunStepDetailsToolCallsFileSearchResultObject>,
}
#[doc = "The type of tool call. This is always going to be `file_search` for this type of tool call."]
pub enum RunStepDetailsToolCallsFileSearchObjectType {
    FileSearch,
}
#[doc = "The ranking options for the file search."]
pub struct RunStepDetailsToolCallsFileSearchRankingOptionsObject {
    pub ranker: FileSearchRanker,
    #[doc = "The score threshold for the file search. All values must be a floating point number between 0 and 1."]
    pub score_threshold: f64,
}
#[doc = "A result instance of the file search."]
pub struct RunStepDetailsToolCallsFileSearchResultObject {
    #[doc = "The content of the result that was found. The content is only included if requested via the include query parameter."]
    pub content: Vec<RunStepDetailsToolCallsFileSearchResultObjectContentInner>,
    #[doc = "The ID of the file that result was found in."]
    pub file_id: String,
    #[doc = "The name of the file that result was found in."]
    pub file_name: String,
    #[doc = "The score of the result. All values must be a floating point number between 0 and 1."]
    pub score: f64,
}
#[doc = "The type of the content."]
pub enum RunStepDetailsToolCallsFileSearchResultObjectContentInnerType {
    Text,
}
pub struct RunStepDetailsToolCallsFileSearchResultObjectContentInner {
    #[doc = "The text content of the file."]
    pub text: String,
    #[doc = "The type of the content."]
    pub type_: RunStepDetailsToolCallsFileSearchResultObjectContentInnerType,
}
pub struct RunStepDetailsToolCallsFunctionObject {
    #[doc = "The definition of the function that was called."]
    pub function: RunStepDetailsToolCallsFunctionObjectFunction,
    #[doc = "The ID of the tool call object."]
    pub id: String,
    #[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
    pub type_: RunStepDetailsToolCallsFunctionObjectType,
}
#[doc = "The definition of the function that was called."]
pub struct RunStepDetailsToolCallsFunctionObjectFunction {
    #[doc = "The arguments passed to the function."]
    pub arguments: String,
    #[doc = "The name of the function."]
    pub name: String,
    #[doc = "The output of the function. This will be `null` if the outputs have not been [submitted](/docs/api-reference/runs/submitToolOutputs) yet."]
    pub output: String,
}
#[doc = "The type of tool call. This is always going to be `function` for this type of tool call."]
pub enum RunStepDetailsToolCallsFunctionObjectType {
    Function,
}
#[doc = "Details of the tool call."]
pub struct RunStepDetailsToolCallsObject {
    #[doc = "An array of tool calls the run step was involved in. These can be associated with one of three types of tools: `code_interpreter`, `file_search`, or `function`.\n"]
    pub tool_calls: Vec<RunStepDetailsToolCallsObjectToolCallsInner>,
    #[doc = "Always `tool_calls`."]
    pub type_: RunStepDetailsToolCallsObjectType,
}
#[allow(clippy::large_enum_variant)]
pub enum RunStepDetailsToolCallsObjectToolCallsInner {
    OneOf0(RunStepDetailsToolCallsCodeObject),
    OneOf1(RunStepDetailsToolCallsFileSearchObject),
    OneOf2(RunStepDetailsToolCallsFunctionObject),
}
#[doc = "Always `tool_calls`."]
pub enum RunStepDetailsToolCallsObjectType {
    ToolCalls,
}
#[doc = "Represents a step in execution of a run.\n"]
pub struct RunStepObject {
    #[doc = "The ID of the [assistant](/docs/api-reference/assistants) associated with the run step."]
    pub assistant_id: String,
    #[doc = "The Unix timestamp (in seconds) for when the run step was cancelled."]
    pub cancelled_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run step completed."]
    pub completed_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run step was created."]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run step expired. A step is considered expired if the parent run is expired."]
    pub expired_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the run step failed."]
    pub failed_at: u64,
    #[doc = "The identifier of the run step, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The last error associated with this run step. Will be `null` if there are no errors."]
    pub last_error: RunStepObjectLastError,
    pub metadata: Metadata,
    #[doc = "The object type, which is always `thread.run.step`."]
    pub object: RunStepObjectObject,
    #[doc = "The ID of the [run](/docs/api-reference/runs) that this run step is a part of."]
    pub run_id: String,
    #[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
    pub status: RunStepObjectStatus,
    #[doc = "The details of the run step."]
    pub step_details: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The ID of the [thread](/docs/api-reference/threads) that was run."]
    pub thread_id: String,
    #[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
    pub type_: RunStepObjectType,
    pub usage: RunStepCompletionUsage,
}
#[doc = "One of `server_error` or `rate_limit_exceeded`."]
pub enum RunStepObjectLastErrorCode {
    ServerError,
    RateLimitExceeded,
}
#[doc = "The last error associated with this run step. Will be `null` if there are no errors."]
pub struct RunStepObjectLastError {
    #[doc = "One of `server_error` or `rate_limit_exceeded`."]
    pub code: RunStepObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: String,
}
#[doc = "The object type, which is always `thread.run.step`."]
pub enum RunStepObjectObject {
    ThreadRunStep,
}
#[doc = "The status of the run step, which can be either `in_progress`, `cancelled`, `failed`, `completed`, or `expired`."]
pub enum RunStepObjectStatus {
    InProgress,
    Cancelled,
    Failed,
    Completed,
    Expired,
}
#[doc = "The type of run step, which can be either `message_creation` or `tool_calls`."]
pub enum RunStepObjectType {
    MessageCreation,
    ToolCalls,
}
#[allow(clippy::large_enum_variant)]
pub enum RunStepStreamEvent {
    OneOf0(RunStepStreamEvent0),
    OneOf1(RunStepStreamEvent1),
    OneOf2(RunStepStreamEvent2),
    OneOf3(RunStepStreamEvent3),
    OneOf4(RunStepStreamEvent4),
    OneOf5(RunStepStreamEvent5),
    OneOf6(RunStepStreamEvent6),
}
pub enum RunStepStreamEvent0Event {
    ThreadRunStepCreated,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is created."]
pub struct RunStepStreamEvent0 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent0Event,
}
pub enum RunStepStreamEvent1Event {
    ThreadRunStepInProgress,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) moves to an `in_progress` state."]
pub struct RunStepStreamEvent1 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent1Event,
}
pub enum RunStepStreamEvent2Event {
    ThreadRunStepDelta,
}
#[doc = "Occurs when parts of a [run step](/docs/api-reference/run-steps/step-object) are being streamed."]
pub struct RunStepStreamEvent2 {
    pub data: RunStepDeltaObject,
    pub event: RunStepStreamEvent2Event,
}
pub enum RunStepStreamEvent3Event {
    ThreadRunStepCompleted,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is completed."]
pub struct RunStepStreamEvent3 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent3Event,
}
pub enum RunStepStreamEvent4Event {
    ThreadRunStepFailed,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) fails."]
pub struct RunStepStreamEvent4 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent4Event,
}
pub enum RunStepStreamEvent5Event {
    ThreadRunStepCancelled,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) is cancelled."]
pub struct RunStepStreamEvent5 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent5Event,
}
pub enum RunStepStreamEvent6Event {
    ThreadRunStepExpired,
}
#[doc = "Occurs when a [run step](/docs/api-reference/run-steps/step-object) expires."]
pub struct RunStepStreamEvent6 {
    pub data: RunStepObject,
    pub event: RunStepStreamEvent6Event,
}
#[allow(clippy::large_enum_variant)]
pub enum RunStreamEvent {
    OneOf0(RunStreamEvent0),
    OneOf1(RunStreamEvent1),
    OneOf2(RunStreamEvent2),
    OneOf3(RunStreamEvent3),
    OneOf4(RunStreamEvent4),
    OneOf5(RunStreamEvent5),
    OneOf6(RunStreamEvent6),
    OneOf7(RunStreamEvent7),
    OneOf8(RunStreamEvent8),
    OneOf9(RunStreamEvent9),
}
pub enum RunStreamEvent0Event {
    ThreadRunCreated,
}
#[doc = "Occurs when a new [run](/docs/api-reference/runs/object) is created."]
pub struct RunStreamEvent0 {
    pub data: RunObject,
    pub event: RunStreamEvent0Event,
}
pub enum RunStreamEvent1Event {
    ThreadRunQueued,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `queued` status."]
pub struct RunStreamEvent1 {
    pub data: RunObject,
    pub event: RunStreamEvent1Event,
}
pub enum RunStreamEvent2Event {
    ThreadRunInProgress,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to an `in_progress` status."]
pub struct RunStreamEvent2 {
    pub data: RunObject,
    pub event: RunStreamEvent2Event,
}
pub enum RunStreamEvent3Event {
    ThreadRunRequiresAction,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `requires_action` status."]
pub struct RunStreamEvent3 {
    pub data: RunObject,
    pub event: RunStreamEvent3Event,
}
pub enum RunStreamEvent4Event {
    ThreadRunCompleted,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) is completed."]
pub struct RunStreamEvent4 {
    pub data: RunObject,
    pub event: RunStreamEvent4Event,
}
pub enum RunStreamEvent5Event {
    ThreadRunIncomplete,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) ends with status `incomplete`."]
pub struct RunStreamEvent5 {
    pub data: RunObject,
    pub event: RunStreamEvent5Event,
}
pub enum RunStreamEvent6Event {
    ThreadRunFailed,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) fails."]
pub struct RunStreamEvent6 {
    pub data: RunObject,
    pub event: RunStreamEvent6Event,
}
pub enum RunStreamEvent7Event {
    ThreadRunCancelling,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) moves to a `cancelling` status."]
pub struct RunStreamEvent7 {
    pub data: RunObject,
    pub event: RunStreamEvent7Event,
}
pub enum RunStreamEvent8Event {
    ThreadRunCancelled,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) is cancelled."]
pub struct RunStreamEvent8 {
    pub data: RunObject,
    pub event: RunStreamEvent8Event,
}
pub enum RunStreamEvent9Event {
    ThreadRunExpired,
}
#[doc = "Occurs when a [run](/docs/api-reference/runs/object) expires."]
pub struct RunStreamEvent9 {
    pub data: RunObject,
    pub event: RunStreamEvent9Event,
}
#[doc = "Tool call objects"]
pub struct RunToolCallObject {
    #[doc = "The function definition."]
    pub function: RunToolCallObjectFunction,
    #[doc = "The ID of the tool call. This ID must be referenced when you submit the tool outputs in using the [Submit tool outputs to run](/docs/api-reference/runs/submitToolOutputs) endpoint."]
    pub id: String,
    #[doc = "The type of tool call the output is required for. For now, this is always `function`."]
    pub type_: RunToolCallObjectType,
}
#[doc = "The function definition."]
pub struct RunToolCallObjectFunction {
    #[doc = "The arguments that the model expects you to pass to the function."]
    pub arguments: String,
    #[doc = "The name of the function."]
    pub name: String,
}
#[doc = "The type of tool call the output is required for. For now, this is always `function`."]
pub enum RunToolCallObjectType {
    Function,
}
#[doc = "A screenshot action.\n"]
pub struct Screenshot {
    #[doc = "Specifies the event type. For a screenshot action, this property is \nalways set to `screenshot`.\n"]
    pub type_: ScreenshotType,
}
#[doc = "Specifies the event type. For a screenshot action, this property is \nalways set to `screenshot`.\n"]
pub enum ScreenshotType {
    Screenshot,
}
#[doc = "A scroll action.\n"]
pub struct Scroll {
    #[doc = "The horizontal scroll distance.\n"]
    pub scroll_x: u64,
    #[doc = "The vertical scroll distance.\n"]
    pub scroll_y: u64,
    #[doc = "Specifies the event type. For a scroll action, this property is \nalways set to `scroll`.\n"]
    pub type_: ScrollType,
    #[doc = "The x-coordinate where the scroll occurred.\n"]
    pub x: u64,
    #[doc = "The y-coordinate where the scroll occurred.\n"]
    pub y: u64,
}
#[doc = "Specifies the event type. For a scroll action, this property is \nalways set to `scroll`.\n"]
pub enum ScrollType {
    Scroll,
}
#[doc = "Specifies the latency tier to use for processing the request. This parameter is relevant for customers subscribed to the scale tier service:\n  - If set to 'auto', and the Project is Scale tier enabled, the system\n    will utilize scale tier credits until they are exhausted.\n  - If set to 'auto', and the Project is not Scale tier enabled, the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.\n  - If set to 'default', the request will be processed using the default service tier with a lower uptime SLA and no latency guarentee.\n  - If set to 'flex', the request will be processed with the Flex Processing service tier. [Learn more](/docs/guides/flex-processing).\n  - When not set, the default behavior is 'auto'.\n\n  When this parameter is set, the response body will include the `service_tier` utilized.\n"]
pub enum ServiceTier {
    Auto,
    Default,
    Flex,
}
pub struct StaticChunkingStrategy {
    #[doc = "The number of tokens that overlap between chunks. The default value is `400`.\n\nNote that the overlap must not exceed half of `max_chunk_size_tokens`.\n"]
    pub chunk_overlap_tokens: u64,
    #[doc = "The maximum number of tokens in each chunk. The default value is `800`. The minimum value is `100` and the maximum value is `4096`."]
    pub max_chunk_size_tokens: u64,
}
#[doc = "Customize your own chunking strategy by setting chunk size and chunk overlap."]
pub struct StaticChunkingStrategyRequestParam {
    pub static_: StaticChunkingStrategy,
    #[doc = "Always `static`."]
    pub type_: StaticChunkingStrategyRequestParamType,
}
#[doc = "Always `static`."]
pub enum StaticChunkingStrategyRequestParamType {
    Static,
}
pub struct StaticChunkingStrategyResponseParam {
    pub static_: StaticChunkingStrategy,
    #[doc = "Always `static`."]
    pub type_: StaticChunkingStrategyResponseParamType,
}
#[doc = "Always `static`."]
pub enum StaticChunkingStrategyResponseParamType {
    Static,
}
#[doc = "Not supported with latest reasoning models `o3` and `o4-mini`.\n\nUp to 4 sequences where the API will stop generating further tokens. The\nreturned text will not contain the stop sequence.\n"]
#[allow(clippy::large_enum_variant)]
pub enum StopConfiguration {
    OneOf0(String),
    OneOf1(Vec<String>),
}
pub struct SubmitToolOutputsRunRequest {
    #[doc = "If `true`, returns a stream of events that happen during the Run as server-sent events, terminating when the Run enters a terminal state with a `data: [DONE]` message.\n"]
    pub stream: Vec<bool>,
    #[doc = "A list of tools for which the outputs are being submitted."]
    pub tool_outputs: Vec<SubmitToolOutputsRunRequestToolOutputsInner>,
}
pub struct SubmitToolOutputsRunRequestToolOutputsInner {
    #[doc = "The output of the tool call to be submitted to continue the run."]
    pub output: String,
    #[doc = "The ID of the tool call in the `required_action` object within the run object the output is being submitted for."]
    pub tool_call_id: String,
}
#[doc = "An object specifying the format that the model must output.\n\nConfiguring `{ \"type\": \"json_schema\" }` enables Structured Outputs, \nwhich ensures the model will match your supplied JSON schema. Learn more in the \n[Structured Outputs guide](/docs/guides/structured-outputs).\n\nThe default format is `{ \"type\": \"text\" }` with no additional options.\n\n**Not recommended for gpt-4o and newer models:**\n\nSetting to `{ \"type\": \"json_object\" }` enables the older JSON mode, which\nensures the message the model generates is valid JSON. Using `json_schema`\nis preferred for models that support it.\n"]
#[allow(clippy::large_enum_variant)]
pub enum TextResponseFormatConfiguration {
    OneOf0(ResponseFormatText),
    OneOf1(TextResponseFormatJsonSchema),
    OneOf2(ResponseFormatJsonObject),
}
#[doc = "JSON Schema response format. Used to generate structured JSON responses.\nLearn more about [Structured Outputs](/docs/guides/structured-outputs).\n"]
pub struct TextResponseFormatJsonSchema {
    #[doc = "A description of what the response format is for, used by the model to\ndetermine how to respond in the format.\n"]
    pub description: String,
    #[doc = "The name of the response format. Must be a-z, A-Z, 0-9, or contain\nunderscores and dashes, with a maximum length of 64.\n"]
    pub name: String,
    pub schema: ResponseFormatJsonSchemaSchema,
    #[doc = "Whether to enable strict schema adherence when generating the output.\nIf set to true, the model will always follow the exact schema defined\nin the `schema` field. Only a subset of JSON Schema is supported when\n`strict` is `true`. To learn more, read the [Structured Outputs\nguide](/docs/guides/structured-outputs).\n"]
    pub strict: Vec<bool>,
    #[doc = "The type of response format being defined. Always `json_schema`."]
    pub type_: TextResponseFormatJsonSchemaType,
}
#[doc = "The type of response format being defined. Always `json_schema`."]
pub enum TextResponseFormatJsonSchemaType {
    JsonSchema,
}
#[doc = "Represents a thread that contains [messages](/docs/api-reference/messages)."]
pub struct ThreadObject {
    #[doc = "The Unix timestamp (in seconds) for when the thread was created."]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    pub metadata: Metadata,
    #[doc = "The object type, which is always `thread`."]
    pub object: ThreadObjectObject,
    #[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
    pub tool_resources: ThreadObjectToolResources,
}
#[doc = "The object type, which is always `thread`."]
pub enum ThreadObjectObject {
    Thread,
}
pub struct ThreadObjectToolResourcesCodeInterpreter {
    #[doc = "A list of [file](/docs/api-reference/files) IDs made available to the `code_interpreter` tool. There can be a maximum of 20 files associated with the tool.\n"]
    pub file_ids: Vec<String>,
}
pub struct ThreadObjectToolResourcesFileSearch {
    #[doc = "The [vector store](/docs/api-reference/vector-stores/object) attached to this thread. There can be a maximum of 1 vector store attached to the thread.\n"]
    pub vector_store_ids: Vec<String>,
}
#[doc = "A set of resources that are made available to the assistant's tools in this thread. The resources are specific to the type of tool. For example, the `code_interpreter` tool requires a list of file IDs, while the `file_search` tool requires a list of vector store IDs.\n"]
pub struct ThreadObjectToolResources {
    pub code_interpreter: ThreadObjectToolResourcesCodeInterpreter,
    pub file_search: ThreadObjectToolResourcesFileSearch,
}
#[allow(clippy::large_enum_variant)]
pub enum ThreadStreamEvent {
    OneOf0(ThreadStreamEvent0),
}
pub enum ThreadStreamEvent0Event {
    ThreadCreated,
}
#[doc = "Occurs when a new [thread](/docs/api-reference/threads/object) is created."]
pub struct ThreadStreamEvent0 {
    pub data: ThreadObject,
    #[doc = "Whether to enable input audio transcription."]
    pub enabled: Vec<bool>,
    pub event: ThreadStreamEvent0Event,
}
pub struct ToggleCertificatesRequest {
    pub certificate_ids: Vec<String>,
}
#[allow(clippy::large_enum_variant)]
pub enum Tool {
    OneOf0(FileSearchTool),
    OneOf1(FunctionTool),
    OneOf2(WebSearchPreviewTool),
    OneOf3(ComputerUsePreviewTool),
}
#[doc = "Use this option to force the model to call a specific function.\n"]
pub struct ToolChoiceFunction {
    #[doc = "The name of the function to call."]
    pub name: String,
    #[doc = "For function calling, the type is always `function`."]
    pub type_: ToolChoiceFunctionType,
}
#[doc = "For function calling, the type is always `function`."]
pub enum ToolChoiceFunctionType {
    Function,
}
#[doc = "Controls which (if any) tool is called by the model.\n\n`none` means the model will not call any tool and instead generates a message.\n\n`auto` means the model can pick between generating a message or calling one or\nmore tools.\n\n`required` means the model must call one or more tools.\n"]
pub enum ToolChoiceOptions {
    None,
    Auto,
    Required,
}
#[doc = "Indicates that the model should use a built-in tool to generate a response.\n[Learn more about built-in tools](/docs/guides/tools).\n"]
pub struct ToolChoiceTypes {
    #[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer_use_preview`\n"]
    pub type_: ToolChoiceTypesType,
}
#[doc = "The type of hosted tool the model should to use. Learn more about\n[built-in tools](/docs/guides/tools).\n\nAllowed values are:\n- `file_search`\n- `web_search_preview`\n- `computer_use_preview`\n"]
pub enum ToolChoiceTypesType {
    FileSearch,
    WebSearchPreview,
    ComputerUsePreview,
    WebSearchPreview20250311,
}
#[doc = "Emitted when there is an additional text delta. This is also the first event emitted when the transcription starts. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`."]
pub struct TranscriptTextDeltaEvent {
    #[doc = "The text delta that was additionally transcribed.\n"]
    pub delta: String,
    #[doc = "The log probabilities of the delta. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.\n"]
    pub logprobs: Vec<TranscriptTextDeltaEventLogprobsInner>,
    #[doc = "The type of the event. Always `transcript.text.delta`.\n"]
    pub type_: TranscriptTextDeltaEventType,
}
pub struct TranscriptTextDeltaEventLogprobsInner {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    pub bytes: Vec<serde_json::Value>,
    #[doc = "The log probability of the token.\n"]
    pub logprob: f64,
    #[doc = "The token that was used to generate the log probability.\n"]
    pub token: String,
}
#[doc = "The type of the event. Always `transcript.text.delta`.\n"]
pub enum TranscriptTextDeltaEventType {
    TranscriptTextDelta,
}
#[doc = "Emitted when the transcription is complete. Contains the complete transcription text. Only emitted when you [create a transcription](/docs/api-reference/audio/create-transcription) with the `Stream` parameter set to `true`."]
pub struct TranscriptTextDoneEvent {
    #[doc = "The log probabilities of the individual tokens in the transcription. Only included if you [create a transcription](/docs/api-reference/audio/create-transcription) with the `include[]` parameter set to `logprobs`.\n"]
    pub logprobs: Vec<TranscriptTextDoneEventLogprobsInner>,
    #[doc = "The text that was transcribed.\n"]
    pub text: String,
    #[doc = "The type of the event. Always `transcript.text.done`.\n"]
    pub type_: TranscriptTextDoneEventType,
}
pub struct TranscriptTextDoneEventLogprobsInner {
    #[doc = "The bytes that were used to generate the log probability.\n"]
    pub bytes: Vec<serde_json::Value>,
    #[doc = "The log probability of the token.\n"]
    pub logprob: f64,
    #[doc = "The token that was used to generate the log probability.\n"]
    pub token: String,
}
#[doc = "The type of the event. Always `transcript.text.done`.\n"]
pub enum TranscriptTextDoneEventType {
    TranscriptTextDone,
}
pub enum TranscriptionInclude {
    Logprobs,
}
pub struct TranscriptionSegment {
    #[doc = "Average logprob of the segment. If the value is lower than -1, consider the logprobs failed."]
    pub avg_logprob: f64,
    #[doc = "Compression ratio of the segment. If the value is greater than 2.4, consider the compression failed."]
    pub compression_ratio: f64,
    #[doc = "End time of the segment in seconds."]
    pub end: f64,
    #[doc = "Unique identifier of the segment."]
    pub id: u64,
    #[doc = "Probability of no speech in the segment. If the value is higher than 1.0 and the `avg_logprob` is below -1, consider this segment silent."]
    pub no_speech_prob: f64,
    #[doc = "Seek offset of the segment."]
    pub seek: u64,
    #[doc = "Start time of the segment in seconds."]
    pub start: f64,
    #[doc = "Temperature parameter used for generating the segment."]
    pub temperature: f64,
    #[doc = "Text content of the segment."]
    pub text: String,
    #[doc = "Array of token IDs for the text content."]
    pub tokens: Vec<u64>,
}
pub struct TranscriptionWord {
    #[doc = "End time of the word in seconds."]
    pub end: f64,
    #[doc = "Start time of the word in seconds."]
    pub start: f64,
    #[doc = "The text content of the word."]
    pub word: String,
}
#[doc = "Controls for how a thread will be truncated prior to the run. Use this to control the intial context window of the run."]
pub struct TruncationObject {
    #[doc = "The number of most recent messages from the thread when constructing the context for the run."]
    pub last_messages: u64,
    #[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
    pub type_: TruncationObjectType,
}
#[doc = "The truncation strategy to use for the thread. The default is `auto`. If set to `last_messages`, the thread will be truncated to the n most recent messages in the thread. When set to `auto`, messages in the middle of the thread will be dropped to fit the context length of the model, `max_prompt_tokens`."]
pub enum TruncationObjectType {
    Auto,
    LastMessages,
}
#[doc = "An action to type in text.\n"]
pub struct Type {
    #[doc = "The text to type.\n"]
    pub text: String,
    #[doc = "Specifies the event type. For a type action, this property is \nalways set to `type`.\n"]
    pub type_: TypeType,
}
#[doc = "Specifies the event type. For a type action, this property is \nalways set to `type`.\n"]
pub enum TypeType {
    Type,
}
pub struct UpdateVectorStoreFileAttributesRequest {
    pub attributes: VectorStoreFileAttributes,
}
pub struct UpdateVectorStoreRequest {
    pub expires_after: UpdateVectorStoreRequestExpiresAfter,
    pub metadata: Metadata,
    #[doc = "The name of the vector store."]
    pub name: String,
}
pub struct UpdateVectorStoreRequestExpiresAfter {
    pub all_of_0: VectorStoreExpirationAfter,
    pub all_of_1: serde_json::Value,
}
#[doc = "The Upload object can accept byte chunks in the form of Parts.\n"]
pub struct Upload {
    #[doc = "The intended number of bytes to be uploaded."]
    pub bytes: u64,
    #[doc = "The Unix timestamp (in seconds) for when the Upload was created."]
    pub created_at: u64,
    #[doc = "The Unix timestamp (in seconds) for when the Upload will expire."]
    pub expires_at: u64,
    pub file: UploadFile,
    #[doc = "The name of the file to be uploaded."]
    pub filename: String,
    #[doc = "The Upload unique identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always \"upload\"."]
    pub object: UploadObject,
    #[doc = "The intended purpose of the file. [Please refer here](/docs/api-reference/files/object#files/object-purpose) for acceptable values."]
    pub purpose: String,
    #[doc = "The status of the Upload."]
    pub status: UploadStatus,
}
pub struct UploadFile {
    pub all_of_0: OpenAiFile,
    pub all_of_1: serde_json::Value,
}
#[doc = "The object type, which is always \"upload\"."]
pub enum UploadObject {
    Upload,
}
#[doc = "The status of the Upload."]
pub enum UploadStatus {
    Pending,
    Completed,
    Cancelled,
    Expired,
}
pub struct UploadCertificateRequest {
    #[doc = "The certificate content in PEM format"]
    pub content: String,
    #[doc = "An optional name for the certificate"]
    pub name: String,
}
#[doc = "The upload Part represents a chunk of bytes we can add to an Upload object.\n"]
pub struct UploadPart {
    #[doc = "The Unix timestamp (in seconds) for when the Part was created."]
    pub created_at: u64,
    #[doc = "The upload Part unique identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always `upload.part`."]
    pub object: UploadPartObject,
    #[doc = "The ID of the Upload object that this Part was added to."]
    pub upload_id: String,
}
#[doc = "The object type, which is always `upload.part`."]
pub enum UploadPartObject {
    UploadPart,
}
#[doc = "A citation for a web resource used to generate a model response."]
pub struct UrlCitationBody {
    #[doc = "The index of the last character of the URL citation in the message."]
    pub end_index: u64,
    #[doc = "The index of the first character of the URL citation in the message."]
    pub start_index: u64,
    #[doc = "The title of the web resource."]
    pub title: String,
    #[doc = "The type of the URL citation. Always `url_citation`."]
    pub type_: UrlCitationBodyType,
    #[doc = "The URL of the web resource."]
    pub url: String,
}
#[doc = "The type of the URL citation. Always `url_citation`."]
pub enum UrlCitationBodyType {
    UrlCitation,
}
#[doc = "The aggregated audio speeches usage details of the specific time bucket."]
pub struct UsageAudioSpeechesResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "The number of characters processed."]
    pub characters: u64,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageAudioSpeechesResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageAudioSpeechesResultObject {
    OrganizationUsageAudioSpeechesResult,
}
#[doc = "The aggregated audio transcriptions usage details of the specific time bucket."]
pub struct UsageAudioTranscriptionsResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageAudioTranscriptionsResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "The number of seconds processed."]
    pub seconds: u64,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageAudioTranscriptionsResultObject {
    OrganizationUsageAudioTranscriptionsResult,
}
#[doc = "The aggregated code interpreter sessions usage details of the specific time bucket."]
pub struct UsageCodeInterpreterSessionsResult {
    #[doc = "The number of code interpreter sessions."]
    pub num_sessions: u64,
    pub object: UsageCodeInterpreterSessionsResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
}
pub enum UsageCodeInterpreterSessionsResultObject {
    OrganizationUsageCodeInterpreterSessionsResult,
}
#[doc = "The aggregated completions usage details of the specific time bucket."]
pub struct UsageCompletionsResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "When `group_by=batch`, this field tells whether the grouped usage result is batch or not."]
    pub batch: Vec<bool>,
    #[doc = "The aggregated number of audio input tokens used, including cached tokens."]
    pub input_audio_tokens: u64,
    #[doc = "The aggregated number of text input tokens that has been cached from previous requests. For customers subscribe to scale tier, this includes scale tier tokens."]
    pub input_cached_tokens: u64,
    #[doc = "The aggregated number of text input tokens used, including cached tokens. For customers subscribe to scale tier, this includes scale tier tokens."]
    pub input_tokens: u64,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageCompletionsResultObject,
    #[doc = "The aggregated number of audio output tokens used."]
    pub output_audio_tokens: u64,
    #[doc = "The aggregated number of text output tokens used. For customers subscribe to scale tier, this includes scale tier tokens."]
    pub output_tokens: u64,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageCompletionsResultObject {
    OrganizationUsageCompletionsResult,
}
#[doc = "The aggregated embeddings usage details of the specific time bucket."]
pub struct UsageEmbeddingsResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "The aggregated number of input tokens used."]
    pub input_tokens: u64,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageEmbeddingsResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageEmbeddingsResultObject {
    OrganizationUsageEmbeddingsResult,
}
#[doc = "The aggregated images usage details of the specific time bucket."]
pub struct UsageImagesResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "The number of images processed."]
    pub images: u64,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageImagesResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "When `group_by=size`, this field provides the image size of the grouped usage result."]
    pub size: String,
    #[doc = "When `group_by=source`, this field provides the source of the grouped usage result, possible values are `image.generation`, `image.edit`, `image.variation`."]
    pub source: String,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageImagesResultObject {
    OrganizationUsageImagesResult,
}
#[doc = "The aggregated moderations usage details of the specific time bucket."]
pub struct UsageModerationsResult {
    #[doc = "When `group_by=api_key_id`, this field provides the API key ID of the grouped usage result."]
    pub api_key_id: String,
    #[doc = "The aggregated number of input tokens used."]
    pub input_tokens: u64,
    #[doc = "When `group_by=model`, this field provides the model name of the grouped usage result."]
    pub model: String,
    #[doc = "The count of requests made to the model."]
    pub num_model_requests: u64,
    pub object: UsageModerationsResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "When `group_by=user_id`, this field provides the user ID of the grouped usage result."]
    pub user_id: String,
}
pub enum UsageModerationsResultObject {
    OrganizationUsageModerationsResult,
}
pub struct UsageResponse {
    pub data: Vec<UsageTimeBucket>,
    pub has_more: Vec<bool>,
    pub next_page: String,
    pub object: UsageResponseObject,
}
pub enum UsageResponseObject {
    Page,
}
pub struct UsageTimeBucket {
    pub end_time: u64,
    pub object: UsageTimeBucketObject,
    pub result: Vec<UsageTimeBucketResultInner>,
    pub start_time: u64,
}
pub enum UsageTimeBucketObject {
    Bucket,
}
#[allow(clippy::large_enum_variant)]
pub enum UsageTimeBucketResultInner {
    OneOf0(UsageCompletionsResult),
    OneOf1(UsageEmbeddingsResult),
    OneOf2(UsageModerationsResult),
    OneOf3(UsageImagesResult),
    OneOf4(UsageAudioSpeechesResult),
    OneOf5(UsageAudioTranscriptionsResult),
    OneOf6(UsageVectorStoresResult),
    OneOf7(UsageCodeInterpreterSessionsResult),
    OneOf8(CostsResult),
}
#[doc = "The aggregated vector stores usage details of the specific time bucket."]
pub struct UsageVectorStoresResult {
    pub object: UsageVectorStoresResultObject,
    #[doc = "When `group_by=project_id`, this field provides the project ID of the grouped usage result."]
    pub project_id: String,
    #[doc = "The vector stores usage in bytes."]
    pub usage_bytes: u64,
}
pub enum UsageVectorStoresResultObject {
    OrganizationUsageVectorStoresResult,
}
#[doc = "Represents an individual `user` within an organization."]
pub struct User {
    #[doc = "The Unix timestamp (in seconds) of when the user was added."]
    pub added_at: u64,
    #[doc = "The email address of the user"]
    pub email: String,
    #[doc = "The identifier, which can be referenced in API endpoints"]
    pub id: String,
    #[doc = "The name of the user"]
    pub name: String,
    #[doc = "The object type, which is always `organization.user`"]
    pub object: UserObject,
    #[doc = "`owner` or `reader`"]
    pub role: UserRole,
}
#[doc = "The object type, which is always `organization.user`"]
pub enum UserObject {
    OrganizationUser,
}
#[doc = "`owner` or `reader`"]
pub enum UserRole {
    Owner,
    Reader,
}
pub struct UserDeleteResponse {
    pub deleted: Vec<bool>,
    pub id: String,
    pub object: UserDeleteResponseObject,
}
pub enum UserDeleteResponseObject {
    OrganizationUserDeleted,
}
pub struct UserListResponse {
    pub data: Vec<User>,
    pub first_id: String,
    pub has_more: Vec<bool>,
    pub last_id: String,
    pub object: UserListResponseObject,
}
pub enum UserListResponseObject {
    List,
}
pub struct UserRoleUpdateRequest {
    #[doc = "`owner` or `reader`"]
    pub role: UserRoleUpdateRequestRole,
}
#[doc = "`owner` or `reader`"]
pub enum UserRoleUpdateRequestRole {
    Owner,
    Reader,
}
#[doc = "The expiration policy for a vector store."]
pub struct VectorStoreExpirationAfter {
    #[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
    pub anchor: VectorStoreExpirationAfterAnchor,
    #[doc = "The number of days after the anchor time that the vector store will expire."]
    pub days: u64,
}
#[doc = "Anchor timestamp after which the expiration policy applies. Supported anchors: `last_active_at`."]
pub enum VectorStoreExpirationAfterAnchor {
    LastActiveAt,
}
#[doc = "Set of 16 key-value pairs that can be attached to an object. This can be \nuseful for storing additional information about the object in a structured \nformat, and querying for objects via API or the dashboard. Keys are strings \nwith a maximum length of 64 characters. Values are strings with a maximum \nlength of 512 characters, booleans, or numbers.\n"]
pub type VectorStoreFileAttributes =
    std::collections::BTreeMap<String, VectorStoreFileAttributesInner>;
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreFileAttributesInner {
    OneOf0(String),
    OneOf1(f64),
    OneOf2(Vec<bool>),
}
#[doc = "A batch of files attached to a vector store."]
pub struct VectorStoreFileBatchObject {
    #[doc = "The Unix timestamp (in seconds) for when the vector store files batch was created."]
    pub created_at: u64,
    pub file_counts: VectorStoreFileBatchObjectFileCounts,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The object type, which is always `vector_store.file_batch`."]
    pub object: VectorStoreFileBatchObjectObject,
    #[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
    pub status: VectorStoreFileBatchObjectStatus,
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to."]
    pub vector_store_id: String,
}
pub struct VectorStoreFileBatchObjectFileCounts {
    #[doc = "The number of files that where cancelled."]
    pub cancelled: u64,
    #[doc = "The number of files that have been processed."]
    pub completed: u64,
    #[doc = "The number of files that have failed to process."]
    pub failed: u64,
    #[doc = "The number of files that are currently being processed."]
    pub in_progress: u64,
    #[doc = "The total number of files."]
    pub total: u64,
}
#[doc = "The object type, which is always `vector_store.file_batch`."]
pub enum VectorStoreFileBatchObjectObject {
    VectorStoreFilesBatch,
}
#[doc = "The status of the vector store files batch, which can be either `in_progress`, `completed`, `cancelled` or `failed`."]
pub enum VectorStoreFileBatchObjectStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
}
#[doc = "Represents the parsed content of a vector store file."]
pub struct VectorStoreFileContentResponse {
    #[doc = "Parsed content of the file."]
    pub data: Vec<VectorStoreFileContentResponseDataInner>,
    #[doc = "Indicates if there are more content pages to fetch."]
    pub has_more: Vec<bool>,
    #[doc = "The token for the next page, if any."]
    pub next_page: String,
    #[doc = "The object type, which is always `vector_store.file_content.page`"]
    pub object: VectorStoreFileContentResponseObject,
}
pub struct VectorStoreFileContentResponseDataInner {
    #[doc = "The text content"]
    pub text: String,
    #[doc = "The content type (currently only `\"text\"`)"]
    pub type_: String,
}
#[doc = "The object type, which is always `vector_store.file_content.page`"]
pub enum VectorStoreFileContentResponseObject {
    VectorStoreFileContentPage,
}
#[doc = "A list of files attached to a vector store."]
pub struct VectorStoreFileObject {
    pub attributes: VectorStoreFileAttributes,
    #[doc = "The strategy used to chunk the file."]
    pub chunking_strategy: std::collections::BTreeMap<String, serde_json::Value>,
    #[doc = "The Unix timestamp (in seconds) for when the vector store file was created."]
    pub created_at: u64,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The last error associated with this vector store file. Will be `null` if there are no errors."]
    pub last_error: VectorStoreFileObjectLastError,
    #[doc = "The object type, which is always `vector_store.file`."]
    pub object: VectorStoreFileObjectObject,
    #[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
    pub status: VectorStoreFileObjectStatus,
    #[doc = "The total vector store usage in bytes. Note that this may be different from the original file size."]
    pub usage_bytes: u64,
    #[doc = "The ID of the [vector store](/docs/api-reference/vector-stores/object) that the [File](/docs/api-reference/files) is attached to."]
    pub vector_store_id: String,
}
#[doc = "One of `server_error` or `rate_limit_exceeded`."]
pub enum VectorStoreFileObjectLastErrorCode {
    ServerError,
    UnsupportedFile,
    InvalidFile,
}
#[doc = "The last error associated with this vector store file. Will be `null` if there are no errors."]
pub struct VectorStoreFileObjectLastError {
    #[doc = "One of `server_error` or `rate_limit_exceeded`."]
    pub code: VectorStoreFileObjectLastErrorCode,
    #[doc = "A human-readable description of the error."]
    pub message: String,
}
#[doc = "The object type, which is always `vector_store.file`."]
pub enum VectorStoreFileObjectObject {
    VectorStoreFile,
}
#[doc = "The status of the vector store file, which can be either `in_progress`, `completed`, `cancelled`, or `failed`. The status `completed` indicates that the vector store file is ready for use."]
pub enum VectorStoreFileObjectStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
}
#[doc = "A vector store is a collection of processed files can be used by the `file_search` tool."]
pub struct VectorStoreObject {
    #[doc = "The Unix timestamp (in seconds) for when the vector store was created."]
    pub created_at: u64,
    pub expires_after: VectorStoreExpirationAfter,
    #[doc = "The Unix timestamp (in seconds) for when the vector store will expire."]
    pub expires_at: u64,
    pub file_counts: VectorStoreObjectFileCounts,
    #[doc = "The identifier, which can be referenced in API endpoints."]
    pub id: String,
    #[doc = "The Unix timestamp (in seconds) for when the vector store was last active."]
    pub last_active_at: u64,
    pub metadata: Metadata,
    #[doc = "The name of the vector store."]
    pub name: String,
    #[doc = "The object type, which is always `vector_store`."]
    pub object: VectorStoreObjectObject,
    #[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
    pub status: VectorStoreObjectStatus,
    #[doc = "The total number of bytes used by the files in the vector store."]
    pub usage_bytes: u64,
}
pub struct VectorStoreObjectFileCounts {
    #[doc = "The number of files that were cancelled."]
    pub cancelled: u64,
    #[doc = "The number of files that have been successfully processed."]
    pub completed: u64,
    #[doc = "The number of files that have failed to process."]
    pub failed: u64,
    #[doc = "The number of files that are currently being processed."]
    pub in_progress: u64,
    #[doc = "The total number of files."]
    pub total: u64,
}
#[doc = "The object type, which is always `vector_store`."]
pub enum VectorStoreObjectObject {
    VectorStore,
}
#[doc = "The status of the vector store, which can be either `expired`, `in_progress`, or `completed`. A status of `completed` indicates that the vector store is ready for use."]
pub enum VectorStoreObjectStatus {
    Expired,
    InProgress,
    Completed,
}
pub struct VectorStoreSearchRequest {
    #[doc = "A filter to apply based on file attributes."]
    pub filters: VectorStoreSearchRequestFilters,
    #[doc = "The maximum number of results to return. This number should be between 1 and 50 inclusive."]
    pub max_num_results: u64,
    #[doc = "A query string for a search"]
    pub query: VectorStoreSearchRequestQuery,
    #[doc = "Ranking options for search."]
    pub ranking_options: VectorStoreSearchRequestRankingOptions,
    #[doc = "Whether to rewrite the natural language query for vector search."]
    pub rewrite_query: Vec<bool>,
}
#[doc = "A filter to apply based on file attributes."]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreSearchRequestFilters {
    OneOf0(ComparisonFilter),
    OneOf1(CompoundFilter),
}
#[doc = "A query string for a search"]
#[allow(clippy::large_enum_variant)]
pub enum VectorStoreSearchRequestQuery {
    OneOf0(String),
    OneOf1(Vec<String>),
}
pub enum VectorStoreSearchRequestRankingOptionsRanker {
    Auto,
    Default20241115,
}
#[doc = "Ranking options for search."]
pub struct VectorStoreSearchRequestRankingOptions {
    pub ranker: VectorStoreSearchRequestRankingOptionsRanker,
    pub score_threshold: f64,
}
pub struct VectorStoreSearchResultContentObject {
    #[doc = "The text content returned from search."]
    pub text: String,
    #[doc = "The type of content."]
    pub type_: VectorStoreSearchResultContentObjectType,
}
#[doc = "The type of content."]
pub enum VectorStoreSearchResultContentObjectType {
    Text,
}
pub struct VectorStoreSearchResultItem {
    pub attributes: VectorStoreFileAttributes,
    #[doc = "Content chunks from the file."]
    pub content: Vec<VectorStoreSearchResultContentObject>,
    #[doc = "The ID of the vector store file."]
    pub file_id: String,
    #[doc = "The name of the vector store file."]
    pub filename: String,
    #[doc = "The similarity score for the result."]
    pub score: f64,
}
pub struct VectorStoreSearchResultsPage {
    #[doc = "The list of search result items."]
    pub data: Vec<VectorStoreSearchResultItem>,
    #[doc = "Indicates if there are more results to fetch."]
    pub has_more: Vec<bool>,
    #[doc = "The token for the next page, if any."]
    pub next_page: String,
    #[doc = "The object type, which is always `vector_store.search_results.page`"]
    pub object: VectorStoreSearchResultsPageObject,
    pub search_query: Vec<String>,
}
#[doc = "The object type, which is always `vector_store.search_results.page`"]
pub enum VectorStoreSearchResultsPageObject {
    VectorStoreSearchResultsPage,
}
#[allow(clippy::large_enum_variant)]
pub enum VoiceIdsShared {
    AnyOf0(String),
    AnyOf1(VoiceIdsShared1),
}
pub enum VoiceIdsShared1 {
    Alloy,
    Ash,
    Ballad,
    Coral,
    Echo,
    Fable,
    Onyx,
    Nova,
    Sage,
    Shimmer,
    Verse,
}
#[doc = "A wait action.\n"]
pub struct Wait {
    #[doc = "Specifies the event type. For a wait action, this property is \nalways set to `wait`.\n"]
    pub type_: WaitType,
}
#[doc = "Specifies the event type. For a wait action, this property is \nalways set to `wait`.\n"]
pub enum WaitType {
    Wait,
}
#[doc = "High level guidance for the amount of context window space to use for the \nsearch. One of `low`, `medium`, or `high`. `medium` is the default.\n"]
pub enum WebSearchContextSize {
    Low,
    Medium,
    High,
}
#[doc = "Approximate location parameters for the search."]
pub struct WebSearchLocation {
    #[doc = "Free text input for the city of the user, e.g. `San Francisco`.\n"]
    pub city: String,
    #[doc = "The two-letter \n[ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1) of the user,\ne.g. `US`.\n"]
    pub country: String,
    #[doc = "Free text input for the region of the user, e.g. `California`.\n"]
    pub region: String,
    #[doc = "The [IANA timezone](https://timeapi.io/documentation/iana-timezones) \nof the user, e.g. `America/Los_Angeles`.\n"]
    pub timezone: String,
}
#[doc = "This tool searches the web for relevant results to use in a response. Learn more about the [web search tool](https://platform.openai.com/docs/guides/tools-web-search)."]
pub struct WebSearchPreviewTool {
    #[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
    pub search_context_size: WebSearchPreviewToolSearchContextSize,
    #[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
    pub type_: WebSearchPreviewToolType,
    pub user_location: Vec<ApproximateLocation>,
}
#[doc = "High level guidance for the amount of context window space to use for the search. One of `low`, `medium`, or `high`. `medium` is the default."]
pub enum WebSearchPreviewToolSearchContextSize {
    Low,
    Medium,
    High,
}
#[doc = "The type of the web search tool. One of `web_search_preview` or `web_search_preview_2025_03_11`."]
pub enum WebSearchPreviewToolType {
    WebSearchPreview,
    WebSearchPreview20250311,
}
#[doc = "The results of a web search tool call. See the \n[web search guide](/docs/guides/tools-web-search) for more information.\n"]
pub struct WebSearchToolCall {
    #[doc = "The unique ID of the web search tool call.\n"]
    pub id: String,
    #[doc = "The status of the web search tool call.\n"]
    pub status: WebSearchToolCallStatus,
    #[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
    pub type_: WebSearchToolCallType,
}
#[doc = "The status of the web search tool call.\n"]
pub enum WebSearchToolCallStatus {
    InProgress,
    Searching,
    Completed,
    Failed,
}
#[doc = "The type of the web search tool call. Always `web_search_call`.\n"]
pub enum WebSearchToolCallType {
    WebSearchCall,
}
