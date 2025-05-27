mod openapi;
mod visit;

use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use quote::ToTokens;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash;
use std::io;
use std::iter;
use std::ptr;

fn main() -> anyhow::Result<()> {
    let mut document = serde_yaml::from_reader::<_, openapi::Document>(io::stdin().lock())?;

    for (name, schema) in &mut document.components.schemas {
        resolve_recursive_ref(name, schema);
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

fn resolve_recursive_ref(name: &str, schema: &mut openapi::Schema) {
    if schema.recursive_ref.as_deref() == Some("#") {
        schema.recursive_ref = None;
        schema.ref_ = Some(format!("#/components/schemas/{name}"));
    }
    for (_, schema) in visit::iter_mut(schema) {
        resolve_recursive_ref(name, schema)
    }
}

fn extract_const(schema: &openapi::Schema) -> Option<(&String, Vec<&String>)> {
    if let openapi::Schema {
        default,
        enum_: Some(enum_),
        format: None,
        type_: Some(openapi::Type::String),
        x_stainless_const: Some(true),
        ..
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

fn to_node(
    name: &str,
    schema: &openapi::Schema,
    discriminators: &[(&openapi::Schema, Discriminator<'_>)],
    inline: &mut Vec<(syn::Ident, syn::Item)>,
) -> anyhow::Result<Node> {
    let derive = quote::quote!(#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]);

    match schema {
        openapi::Schema {
            ref_: Some(ref_), ..
        } => {
            let ref_ = ref_
                .strip_prefix("#/components/schemas/")
                .ok_or_else(|| anyhow::format_err!("invalid: {ref_:?}"))
                .context("$ref")?;
            let ref_ = to_ident_pascal(ref_);
            let value = syn::parse_quote!(#ref_);
            Ok(Node::Type { value })
        }

        openapi::Schema {
            x_oai_type_label: Some(openapi::XOaiTypeLabel::String),
            ..
        } => {
            let value = syn::parse_quote!(String);
            Ok(Node::Type { value })
        }

        openapi::Schema {
            additional_properties: None,
            all_of: None,
            any_of: None,
            default: None,
            description: _,
            discriminator: None,
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
            x_oai_meta: _,
            x_oai_type_label: None,
            x_stainless_const: None,
        } => {
            let value = syn::parse_quote!(serde_json::Value);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            items: None,
            type_: Some(openapi::Type::Array),
            ..
        } => {
            let value = syn::parse_quote!(Vec<serde_json::Value>);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            type_: Some(openapi::Type::Boolean),
            ..
        } => {
            let value = syn::parse_quote!(bool);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            format: None | Some(openapi::Format::Int64),
            type_: Some(openapi::Type::Integer),
            ..
        } => {
            let value = syn::parse_quote!(u64);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            format: None | Some(openapi::Format::Float),
            type_: Some(openapi::Type::Number),
            ..
        } => {
            let value = syn::parse_quote!(f64);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            additional_properties: None | Some(openapi::AdditionalProperties::Bool(true)),
            properties: None,
            type_: Some(openapi::Type::Object),
            ..
        } => {
            let value = syn::parse_quote!(std::collections::HashMap<String, serde_json::Value>);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            enum_: None,
            format: None | Some(openapi::Format::Uri),
            type_: Some(openapi::Type::String),
            ..
        } => {
            let value = syn::parse_quote!(String);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            enum_: None,
            format: Some(openapi::Format::Binary),
            type_: Some(openapi::Type::String),
            ..
        } => {
            let value = syn::parse_quote!(Vec<u8>);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            enum_: Some(enum_),
            format: None | Some(openapi::Format::Uri),
            type_: Some(openapi::Type::String),
            ..
        } => {
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

        openapi::Schema {
            items: Some(items),
            type_: None | Some(openapi::Type::Array),
            ..
        } => {
            let type_ =
                to_type(&format!("{name}.item"), items, discriminators, inline).context("items")?;
            let value = syn::parse_quote!(Vec<#type_>);
            Ok(Node::Type { value })
        }

        openapi::Schema {
            additional_properties:
                Some(openapi::AdditionalProperties::Schema(additional_properties)),
            properties: None,
            type_: Some(openapi::Type::Object),
            ..
        } => {
            let type_ = to_type(
                &format!("{name}.additionalProperties"),
                additional_properties,
                discriminators,
                inline,
            )
            .context("additionalProperties")?;
            let value = syn::parse_quote!(std::collections::HashMap<String, #type_>);
            Ok(Node::Type { value })
        }
        openapi::Schema {
            properties: Some(properties),
            type_: None | Some(openapi::Type::Object),
            ..
        } => {
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

        openapi::Schema {
            all_of: Some(all_of),
            ..
        } => {
            let ident = to_ident_pascal(name);
            let fields = all_of
                .iter()
                .enumerate()
                .map(|(i, all_of)| {
                    let ident = to_ident_snake(&i.to_string());
                    to_type(&format!("{name}[{i}]"), all_of, discriminators, inline)
                        .with_context(|| format!("allOf[{i}]"))
                        .map(|type_| {
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
        openapi::Schema {
            any_of: Some(of), ..
        }
        | openapi::Schema {
            one_of: Some(of), ..
        } => {
            if let [
                of,
                openapi::Schema {
                    type_: Some(openapi::Type::Null),
                    ..
                },
            ] = &of[..]
            {
                let type_ = to_type(&format!("{name}[0]"), of, discriminators, inline)
                    .context("anyOf/oneOf[0]")?;
                let value = if of.nullable.unwrap_or_default() {
                    syn::parse_quote!(Option<#type_>)
                } else {
                    type_
                };
                Ok(Node::Type { value })
            } else if let Some((_, discriminator)) =
                discriminators.iter().find(|(s, _)| ptr::eq(*s, schema))
            {
                let ident = to_ident_pascal(name);
                let property_name = discriminator.property_name;
                let variants = of
                    .iter()
                    .zip(&discriminator.mapping)
                    .enumerate()
                    .map(|(i, (of, (_, const_, aliases)))| {
                        let ident = to_ident_pascal(const_);
                        to_type(&format!("{name}[{i}]"), of, discriminators, inline)
                            .with_context(|| format!("anyOf/oneOf[{i}]"))
                            .map(|type_| {
                                quote::quote! {
                                    #[serde(rename = #const_, #(alias = #aliases),*)]
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
                    #[serde(tag = #property_name)]
                    pub enum #ident {
                        #(#variants),*
                    }
                };
                Ok(Node::Item { value, ident })
            } else {
                let ident = to_ident_pascal(name);
                let variants = of
                    .iter()
                    .enumerate()
                    .map(|(i, of)| {
                        let ident = to_ident_pascal(&i.to_string());
                        to_type(&format!("{name}[{i}]"), of, discriminators, inline)
                            .with_context(|| format!("anyOf/oneOf[{i}]"))
                            .map(|type_| {
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
                    #[serde(untagged)]
                    pub enum #ident {
                        #(#variants),*
                    }
                };
                Ok(Node::Item { value, ident })
            }
        }

        _ => Err(anyhow::format_err!("unknown: {schema:#?}")),
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
