mod convert;
mod openapi;
mod patch;
mod visit;

use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use quote::ToTokens;
use std::fmt::Write;
use std::{collections::HashMap, io};

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
            convert::convert(schema, &document.components.schemas)
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
    EnumOf(Vec<Schema<'a>>),
    EnumString(Vec<(&'a str, bool)>),
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
    let derive = quote::quote!(#[derive(Clone, Debug, PartialEq)]);
    let derive_copy = is_copy(schema, schemas).then_some(quote::quote!(#[derive(Copy)]));
    let derive_default = is_default(schema, schemas).then_some(quote::quote!(#[derive(Default)]));
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);
    match &schema.type_ {
        Type::Const(value) => {
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
        Type::EnumOf(variants) => {
            let variant_names = {
                let tags = variants
                    .iter()
                    .map(|variant| {
                        let fields = if let Type::Ref(ref_) = &variant.type_ {
                            if let Type::Struct(fields) = &schemas.get(ref_).unwrap().type_ {
                                Some(fields)
                            } else {
                                None
                            }
                        } else if let Type::Struct(fields) = &variant.type_ {
                            Some(fields)
                        } else {
                            None
                        };
                        fields
                            .into_iter()
                            .flatten()
                            .filter_map(|field| {
                                if let Field::Property {
                                    name,
                                    schema,
                                    required: true,
                                } = field
                                {
                                    if let Type::Const(value) = &schema.type_ {
                                        Some((*name, *value))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .collect::<Vec<HashMap<_, _>>>();
                let key = tags
                    .iter()
                    .flatten()
                    .fold(HashMap::<_, usize>::new(), |mut counts, (key, _)| {
                        *counts.entry(*key).or_default() += 1;
                        counts
                    })
                    .into_iter()
                    .filter(|(_, count)| *count > 1)
                    .max_by_key(|(_, count)| *count)
                    .map(|(key, _)| key);

                let mut names = variants
                    .iter()
                    .zip(tags)
                    .enumerate()
                    .map(|(i, (variant, mut tags))| {
                        if let Some(tag) = key.and_then(|key| tags.remove(key)) {
                            tag.to_owned()
                        } else if let Schema {
                            type_: Type::Ref(ref_),
                            ..
                        } = variant
                        {
                            (*ref_).to_owned()
                        } else {
                            i.to_string()
                        }
                    })
                    .collect::<Vec<_>>();

                let mut i = 0;
                while i < names.len() {
                    let conflicts = (i..names.len())
                        .filter(|j| names[i] == names[*j])
                        .collect::<Vec<_>>();
                    if conflicts.len() > 1 {
                        for (i, j) in conflicts.into_iter().enumerate() {
                            write!(&mut names[j], ".{i}").unwrap();
                        }
                    } else {
                        i += 1;
                    }
                }

                names
            };

            let variants = variants
                .iter()
                .zip(variant_names)
                .map(|(variant, variant_name)| {
                    let ident = to_ident_pascal(&variant_name);
                    let type_ = to_type(
                        &format!("{name}.{variant_name}"),
                        variant,
                        schemas,
                        public,
                        items,
                    );
                    quote::quote! {
                        #ident(#type_)
                    }
                });
            syn::parse_quote! {
                #description
                #derive
                #derive_copy
                #derive_default
                #[derive(serde::Deserialize, serde::Serialize)]
                #[serde(untagged)]
                #[allow(clippy::large_enum_variant)]
                #vis enum #ident {
                    #(#variants),*
                }
            }
        }
        Type::EnumString(variants) => {
            let variants = variants.iter().map(|(value, default)| {
                let attr_default = default.then_some(quote::quote!(#[default]));
                let ident = to_ident_pascal(value);
                quote::quote! {
                    #attr_default
                    #[serde(rename = #value)]
                    #ident
                }
            });
            syn::parse_quote! {
                #description
                #derive
                #derive_copy
                #derive_default
                #[derive(serde::Deserialize, serde::Serialize)]
                #vis enum #ident {
                    #(#variants),*
                }
            }
        }
        Type::Struct(fields) => {
            let mut items_inner = Vec::new();
            let fields = fields
                .iter()
                .map(|field| match field {
                    Field::Property {
                        name: field_name,
                        schema,
                        required,
                    } => {
                        let public = public
                            && (!matches!(
                                schema,
                                Schema {
                                    nullable: false,
                                    type_: Type::Const(_),
                                    ..
                                },
                            ) || !required);
                        let ident = to_ident_snake(field_name);
                        let type_ = to_type(
                            &format!("{name}.{field_name}"),
                            schema,
                            schemas,
                            public,
                            if public { items } else { &mut items_inner },
                        );
                        let option = schema.nullable || !required;
                        let type_ = if option {
                            syn::parse_quote!(Option<#type_>)
                        } else {
                            type_
                        };
                        (
                            ident,
                            type_,
                            public,
                            to_description(schema.description),
                            option,
                            is_default(schema, schemas),
                            Some(field_name),
                        )
                    }
                    Field::Ref(ref_) => {
                        let ident = to_ident_snake(ref_);
                        let type_ = to_ident_pascal(ref_);
                        (
                            ident,
                            syn::parse_quote!(#type_),
                            true,
                            None,
                            false,
                            is_default(schemas.get(ref_).unwrap(), schemas),
                            None,
                        )
                    }
                })
                .collect::<Vec<_>>();

            {
                let fields_inner = fields.iter().map(|(ident, type_, _, _, _, _, name)| {
                    let attr_serde_rename = name.map_or_else(
                        || quote::quote!(#[serde(flatten)]),
                        |name| quote::quote!(#[serde(rename = #name)]),
                    );
                    quote::quote! {
                        #attr_serde_rename
                        #[allow(dead_code)]
                        #ident: #type_
                    }
                });
                let idents_outer = fields
                    .iter()
                    .filter_map(|(ident, _, public, _, _, _, _)| public.then_some(ident));
                let field_values_outer = fields
                    .iter()
                    .filter_map(|(ident, _, public, _, _, _, _)| public.then_some(ident));
                items.push(syn::parse_quote! {
                    impl<'de> serde::Deserialize<'de> for #ident {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where D: serde::Deserializer<'de> {
                            #(#items_inner)*
                            #[derive(serde::Deserialize)]
                            struct _D {
                                #(#fields_inner),*
                            }
                            let _D {
                                #(#idents_outer,)*
                                ..
                            } = _D::deserialize(deserializer)?;
                            Ok(#ident {
                                #(#field_values_outer),*
                            })
                        }
                    }
                });
            }

            {
                let fields_inner = fields.iter().map(|(ident, type_, _, _, option, _, name)| {
                    let attr_serde_rename = name.map_or_else(
                        || quote::quote!(#[serde(flatten)]),
                        |name| quote::quote!(#[serde(rename = #name)]),
                    );
                    let attr_serde_skip_serializing_if = option.then_some(
                        quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]),
                    );
                    quote::quote! {
                        #attr_serde_rename
                        #attr_serde_skip_serializing_if
                        #ident: &'a #type_
                    }
                });
                let idents_outer = fields
                    .iter()
                    .filter_map(|(ident, _, public, _, _, _, _)| public.then_some(ident));
                let field_values_inner = fields.iter().map(|(ident, _, public, _, _, _, _)| {
                    if *public {
                        quote::quote!(#ident)
                    } else {
                        quote::quote!(#ident: &Default::default())
                    }
                });
                items.push(syn::parse_quote! {
                    impl serde::Serialize for #ident {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where S: serde::Serializer {
                            #(#items_inner)*
                            #[derive(serde::Serialize)]
                            struct _S<'a> {
                                #(#fields_inner),*
                            }
                            let #ident {
                                #(#idents_outer),*
                            } = self;
                            _S { #(#field_values_inner),* }.serialize(serializer)
                        }
                    }
                });
            }

            {
                let fields = fields.iter().filter_map(
                    |(ident, type_, public, description, option, default, _)| {
                        let attr_builder =
                            (*option || *default).then_some(quote::quote!(#[builder(default)]));
                        public.then_some(quote::quote! {
                            #description
                            #attr_builder
                            pub #ident: #type_
                        })
                    },
                );
                syn::parse_quote! {
                    #description
                    #derive
                    #derive_copy
                    #derive_default
                    #[derive(typed_builder::TypedBuilder)]
                    #vis struct #ident {
                        #(#fields),*
                    }
                }
            }
        }
        _ => {
            let type_ = to_type(name, schema, schemas, public, items);
            syn::parse_quote!(#vis type #ident = #type_;)
        }
    }
}

fn is_copy(schema: &Schema<'_>, schemas: &IndexMap<&str, Schema<'_>>) -> bool {
    match &schema.type_ {
        Type::Boolean | Type::Const(_) | Type::EnumString(_) | Type::Float | Type::Integer => true,
        Type::EnumOf(variants) => variants.iter().all(|variant| is_copy(variant, schemas)),
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
        Type::EnumString(variants) => variants.iter().any(|(_, default)| *default),
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
