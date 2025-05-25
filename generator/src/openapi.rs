use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Deserialize)]
pub struct Document {
    pub components: Components,
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub schemas: BTreeMap<String, Schema>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub additional_properties: Option<AdditionalProperties>,
    pub all_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub description: Option<String>,
    #[serde(rename = "enum")]
    pub enum_: Option<Vec<String>>,
    pub format: Option<Format>,
    pub items: Option<Box<Schema>>,
    pub nullable: Option<bool>,
    pub one_of: Option<Vec<Schema>>,
    pub properties: Option<BTreeMap<String, Schema>>,
    #[serde(rename = "$recursiveRef")]
    pub recursive_ref: Option<String>,
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    pub required: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub type_: Option<Type>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Boolean(bool),
    Schema(Box<Schema>),
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
