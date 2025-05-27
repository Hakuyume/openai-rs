use indexmap::IndexMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Document {
    pub components: Components,
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
    #[serde(rename = "x-oaiMeta")]
    pub x_oai_meta: Option<XOaiMeta>,
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

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    pub property_name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Format {
    Binary,
    Float,
    Int64,
    Uri,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum XOaiTypeLabel {
    File,
    Map,
    String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct XOaiMeta {
    pub example: Option<String>,
}
