use crate::{Schema, to_derive, to_description, to_ident_pascal};
use indexmap::IndexMap;

pub fn to_item_const(
    name: &str,
    schema: &Schema,
    value: &str,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) {
    let description = to_description(Some(schema.description.as_deref().unwrap_or(value)));
    let derive = to_derive(schema, schemas);
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);

    items.push(syn::parse_quote! {
        #description
        #derive
        #vis struct #ident;
    });
    items.push(syn::parse_quote! {
        impl_serde!(#ident, #value);
    });
}
