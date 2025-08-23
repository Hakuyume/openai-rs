mod openapi;
mod parse;
mod patch;
mod to_item_const;
mod to_item_enum;
mod to_item_fn;
mod to_item_struct;
mod visit;

use anyhow::Context;
use clap::Parser;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use quote::ToTokens;
use std::fs::{self, File};
use std::iter;
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
        patch::patch(Some(name), schema);
    }
    for operations in document.paths.values_mut() {
        for operation in operations.values_mut() {
            if let Some(parameters) = &mut operation.parameters {
                for parameter in parameters {
                    patch::patch(None, &mut parameter.schema);
                }
            }
            if let Some(request_body) = &mut operation.request_body {
                for content in request_body.content.values_mut() {
                    patch::patch(None, &mut content.schema);
                }
            }
            for response in operation.responses.values_mut() {
                if let Some(content) = &mut response.content {
                    for content in content.values_mut() {
                        patch::patch(None, &mut content.schema);
                    }
                }
            }
        }
    }

    let schemas = document
        .components
        .schemas
        .iter()
        .map(|(name, schema)| {
            parse::parse(schema, &document.components.schemas)
                .map(|schema| (name.clone(), schema))
                .with_context(|| format!("schemas[{name:?}]"))
                .context("components")
        })
        .collect::<Result<IndexMap<_, _>, _>>()?;

    let operations = document
        .paths
        .iter()
        .flat_map(|(path, operations)| {
            let schemas = &document.components.schemas;
            operations.iter().map(move |(method, operation)| {
                parse::parse_operation(*method, path, operation, schemas)
                    .with_context(|| format!("path[{path:?}][{method:?}]"))
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    if let Some(path) = &args.types {
        let mut items = Items::default();
        for (name, schema) in &schemas {
            to_item((&[], name), schema, &schemas, true, &mut items);
        }
        for operation in &operations {
            if let Some(parameters) = &operation.parameters {
                let schema = Schema {
                    description: None,
                    nullable: false,
                    type_: Type::Struct(
                        parameters
                            .iter()
                            .map(|parameter| {
                                let mut schema = parameter.schema.clone();
                                schema.description = parameter.description.clone();
                                (parameter.name.clone(), (schema, parameter.required))
                            })
                            .collect(),
                    ),
                };
                to_type(
                    (&[&operation.id], "params"),
                    &schema,
                    &schemas,
                    true,
                    Some(&mut items),
                );
            }
            if let Some(requests) = &operation.requests {
                for (_, schema, _) in requests {
                    to_type(
                        (&[&operation.id], "request"),
                        schema,
                        &schemas,
                        true,
                        Some(&mut items),
                    );
                }
            }
            for (_, response) in &operation.responses {
                if let Some((_, schema)) = response {
                    to_type(
                        (&[&operation.id], "response"),
                        schema,
                        &schemas,
                        true,
                        Some(&mut items),
                    );
                }
            }
        }
        fs::write(path, items.to_token_stream().to_string())?;
    }

    if let Some(path) = &args.functions {
        let mut items = Items::default();
        for operation in &operations {
            to_item_fn::to_item_fn(operation, &schemas, &mut items);
        }
        fs::write(path, items.to_token_stream().to_string())?;
    }

    Ok(())
}

#[derive(Clone, Debug)]
struct Operation {
    description: String,
    id: String,
    method: openapi::Method,
    parameters: Option<Vec<Parameter>>,
    path: String,
    requests: Option<Vec<(openapi::ContentType, Schema, bool)>>,
    responses: Vec<(u16, Option<(openapi::ContentType, Schema)>)>,
}

#[derive(Clone, Debug)]
struct Parameter {
    description: Option<String>,
    in_: openapi::In,
    name: String,
    required: bool,
    schema: Schema,
}

#[derive(Clone, Debug)]
struct Schema {
    description: Option<String>,
    nullable: bool,
    type_: Type,
}

#[derive(Clone, Debug)]
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
    Struct(IndexMap<String, (Schema, bool)>),
}

#[derive(Default)]
struct Items(Vec<either::Either<(bool, syn::Ident, Items), syn::Item>>);

impl ToTokens for Items {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.0.iter().map(|item| match item {
            either::Left((pub_, ident, this)) => {
                let vis = to_vis(*pub_);
                quote::quote! {
                    #[allow(clippy::module_inception)]
                    #vis mod #ident { #this }
                }
            }
            either::Right(item) => item.to_token_stream(),
        }));
    }
}

impl Items {
    fn push(&mut self, module: &[&str], pub_: bool, item: syn::Item) {
        module
            .iter()
            .fold(self, |this, m| {
                let ident = to_ident_snake(m);
                let index = this.0.iter_mut().position(|item| {
                    if let either::Left((_, i, _)) = item {
                        *i == ident
                    } else {
                        false
                    }
                });
                let item = if let Some(index) = index {
                    this.0.get_mut(index)
                } else {
                    this.0.push(either::Left((false, ident, Self::default())));
                    this.0.last_mut()
                };
                let Some(either::Left((p, _, this))) = item else {
                    unreachable!()
                };
                *p |= pub_;
                this
            })
            .0
            .push(either::Right(item));
    }
}

fn to_type(
    (module, name): (&[&str], &str),
    schema: &Schema,
    schemas: &IndexMap<String, Schema>,
    pub_: bool,
    items: Option<&mut Items>,
) -> syn::Type {
    match &schema.type_ {
        Type::Any => syn::parse_quote!(serde_json::Value),
        Type::Array(item) => {
            let type_ = to_type(
                (&[module, &[name]].concat(), "item"),
                item,
                schemas,
                pub_,
                items,
            );
            syn::parse_quote!(Vec<#type_>)
        }
        Type::Binary => syn::parse_quote!(Vec<u8>),
        Type::Boolean => syn::parse_quote!(bool),
        Type::Float => syn::parse_quote!(f64),
        Type::Integer => syn::parse_quote!(i64),
        Type::Map(item) => {
            let type_ = to_type(
                (&[module, &[name]].concat(), "item"),
                item,
                schemas,
                pub_,
                items,
            );
            syn::parse_quote!(indexmap::IndexMap<String, #type_>)
        }
        Type::Number => syn::parse_quote!(serde_json::Number),
        Type::Ref(ref_) => {
            let ident = to_ident_pascal(ref_);
            syn::parse_quote!(crate::__types::#ident)
        }
        Type::String => syn::parse_quote!(String),
        _ => {
            if let Some(items) = items {
                to_item((module, name), schema, schemas, pub_, items);
            }
            let path = module
                .iter()
                .map(|m| to_ident_snake(m))
                .chain(iter::once(to_ident_pascal(name)));
            syn::parse_quote!(crate::__types::#(#path)::*)
        }
    }
}

fn to_item(
    (module, name): (&[&str], &str),
    schema: &Schema,
    schemas: &IndexMap<String, Schema>,
    pub_: bool,
    items: &mut Items,
) {
    match &schema.type_ {
        Type::Const(value) => {
            to_item_const::to_item_const((module, name), schema, value, schemas, pub_, items)
        }
        Type::Enum(variants) => {
            to_item_enum::to_item_enum((module, name), schema, variants, schemas, pub_, items)
        }
        Type::Struct(fields) => {
            to_item_struct::to_item_struct((module, name), schema, fields, schemas, pub_, items)
        }
        _ => {
            let description = to_description(schema.description.as_deref());
            let vis = to_vis(pub_);
            let ident = to_ident_pascal(name);
            let type_ = to_type((module, name), schema, schemas, pub_, Some(items));
            items.push(
                module,
                pub_,
                syn::parse_quote! {
                    #description
                    #vis type #ident = #type_;
                },
            );
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
        "move" => "r#move",
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

fn to_vis(pub_: bool) -> syn::Visibility {
    if pub_ {
        syn::parse_quote!(pub)
    } else {
        syn::parse_quote!(pub(crate))
    }
}

fn is_copy(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Boolean | Type::Const(_) | Type::Float | Type::Integer => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_copy(variant, schemas)),
        Type::Ref(ref_) => is_copy(&schemas[ref_], schemas),
        Type::Struct(fields) => fields.values().all(|(schema, _)| is_copy(schema, schemas)),
        _ => false,
    }
}

fn is_default(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants.iter().any(|(_, default)| *default),
        Type::Ref(ref_) => is_default(&schemas[ref_], schemas),
        Type::Struct(fields) => fields
            .values()
            .all(|(schema, required)| is_default(schema, schemas) || schema.nullable || !required),
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
