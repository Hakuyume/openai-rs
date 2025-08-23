use crate::{
    Items, Schema, Type, is_default, is_nullable, to_derive, to_description, to_ident_pascal,
    to_ident_snake, to_serde_as, to_type, to_vis,
};
use indexmap::IndexMap;

pub fn to_item_struct(
    (module, name): (&[&str], &str),
    schema: &Schema,
    fields: &IndexMap<String, (Schema, bool)>,
    schemas: &IndexMap<String, Schema>,
    pub_: bool,
    items: &mut Items,
) {
    struct FieldInfo<'a> {
        default: bool,
        description: Option<String>,
        ident: syn::Ident,
        name: &'a str,
        nullable: bool,
        optional: bool,
        pub_: bool,
        serde_as: Option<String>,
        type_: syn::Type,
    }

    let description = to_description(schema.description.as_deref());
    let derive = to_derive(schema, schemas);
    let vis = to_vis(pub_);
    let ident = to_ident_pascal(name);

    let fields = fields
        .iter()
        .map(|(field_name, (schema, required))| {
            let pub_ = pub_
                && (!matches!(
                    schema,
                    Schema {
                        nullable: false,
                        type_: Type::Const(_),
                        ..
                    },
                ) || !required);
            FieldInfo {
                default: is_default(schema, schemas),
                description: schema.description.clone(),
                ident: to_ident_snake(field_name),
                name: field_name,
                nullable: is_nullable(schema, schemas),
                optional: !required,
                pub_,
                serde_as: to_serde_as(schema),
                type_: to_type(
                    (&[module, &[name]].concat(), field_name),
                    schema,
                    schemas,
                    pub_,
                    Some(items),
                ),
            }
        })
        .collect::<Vec<_>>();

    if fields.iter().all(|FieldInfo { pub_, .. }| *pub_) {
        let fields = fields.iter().map(
            |FieldInfo {
                 default,
                 description,
                 ident,
                 name,
                 nullable,
                 optional,
                 serde_as,
                 type_,
                 ..
             }| {
                let description = to_description(description.as_deref());
                let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                    let serde_as = if *nullable || *optional {
                        format!("Option<{serde_as}>")
                    } else {
                        serde_as.clone()
                    };
                    quote::quote!(#[serde_as(as = #serde_as)])
                });
                let attr_serde_skip_serializing_if = (*optional || *nullable)
                    .then_some(quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]));
                let attr_builder = (*default || *nullable || *optional)
                    .then_some(quote::quote!(#[builder(default)]));
                let type_ = if *nullable || *optional {
                    quote::quote!(Option<#type_>)
                } else {
                    quote::quote!(#type_)
                };
                quote::quote! {
                    #description
                    #attr_serde_as
                    #[serde(rename = #name)]
                    #attr_serde_skip_serializing_if
                    #attr_builder
                    #vis #ident: #type_
                }
            },
        );
        items.push(
            module,
            true,
            syn::parse_quote! {
                #description
                #derive
                #[serde_with::serde_as]
                #[derive(serde::Deserialize, serde::Serialize)]
                #[derive(typed_builder::TypedBuilder)]
                #vis struct #ident {
                    #(#fields),*
                }
            },
        );
    } else {
        {
            let fields = fields.iter().filter_map(
                |FieldInfo {
                     default,
                     description,
                     ident,
                     nullable,
                     optional,
                     pub_,
                     type_,
                     ..
                 }| {
                    let description = to_description(description.as_deref());
                    let attr_builder = (*default || *nullable || *optional)
                        .then_some(quote::quote!(#[builder(default)]));
                    let type_ = if *nullable || *optional {
                        quote::quote!(Option<#type_>)
                    } else {
                        quote::quote!(#type_)
                    };
                    pub_.then_some(quote::quote! {
                        #description
                        #attr_builder
                        #vis #ident: #type_
                    })
                },
            );
            items.push(
                module,
                true,
                syn::parse_quote! {
                    #description
                    #derive
                    #[derive(typed_builder::TypedBuilder)]
                    #vis struct #ident {
                        #(#fields),*
                    }
                },
            );
        }

        {
            let fields_inner = fields.iter().map(
                |FieldInfo {
                     ident,
                     name,
                     nullable,
                     optional,
                     pub_,
                     serde_as,
                     type_,
                     ..
                 }| {
                    let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                        let serde_as = if *nullable || *optional {
                            format!("Option<{serde_as}>")
                        } else {
                            serde_as.clone()
                        };
                        quote::quote!(#[serde_as(as = #serde_as)])
                    });
                    let type_ = if *nullable || *optional {
                        quote::quote!(Option<#type_>)
                    } else {
                        quote::quote!(#type_)
                    };
                    if *pub_ {
                        quote::quote! {
                            #attr_serde_as
                            #[serde(rename = #name)]
                            #ident: #type_
                        }
                    } else {
                        quote::quote! {
                            #attr_serde_as
                            #[serde(rename = #name)]
                            #[allow(dead_code)]
                            #ident: #type_
                        }
                    }
                },
            );
            let idents_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, pub_, .. }| pub_.then_some(ident));
            let field_values_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, pub_, .. }| pub_.then_some(ident));
            items.push(
                module,
                false,
                syn::parse_quote! {
                    impl<'de> serde::Deserialize<'de> for #ident {
                        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                        where D: serde::Deserializer<'de> {
                            #[serde_with::serde_as]
                            #[derive(serde::Deserialize)]
                            struct #ident {
                                #(#fields_inner),*
                            }
                            let #ident {
                                #(#idents_outer,)*
                                ..
                            } = #ident::deserialize(deserializer)?;
                            Ok(Self {
                                #(#field_values_outer),*
                            })
                        }
                    }
                },
            );
        }

        {
            let fields_inner = fields.iter().map(
                |FieldInfo {
                     ident,
                     name,
                     nullable,
                     optional,
                     serde_as,
                     type_,
                     ..
                 }| {
                    let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                        let serde_as = if *nullable || *optional {
                            format!("Option<{serde_as}>")
                        } else {
                            serde_as.clone()
                        };
                        quote::quote!(#[serde_as(as = #serde_as)])
                    });
                    let attr_serde_skip_serializing_if = (*optional || *nullable).then_some(
                        quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]),
                    );
                    let type_ = if *nullable || *optional {
                        quote::quote!(Option<#type_>)
                    } else {
                        quote::quote!(#type_)
                    };
                    quote::quote! {
                        #attr_serde_as
                        #[serde(rename = #name)]
                        #attr_serde_skip_serializing_if
                        #ident: &'a #type_
                    }
                },
            );
            let idents_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, pub_, .. }| pub_.then_some(ident));
            let field_values_inner = fields.iter().map(|FieldInfo { ident, pub_, .. }| {
                if *pub_ {
                    quote::quote!(#ident)
                } else {
                    quote::quote!(#ident: &Default::default())
                }
            });
            items.push(
                module,
                false,
                syn::parse_quote! {
                    impl serde::Serialize for #ident {
                        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                        where S: serde::Serializer {
                            #[serde_with::serde_as]
                            #[derive(serde::Serialize)]
                            struct #ident<'a> {
                                #(#fields_inner),*
                            }
                            let Self {
                                #(#idents_outer),*
                            } = self;
                            #ident { #(#field_values_inner),* }.serialize(serializer)
                        }
                    }
                },
            );
        }
    }
}
