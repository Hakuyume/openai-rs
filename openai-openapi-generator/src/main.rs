mod openapi;
mod parse;
mod patch;
mod to_item_enum;
mod to_item_struct;
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

    let schemas = document
        .components
        .schemas
        .iter()
        .map(|(name, schema)| {
            parse::parse(schema, &document.components.schemas)
                .with_context(|| name.clone())
                .map(|schema| (name.as_str(), schema))
        })
        .collect::<Result<IndexMap<_, _>, _>>()?;

    let mut items = Vec::new();
    for (name, schema) in &schemas {
        let item = to_item(name, schema, &schemas, true, &mut items);
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
    Const(&'a str),
    Enum(Vec<(Schema<'a>, bool)>),
    Float,
    Integer,
    Map(Box<Schema<'a>>),
    Ref(&'a str),
    String,
    Struct(Vec<Field<'a>>),
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

fn to_type(
    name: &str,
    schema: &Schema<'_>,
    schemas: &IndexMap<&str, Schema<'_>>,
    public: bool,
    items: &mut Vec<syn::Item>,
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
                    name.strip_suffix(&plural.to_pascal_case())
                        .map(|name| format!("{name}{}", singular.to_pascal_case()))
                        .or_else(|| {
                            name.strip_suffix(&format!("_{plural}"))
                                .map(|name| format!("{name}_{singular}"))
                        })
                        .or_else(|| {
                            name.strip_suffix(&format!(".{plural}"))
                                .map(|name| format!("{name}.{singular}"))
                        })
                })
                .unwrap_or_else(|| name.to_owned());
            let type_ = to_type(&name, item, schemas, public, items);
            syn::parse_quote!(Vec<#type_>)
        }
        Type::Binary => syn::parse_quote!(Vec<u8>),
        Type::Boolean => syn::parse_quote!(bool),
        Type::Float => syn::parse_quote!(f64),
        Type::Integer => syn::parse_quote!(u64),
        Type::Map(item) => {
            let type_ = to_type(name, item, schemas, public, items);
            syn::parse_quote!(std::collections::HashMap<String, #type_>)
        }
        Type::Ref(ref_) => {
            let ident = to_ident_pascal(ref_);
            syn::parse_quote!(#ident)
        }
        Type::String => syn::parse_quote!(String),
        _ => {
            let ident = to_ident_pascal(name);
            let item = to_item(name, schema, schemas, public, items);
            items.push(item);
            syn::parse_quote!(#ident)
        }
    }
}

fn to_item(
    name: &str,
    schema: &Schema<'_>,
    schemas: &IndexMap<&str, Schema<'_>>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    let description = to_description(schema.description);
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);
    match &schema.type_ {
        Type::Const(value) => {
            let derive = quote::quote!(#[derive(Clone, Debug, PartialEq)]);
            let derive_copy = is_copy(schema, schemas).then_some(quote::quote!(#[derive(Copy)]));
            let derive_default =
                is_default(schema, schemas).then_some(quote::quote!(#[derive(Default)]));
            let variant = to_ident_pascal(value);
            syn::parse_quote! {
                #description
                #derive
                #derive_copy
                #derive_default
                #[derive(serde::Deserialize, serde::Serialize)]
                #vis enum #ident {
                    #[default]
                    #[serde(rename = #value)]
                    #variant
                }
            }
        }
        Type::Enum(variants) => {
            to_item_enum::to_item_enum(name, schema, variants, schemas, public, items)
        }
        Type::Struct(fields) => {
            to_item_struct::to_item_struct(name, schema, fields, schemas, public, items)
        }
        _ => {
            let type_ = to_type(name, schema, schemas, public, items);
            syn::parse_quote! {
                #description
                #vis type #ident = #type_;
            }
        }
    }
}

fn to_ident_pascal(name: &str) -> syn::Ident {
    let name = name.replace(['-', '.', '[', ']'], "_");
    let name = name.split('_').fold(String::new(), |mut name, part| {
        let part = part.to_pascal_case();
        if name.chars().last().is_some_and(char::is_numeric)
            && part.chars().next().is_some_and(char::is_numeric)
        {
            name.push('_');
        }
        name.push_str(&part);
        name
    });
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
    description.map(|description| {
        let description = description.replace(
            "](/docs/api-reference/",
            "](https://platform.openai.com/docs/api-reference/",
        );
        syn::parse_quote!(#[doc = #description])
    })
}

fn is_copy(schema: &Schema<'_>, schemas: &IndexMap<&str, Schema<'_>>) -> bool {
    match &schema.type_ {
        Type::Boolean | Type::Const(_) | Type::Float | Type::Integer => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_copy(variant, schemas)),
        Type::Ref(ref_) => is_copy(schemas.get(ref_).unwrap(), schemas),
        Type::Struct(fields) => fields.iter().all(|field| match field {
            Field::Property { schema, .. } => is_copy(schema, schemas),
            Field::Ref(ref_) => is_copy(schemas.get(ref_).unwrap(), schemas),
        }),
        _ => false,
    }
}

fn is_default(schema: &Schema<'_>, schemas: &IndexMap<&str, Schema<'_>>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants.iter().any(|(_, default)| *default),
        Type::Ref(ref_) => is_default(schemas.get(ref_).unwrap(), schemas),
        Type::Struct(fields) => fields.iter().all(|field| match field {
            Field::Property {
                schema, required, ..
            } => is_default(schema, schemas) || schema.nullable || !required,
            Field::Ref(ref_) => is_default(schemas.get(ref_).unwrap(), schemas),
        }),
        _ => false,
    }
}

fn to_serde_as(schema: &Schema<'_>) -> Option<String> {
    match &schema.type_ {
        Type::Array(item) => to_serde_as(item).map(|item| format!("Vec<{item}>")),
        Type::Binary => Some("serde_with::base64::Base64".to_owned()),
        Type::Map(item) => {
            to_serde_as(item).map(|item| format!("std::collections::HashMap<String, {item}>"))
        }
        _ => None,
    }
}
