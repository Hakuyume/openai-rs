use crate::{Items, Schema, to_derive, to_description, to_ident_pascal, to_vis};
use indexmap::IndexMap;

pub fn to_item_const(
    (module, name): (&[&str], &str),
    schema: &Schema,
    value: &str,
    schemas: &IndexMap<String, Schema>,
    pub_: bool,
    items: &mut Items,
) {
    let description = to_description(Some(schema.description.as_deref().unwrap_or(value)));
    let derive = to_derive(schema, schemas);
    let vis = to_vis(pub_);
    let ident = to_ident_pascal(name);

    items.push(
        module,
        pub_,
        syn::parse_quote! {
            #description
            #derive
            #vis struct #ident;
        },
    );
    items.push(
        module,
        false,
        syn::parse_quote! {
            impl_serde!(#ident, #value);
        },
    );
}
