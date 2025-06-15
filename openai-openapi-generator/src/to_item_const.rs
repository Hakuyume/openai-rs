use crate::{Schema, is_copy, is_default, to_description, to_ident_pascal};
use indexmap::IndexMap;

pub fn to_item_const(
    name: &str,
    schema: &Schema<'_>,
    value: &str,
    schemas: &IndexMap<&str, Schema<'_>>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    let description = to_description(Some(schema.description.unwrap_or(value)));
    let derive = quote::quote!(#[derive(Clone, Debug, PartialEq)]);
    let derive_copy = is_copy(schema, schemas).then_some(quote::quote!(#[derive(Copy)]));
    let derive_default = is_default(schema, schemas).then_some(quote::quote!(#[derive(Default)]));
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);

    {
        items.push(syn::parse_quote! {
            impl<'de> serde::Deserialize<'de> for #ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: serde::Deserializer<'de> {
                    let value = String::deserialize(deserializer)?;
                    if value == #value {
                        Ok(Self)
                    } else {
                        Err(<D::Error as serde::de::Error>::invalid_value(
                            serde::de::Unexpected::Str(&value),
                            &#value,
                        ))
                    }
                }
            }
        });
    }

    {
        items.push(syn::parse_quote! {
            impl serde::Serialize for #ident {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: serde::Serializer {
                    #value.serialize(serializer)
                }
            }
        });
    }

    {
        syn::parse_quote! {
            #description
            #derive
            #derive_copy
            #derive_default
            #vis struct #ident;
        }
    }
}
