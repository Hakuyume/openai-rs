mod openapi;
mod parse;
mod patch;
mod to_item_const;
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
                .map(|schema| (name.clone(), schema))
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
struct Schema {
    description: Option<String>,
    nullable: bool,
    type_: Type,
}

#[derive(Debug)]
enum Type {
    Any,
    Array(Box<Schema>),
    Binary,
    Boolean,
    Const(String),
    Enum(Vec<(Schema, bool)>),
    Float,
    Integer,
    Map(Box<Schema>),
    Number,
    Ref(String),
    String,
    Struct(Vec<Field>),
}

#[derive(Debug)]
enum Field {
    Property {
        name: String,
        schema: Schema,
        required: bool,
    },
    Ref(String),
}

fn to_type(
    name: &str,
    schema: &Schema,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Type {
    match &schema.type_ {
        Type::Any => syn::parse_quote!(serde_json::Value),
        Type::Array(item) => {
            let type_ = to_type(&to_singular(name), item, schemas, public, items);
            syn::parse_quote!(Vec<#type_>)
        }
        Type::Binary => syn::parse_quote!(Vec<u8>),
        Type::Boolean => syn::parse_quote!(bool),
        Type::Float | Type::Number => syn::parse_quote!(f64),
        Type::Integer => syn::parse_quote!(i64),
        Type::Map(item) => {
            let type_ = to_type(&to_singular(name), item, schemas, public, items);
            syn::parse_quote!(indexmap::IndexMap<String, #type_>)
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
    schema: &Schema,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    let description = to_description(schema.description.as_deref());
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);
    match &schema.type_ {
        Type::Const(value) => {
            to_item_const::to_item_const(name, schema, value, schemas, public, items)
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

fn to_singular(name: &str) -> String {
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
    vocab
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
        .unwrap_or_else(|| name.to_owned())
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
        "static" => "r#static",
        "type" => "r#type",
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
        let description = description.replace("](/", "](https://platform.openai.com/");
        let description = description.split("```\n").enumerate().fold(
            String::new(),
            |mut description, (i, part)| {
                if i % 2 == 1 {
                    description.push_str("```text\n");
                } else if i > 0 {
                    description.push_str("```\n");
                }
                description.push_str(part);
                description
            },
        );
        syn::parse_quote!(#[doc = #description])
    })
}

fn to_derive(schema: &Schema, schemas: &IndexMap<String, Schema>) -> syn::Attribute {
    let derives = [
        Some(quote::quote!(Clone)),
        is_copy(schema, schemas).then_some(quote::quote!(Copy)),
        Some(quote::quote!(Debug)),
        is_default(schema, schemas).then_some(quote::quote!(Default)),
        Some(quote::quote!(PartialEq)),
    ]
    .into_iter()
    .flatten();
    syn::parse_quote!(#[derive(#(#derives),*)])
}

fn is_copy(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Boolean | Type::Const(_) | Type::Float | Type::Integer | Type::Number => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_copy(variant, schemas)),
        Type::Ref(ref_) => is_copy(&schemas[ref_], schemas),
        Type::Struct(fields) => fields.iter().all(|field| match field {
            Field::Property { schema, .. } => is_copy(schema, schemas),
            Field::Ref(ref_) => is_copy(&schemas[ref_], schemas),
        }),
        _ => false,
    }
}

fn is_default(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants.iter().any(|(_, default)| *default),
        Type::Ref(ref_) => is_default(&schemas[ref_], schemas),
        Type::Struct(fields) => fields.iter().all(|field| match field {
            Field::Property {
                schema, required, ..
            } => is_default(schema, schemas) || schema.nullable || !required,
            Field::Ref(ref_) => is_default(&schemas[ref_], schemas),
        }),
        _ => false,
    }
}

fn is_nullable(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    schema.nullable
        || if let Type::Ref(ref_) = &schema.type_ {
            is_nullable(&schemas[ref_], schemas)
        } else {
            false
        }
}

fn to_serde_as(schema: &Schema) -> Option<String> {
    match &schema.type_ {
        Type::Array(item) => to_serde_as(item).map(|item| format!("Vec<{item}>")),
        Type::Binary => Some("serde_with::base64::Base64".to_owned()),
        Type::Map(item) => {
            to_serde_as(item).map(|item| format!("indexmap::IndexMap<String, {item}>"))
        }
        _ => None,
    }
}
