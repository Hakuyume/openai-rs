use quote::ToTokens;

#[proc_macro_attribute]
pub fn strict(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    struct Visitor(syn::Path);

    impl syn::visit_mut::VisitMut for Visitor {
        fn visit_pat_struct_mut(&mut self, i: &mut syn::PatStruct) {
            let syn::PatStruct {
                path, fields, rest, ..
            } = i;

            if *path == self.0 && rest.is_none() {
                let mut members = vec![
                    "additional_properties",
                    "all_of",
                    "any_of",
                    "default",
                    "description",
                    "discriminator",
                    "enum_",
                    "format",
                    "items",
                    "nullable",
                    "one_of",
                    "properties",
                    "recursive_ref",
                    "ref_",
                    "required",
                    "type_",
                    "x_oai_type_label",
                    "x_stainless_const",
                ];
                for field in fields.iter() {
                    if let syn::Member::Named(name) = &field.member {
                        members.retain(|member| name != member);
                    }
                }
                for member in members {
                    fields.push(syn::FieldPat {
                        attrs: Vec::new(),
                        member: syn::Member::Named(quote::format_ident!("{member}")),
                        colon_token: Some(syn::parse_quote!(:)),
                        pat: Box::new(syn::Pat::Path(syn::parse_quote!(None))),
                    });
                }
            }
        }
    }

    let path = syn::parse_macro_input!(attr as syn::Path);
    let mut item = syn::parse_macro_input!(input as syn::Item);
    syn::visit_mut::visit_item_mut(&mut Visitor(path), &mut item);
    item.into_token_stream().into()
}
