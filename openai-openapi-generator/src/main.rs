mod convert;
mod discriminator;
mod misc;
mod openapi;
mod patch;
mod visit;

use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use quote::ToTokens;
use std::io;

fn main() -> anyhow::Result<()> {
    let mut document = serde_yaml::from_reader::<_, openapi::Document>(io::stdin().lock())?;

    for (name, schema) in &mut document.components.schemas {
        patch::patch(name, schema);
    }

    let mut discriminators = Vec::new();
    for schema in document.components.schemas.values() {
        discriminator::collect(schema, &document.components.schemas, &mut discriminators);
    }

    let schemas = document
        .components
        .schemas
        .iter()
        .map(|(name, schema)| {
            convert::convert(schema, &document.components.schemas, &discriminators)
                .with_context(|| name.clone())
                .map(|schema| (name.as_str(), schema))
        })
        .collect::<Result<IndexMap<_, _>, _>>()?;

    let mut items = Vec::new();
    for (name, schema) in &schemas {
        let item = to_item(name, schema, &schemas, &mut items);
        items.push(item);
    }

    for item in items {
        println!("{}", item.to_token_stream());
    }
    println!(
        "{}",
        quote::quote! {
            #[cfg(test)]
            mod tests;
        }
        .to_token_stream()
    );

    Ok(())
}

#[derive(Debug)]
struct Schema<'a> {
    description: Option<&'a str>,
    nullable: bool,
    type_: Type<'a>,
}

#[derive(Debug)]
enum Type<'a> {
    Any,
    Array(Box<Schema<'a>>),
    Binary,
    Boolean,
    Enum {
        variants: Vec<Variant<'a>>,
        tag: Option<&'a str>,
    },
    Float,
    Integer,
    Map(Box<Schema<'a>>),
    Ref(&'a str),
    String,
    Struct {
        fields: Vec<Field<'a>>,
    },
}

#[derive(Debug)]
struct Variant<'a> {
    schema: Option<Schema<'a>>,
    default: bool,
    tag: Option<(&'a str, Vec<&'a str>)>,
}

#[derive(Debug)]
enum Field<'a> {
    Property {
        name: &'a str,
        schema: Schema<'a>,
        required: bool,
    },
    Ref(&'a str),
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

fn to_type(
    name: &str,
    schema: &Schema<'_>,
    schemas: &IndexMap<&str, Schema<'_>>,
    inline: &mut Vec<syn::Item>,
) -> syn::Type {
    match &schema.type_ {
        Type::Any => syn::parse_quote!(serde_json::Value),
        Type::Array(item) => {
            let vocab = [
                ("annotation", "annotations"),
                ("attribute", "attributes"),
                ("certificate", "certificates"),
                ("choice", "choices"),
                ("datum", "data"),
                ("file", "files"),
                ("filter", "filters"),
                ("integration", "integrations"),
                ("logprob", "logprobs"),
                ("modality", "modalities"),
                ("result", "results"),
                ("store", "stores"),
                ("tool", "tools"),
            ];
            let name = vocab
                .iter()
                .find_map(|(singular, plural)| {
                    if let Some(name) = name.strip_suffix(&plural.to_pascal_case()) {
                        Some(format!("{name}{}", singular.to_pascal_case()))
                    } else if let Some(name) = name.strip_suffix(&format!("_{plural}")) {
                        Some(format!("{name}_{singular}"))
                    } else if let Some(name) = name.strip_suffix(&format!(".{plural}")) {
                        Some(format!("{name}.{singular}"))
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| name.to_owned());
            let type_ = to_type(&name, item, schemas, inline);
            syn::parse_quote!(Vec<#type_>)
        }
        Type::Binary => syn::parse_quote!(Vec<u8>),
        Type::Boolean => syn::parse_quote!(bool),
        Type::Float => syn::parse_quote!(f64),
        Type::Integer => syn::parse_quote!(u64),
        Type::Map(item) => {
            let type_ = to_type(name, item, schemas, inline);
            syn::parse_quote!(std::collections::HashMap<String, #type_>)
        }
        Type::Ref(ref_) => {
            let ident = to_ident_pascal(ref_);
            syn::parse_quote!(#ident)
        }
        Type::String => syn::parse_quote!(String),
        _ => {
            let ident = to_ident_pascal(name);
            let item = to_item(name, schema, schemas, inline);
            inline.push(item);
            syn::parse_quote!(#ident)
        }
    }
}

fn to_item(
    name: &str,
    schema: &Schema<'_>,
    schemas: &IndexMap<&str, Schema<'_>>,
    inline: &mut Vec<syn::Item>,
) -> syn::Item {
    let description = to_description(schema.description);
    let derive =
        quote::quote!(#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]);
    let derive_default = is_default(schema, schemas).then_some(quote::quote!(#[derive(Default)]));
    let ident = to_ident_pascal(name);
    match &schema.type_ {
        Type::Enum { variants, tag } => {
            let attr_serde = tag
                .as_ref()
                .map(|tag| quote::quote!(#[serde(tag = #tag)]))
                .or(variants
                    .iter()
                    .all(|variant| variant.tag.is_none())
                    .then_some(quote::quote!(#[serde(untagged)])));
            let variants = variants.iter().enumerate().map(|(i, variant)| {
                let attr_default = variant.default.then_some(quote::quote!(#[default]));
                let attr_serde = variant.tag.as_ref().map(
                    |(tag, aliases)| quote::quote!(#[serde(rename = #tag, #(alias = #aliases),*)]),
                );
                let (ident, name) = if let Some((tag, _)) = &variant.tag {
                    (to_ident_pascal(tag), format!("{name}.{tag}"))
                } else {
                    (to_ident_pascal(&i.to_string()), format!("{name}.{i}"))
                };
                if let Some(schema) = &variant.schema {
                    let type_ = to_type(&name, schema, schemas, inline);
                    quote::quote! {
                        #attr_default
                        #attr_serde
                        #ident(#type_)
                    }
                } else {
                    quote::quote! {
                        #attr_default
                        #attr_serde
                        #ident
                    }
                }
            });
            syn::parse_quote! {
                #description
                #derive
                #derive_default
                #attr_serde
                #[allow(clippy::large_enum_variant)]
                pub enum #ident {
                    #(#variants),*
                }
            }
        }
        Type::Struct { fields } => {
            let fields = fields.iter().map(|field| match field {
                Field::Property {
                    name: field_name,
                    schema,
                    required,
                } => {
                    let description = to_description(schema.description);
                    let attr_builder =
                        (is_default(schema, schemas) || schema.nullable || !required)
                            .then_some(quote::quote!(#[builder(default)]));
                    let attr_serde = (schema.nullable || !required).then_some(
                        quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]),
                    );
                    let ident = to_ident_snake(field_name);
                    let type_ = to_type(&format!("{name}.{field_name}"), schema, schemas, inline);
                    let type_ = if !schema.nullable && *required {
                        type_
                    } else {
                        syn::parse_quote!(Option<#type_>)
                    };
                    quote::quote! {
                        #description
                        #attr_builder
                        #attr_serde
                        #[serde(rename = #field_name)]
                        pub #ident: #type_
                    }
                }
                Field::Ref(ref_) => {
                    let attr_builder = is_default(schemas.get(ref_).unwrap(), schemas)
                        .then_some(quote::quote!(#[builder(default)]));
                    let ident = to_ident_snake(ref_);
                    let type_ = to_ident_pascal(ref_);
                    quote::quote! {
                        #attr_builder
                        #[serde(flatten)]
                        pub #ident: #type_
                    }
                }
            });
            syn::parse_quote! {
                #description
                #derive
                #derive_default
                #[derive(typed_builder::TypedBuilder)]
                pub struct #ident {
                    #(#fields),*
                }
            }
        }
        _ => {
            let type_ = to_type(name, schema, schemas, inline);
            syn::parse_quote!(pub type #ident = #type_;)
        }
    }
}

fn is_default(schema: &Schema<'_>, schemas: &IndexMap<&str, Schema<'_>>) -> bool {
    match &schema.type_ {
        Type::Enum { variants, .. } => variants.iter().any(|variant| variant.default),
        Type::Ref(ref_) => is_default(schemas.get(ref_).unwrap(), schemas),
        Type::Struct { fields } => fields.iter().all(|field| match field {
            Field::Property {
                schema, required, ..
            } => is_default(schema, schemas) || schema.nullable || !required,
            Field::Ref(ref_) => is_default(schemas.get(ref_).unwrap(), schemas),
        }),
        _ => false,
    }
}
