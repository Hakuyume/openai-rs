use crate::openapi;
use anyhow::Context;
use indexmap::IndexMap;

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
    } else if let Some((description, value)) = if let openapi::Schema {
        enum_: Some(enum_),
        default: _,
        description,
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
    } = schema
    {
        if let [value] = &enum_[..] {
            Some((description, value))
        } else {
            None
        }
    } else {
        None
    } {
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
    if let Some((description, nullable, ref_)) = if let openapi::Schema {
        description,
        nullable,
        ref_: Some(ref_),
        type_: _,
    } = schema
    {
        ref_.strip_prefix("#/components/schemas/")
            .map(|ref_| (description, nullable, ref_))
    } else {
        None
    } {
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
    if let Some((description, nullable, of, context)) = if let openapi::Schema {
        any_of: Some(any_of),
        description,
        discriminator: _,
        nullable,
    } = schema
    {
        Some((description, nullable, any_of, "anyOf"))
    } else if let openapi::Schema {
        default: _,
        description,
        discriminator: _,
        nullable,
        one_of: Some(one_of),
    } = schema
    {
        Some((description, nullable, one_of, "oneOf"))
    } else {
        None
    } {
        let variants = of
            .iter()
            .enumerate()
            .map(|(i, of)| {
                parse(of, schemas)
                    .with_context(|| format!("{context}[{i}]"))
                    .map(|of| (of, false))
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
    if let Some((description, nullable, fields)) = if let openapi::Schema {
        all_of: Some(all_of),
        description,
        nullable,
        required: required_outer,
        type_: None | Some(openapi::Type::Object),
    } = schema
    {
        all_of
            .iter()
            .enumerate()
            .try_fold(Vec::new(), |mut fields, (i, all_of)| {
                if let openapi::Schema {
                    properties: Some(properties),
                    required: required_inner,
                    type_: Some(openapi::Type::Object),
                } = all_of
                {
                    for (property_name, property) in properties {
                        fields.push((
                            either::Left((
                                property_name,
                                property,
                                required_outer
                                    .as_ref()
                                    .is_some_and(|required| required.contains(property_name))
                                    || required_inner
                                        .as_ref()
                                        .is_some_and(|required| required.contains(property_name)),
                            )),
                            vec![
                                format!("properties[{property_name:?}]"),
                                format!("allOf[{i}]"),
                            ],
                        ));
                    }
                    Some(fields)
                } else if let Some(ref_) = if let openapi::Schema { ref_: Some(ref_) } = all_of {
                    ref_.strip_prefix("#/components/schemas/")
                } else {
                    None
                } {
                    fields.push((
                        either::Right(ref_),
                        vec!["ref".to_owned(), format!("allOf[{i}]")],
                    ));
                    Some(fields)
                } else {
                    None
                }
            })
            .map(|fields| (description, nullable, fields))
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
                (
                    either::Left((
                        property_name,
                        property,
                        required
                            .as_ref()
                            .is_some_and(|required| required.contains(property_name)),
                    )),
                    vec![format!("properties[{property_name:?}]")],
                )
            })
            .collect();
        Some((description, nullable, fields))
    } else {
        None
    } {
        let fields = fields
            .into_iter()
            .map(|(field, contexts)| {
                field.either(
                    |(property_name, property, required)| {
                        contexts
                            .into_iter()
                            .fold(parse(property, schemas), |output, context| {
                                output.context(context)
                            })
                            .map(|schema| crate::Field::Property {
                                name: property_name.clone(),
                                schema,
                                required,
                            })
                    },
                    |ref_| {
                        anyhow::ensure!(schemas.contains_key(ref_));
                        Ok(crate::Field::Ref(ref_.to_owned()))
                    },
                )
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
