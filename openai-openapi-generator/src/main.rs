mod openapi;
mod visit;

use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use quote::ToTokens;
use std::collections::{HashMap, HashSet};
use std::hash;
use std::io;
use std::iter;
use std::ptr;

fn main() -> anyhow::Result<()> {
    let mut document = serde_yaml::from_reader::<_, openapi::Document>(io::stdin().lock())?;

    for (name, schema) in &mut document.components.schemas {
        patch(name, schema);
    }

    let mut discriminators = Vec::new();
    for schema in document.components.schemas.values() {
        collect_discriminator(schema, &document.components.schemas, &mut discriminators);
    }

    let mut items = Vec::new();
    for (name, schema) in &document.components.schemas {
        let (ident, value) = match to_node(name, schema, &discriminators, &mut items)
            .with_context(|| name.clone())?
        {
            Node::Item { ident, value } => (ident, value),
            Node::Type { value } => {
                let ident = to_ident_pascal(name);
                let description = to_description(schema.description.as_deref());
                let value = syn::parse_quote! {
                    #description
                    pub type #ident = #value;
                };
                (ident, value)
            }
        };
        items.push((ident, value));
    }

    items.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
    for (_, value) in items {
        println!("{}", value.to_token_stream());
    }

    let tests =
        document
            .components
            .schemas
            .iter()
            .filter_map(|(name, schema)| -> Option<syn::ItemFn> {
                if let Some(openapi::XOaiMeta {
                    example: Some(example),
                    ..
                }) = &schema.x_oai_meta
                {
                    let ident = to_ident_snake(&format!("test_{name}"));
                    let type_ = to_ident_pascal(name);
                    let example = serde_json::to_string(
                        &serde_json::from_str::<serde_json::Value>(example).ok()?,
                    )
                    .ok()?;
                    Some(syn::parse_quote! {
                        #[test]
                        fn #ident() {
                            serde_json::from_str::<super::#type_>(#example).unwrap();
                        }
                    })
                } else {
                    None
                }
            });
    println!(
        "{}",
        quote::quote! {
            #[cfg(test)]
            mod tests {
                #(#tests)*
            }
        }
    );

    Ok(())
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn patch(name: &str, schema: &mut openapi::Schema) {
    for (_, schema) in visit::iter_mut(schema) {
        patch(name, schema)
    }

    if let Some(ref_) = if let openapi::Schema {
        all_of: Some(all_of),
    } = schema
    {
        if let [
            openapi::Schema {
                ref_: ref_ @ Some(_),
            },
        ] = &mut all_of[..]
        {
            Some(ref_)
        } else {
            None
        }
    } else {
        None
    } {
        schema.ref_ = ref_.take();
        schema.all_of = None;
    }

    if let Some((ref_, description, nullable)) = if let openapi::Schema {
        all_of: Some(all_of),
    } = schema
    {
        if let [
            openapi::Schema {
                ref_: ref_ @ Some(_),
            },
            openapi::Schema {
                description,
                nullable: nullable @ Some(true),
            },
        ] = &mut all_of[..]
        {
            Some((ref_, description, nullable))
        } else {
            None
        }
    } else {
        None
    } {
        schema.ref_ = ref_.take();
        schema.description = description.take();
        schema.nullable = nullable.take();
        schema.all_of = None;
    }

    if let Some((
        additional_properties,
        default,
        description,
        enum_,
        items,
        ref_,
        type_,
        x_stainless_const,
    )) = if let openapi::Schema {
        any_of: Some(any_of),
    } = schema
    {
        if let [
            openapi::Schema {
                additional_properties,
                default,
                description,
                enum_,
                items,
                ref_,
                type_,
                x_stainless_const,
            },
            openapi::Schema {
                type_: Some(openapi::Type::Null),
            },
        ] = &mut any_of[..]
        {
            Some((
                additional_properties,
                default,
                description,
                enum_,
                items,
                ref_,
                type_,
                x_stainless_const,
            ))
        } else {
            None
        }
    } else {
        None
    } {
        schema.additional_properties = additional_properties.take();
        schema.default = default.take();
        schema.description = description.take();
        schema.enum_ = enum_.take();
        schema.items = items.take();
        schema.nullable = Some(true);
        schema.ref_ = ref_.take();
        schema.type_ = type_.take();
        schema.x_stainless_const = x_stainless_const.take();
        schema.any_of = None;
    }

    if let Some(items) = &mut schema.items {
        if let Some(required) = schema.required.take() {
            items.required = Some(required.clone());
        }
    }

    if let Some(one_of) = &mut schema.one_of {
        if let Some(properties) = schema.properties.take() {
            for one_of in &mut *one_of {
                one_of.properties = Some(properties.clone());
            }
        }
        if let Some(type_) = schema.type_.take() {
            for one_of in &mut *one_of {
                one_of.type_ = Some(type_);
            }
        }
    }

    if schema.recursive_ref.as_deref() == Some("#") {
        schema.recursive_ref = None;
        schema.ref_ = Some(format!("#/components/schemas/{name}"));
    }
}

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn extract_const(schema: &openapi::Schema) -> Option<(&String, Vec<&String>)> {
    if let openapi::Schema {
        description: _,
        default,
        enum_: Some(enum_),
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
    } = schema
    {
        if let [enum_] = &enum_[..] {
            Some((enum_, Vec::new()))
        } else if let Some(serde_json::Value::String(default)) = default {
            Some((
                default,
                enum_.iter().filter(|enum_| *enum_ != default).collect(),
            ))
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug)]
struct Discriminator<'a> {
    property_name: &'a String,
    mapping: Vec<(&'a openapi::Schema, &'a String, Vec<&'a String>)>,
}

fn is_unique<I>(iter: I) -> bool
where
    I: IntoIterator,
    I::Item: Eq + hash::Hash,
{
    let mut items = HashSet::new();
    iter.into_iter().all(|item| items.insert(item))
}

fn extract_discriminator<'a>(
    schema: &'a openapi::Schema,
    schemas: &'a IndexMap<String, openapi::Schema>,
) -> Option<Discriminator<'a>> {
    let (openapi::Schema {
        any_of: Some(of),
        discriminator,
        ..
    }
    | openapi::Schema {
        one_of: Some(of),
        discriminator,
        ..
    }) = schema
    else {
        return None;
    };
    let of_consts = of
        .iter()
        .map(|of| {
            let of = if let Some(ref_) = &of.ref_ {
                ref_.strip_prefix("#/components/schemas/")
                    .and_then(|ref_| schemas.get(ref_))
            } else {
                Some(of)
            }?;
            let consts = of
                .properties
                .iter()
                .flatten()
                .filter_map(|(property_name, property)| {
                    let (const_, aliases) = extract_const(property)?;
                    Some((property_name, (const_, aliases)))
                })
                .collect();
            Some((of, consts))
        })
        .collect::<Option<Vec<(_, HashMap<_, _>)>>>()?;
    let property_name = if let Some(openapi::Discriminator { property_name }) = discriminator {
        Some(property_name)
    } else if let [(_, consts), tail @ ..] = &of_consts[..] {
        consts
            .keys()
            .find(|property_name| {
                tail.iter()
                    .all(|(_, consts)| consts.contains_key(*property_name))
            })
            .copied()
    } else {
        None
    }?;
    let mapping = of_consts
        .into_iter()
        .map(|(of, mut consts)| {
            let (const_, aliases) = consts.remove(property_name)?;
            Some((of, const_, aliases))
        })
        .collect::<Option<Vec<_>>>()?;
    is_unique(
        mapping
            .iter()
            .flat_map(|(_, const_, aliases)| iter::once(const_).chain(aliases)),
    )
    .then_some(Discriminator {
        property_name,
        mapping,
    })
}

fn collect_discriminator<'a>(
    schema: &'a openapi::Schema,
    schemas: &'a IndexMap<String, openapi::Schema>,
    discriminators: &mut Vec<(&'a openapi::Schema, Discriminator<'a>)>,
) {
    if let Some(discriminator) = extract_discriminator(schema, schemas) {
        discriminators.push((schema, discriminator));
    }
    for (_, schema) in visit::iter(schema) {
        collect_discriminator(schema, schemas, discriminators);
    }
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

#[openai_openapi_generator_macros::strict(openapi::Schema)]
fn to_node(
    name: &str,
    schema: &openapi::Schema,
    discriminators: &[(&openapi::Schema, Discriminator<'_>)],
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<Node> {
    let derive = quote::quote!(#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]);

    if let openapi::Schema {
        additional_properties: None | Some(openapi::AdditionalProperties::Bool(false)),
        description,
        nullable: _,
        properties: Some(properties),
        required: _,
        type_: None | Some(openapi::Type::Object),
        x_oai_meta: _,
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        let discriminator = discriminators.iter().find_map(|(_, discriminator)| {
            discriminator
                .mapping
                .iter()
                .any(|(s, _, _)| ptr::eq(*s, schema))
                .then_some(discriminator)
        });
        let ident = to_ident_pascal(name);
        let fields = properties
            .iter()
            .filter(|(property_name, _)| {
                Some(*property_name)
                    != discriminator.map(|discriminator| discriminator.property_name)
            })
            .map(|(property_name, property)| {
                let ident = to_ident_snake(property_name);
                let description = to_description(property.description.as_deref());
                to_type(
                    &format!("{name}.{property_name}"),
                    property,
                    discriminators,
                    inline,
                )
                .with_context(|| format!("properties[{property_name:?}]"))
                .map(|type_| {
                    quote::quote! {
                        #description
                        #[serde(rename = #property_name)]
                        pub #ident: Option<#type_>
                    }
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let description = to_description(description.as_deref());
        let value = syn::parse_quote! {
            #description
            #derive
            pub struct #ident {
                #(#fields),*
            }
        };
        Ok(Node::Item { value, ident })
    } else if let openapi::Schema {
        default: _,
        description: _,
        enum_: _,
        format: None | Some(openapi::Format::Int64),
        nullable: _,
        type_: Some(openapi::Type::Integer),
        x_oai_meta: _,
    } = schema
    {
        let value = syn::parse_quote!(u64);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        default: _,
        description: _,
        format: None | Some(openapi::Format::Uri),
        nullable: _,
        type_: Some(openapi::Type::String),
        x_stainless_const: _,
    } = schema
    {
        let value = syn::parse_quote!(String);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        description: _,
        format: Some(openapi::Format::Binary),
        type_: Some(openapi::Type::String),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::File),
    } = schema
    {
        let value = syn::parse_quote!(Vec<u8>);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        default: _,
        description: _,
        items: Some(items),
        nullable: _,
        type_: None | Some(openapi::Type::Array),
    } = schema
    {
        let type_ =
            to_type(&format!("{name}.item"), items, discriminators, inline).context("items")?;
        let value = syn::parse_quote!(Vec<#type_>);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        description: _,
        type_: Some(openapi::Type::Array),
    } = schema
    {
        let value = syn::parse_quote!(Vec<serde_json::Value>);
        Ok(Node::Type { value })
    } else if let Some(ref_) = if let openapi::Schema {
        description: _,
        nullable: _,
        ref_: Some(ref_),
        type_: _,
    } = schema
    {
        ref_.strip_prefix("#/components/schemas/")
    } else {
        None
    } {
        let ref_ = to_ident_pascal(ref_);
        let value = syn::parse_quote!(#ref_);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        default: _,
        description: _,
        nullable: _,
        type_: Some(openapi::Type::Boolean),
    } = schema
    {
        let value = syn::parse_quote!(bool);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        default: _,
        description,
        enum_: Some(enum_),
        nullable: _,
        type_: None | Some(openapi::Type::String),
        x_stainless_const: _,
    } = schema
    {
        let ident = to_ident_pascal(name);
        let variants = enum_.iter().map(|enum_| {
            let ident = to_ident_pascal(enum_);
            quote::quote! {
                #[serde(rename = #enum_)]
                #ident
            }
        });
        let description = to_description(description.as_deref());
        let value = syn::parse_quote! {
            #description
            #derive
            pub enum #ident {
                #(#variants),*
            }
        };
        Ok(Node::Item { value, ident })
    } else if let openapi::Schema {
        default: _,
        description,
        discriminator: _,
        nullable: _,
        one_of: Some(one_of),
        x_oai_meta: _,
    } = schema
    {
        if let Some((_, discriminator)) = discriminators.iter().find(|(s, _)| ptr::eq(*s, schema)) {
            let ident = to_ident_pascal(name);
            let property_name = discriminator.property_name;
            let variants = one_of
                .iter()
                .zip(&discriminator.mapping)
                .enumerate()
                .map(|(i, (of, (_, const_, aliases)))| {
                    let ident = to_ident_pascal(const_);
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("oneOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #[serde(rename = #const_, #(alias = #aliases),*)]
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(tag = #property_name)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        } else {
            let ident = to_ident_pascal(name);
            let variants = one_of
                .iter()
                .enumerate()
                .map(|(i, of)| {
                    let ident = to_ident_pascal(&i.to_string());
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("oneOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(untagged)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        }
    } else if let openapi::Schema {
        any_of: Some(any_of),
        description,
        discriminator: _,
    } = schema
    {
        if let Some((_, discriminator)) = discriminators.iter().find(|(s, _)| ptr::eq(*s, schema)) {
            let ident = to_ident_pascal(name);
            let property_name = discriminator.property_name;
            let variants = any_of
                .iter()
                .zip(&discriminator.mapping)
                .enumerate()
                .map(|(i, (of, (_, const_, aliases)))| {
                    let ident = to_ident_pascal(const_);
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("anyOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #[serde(rename = #const_, #(alias = #aliases),*)]
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(tag = #property_name)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        } else {
            let ident = to_ident_pascal(name);
            let variants = any_of
                .iter()
                .enumerate()
                .map(|(i, of)| {
                    let ident = to_ident_pascal(&i.to_string());
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("anyOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(untagged)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        }
    } else if let openapi::Schema {
        any_of: Some(any_of),
        description,
    } = schema
    {
        if let Some((_, discriminator)) = discriminators.iter().find(|(s, _)| ptr::eq(*s, schema)) {
            let ident = to_ident_pascal(name);
            let property_name = discriminator.property_name;
            let variants = any_of
                .iter()
                .zip(&discriminator.mapping)
                .enumerate()
                .map(|(i, (of, (_, const_, aliases)))| {
                    let ident = to_ident_pascal(const_);
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("anyOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #[serde(rename = #const_, #(alias = #aliases),*)]
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(tag = #property_name)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        } else {
            let ident = to_ident_pascal(name);
            let variants = any_of
                .iter()
                .enumerate()
                .map(|(i, of)| {
                    let ident = to_ident_pascal(&i.to_string());
                    to_type(&format!("{name}.{i}"), of, discriminators, inline)
                        .with_context(|| format!("anyOf[{i}]"))
                        .map(|type_| {
                            quote::quote! {
                                #ident(#type_)
                            }
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let description = to_description(description.as_deref());
            let value = syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #[serde(untagged)]
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Node::Item { value, ident })
        }
    } else if let openapi::Schema {
        default: _,
        description: _,
        format: None | Some(openapi::Format::Float),
        nullable: _,
        type_: Some(openapi::Type::Number),
    } = schema
    {
        let value = syn::parse_quote!(f64);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        description: _,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        let value = syn::parse_quote!(std::collections::HashMap<String, serde_json::Value>);
        Ok(Node::Type { value })
    } else if let Some((description, all_of)) = if let openapi::Schema {
        all_of: Some(all_of),
        description,
        required: _,
    } = schema
    {
        all_of
            .iter()
            .map(|all_of| {
                if let openapi::Schema {
                    properties: Some(properties),
                    required: _,
                    type_: Some(openapi::Type::Object),
                    x_oai_meta: _,
                } = all_of
                {
                    Some(either::Left(properties))
                } else {
                    if let openapi::Schema { ref_: Some(ref_) } = all_of {
                        ref_.strip_prefix("#/components/schemas/")
                    } else {
                        None
                    }
                    .map(either::Right)
                }
            })
            .collect::<Option<Vec<_>>>()
            .map(|all_of| (description, all_of))
    } else {
        None
    } {
        let discriminator = discriminators.iter().find_map(|(_, discriminator)| {
            discriminator
                .mapping
                .iter()
                .any(|(s, _, _)| ptr::eq(*s, schema))
                .then_some(discriminator)
        });
        let ident = to_ident_pascal(name);
        let fields = all_of
            .into_iter()
            .flat_map(|all_of| {
                all_of.map_either(
                    |properties| properties.iter().map(either::Left),
                    |ref_| iter::once(either::Right(ref_)),
                )
            })
            .map(|all_of| {
                all_of.either(
                    |(property_name, property)| {
                        // .iter()
                        // .filter(|(property_name, _)| {
                        //     Some(*property_name)
                        //         != discriminator
                        //             .map(|discriminator| discriminator.property_name)
                        // })
                        // .map(|| {
                        let ident = to_ident_snake(property_name);
                        let description = to_description(property.description.as_deref());
                        to_type(
                            &format!("{name}.{property_name}"),
                            property,
                            discriminators,
                            inline,
                        )
                        .with_context(|| format!("properties[{property_name:?}]"))
                        .map(|type_| {
                            quote::quote! {
                                #description
                                #[serde(rename = #property_name)]
                                pub #ident: Option<#type_>
                            }
                        })
                    },
                    |ref_| {
                        let ident = to_ident_snake(ref_);
                        let type_ = to_ident_pascal(ref_);
                        Ok(quote::quote! {
                            #description
                            #[serde(flatten)]
                            pub #ident: Option<#type_>
                        })
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let description = to_description(description.as_deref());
        let value = syn::parse_quote! {
            #description
            #derive
            pub struct #ident {
                #(#fields),*
            }
        };
        Ok(Node::Item { value, ident })
    } else if let openapi::Schema {
        any_of: Some(_),
        default: _,
        description: _,
        nullable: _,
        x_oai_type_label: Some(openapi::XOaiTypeLabel::String),
    } = schema
    {
        let value = syn::parse_quote!(String);
        Ok(Node::Type { value })
    } else if let openapi::Schema { description: _ } = schema {
        let value = syn::parse_quote!(serde_json::Value);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        additional_properties: Some(openapi::AdditionalProperties::Schema(additional_properties)),
        description: _,
        nullable: _,
        type_: Some(openapi::Type::Object),
        x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
    } = schema
    {
        let type_ = to_type(
            &format!("{name}.item"),
            additional_properties,
            discriminators,
            inline,
        )
        .context("additionalProperties")?;
        let value = syn::parse_quote!(std::collections::HashMap<String, #type_>);
        Ok(Node::Type { value })
    } else if let openapi::Schema {
        additional_properties: None | Some(openapi::AdditionalProperties::Bool(true)),
        description: _,
        nullable: _,
        type_: Some(openapi::Type::Object),
    } = schema
    {
        let value = syn::parse_quote!(std::collections::HashMap<String, serde_json::Value>);
        Ok(Node::Type { value })
    } else {
        Err(anyhow::format_err!("unknown: {schema:#?}"))
    }
}

fn to_type(
    name: &str,
    schema: &openapi::Schema,
    discriminators: &[(&openapi::Schema, Discriminator<'_>)],
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<syn::Type> {
    match to_node(name, schema, discriminators, inline)? {
        Node::Item { ident, value } => {
            inline.push((ident.clone(), value));
            Ok(syn::parse_quote!(#ident))
        }
        Node::Type { value } => Ok(value),
    }
}
