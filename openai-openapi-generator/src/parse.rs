use crate::openapi;
use anyhow::Context;
use indexmap::IndexMap;
use std::iter;

pub fn parse(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<crate::Schema> {
    if let Some(schema) = parse_additional_properties(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_all_of(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_any_of(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_enum(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_items(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_one_of(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_properties(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_ref(schema, schemas)? {
        Ok(schema)
    } else if let Some(schema) = parse_other(schema, schemas) {
        Ok(schema)
    } else {
        Err(anyhow::format_err!("unhandled: {schema:#?}"))
    }
}

pub fn parse_operation(
    method: openapi::Method,
    path: &str,
    operation: &openapi::Operation,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<crate::Operation> {
    Ok(crate::Operation {
        description: operation.summary.clone(),
        id: operation.operation_id.clone(),
        method,
        parameters: operation
            .parameters
            .as_ref()
            .map(|parameters| {
                parameters
                    .iter()
                    .enumerate()
                    .map(|(i, parameter)| {
                        parse(&parameter.schema, schemas)
                            .map(|schema| crate::Parameter {
                                description: parameter.description.clone(),
                                in_: parameter.in_,
                                name: parameter.name.clone(),
                                required: parameter.required.unwrap_or_default(),
                                schema,
                            })
                            .context("schema")
                            .with_context(|| format!("parameters[{i}]"))
                    })
                    .collect()
            })
            .transpose()?,
        path: path.to_owned(),
        requests: operation
            .request_body
            .as_ref()
            .map(|request_body| {
                request_body
                    .content
                    .iter()
                    .map(|(content_type, content)| {
                        parse(&content.schema, schemas)
                            .map(|schema| {
                                (
                                    *content_type,
                                    schema,
                                    request_body.required.unwrap_or_default(),
                                )
                            })
                            .context("schema")
                            .with_context(|| format!("content[{content_type:?}]"))
                            .context("requestBody")
                    })
                    .collect()
            })
            .transpose()?,
        responses: operation
            .responses
            .iter()
            .flat_map(|(status, response)| {
                if let Some(content) = &response.content {
                    either::Left(content.iter().map(|(content_type, content)| {
                        parse(&content.schema, schemas)
                            .map(|schema| (*status, Some((*content_type, schema))))
                            .context("schema")
                            .with_context(|| format!("content[{content_type:?}]"))
                            .context("requestBody")
                    }))
                } else {
                    either::Right(iter::once(Ok((*status, None))))
                }
            })
            .collect::<Result<_, _>>()?,
    })
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_additional_properties(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        additional_properties:
            Some(openapi::AdditionalProperties::Schema(mut additional_properties)),
        description,
        nullable,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema.clone()
    {
        additional_properties.x_oai_type_label = None;
        let item = parse(&additional_properties, schemas).context("additionalProperties")?;
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Map(Box::new(item)),
        }))
    } else if let openapi::Schema {
        additional_properties: None | Some(openapi::AdditionalProperties::Bool(true)),
        description,
        nullable,
        type_: Some(openapi::Type::Object),
    } = schema
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Map(Box::new(crate::Schema {
                description: None,
                nullable: false,
                type_: crate::Type::Any,
            })),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_all_of(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        all_of: Some(all_of),
    } = schema
        && let [
            all_of,
            openapi::Schema {
                description,
                nullable: Some(true),
            },
        ] = &all_of[..]
    {
        let mut all_of = parse(all_of, schemas).context("allOf[0]")?;
        all_of.description = all_of.description.or_else(|| description.clone());
        all_of.nullable = true;
        Ok(Some(all_of))
    } else if let openapi::Schema {
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
                    required: _,
                    type_: Some(openapi::Type::Object),
                } = all_of
                {
                    either::Left(properties.iter().map(move |(property_name, property)| {
                        Some(
                            parse(property, schemas)
                                .context(format!("properties[{property_name:?}]"))
                                .context(format!("allOf[{i}]"))
                                .map(|schema| either::Left((property_name.clone(), schema))),
                        )
                    }))
                } else if let openapi::Schema { ref_: Some(ref_) } = all_of
                    && let Some(ref_) = ref_.strip_prefix("#/components/schemas/")
                    && schemas.contains_key(ref_)
                {
                    let f = || {
                        anyhow::ensure!(schemas.contains_key(ref_));
                        Ok(either::Right(ref_.to_owned()))
                    };
                    either::Right(iter::once(Some(f())))
                } else {
                    either::Right(iter::once(None))
                }
            })
            .collect::<Option<Result<Vec<_>, _>>>()
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Struct {
                fields: fields?,
                required: required
                    .iter()
                    .flatten()
                    .chain(
                        all_of
                            .iter()
                            .flat_map(|all_of| all_of.required.iter().flatten()),
                    )
                    .cloned()
                    .collect(),
            },
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_any_of(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        any_of: Some(any_of),
    } = schema
        && let [
            any_of,
            openapi::Schema {
                type_: Some(openapi::Type::Null),
            },
        ] = &any_of[..]
    {
        let mut any_of = parse(any_of, schemas).context("anyOf[0]")?;
        any_of.nullable = true;
        Ok(Some(any_of))
    } else if let openapi::Schema {
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
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_enum(
    schema: &openapi::Schema,
    _: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
        enum_: Some(enum_),
        default: _,
        description,
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
    } = schema
        && let [value] = &enum_[..]
    {
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: false,
            type_: crate::Type::Const(value.clone()),
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

fn parse_items(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    let mut schema = schema.clone();
    if let (_, description, Some(items), nullable, None | Some(openapi::Type::Array)) = (
        schema.default.take(),
        schema.description.take(),
        schema.items.take(),
        schema.nullable.take(),
        schema.type_.take(),
    ) {
        let items = or(&items, &schema);
        let item = parse(&items, schemas).context("items")?;
        Ok(Some(crate::Schema {
            description,
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Array(Box::new(item)),
        }))
    } else {
        Ok(None)
    }
}

fn parse_one_of(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    let mut schema = schema.clone();
    if let (_, description, _, nullable, Some(one_of)) = (
        schema.default.take(),
        schema.description.take(),
        schema.discriminator.take(),
        schema.nullable.take(),
        schema.one_of.take(),
    ) {
        let variants = one_of
            .into_iter()
            .enumerate()
            .map(|(i, one_of)| {
                let one_of = or(&one_of, &schema);
                parse(&one_of, schemas)
                    .with_context(|| format!("oneOf[{i}]"))
                    .map(|one_of| (one_of, false))
            })
            .collect::<Result<_, _>>()?;
        Ok(Some(crate::Schema {
            description,
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Enum(variants),
        }))
    } else {
        Ok(None)
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn parse_properties(
    schema: &openapi::Schema,
    schemas: &IndexMap<String, openapi::Schema>,
) -> anyhow::Result<Option<crate::Schema>> {
    if let openapi::Schema {
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
                    .map(|schema| either::Left((property_name.clone(), schema)))
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Some(crate::Schema {
            description: description.clone(),
            nullable: nullable.unwrap_or_default(),
            type_: crate::Type::Struct {
                fields,
                required: required.iter().flatten().cloned().collect(),
            },
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
fn parse_other(
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

fn or(a: &openapi::Schema, b: &openapi::Schema) -> openapi::Schema {
    fn or<T>(a: &Option<T>, b: &Option<T>) -> Option<T>
    where
        T: Clone,
    {
        a.as_ref().or(b.as_ref()).cloned()
    }

    openapi::Schema {
        additional_properties: or(&a.additional_properties, &b.additional_properties),
        all_of: or(&a.all_of, &b.all_of),
        any_of: or(&a.any_of, &b.any_of),
        default: or(&a.default, &b.default),
        description: or(&a.description, &b.description),
        discriminator: or(&a.discriminator, &b.discriminator),
        enum_: or(&a.enum_, &b.enum_),
        format: or(&a.format, &b.format),
        items: or(&a.items, &b.items),
        nullable: or(&a.nullable, &b.nullable),
        one_of: or(&a.one_of, &b.one_of),
        properties: or(&a.properties, &b.properties),
        recursive_ref: or(&a.recursive_ref, &b.recursive_ref),
        ref_: or(&a.ref_, &b.ref_),
        required: or(&a.required, &b.required),
        type_: or(&a.type_, &b.type_),
        x_oai_type_label: or(&a.x_oai_type_label, &b.x_oai_type_label),
        x_stainless_const: or(&a.x_stainless_const, &b.x_stainless_const),
    }
}
