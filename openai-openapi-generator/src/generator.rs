mod discriminator;

use crate::openapi;
use anyhow::Context;
use heck::{ToPascalCase, ToSnakeCase};
use indexmap::IndexMap;
use std::ptr;

pub(super) struct Generator<'a> {
    schemas: &'a IndexMap<String, openapi::Schema>,
    discriminators: Vec<(&'a openapi::Schema, discriminator::Discriminator<'a>)>,
}

impl<'a> Generator<'a> {
    pub(super) fn new(schemas: &'a IndexMap<String, openapi::Schema>) -> Self {
        let mut discriminators = Vec::new();
        for schema in schemas.values() {
            discriminator::collect_discriminator(schema, schemas, &mut discriminators);
        }
        Self {
            schemas,
            discriminators,
        }
    }

    pub(super) fn types(&self) -> anyhow::Result<Vec<syn::Item>> {
        let mut items = Vec::new();
        for (name, schema) in self.schemas {
            let (ident, value) = match self
                .to_node(schema, name, &mut items)
                .with_context(|| name.clone())?
            {
                Node::Item { ident, value } => (ident, value),
                Node::Type { value } => {
                    let description = Self::to_description(schema.description.as_deref());
                    let ident = Self::to_ident_pascal(name);
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
        Ok(items.into_iter().map(|(_, value)| value).collect())
    }
}

impl Generator<'_> {
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

    fn derive() -> syn::Attribute {
        syn::parse_quote!(#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)])
    }
}

enum Node {
    Item { ident: syn::Ident, value: syn::Item },
    Type { value: syn::Type },
}

impl Generator<'_> {
    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<Node> {
        if let Some(node) = self.to_node_primitive(schema) {
            Ok(node)
        } else if let Some(node) = self.to_node_array(schema, name, inline)? {
            Ok(node)
        } else if let Some(node) = self.to_node_map(schema, name, inline)? {
            Ok(node)
        } else if let Some(node) = self.to_node_ref(schema)? {
            Ok(node)
        } else if let Some(node) = self.to_node_enum(schema, name, inline)? {
            Ok(node)
        } else if let Some(node) = self.to_node_struct(schema, name, inline)? {
            Ok(node)
        } else {
            Err(anyhow::format_err!("unknown: {schema:#?}"))
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_primitive(&self, schema: &openapi::Schema) -> Option<Node> {
        if let openapi::Schema { description: _ } = schema {
            // Any
            let value = syn::parse_quote!(serde_json::Value);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            description: _,
            type_: Some(openapi::Type::Array),
        } = schema
        {
            // Array
            let value = syn::parse_quote!(Vec<serde_json::Value>);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            description: _,
            format: Some(openapi::Format::Binary),
            type_: Some(openapi::Type::String),
            x_oai_type_label: None | Some(openapi::XOaiTypeLabel::File),
        } = schema
        {
            // Binary
            let value = syn::parse_quote!(Vec<u8>);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            default: _,
            description: _,
            nullable: _,
            type_: Some(openapi::Type::Boolean),
        } = schema
        {
            // Boolean
            let value = syn::parse_quote!(bool);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            default: _,
            description: _,
            format: None | Some(openapi::Format::Float),
            nullable: _,
            type_: Some(openapi::Type::Number),
        } = schema
        {
            // Float
            let value = syn::parse_quote!(f64);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            default: _,
            description: _,
            enum_: _,
            format: None | Some(openapi::Format::Int64),
            nullable: _,
            type_: Some(openapi::Type::Integer),
        } = schema
        {
            // Integer
            let value = syn::parse_quote!(u64);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            additional_properties: None | Some(openapi::AdditionalProperties::Bool(true)),
            description: _,
            nullable: _,
            type_: Some(openapi::Type::Object),
        }
        | openapi::Schema {
            description: _,
            type_: Some(openapi::Type::Object),
            x_oai_type_label: Some(openapi::XOaiTypeLabel::Map),
        } = schema
        {
            // Map
            let value = syn::parse_quote!(std::collections::HashMap<String, serde_json::Value>);
            Some(Node::Type { value })
        } else if let openapi::Schema {
            default: _,
            description: _,
            format: None | Some(openapi::Format::Uri),
            nullable: _,
            type_: Some(openapi::Type::String),
            x_stainless_const: _,
        }
        | openapi::Schema {
            any_of: Some(_),
            default: _,
            description: _,
            nullable: _,
            x_oai_type_label: Some(openapi::XOaiTypeLabel::String),
        } = schema
        {
            // String
            let value = syn::parse_quote!(String);
            Some(Node::Type { value })
        } else {
            None
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_array(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<Option<Node>> {
        if let openapi::Schema {
            default: _,
            description: _,
            items: Some(items),
            nullable: _,
            type_: None | Some(openapi::Type::Array),
        } = schema
        {
            let type_ = self
                .to_type(items, &format!("{name}.item"), inline)
                .context("items")?;
            let value = syn::parse_quote!(Vec<#type_>);
            Ok(Some(Node::Type { value }))
        } else {
            Ok(None)
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_map(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<Option<Node>> {
        if let openapi::Schema {
            additional_properties:
                Some(openapi::AdditionalProperties::Schema(additional_properties)),
            description: _,
            nullable: _,
            type_: Some(openapi::Type::Object),
            x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
        } = schema
        {
            let type_ = self
                .to_type(additional_properties, &format!("{name}.item"), inline)
                .context("additionalProperties")?;
            let value = syn::parse_quote!(std::collections::HashMap<String, #type_>);
            Ok(Some(Node::Type { value }))
        } else {
            Ok(None)
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_ref(&self, schema: &openapi::Schema) -> anyhow::Result<Option<Node>> {
        if let Some(ref_) = if let openapi::Schema {
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
            let ref_ = Self::to_ident_pascal(ref_);
            let value = syn::parse_quote!(#ref_);
            Ok(Some(Node::Type { value }))
        } else {
            Ok(None)
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_enum(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<Option<Node>> {
        if let openapi::Schema {
            default: _,
            description,
            enum_: Some(enum_),
            nullable: _,
            type_: None | Some(openapi::Type::String),
            x_stainless_const: _,
        } = schema
        {
            let derive = Self::derive();
            let description = Self::to_description(description.as_deref());
            let ident = Self::to_ident_pascal(name);
            let variants = enum_.iter().map(|enum_| {
                let ident = Self::to_ident_pascal(enum_);
                quote::quote! {
                    #[serde(rename = #enum_)]
                    #ident
                }
            });
            let value = syn::parse_quote! {
                #description
                #derive
                pub enum #ident {
                    #(#variants),*
                }
            };
            Ok(Some(Node::Item { value, ident }))
        } else if let Some((description, of, context)) = if let openapi::Schema {
            any_of: Some(any_of),
            description,
            discriminator: _,
        } = schema
        {
            Some((description, any_of, "anyOf"))
        } else if let openapi::Schema {
            default: _,
            description,
            discriminator: _,
            nullable: _,
            one_of: Some(one_of),
        } = schema
        {
            Some((description, one_of, "oneOf"))
        } else {
            None
        } {
            let description = Self::to_description(description.as_deref());
            let derive = Self::derive();
            let ident = Self::to_ident_pascal(name);
            if let Some((_, discriminator)) = self
                .discriminators
                .iter()
                .find(|(s, _)| ptr::eq(*s, schema))
            {
                let property_name = discriminator.property_name;
                let variants = of
                    .iter()
                    .zip(&discriminator.mapping)
                    .enumerate()
                    .map(|(i, (of, (_, const_, aliases)))| {
                        let ident = Self::to_ident_pascal(const_);
                        self.to_type(of, &format!("{name}.{i}"), inline)
                            .with_context(|| format!("{context}[{i}]"))
                            .map(|type_| {
                                quote::quote! {
                                    #[serde(rename = #const_, #(alias = #aliases),*)]
                                    #ident(#type_)
                                }
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let value = syn::parse_quote! {
                    #description
                    #derive
                    #[allow(clippy::large_enum_variant)]
                    #[serde(tag = #property_name)]
                    pub enum #ident {
                        #(#variants),*
                    }
                };
                Ok(Some(Node::Item { value, ident }))
            } else {
                let variants = of
                    .iter()
                    .enumerate()
                    .map(|(i, of)| {
                        let ident = Self::to_ident_pascal(&i.to_string());
                        self.to_type(of, &format!("{name}.{i}"), inline)
                            .with_context(|| format!("{context}[{i}]"))
                            .map(|type_| {
                                quote::quote! {
                                    #ident(#type_)
                                }
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let value = syn::parse_quote! {
                    #description
                    #derive
                    #[allow(clippy::large_enum_variant)]
                    #[serde(untagged)]
                    pub enum #ident {
                        #(#variants),*
                    }
                };
                Ok(Some(Node::Item { value, ident }))
            }
        } else {
            Ok(None)
        }
    }

    #[openai_openapi_generator_macros::strict(openapi::Schema)]
    fn to_node_struct(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<Option<Node>> {
        if let Some((description, fields)) = if let openapi::Schema {
            all_of: Some(all_of),
            description,
            required: required_outer,
        } = schema
        {
            all_of
                .iter()
                .enumerate()
                .try_fold(Vec::new(), |mut fields, (i, all_of)| {
                    if let openapi::Schema {
                        properties: Some(properties),
                        required: required_inner,
                        type_: Some(openapi::Type::Object),
                    } = all_of
                    {
                        for (property_name, property) in properties {
                            fields.push((
                                either::Left((
                                    property_name,
                                    property,
                                    required_outer
                                        .as_ref()
                                        .is_some_and(|required| required.contains(property_name))
                                        || required_inner.as_ref().is_some_and(|required| {
                                            required.contains(property_name)
                                        }),
                                )),
                                vec![
                                    format!("properties[{property_name:?}]"),
                                    format!("allOf[{i}]"),
                                ],
                            ));
                        }
                        Some(fields)
                    } else if let Some(ref_) = if let openapi::Schema { ref_: Some(ref_) } = all_of
                    {
                        ref_.strip_prefix("#/components/schemas/")
                    } else {
                        None
                    } {
                        fields.push((
                            either::Right(ref_),
                            vec!["ref".to_owned(), format!("allOf[{i}]")],
                        ));
                        Some(fields)
                    } else {
                        None
                    }
                })
                .map(|fields| (description, fields))
        } else if let openapi::Schema {
            additional_properties: None | Some(openapi::AdditionalProperties::Bool(false)),
            description,
            nullable: _,
            properties: Some(properties),
            required,
            type_: None | Some(openapi::Type::Object),
            x_oai_type_label: None | Some(openapi::XOaiTypeLabel::Map),
        } = schema
        {
            let fields = properties
                .iter()
                .map(|(property_name, property)| {
                    (
                        either::Left((
                            property_name,
                            property,
                            required
                                .as_ref()
                                .is_some_and(|required| required.contains(property_name)),
                        )),
                        vec![format!("properties[{property_name:?}]")],
                    )
                })
                .collect();
            Some((description, fields))
        } else {
            None
        } {
            let description = Self::to_description(description.as_deref());
            let derive = Self::derive();
            let ident = Self::to_ident_pascal(name);
            let discriminator = self.discriminators.iter().find_map(|(_, discriminator)| {
                discriminator
                    .mapping
                    .iter()
                    .any(|(s, _, _)| ptr::eq(*s, schema))
                    .then_some(discriminator)
            });
            let fields = fields
                .into_iter()
                .filter(|(field, _)| {
                    if let either::Left((property_name, _, _)) = field {
                        Some(*property_name)
                            != discriminator.map(|discriminator| discriminator.property_name)
                    } else {
                        true
                    }
                })
                .map(|(field, contexts)| {
                    field.either(
                        |(property_name, property, required)| {
                            let ident = Self::to_ident_snake(property_name);
                            let description = Self::to_description(property.description.as_deref());
                            contexts
                                .into_iter()
                                .fold(
                                    self.to_type(
                                        property,
                                        &format!("{name}.{property_name}"),
                                        inline,
                                    ),
                                    |output, context| output.context(context),
                                )
                                .map(|type_| {
                                    if required && property.nullable != Some(true) {
                                        quote::quote! {
                                            #description
                                            #[serde(rename = #property_name)]
                                            pub #ident: #type_
                                        }
                                    } else {
                                        quote::quote! {
                                            #description
                                            #[serde(rename = #property_name)]
                                            pub #ident: Option<#type_>
                                        }
                                    }
                                })
                        },
                        |ref_| {
                            let ident = Self::to_ident_snake(ref_);
                            let type_ = Self::to_ident_pascal(ref_);
                            Ok(quote::quote! {
                                #description
                                #[serde(flatten)]
                                pub #ident: Option<#type_>
                            })
                        },
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            let value = syn::parse_quote! {
                #description
                #derive
                pub struct #ident {
                    #(#fields),*
                }
            };
            Ok(Some(Node::Item { value, ident }))
        } else {
            Ok(None)
        }
    }

    fn to_type(
        &self,
        schema: &openapi::Schema,
        name: &str,
        inline: &mut Vec<(syn::Ident, syn::Item)>,
    ) -> anyhow::Result<syn::Type> {
        match self.to_node(schema, name, inline)? {
            Node::Item { ident, value } => {
                inline.push((ident.clone(), value));
                Ok(syn::parse_quote!(#ident))
            }
            Node::Type { value } => Ok(value),
        }
    }
}
