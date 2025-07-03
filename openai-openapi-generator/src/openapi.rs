use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Document {
    pub components: Components,
    pub paths: IndexMap<String, IndexMap<Method, Operation>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Method {
    Delete,
    Get,
    Post,
}

#[serde_with::serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub operation_id: String,
    pub parameters: Option<Vec<Parameter>>,
    pub request_body: Option<RequestBody>,
    #[serde_as(as = "IndexMap<serde_with::DisplayFromStr, _>")]
    pub responses: IndexMap<u16, Response>,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub content: IndexMap<ContentType, Content>,
    pub required: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub description: Option<String>,
    #[serde(rename = "in")]
    pub in_: In,
    pub name: String,
    pub required: Option<bool>,
    pub schema: Schema,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum In {
    Path,
    Query,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub content: Option<IndexMap<ContentType, Content>>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize)]
pub enum ContentType {
    #[serde(rename = "application/json")]
    ApplicationJson,
    #[serde(rename = "application/octet-stream")]
    ApplicationOoctetStream,
    #[serde(rename = "multipart/form-data")]
    MultipartFormData,
    #[serde(rename = "text/event-stream")]
    TextEventStream,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub schema: Schema,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Components {
    pub schemas: IndexMap<String, Schema>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub additional_properties: Option<AdditionalProperties>,
    pub all_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub default: Option<serde_json::Value>,
    pub description: Option<String>,
    pub discriminator: Option<Discriminator>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub format: Option<Format>,
    pub items: Option<Box<Schema>>,
    pub nullable: Option<bool>,
    pub one_of: Option<Vec<Schema>>,
    pub properties: Option<IndexMap<String, Schema>>,
    #[serde(rename = "$recursiveRef")]
    pub recursive_ref: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<Type>,
    #[serde(rename = "x-oaiTypeLabel")]
    pub x_oai_type_label: Option<XOaiTypeLabel>,
    #[serde(rename = "x-stainless-const")]
    pub x_stainless_const: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(bool),
    Schema(Box<Schema>),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Binary,
    Float,
    Int64,
    Uri,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum XOaiTypeLabel {
    File,
    Map,
    String,
}
