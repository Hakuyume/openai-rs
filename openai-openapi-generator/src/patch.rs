use crate::{openapi, visit};

#[openai_openapi_generator_macros::strict(openapi::Schema)]
pub fn patch(name: Option<&str>, schema: &mut openapi::Schema) {
    for (_, schema) in visit::iter_mut(schema) {
        patch(name, schema)
    }

    if let openapi::Schema {
        default,
        description: Some(description),
        enum_,
        type_: None | Some(openapi::Type::String),
        x_stainless_const: None | Some(true),
    } = schema
        && let Some(caps) = lazy_regex::lazy_regex!(
            r#"(?x)
            (?:Always|always|must\ be|which\ is\ always)
            \ ['`]?([a-z0-9_]+(?:\.[a-z0-9_]+)*)['`]?
            \.?\s*$
            "#
        )
        .captures(description)
        && let (_, [value]) = caps.extract()
        && default.as_ref().is_none_or(|default| default == value)
        && enum_.as_ref().is_none_or(|enum_| enum_ == &[value])
    {
        schema.default = Some(serde_json::Value::String(value.to_owned()));
        schema.enum_ = Some(vec![value.to_owned()]);
        schema.type_ = Some(openapi::Type::String);
        schema.x_stainless_const = Some(true);
    }

    if let Some(name) = name
        && let openapi::Schema {
            recursive_ref: Some(recursive_ref),
        } = schema
        && recursive_ref == "#"
    {
        schema.recursive_ref = None;
        schema.ref_ = Some(format!("#/components/schemas/{name}"));
    }
}
