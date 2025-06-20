use crate::{Schema, to_derive, to_description, to_ident_pascal};
use indexmap::IndexMap;

pub fn to_item_const(
    name: &str,
    schema: &Schema,
    value: &str,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    let description = to_description(Some(schema.description.as_deref().unwrap_or(value)));
    let derive = to_derive(schema, schemas);
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
            #vis struct #ident;
        }
    }
}
