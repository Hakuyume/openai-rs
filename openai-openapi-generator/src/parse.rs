use crate::openapi;
use anyhow::Context;
use indexmap::IndexMap;
use std::iter;

pub fn parse(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<crate::Schema> {
    if let Some(schema) = parse_primitive(schema, schemas) {
        Ok(schema)
    } else if let Some(schema) = parse_array(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_map(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_ref(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_enum(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_struct(schema, schemas)? {
        Ok(schema)
    } else {
        Err(anyhow::format_err!("unhandled: {schema:#?}"))
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_primitive(
    schema: &openapi::Schema,
    _: &IndexMap<String, openapi::Schema>,
) -> Option<crate::Schema> {
    if let openapi::Schema { description } = schema {
        Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Any,
        })
    } else if let openapi::Schema {
        description,
        type_: Some(openapi::Type::Array),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Array(Box::new(crate::Schema {
                description: None,
                nullable: false,
                type_: crate::Type::Any,
            })),
        })
    } else if let openapi::Schema {
        description,
        format: Some(openapi::Format::Binary),
        type_: Some(openapi::Type::String),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::File),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Binary,
        })
    } else if let openapi::Schema {
        default: _,
        description,
        nullable,
        type_: Some(openapi::Type::Boolean),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Boolean,
        })
    } else if let openapi::Schema {
        enum_: Some(enum_),
        default: _,
        description,
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
    } = schema
        && let [value] = &enum_[..]
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Const(value.clone()),
        })
    } else if let openapi::Schema {
        description,
        format: Some(openapi::Format::Float),
        type_: Some(openapi::Type::Number),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Float,
        })
    } else if let openapi::Schema {
        default: _,
        description,
        enum_: _,
        format: None | Some(openapi::Format::Int64),
        nullable,
        type_: Some(openapi::Type::Integer),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Integer,
        })
    } else if let openapi::Schema {
        default: _,
        description,
        nullable,
        type_: Some(openapi::Type::Number),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Number,
        })
    } else if let openapi::Schema {
        additional_properties: None | Some(openapi::AdditionalProperties::Bool(true)),
        description,
        nullable,
        type_: Some(openapi::Type::Object),
    }
    | openapi::Schema {
        description,
        nullable,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Map(Box::new(crate::Schema {
                description: None,
                nullable: false,
                type_: crate::Type::Any,
            })),
        })
    } else if let openapi::Schema {
        default: _,
        description,
        format: None | Some(openapi::Format::Uri),
        nullable,
        type_: Some(openapi::Type::String),
    }
    | openapi::Schema {
        any_of: Some(_),
        default: _,
        description,
        nullable,
        x_oai_type_label: Some(openapi::XOaiTypeLabel::String),
    } = schema
    {
        Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::String,
        })
    } else {
        None
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_array(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        default: _,
        description,
        items: Some(items),
        nullable,
        type_: None | Some(openapi::Type::Array),
    } = schema
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Array(Box::new(parse(items, schemas).context("items")?)),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_map(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        additional_properties: Some(openapi::AdditionalProperties::Schema(additional_properties)),
        description,
        nullable,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Map(Box::new(
                parse(additional_properties, schemas).context("additionalProperties")?,
            )),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_ref(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        description,
        nullable,
        ref_: Some(ref_),
        type_: _,
    } = schema
        && let Some(ref_) = ref_.strip_prefix("#/components/schemas/")
    {
        anyhow::ensure!(schemas.contains_key(ref_));
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Ref(ref_.to_owned()),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_enum(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        any_of: Some(any_of),
        description,
        discriminator: _,
        nullable,
    } = schema
    {
        let variants = any_of
            .iter()
            .enumerate()
            .map(|(i, any_of)| {
                parse(any_of, schemas)
                    .with_context(|| format!("anyOf[{i}]"))
                    .map(|any_of| (any_of, false))
            })
            .collect::<Result<_, _>>()?;
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Enum(variants),
        }))
    } else if let openapi::Schema {
        default: _,
        description,
        discriminator: _,
        nullable,
        one_of: Some(one_of),
    } = schema
    {
        let variants = one_of
            .iter()
            .enumerate()
            .map(|(i, one_of)| {
                parse(one_of, schemas)
                    .with_context(|| format!("oneOf[{i}]"))
                    .map(|one_of| (one_of, false))
            })
            .collect::<Result<_, _>>()?;
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Enum(variants),
        }))
    } else if let openapi::Schema {
        default,
        description,
        enum_: Some(enum_),
        nullable,
        type_: None | Some(openapi::Type::String),
        x_stainless_const: _,
    } = schema
    {
        let variants = enum_
            .iter()
            .map(|enum_| {
                (
                    crate::Schema {
                        description: None,
                        nullable: false,
                        type_: crate::Type::Const(enum_.clone()),
                    },
                    if let Some(serde_json::Value::String(default)) = default {
                        enum_ == default
                    } else {
                        false
                    },
                )
            })
            .collect();
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Enum(variants),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_struct(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        all_of: Some(all_of),
        description,
        nullable,
        required,
        type_: None | Some(openapi::Type::Object),
    } = schema
        && let Some(fields) = all_of
            .iter()
            .enumerate()
            .flat_map(|(i, all_of)| {
                if let openapi::Schema {
                    properties: Some(properties),
                    required: required_inner,
                    type_: Some(openapi::Type::Object),
                } = all_of
                {
                    either::Left(properties.iter().map(move |(property_name, property)| {
                        Some(
                            parse(property, schemas)
                                .context(format!("properties[{property_name:?}]"))
                                .context(format!("allOf[{i}]"))
                                .map(|schema| crate::Field::Property {
                                    name: property_name.clone(),
                                    schema,
                                    required: required
                                        .as_ref()
                                        .is_some_and(|required| required.contains(property_name))
                                        || required_inner.as_ref().is_some_and(|required| {
                                            required.contains(property_name)
                                        }),
                                }),
                        )
                    }))
                } else if let openapi::Schema { ref_: Some(ref_) } = all_of
                    && let Some(ref_) = ref_.strip_prefix("#/components/schemas/")
                    && schemas.contains_key(ref_)
                {
                    //     .context("ref")
                    //     .context(format!("allOf[{i}]")))
                    either::Right(iter::once(Some(Ok(crate::Field::Ref(ref_.to_owned())))))
                } else {
                    either::Right(iter::once(None))
                }
            })
            .collect::<Option<Result<Vec<_>, _>>>()
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Struct(fields?),
        }))
    } else if let openapi::Schema {
        additional_properties: None | Some(openapi::AdditionalProperties::Bool(false)),
        description,
        nullable,
        properties: Some(properties),
        required,
        type_: None | Some(openapi::Type::Object),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        let fields = properties
            .iter()
            .map(|(property_name, property)| {
                parse(property, schemas)
                    .context(format!("properties[{property_name:?}]"))
                    .map(|schema| crate::Field::Property {
                        name: property_name.clone(),
                        schema,
                        required: required
                            .as_ref()
                            .is_some_and(|required| required.contains(property_name)),
                    })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Struct(fields),
        }))
    } else {
        Ok(None)
    }
}
