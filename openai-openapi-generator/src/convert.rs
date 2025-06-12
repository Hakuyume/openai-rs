use crate::openapi;
use anyhow::Context;
use indexmap::IndexMap;

type Schemas<'a> = &'a IndexMap<String, openapi::Schema>;

pub fn convert<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<crate::Schema<'a>> {
    if let Some(schema) = convert_primitive(schema, schemas) {
        Ok(schema)
    } else if let Some(schema) = convert_array(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = convert_map(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = convert_ref(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = convert_enum(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = convert_struct(schema, schemas)? {
        Ok(schema)
    } else {
        Err(anyhow::format_err!("unhandled: {schema:#?}"))
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_primitive<'a>(schema: &'a openapi::Schema, _: Schemas<'a>) -> Option<crate::Schema<'a>> {
    if let openapi::Schema { description } = schema {
        Some(crate::Schema {
            description: description.as_deref(),
            nullable: false,
            type_: crate::Type::Any,
        })
    } else if let openapi::Schema {
        description,
        type_: Some(openapi::Type::Array),
    } = schema
    {
        Some(crate::Schema {
            description: description.as_deref(),
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
            description: description.as_deref(),
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
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Boolean,
        })
    } else if let openapi::Schema {
        default: _,
        description,
        format: None | Some(openapi::Format::Float),
        nullable,
        type_: Some(openapi::Type::Number),
    } = schema
    {
        Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
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
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Integer,
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
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Map(Box::new(crate::Schema {
                description: None,
                nullable: false,
                type_: crate::Type::Any,
            })),
        })
    } else if let Some((description, nullable)) = if let openapi::Schema {
        default: _,
        description,
        format: None | Some(openapi::Format::Uri),
        nullable,
        type_: Some(openapi::Type::String),
        x_stainless_const: _,
    } = schema
    {
        Some((description, nullable))
    } else if let openapi::Schema {
        any_of: Some(_),
        default: _,
        description,
        nullable,
        x_oai_type_label: Some(openapi::XOaiTypeLabel::String),
    } = schema
    {
        Some((description, nullable))
    } else if let openapi::Schema {
        any_of: Some(any_of),
        description,
        nullable,
    } = schema
    {
        if let [
            openapi::Schema {
                type_: Some(openapi::Type::String),
            },
            openapi::Schema {
                enum_: Some(_),
                type_: Some(openapi::Type::String),
            },
        ] = &any_of[..]
        {
            Some((description, nullable))
        } else {
            None
        }
    } else {
        None
    } {
        Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::String,
        })
    } else {
        None
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_array<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<Option<crate::Schema<'a>>> {
    if let openapi::Schema {
        default: _,
        description,
        items: Some(items),
        nullable,
        type_: None | Some(openapi::Type::Array),
    } = schema
    {
        Ok(Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Array(Box::new(convert(items, schemas).context("items")?)),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_map<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<Option<crate::Schema<'a>>> {
    if let openapi::Schema {
        additional_properties: Some(openapi::AdditionalProperties::Schema(additional_properties)),
        description,
        nullable,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        Ok(Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Array(Box::new(
                convert(additional_properties, schemas).context("additionalProperties")?,
            )),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_ref<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<Option<crate::Schema<'a>>> {
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
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Ref(ref_),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_enum<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<Option<crate::Schema<'a>>> {
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
            .map(|(i, of)| convert(of, schemas).with_context(|| format!("{context}[{i}]")))
            .collect::<Result<_, _>>()?;
        Ok(Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::EnumOf(variants),
        }))
    } else if let openapi::Schema {
        default,
        description,
        enum_: Some(enum_),
        nullable,
        type_: None | Some(openapi::Type::String),
        x_stainless_const,
    } = schema
    {
        let variants = enum_
            .iter()
            .enumerate()
            .map(|(i, enum_)| {
                (
                    enum_.as_str(),
                    if let Some(serde_json::Value::String(default)) = default {
                        enum_ == default
                    } else if *x_stainless_const == Some(true) {
                        i == 0
                    } else {
                        false
                    },
                )
            })
            .collect();
        Ok(Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::EnumString(variants),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn convert_struct<'a>(
    schema: &'a openapi::Schema,
    schemas: Schemas<'a>,
) -> anyhow::Result<Option<crate::Schema<'a>>> {
    if let Some((description, nullable, fields)) = if let openapi::Schema {
        all_of: Some(all_of),
        description,
        nullable,
        required: required_outer,
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
                            .fold(convert(property, schemas), |output, context| {
                                output.context(context)
                            })
                            .map(|schema| crate::Field::Property {
                                name: property_name,
                                schema,
                                required,
                            })
                    },
                    |ref_| {
                        anyhow::ensure!(schemas.contains_key(ref_));
                        Ok(crate::Field::Ref(ref_))
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(crate::Schema {
            description: description.as_deref(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Struct(fields),
        }))
    } else {
        Ok(None)
    }
}
