use crate::{
    Field, Schema, Type, is_copy, is_default, to_description, to_ident_pascal, to_serde_as, to_type,
};
use indexmap::IndexMap;
use std::collections::HashMap;

pub fn to_item_enum(
    name: &str,
    schema: &Schema<'_>,
    _: &Vec<(Schema<'_>, bool)>,
    schemas: &IndexMap<&str, Schema<'_>>,
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

    let description = to_description(schema.description);
    let derive = quote::quote!(#[derive(Clone, Debug, PartialEq)]);
    let derive_copy = is_copy(schema, schemas).then_some(quote::quote!(#[derive(Copy)]));
    let derive_default = is_default(schema, schemas).then_some(quote::quote!(#[derive(Default)]));
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);

    let variants = visit_variants(schema);
    let mut items_inner = Vec::new();
    let variant_info = variants
        .iter()
        .zip(variant_names(&variants, schemas))
        .map(|((variant, default), variant_name)| {
            let (public, description) = if let Type::Const(value) = &variant.type_ {
                (false, Some(*value))
            } else {
                (true, variant.description)
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
                    if public { items } else { &mut items_inner },
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
        let arms = variant_info
            .iter()
            .map(|VariantInfo { ident, public, .. }| {
                if *public {
                    quote::quote!(_D::#ident(_v) => Self::#ident(_v))
                } else {
                    quote::quote!(_D::#ident(_) => Self::#ident)
                }
            });
        items.push(syn::parse_quote! {
            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de> {
                    #(#items_inner)*
                    #[serde_with::serde_as]
                    #[derive(serde::Deserialize)]
                    #[serde(untagged)]
                    #[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
                    enum _D {
                        #(#variants_inner),*
                    }
                    Ok(match _D::deserialize(deserializer)? {
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
        let arms = variant_info.iter().map(|VariantInfo { ident, public, .. }| {
            if *public {
                quote::quote!(Self::#ident(_v) => _S::#ident(_v).serialize(serializer))
            } else {
                quote::quote!(Self::#ident => _S::#ident(&Default::default()).serialize(serializer))
            }
        });
        items.push(syn::parse_quote! {
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer {
                    #(#items_inner)*
                    #[serde_with::serde_as]
                    #[derive(serde::Serialize)]
                    #[serde(untagged)]
                    #[allow(clippy::enum_variant_names)]
                    enum _S<'a> {
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
            #derive_copy
            #derive_default
            #[allow(clippy::large_enum_variant)]
            #vis enum #ident {
                #(#variants),*
            }
        }
    }
}

fn visit_variants<'a>(schema: &'a Schema<'a>) -> Vec<(&'a Schema<'a>, bool)> {
    match &schema.type_ {
        Type::Enum(variants) => variants
            .iter()
            .flat_map(|(variant, default_outer)| {
                visit_variants(variant)
                    .into_iter()
                    .map(|(variant, default_inner)| (variant, *default_outer && default_inner))
            })
            .collect(),
        _ => vec![(schema, true)],
    }
}

fn is_const(schema: &Schema<'_>, schemas: &IndexMap<&str, Schema<'_>>) -> bool {
    match &schema.type_ {
        Type::Const(_) => true,
        Type::Enum(variants) => variants
            .iter()
            .all(|(variant, _)| is_const(variant, schemas)),
        Type::Ref(ref_) => is_const(&schemas[ref_], schemas),
        _ => false,
    }
}

fn variant_names(
    variants: &[(&Schema<'_>, bool)],
    schemas: &IndexMap<&str, Schema<'_>>,
) -> Vec<String> {
    let mut names = vec![Vec::new(); variants.len()];

    let mut tags = variants
        .iter()
        .map(|(variant, _)| {
            visit_fields(variant, schemas)
                .into_iter()
                .filter_map(|(name, schema, required)| {
                    if ["event", "object", "role", "type"].contains(&name) && required {
                        if let Type::Const(value) = &schema.type_ {
                            Some((name, *value))
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
                    .filter(|(variant, _)| matches!(variant.type_, Type::Ref(_) | Type::Struct(_)))
                    .count()
        {
            for (name, tags) in names.iter_mut().zip(&mut tags) {
                if let Some(tag) = tags.remove(key) {
                    name.push(tag.to_owned());
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
        for (name, tags) in names.iter_mut().zip(&mut tags) {
            if let Some(tag) = tags.remove(key) {
                name.push(tag.to_owned());
            }
        }
    }

    if variants
        .iter()
        .all(|(variant, _)| matches!(&variant.type_, Type::Const(_) | Type::String))
    {
        for (name, (variant, _)) in names.iter_mut().zip(variants) {
            if matches!(&variant.type_, Type::String) {
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

fn visit_fields<'a>(
    schema: &'a Schema<'a>,
    schemas: &'a IndexMap<&'a str, Schema<'a>>,
) -> Vec<(&'a str, &'a Schema<'a>, bool)> {
    match &schema.type_ {
        Type::Ref(ref_) => visit_fields(&schemas[ref_], schemas),
        Type::Struct(fields) => fields
            .iter()
            .flat_map(|field| match field {
                Field::Property {
                    name,
                    schema,
                    required,
                } => vec![(*name, schema, *required)],
                Field::Ref(ref_) => visit_fields(&schemas[ref_], schemas),
            })
            .collect(),
        _ => Vec::new(),
    }
}

fn to_type_name<'a>(schema: &'a Schema<'a>) -> Vec<&'a str> {
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
