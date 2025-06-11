use crate::{misc, openapi, visit};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::iter;

pub(super) struct Discriminator<'a> {
    pub(super) property_name: &'a String,
    pub(super) mapping: Vec<(&'a openapi::Schema, &'a String, Vec<&'a String>)>,
}

pub(super) fn collect_discriminator<'a>(
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
    misc::is_unique(
        mapping
            .iter()
            .flat_map(|(_, const_, aliases)| iter::once(const_).chain(aliases)),
    )
    .then_some(Discriminator {
        property_name,
        mapping,
    })
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
