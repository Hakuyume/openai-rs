mod openapi;
mod parse;
mod patch;
mod to_item_const;
mod to_item_enum;
mod to_item_struct;
mod to_items_fn;
mod visit;

use anyhow::Context;
use clap::Parser;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::{IndexMap, IndexSet};
use std::fs::{self, File};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[clap(long)]
    document: PathBuf,
    #[clap(long)]
    types: Option<PathBuf>,
    #[clap(long)]
    functions: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut document =
        serde_yaml::from_reader::<_, openapi::Document>(File::open(&args.document)?)?;

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

    if let Some(path) = &args.types {
        let mut items = Vec::new();
        for (name, schema) in &schemas {
            to_item(name, schema, &schemas, true, &mut items);
        }
        fs::write(path, quote::quote!(#(#items)*).to_string())?;
    }

    if let Some(path) = &args.functions {
        let items = to_items_fn::to_items_fn(&document, &schemas)?;
        fs::write(path, quote::quote!(#(#items)*).to_string())?;
    }

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
    Struct {
        fields: Vec<either::Either<(String, Schema), String>>,
        required: IndexSet<String>,
    },
}

fn to_type(
    name: &str,
    schema: &Schema,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: Option<&mut Vec<syn::Item>>,
) -> syn::Type {
    match &schema.type_ {
        Type::Any => syn::parse_quote!(serde_json::Value),
        Type::Array(item) => {
            let type_ = to_type(&to_singular(name), item, schemas, public, items);
            syn::parse_quote!(Vec<#type_>)
        }
        Type::Binary => syn::parse_quote!(Vec<u8>),
        Type::Boolean => syn::parse_quote!(bool),
        Type::Float => syn::parse_quote!(f64),
        Type::Integer => syn::parse_quote!(i64),
        Type::Map(item) => {
            let type_ = to_type(&to_singular(name), item, schemas, public, items);
            syn::parse_quote!(indexmap::IndexMap<String, #type_>)
        }
        Type::Number => syn::parse_quote!(serde_json::Number),
        Type::Ref(ref_) => {
            let ident = to_ident_pascal(ref_);
            syn::parse_quote!(#ident)
        }
        Type::String => syn::parse_quote!(String),
        _ => {
            let ident = to_ident_pascal(name);
            if let Some(items) = items {
                to_item(name, schema, schemas, public, items);
            }
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
) {
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
        Type::Struct { fields, required } => {
            to_item_struct::to_item_struct(name, schema, fields, required, schemas, public, items)
        }
        _ => {
            let type_ = to_type(name, schema, schemas, public, Some(items));
            items.push(syn::parse_quote! {
                #description
                #vis type #ident = #type_;
            });
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
        ("variable", "variables"),
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
        Type::Boolean | Type::Const(_) | Type::Float | Type::Integer => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_copy(variant, schemas)),
        Type::Ref(ref_) => is_copy(&schemas[ref_], schemas),
        Type::Struct { fields, required } => {
            let mut missing_fields = required.clone();
            for name in extract_fields(schema, schemas, false).into_keys() {
                missing_fields.swap_remove(name);
            }
            missing_fields.is_empty()
                && fields.iter().all(|field| match field {
                    either::Left((_, schema)) => is_copy(schema, schemas),
                    either::Right(ref_) => is_copy(&schemas[ref_], schemas),
                })
        }
        _ => false,
    }
}

fn is_default(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants.iter().any(|(_, default)| *default),
        Type::Ref(ref_) => is_default(&schemas[ref_], schemas),
        Type::Struct { fields, required } => {
            let mut missing_fields = required.clone();
            for name in extract_fields(schema, schemas, false).into_keys() {
                missing_fields.swap_remove(name);
            }
            missing_fields.is_empty()
                && fields.iter().all(|field| match field {
                    either::Left((name, schema)) => {
                        is_default(schema, schemas) || schema.nullable || !required.contains(name)
                    }
                    either::Right(ref_) => is_default(&schemas[ref_], schemas),
                })
        }
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

fn extract_fields<'a>(
    schema: &'a Schema,
    schemas: &'a IndexMap<String, Schema>,
    follow_ref: bool,
) -> IndexMap<&'a str, (Option<&'a str>, &'a Schema, bool)> {
    match &schema.type_ {
        Type::Ref(ref_) if follow_ref => extract_fields(&schemas[ref_], schemas, follow_ref),
        Type::Struct { fields, required } => {
            fields.iter().fold(IndexMap::new(), |mut fields, field| {
                match field {
                    either::Left((name, schema)) => {
                        fields.shift_remove(name.as_str());
                        fields.insert(name.as_str(), (None, schema, required.contains(name)));
                    }
                    either::Right(ref_) => {
                        for (name, (inner_ref, schema, inner_required)) in
                            extract_fields(&schemas[ref_], schemas, follow_ref)
                        {
                            if !fields.contains_key(name) {
                                fields.insert(
                                    name,
                                    (
                                        Some(inner_ref.unwrap_or(ref_)),
                                        schema,
                                        inner_required || required.contains(name),
                                    ),
                                );
                            }
                        }
                    }
                }
                fields
            })
        }
        _ => IndexMap::new(),
    }
}
