mod openapi;

use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use quote::ToTokens;
use std::collections::BTreeMap;
use std::io;

fn main() -> anyhow::Result<()> {
    let document = serde_yaml::from_reader::<_, openapi::Document>(io::stdin().lock())?;

    let schemas = document
        .components
        .schemas
        .iter()
        .map(|(name, schema)| {
            Schema::try_from((name.as_str(), schema))
                .with_context(|| name.clone())
                .map(|schema| (name.clone(), schema))
        })
        .collect::<Result<BTreeMap<_, _>, _>>()?;

    for (name, schema) in &schemas {
        let mut inline = Vec::new();
        println!("{}", to_item(name, schema, &mut inline)?.to_token_stream());
        for (_, item) in inline {
            println!("{}", item.to_token_stream());
        }
    }
    Ok(())
}

fn to_ident_pascal(name: &str) -> syn::Ident {
    let name = name.replace(['-', '.', '[', ']'], "_");
    let name = name.to_pascal_case();
    if name.chars().next().is_some_and(char::is_alphabetic) {
        quote::format_ident!("{name}")
    } else {
        quote::format_ident!("_{name}")
    }
}

fn to_ident_snake(name: &str) -> syn::Ident {
    let name = name.replace(['-', '.', '[', ']'], "_");
    let name = name.to_snake_case();
    let name = match &*name {
        "static" => "static_",
        "type" => "type_",
        _ => &name,
    };
    if name.chars().next().is_some_and(char::is_alphabetic) {
        quote::format_ident!("{name}")
    } else {
        quote::format_ident!("_{name}")
    }
}

fn to_description(description: Option<&str>) -> Option<syn::Attribute> {
    description.map(|description| syn::parse_quote!(#[doc = #description]))
}

enum Node {
    Item { ident: syn::Ident, value: syn::Item },
    Type { value: syn::Type },
}

fn to_node(
    name: &str,
    schema: &Schema,
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<Node> {
    let derive = quote::quote!(#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]);

    match &schema.type_ {
        Type::AllOf(schemas) => {
            let ident = to_ident_pascal(name);
            let fields = schemas
                .iter()
                .enumerate()
                .map(|(i, schema)| {
                    let ident = to_ident_snake(&format!("allOf[{i}]"));
                    to_type(&format!("{name}[{i}]"), schema, inline).map(|type_| {
                        quote::quote! {
                            #[serde(flatten)]
                            pub #ident: #type_
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(schema.description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                pub struct #ident {
                    #(#fields),*
                }
            };
            Ok(Node::Item { value, ident })
        }
        Type::Any => {
            let value = syn::parse_quote!(serde_json::Value);
            Ok(Node::Type { value })
        }
        Type::AnyOf(any_of) => {
            if let [
                any_of,
                Schema {
                    type_: Type::Null, ..
                },
            ] = &any_of[..]
            {
                let type_ = to_type(&format!("{name}.inner"), any_of, inline)?;
                let value = if any_of.nullable {
                    syn::parse_quote!(Vec<Option<#type_>>)
                } else {
                    syn::parse_quote!(Vec<#type_>)
                };
                Ok(Node::Type { value })
            } else {
                let ident = to_ident_pascal(name);
                let variants = any_of
                    .iter()
                    .enumerate()
                    .map(|(i, any_of)| {
                        let ident = to_ident_pascal(&format!("anyOf[{i}]"));
                        to_type(&format!("{name}[{i}]"), any_of, inline).map(|type_| {
                            quote::quote! {
                                #ident(#type_)
                            }
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let description = to_description(schema.description.as_deref());
                let value = syn::parse_quote! {
                    #description
                    #derive
                    #[allow(clippy::large_enum_variant)]
                    pub enum #ident {
                        #(#variants),*
                    }
                };
                Ok(Node::Item { value, ident })
            }
        }
        Type::Array(items) => {
            let type_ = to_type(&format!("{name}.inner"), items, inline)?;
            let value = if items.nullable {
                syn::parse_quote!(Vec<Option<#type_>>)
            } else {
                syn::parse_quote!(Vec<#type_>)
            };
            Ok(Node::Type { value })
        }
        Type::Binary => {
            let value = syn::parse_quote!(Vec<u8>);
            Ok(Node::Type { value })
        }
        Type::Boolean => {
            let value = syn::parse_quote!(Vec<bool>);
            Ok(Node::Type { value })
        }
        Type::Float => {
            let value = syn::parse_quote!(f64);
            Ok(Node::Type { value })
        }
        Type::Integer => {
            let value = syn::parse_quote!(u64);
            Ok(Node::Type { value })
        }
        Type::Object {
            additional_properties,
            properties: None,
        } => {
            let type_ = if let Some(schema) = additional_properties {
                to_type(&format!("{name}.inner"), schema, inline)?
            } else {
                to_type(
                    &format!("{name}.inner"),
                    &Schema {
                        description: None,
                        type_: Type::Any,
                        nullable: false,
                    },
                    inline,
                )?
            };
            let value = syn::parse_quote!(std::collections::BTreeMap<String, #type_>);
            Ok(Node::Type { value })
        }
        Type::Object {
            additional_properties: None,
            properties: Some(properties),
        } => {
            let ident = to_ident_pascal(name);
            let fields = properties
                .iter()
                .map(|(key, (schema, _))| {
                    let ident = to_ident_snake(key);
                    let description = to_description(schema.description.as_deref());
                    to_type(&format!("{name}.{key}"), schema, inline).map(|type_| {
                        quote::quote! {
                            #description
                            pub #ident: #type_
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(schema.description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                pub struct #ident {
                    #(#fields),*
                }
            };
            Ok(Node::Item { value, ident })
        }
        Type::OneOf(one_of) => {
            let ident = to_ident_pascal(name);
            let variants = one_of
                .iter()
                .enumerate()
                .map(|(i, one_of)| {
                    let ident = to_ident_pascal(&format!("oneOf[{i}]"));
                    to_type(&format!("{name}[{i}]"), one_of, inline).map(|type_| {
                        quote::quote! {
                            #ident(#type_)
                        }
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(schema.description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        }
        Type::Ref(ref_) => {
            let ref_ = to_ident_pascal(ref_);
            let value = syn::parse_quote!(#ref_);
            Ok(Node::Type { value })
        }
        Type::String { enum_: None } => {
            let value = syn::parse_quote!(String);
            Ok(Node::Type { value })
        }
        Type::String { enum_: Some(enum_) } => {
            let ident = to_ident_pascal(name);
            let variants = enum_.iter().map(|enum_| {
                let ident = to_ident_pascal(enum_);
                quote::quote! {
                    #[serde(rename = #enum_)]
                    #ident
                }
            });
            let description = to_description(schema.description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        }
        _ => Err(anyhow::format_err!("unknown: {name} {schema:#?}")),
    }
}

fn to_item(
    name: &str,
    schema: &Schema,
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<syn::Item> {
    match to_node(name, schema, inline)? {
        Node::Item { value, .. } => Ok(value),
        Node::Type { value } => {
            let ident = to_ident_pascal(name);
            let description = to_description(schema.description.as_deref());
            Ok(syn::parse_quote! {
                #description
                pub type #ident = #value;
            })
        }
    }
}

fn to_type(
    name: &str,
    schema: &Schema,
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<syn::Type> {
    match to_node(name, schema, inline)? {
        Node::Item { ident, value } => {
            inline.push((ident.clone(), value));
            Ok(syn::parse_quote!(#ident))
        }
        Node::Type { value } => Ok(value),
    }
}

#[derive(Debug)]
struct Schema {
    description: Option<String>,
    nullable: bool,
    type_: Type,
}

#[derive(Debug)]
enum Type {
    AllOf(Vec<Schema>),
    Any,
    AnyOf(Vec<Schema>),
    Array(Box<Schema>),
    Binary,
    Boolean,
    Float,
    Integer,
    Null,
    Object {
        additional_properties: Option<Box<Schema>>,
        properties: Option<BTreeMap<String, (Schema, bool)>>,
    },
    OneOf(Vec<Schema>),
    Ref(String),
    String {
        enum_: Option<Vec<String>>,
    },
}

impl TryFrom<(&str, &openapi::Schema)> for Schema {
    type Error = anyhow::Error;

    fn try_from((name, schema): (&str, &openapi::Schema)) -> Result<Self, Self::Error> {
        let type_ = match schema {
            openapi::Schema {
                all_of: Some(all_of),
                ..
            } => Ok(Type::AllOf(
                all_of
                    .iter()
                    .enumerate()
                    .map(|(i, all_of)| {
                        (name, all_of)
                            .try_into()
                            .with_context(|| format!("allOf[{i}]"))
                    })
                    .collect::<Result<_, _>>()?,
            )),
            openapi::Schema {
                any_of: Some(any_of),
                ..
            } => Ok(Type::AnyOf(
                any_of
                    .iter()
                    .enumerate()
                    .map(|(i, any_of)| {
                        (name, any_of)
                            .try_into()
                            .with_context(|| format!("anyOf[{i}]"))
                    })
                    .collect::<Result<_, _>>()?,
            )),
            openapi::Schema {
                items,
                type_: type_ @ (None | Some(openapi::Type::Array)),
                ..
            } if items.is_some() || type_.is_some() => {
                if let Some(items) = items {
                    Ok(Type::Array(Box::new(
                        (name, items.as_ref()).try_into().context("items")?,
                    )))
                } else {
                    Ok(Type::Array(Box::new(
                        (name, &openapi::Schema::default()).try_into()?,
                    )))
                }
            }
            openapi::Schema {
                format: Some(openapi::Format::Binary),
                type_: Some(openapi::Type::String),
                ..
            } => Ok(Type::Binary),
            openapi::Schema {
                type_: Some(openapi::Type::Boolean),
                ..
            } => Ok(Type::Boolean),
            openapi::Schema {
                format: None | Some(openapi::Format::Float),
                type_: Some(openapi::Type::Number),
                ..
            } => Ok(Type::Float),
            openapi::Schema {
                format: None | Some(openapi::Format::Int64),
                type_: Some(openapi::Type::Integer),
                ..
            } => Ok(Type::Integer),
            openapi::Schema {
                type_: Some(openapi::Type::Null),
                ..
            } => Ok(Type::Null),
            openapi::Schema {
                additional_properties,
                properties,
                required,
                type_: type_ @ (None | Some(openapi::Type::Object)),
                ..
            } if properties.is_some() || type_.is_some() => Ok(Type::Object {
                additional_properties: match additional_properties {
                    None | Some(openapi::AdditionalProperties::Boolean(false)) => None,
                    Some(openapi::AdditionalProperties::Boolean(true)) => {
                        Some(Box::new((name, &openapi::Schema::default()).try_into()?))
                    }
                    Some(openapi::AdditionalProperties::Schema(additional_properties)) => {
                        Some(Box::new(
                            (name, additional_properties.as_ref())
                                .try_into()
                                .context("additionalProperties")?,
                        ))
                    }
                },
                properties: properties
                    .as_ref()
                    .map(|properties| {
                        properties
                            .iter()
                            .map(|(key, property)| {
                                let property = Self::try_from((name, property))
                                    .with_context(|| format!("properties.{key}"));
                                let required = required
                                    .as_ref()
                                    .is_some_and(|required| required.contains(key));
                                property.map(|property| (key.clone(), (property, required)))
                            })
                            .collect()
                    })
                    .transpose()?,
            }),
            openapi::Schema {
                one_of: Some(one_of),
                ..
            } => Ok(Type::OneOf(
                one_of
                    .iter()
                    .enumerate()
                    .map(|(i, one_of)| {
                        (name, one_of)
                            .try_into()
                            .with_context(|| format!("oneOf[{i}]"))
                    })
                    .collect::<Result<_, _>>()?,
            )),
            openapi::Schema {
                recursive_ref: Some(recursive_ref),
                ..
            } => {
                if recursive_ref == "#" {
                    Ok(Type::Ref(name.to_owned()))
                } else {
                    Err(anyhow::format_err!("invalid: {recursive_ref:?}")).context("$recursiveRef")
                }
            }
            openapi::Schema {
                ref_: Some(ref_), ..
            } => Ok(Type::Ref(
                ref_.strip_prefix("#/components/schemas/")
                    .ok_or_else(|| anyhow::format_err!("invalid: {ref_:?}"))
                    .context("$ref")?
                    .to_owned(),
            )),
            openapi::Schema {
                enum_,
                format: None | Some(openapi::Format::Uri),
                type_: Some(openapi::Type::String),
                ..
            } => Ok(Type::String {
                enum_: enum_.clone(),
            }),
            openapi::Schema {
                additional_properties: None,
                all_of: None,
                any_of: None,
                description: _,
                enum_: None,
                format: None,
                items: None,
                nullable: _,
                one_of: None,
                properties: None,
                recursive_ref: None,
                ref_: None,
                required: None,
                type_: None,
            } => Ok(Type::Any),
            _ => Err(anyhow::format_err!("unknown: {schema:#?}")),
        }?;
        Ok(Self {
            description: schema.description.clone(),
            nullable: schema.nullable.unwrap_or_default(),
            type_,
        })
    }
}
