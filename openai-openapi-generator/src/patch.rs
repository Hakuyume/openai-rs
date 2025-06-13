use crate::{openapi, visit};

#[openai_openapi_generator_macros::strict(openapi::Schema)]
pub fn patch(name: &str, schema: &mut openapi::Schema) {
    for (_, schema) in visit::iter_mut(schema) {
        patch(name, schema)
    }

    if let openapi::Schema {
        all_of: Some(all_of),
    } = schema
    {
        if let [
            openapi::Schema {
                ref_: ref_ @ Some(_),
            },
        ] = &mut all_of[..]
        {
            schema.ref_ = ref_.take();
            schema.all_of = None;
        }
    }

    if let openapi::Schema {
        all_of: Some(all_of),
    } = schema
    {
        if let [
            openapi::Schema {
                ref_: ref_ @ Some(_),
            },
            openapi::Schema {
                description,
                nullable: nullable @ Some(true),
            },
        ] = &mut all_of[..]
        {
            schema.ref_ = ref_.take();
            schema.description = description.take();
            schema.nullable = nullable.take();
            schema.all_of = None;
        }
    }

    if let openapi::Schema {
        any_of: Some(any_of),
    } = schema
    {
        if let [
            openapi::Schema {
                additional_properties,
                default,
                description,
                enum_,
                items,
                ref_,
                type_,
                x_stainless_const,
            },
            openapi::Schema {
                type_: Some(openapi::Type::Null),
            },
        ] = &mut any_of[..]
        {
            schema.additional_properties = additional_properties.take();
            schema.default = default.take();
            schema.description = description.take();
            schema.enum_ = enum_.take();
            schema.items = items.take();
            schema.nullable = Some(true);
            schema.ref_ = ref_.take();
            schema.type_ = type_.take();
            schema.x_stainless_const = x_stainless_const.take();
            schema.any_of = None;
        }
    }

    if let openapi::Schema {
        description: Some(description),
        enum_: None,
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
    } = schema
    {
        if let Some(value) = description
            .strip_prefix("The object type, which is always `")
            .and_then(|description| description.strip_suffix('`'))
        {
            schema.enum_ = Some(vec![value.to_owned()]);
        }
    }

    if let Some(items) = &mut schema.items {
        if let Some(required) = schema.required.take() {
            items.required = Some(required.clone());
        }
    }

    if let openapi::Schema {
        one_of: Some(one_of),
    } = schema
    {
        if let [
            openapi::Schema {
                description,
                properties,
                required,
                type_,
            },
        ] = &mut one_of[..]
        {
            schema.description = description.take();
            schema.properties = properties.take();
            schema.required = required.take();
            schema.type_ = type_.take();
            schema.one_of = None;
        }
    }

    if let Some(one_of) = &mut schema.one_of {
        if let Some(properties) = schema.properties.take() {
            for one_of in &mut *one_of {
                one_of.properties = Some(properties.clone());
            }
        }
        if let Some(type_) = schema.type_.take() {
            for one_of in &mut *one_of {
                one_of.type_ = Some(type_);
            }
        }
    }

    if schema.recursive_ref.as_deref() == Some("#") {
        schema.recursive_ref = None;
        schema.ref_ = Some(format!("#/components/schemas/{name}"));
    }
}
