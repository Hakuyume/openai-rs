use crate::{
    Schema, Type, extract_fields, to_derive, to_description, to_ident_pascal, to_serde_as, to_type,
};
use indexmap::IndexMap;
use std::collections::HashMap;

pub fn to_item_enum(
    name: &str,
    schema: &Schema,
    variants: &[(Schema, bool)],
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    struct VariantInfo<'a> {
        const_: bool,
        default: bool,
        description: Option<&'a str>,
        ident: syn::Ident,
        public: bool,
        serde_as: Option<String>,
        type_: syn::Type,
    }

    let description = to_description(schema.description.as_deref());
    let derive = to_derive(schema, schemas);
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);

    if let Some(variants) = variants
        .iter()
        .map(|(variant, default)| {
            if let (false, Type::Const(value)) = (variant.nullable, &variant.type_) {
                Some((variant.description.as_deref(), value.as_str(), default))
            } else {
                None
            }
        })
        .collect::<Option<Vec<_>>>()
    {
        let variants = variants.iter().map(|(description, value, default)| {
            let description = to_description(Some(description.unwrap_or(value)));
            let attr_default = default.then_some(quote::quote!(#[default]));
            let ident = to_ident_pascal(value);
            quote::quote! {
                #description
                #attr_default
                #[serde(rename = #value)]
                #ident
            }
        });
        syn::parse_quote! {
            #description
            #derive
            #[derive(serde::Deserialize, serde::Serialize)]
            #vis enum #ident {
                #(#variants),*
            }
        }
    } else {
        let variants = extract_variants(schema);
        let variant_info = variants
            .iter()
            .zip(variant_names(&variants, schemas))
            .map(|((variant, default), variant_name)| {
                let (public, description) = if let Type::Const(value) = &variant.type_ {
                    (false, Some(value.as_str()))
                } else {
                    (true, variant.description.as_deref())
                };
                VariantInfo {
                    const_: is_const(variant, schemas),
                    default: *default,
                    description,
                    ident: to_ident_pascal(&variant_name),
                    public,
                    serde_as: to_serde_as(variant),
                    type_: to_type(
                        &format!("{name}.{variant_name}"),
                        variant,
                        schemas,
                        public,
                        items,
                    ),
                }
            })
            .collect::<Vec<_>>();

        {
            let variants_inner = variant_info
                .iter()
                .filter(|VariantInfo { const_, .. }| *const_)
                .chain(
                    variant_info
                        .iter()
                        .filter(|VariantInfo { const_, .. }| !const_),
                )
                .map(
                    |VariantInfo {
                         ident,
                         serde_as,
                         type_,
                         ..
                     }| {
                        let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                            quote::quote!(#[serde_as(as = #serde_as)]
                            )
                        });
                        quote::quote! {
                            #ident(
                                #attr_serde_as
                                #[allow(dead_code)]
                                #type_
                            )
                        }
                    },
                );
            let arms = variant_info.iter().map(
                |VariantInfo {
                     ident: variant_ident,
                     public,
                     ..
                 }| {
                    if *public {
                        quote::quote!(#ident::#variant_ident(v) => Self::#variant_ident(v))
                    } else {
                        quote::quote!(#ident::#variant_ident(_) => Self::#variant_ident)
                    }
                },
            );
            items.push(syn::parse_quote! {
                impl<'de> serde::Deserialize<'de> for #ident {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where D: serde::Deserializer<'de> {
                        #[serde_with::serde_as]
                        #[derive(serde::Deserialize)]
                        #[serde(untagged)]
                        #[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
                        enum #ident {
                            #(#variants_inner),*
                        }
                        Ok(match #ident::deserialize(deserializer)? {
                            #(#arms),*
                        })
                    }
                }
            });
        }

        {
            let variants_inner = variant_info.iter().map(
                |VariantInfo {
                     ident,
                     serde_as,
                     type_,
                     ..
                 }| {
                    let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                        quote::quote!(#[serde_as(as = #serde_as)]
                        )
                    });
                    quote::quote! {
                        #ident(
                            #attr_serde_as
                            #[allow(dead_code)]
                            &'a #type_
                        )
                    }
                },
            );
            let arms = variant_info.iter().map(
                |VariantInfo {
                     ident: variant_ident,
                     public,
                     ..
                 }| {
                    if *public {
                        quote::quote! {
                            Self::#variant_ident(v) => {
                                #ident::#variant_ident(v).serialize(serializer)
                            }
                        }
                    } else {
                        quote::quote! {
                            Self::#variant_ident => {
                                #ident::#variant_ident(&Default::default()).serialize(serializer)
                            }
                        }
                    }
                },
            );
            items.push(syn::parse_quote! {
                impl serde::Serialize for #ident {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where S: serde::Serializer {
                        #[serde_with::serde_as]
                        #[derive(serde::Serialize)]
                        #[serde(untagged)]
                        #[allow(clippy::enum_variant_names)]
                        enum #ident<'a> {
                            #(#variants_inner),*
                        }
                        match self { #(#arms),* }
                    }
                }
            });
        }

        {
            let variants = variant_info.iter().map(
                |VariantInfo {
                     default,
                     description,
                     ident,
                     public,
                     type_,
                     ..
                 }| {
                    let description = to_description(*description);
                    let attr_default = default.then_some(quote::quote!(#[default]));
                    if *public {
                        quote::quote! {
                            #description
                            #attr_default
                            #ident(#type_)
                        }
                    } else {
                        quote::quote! {
                            #description
                            #attr_default
                            #ident
                        }
                    }
                },
            );
            syn::parse_quote! {
                #description
                #derive
                #[allow(clippy::large_enum_variant)]
                #vis enum #ident {
                    #(#variants),*
                }
            }
        }
    }
}

fn extract_variants(schema: &Schema) -> Vec<(&Schema, bool)> {
    match &schema.type_ {
        Type::Enum(variants) => variants
            .iter()
            .flat_map(|(variant, default_outer)| {
                extract_variants(variant)
                    .into_iter()
                    .map(|(variant, default_inner)| (variant, *default_outer && default_inner))
            })
            .collect(),
        _ => vec![(schema, true)],
    }
}

fn is_const(schema: &Schema, schemas: &IndexMap<String, Schema>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_const(variant, schemas)),
        Type::Ref(ref_) => is_const(&schemas[ref_], schemas),
        _ => false,
    }
}

fn extract_tags<'a>(
    variants: &'a [(&'a Schema, bool)],
    schemas: &'a IndexMap<String, Schema>,
) -> Vec<HashMap<&'a str, &'a str>> {
    variants
        .iter()
        .map(|(variant, _)| {
            extract_fields(variant, schemas, true)
                .into_iter()
                .filter_map(|(name, (_, schema, required))| {
                    if let (
                        "event" | "object" | "role" | "type",
                        Schema {
                            nullable: false,
                            type_: Type::Const(value),
                            ..
                        },
                        true,
                    ) = (name, schema, required)
                    {
                        Some((name, value.as_str()))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

fn variant_names(variants: &[(&Schema, bool)], schemas: &IndexMap<String, Schema>) -> Vec<String> {
    let mut names = vec![Vec::new(); variants.len()];

    let tags = extract_tags(variants, schemas);
    let key = tags
        .iter()
        .flatten()
        .fold(HashMap::<_, usize>::new(), |mut counts, (key, _)| {
            *counts.entry(*key).or_default() += 1;
            counts
        })
        .into_iter()
        .max_by_key(|(_, count)| *count);

    for (name, (variant, _)) in names.iter_mut().zip(variants) {
        if let Type::Const(value) = &variant.type_ {
            name.push((*value).to_owned());
        }
    }

    if let Some((key, count)) = key {
        if count > 1
            && count
                == variants
                    .iter()
                    .filter(|(variant, _)| {
                        matches!(variant.type_, Type::Ref(_) | Type::Struct { .. })
                    })
                    .count()
        {
            for (name, tags) in names.iter_mut().zip(&tags) {
                if let Some(tag) = tags.get(key) {
                    name.push((*tag).to_owned());
                }
            }
        }
    }

    for (name, (variant, _)) in names.iter_mut().zip(variants) {
        if let Type::Ref(ref_) = &variant.type_ {
            name.push((*ref_).to_owned());
        }
    }

    if let Some((key, _)) = key {
        for (name, tags) in names.iter_mut().zip(&tags) {
            if let Some(tag) = tags.get(key) {
                name.push((*tag).to_owned());
            }
        }
    }

    if variants
        .iter()
        .all(|(variant, _)| matches!(&variant.type_, Type::Const(_) | Type::Ref(_) | Type::String))
    {
        for (name, (variant, _)) in names.iter_mut().zip(variants) {
            if variant.description.is_none() && matches!(&variant.type_, Type::String) {
                name.push("other".to_owned());
            }
        }
    }

    for (name, (variant, _)) in names.iter_mut().zip(variants) {
        let type_name = to_type_name(variant);
        for i in 0..type_name.len() {
            name.push(type_name[..=i].join(".of."));
        }
    }

    for (i, name) in names.iter_mut().enumerate() {
        name.push(i.to_string());
    }

    let mut i = 0;
    while i < names.len() {
        let conflicts = (i..names.len())
            .filter(|j| names[i][0] == names[*j][0])
            .collect::<Vec<_>>();
        if conflicts.len() > 1 {
            for j in conflicts {
                names[j].remove(0);
            }
        } else {
            i += 1;
        }
    }
    names.into_iter().map(|mut name| name.remove(0)).collect()
}

fn to_type_name(schema: &Schema) -> Vec<&str> {
    match &schema.type_ {
        Type::Any => vec!["any"],
        Type::Array(item) => {
            let mut hint = to_type_name(item);
            hint.insert(0, "array");
            hint
        }
        Type::Binary => vec!["bytes"],
        Type::Boolean => vec!["bool"],
        Type::Const(value) => vec![value],
        Type::Float => vec!["float"],
        Type::Integer => vec!["integer"],
        Type::Map(item) => {
            let mut hint = to_type_name(item);
            hint.insert(0, "map");
            hint
        }
        Type::Number => vec!["number"],
        Type::Ref(ref_) => vec![ref_],
        Type::String => vec!["string"],
        _ => Vec::new(),
    }
}
