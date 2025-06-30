use crate::{
    Schema, Type, extract_fields, is_default, is_nullable, to_derive, to_description,
    to_ident_pascal, to_ident_snake, to_serde_as, to_type,
};
use indexmap::{IndexMap, IndexSet};

pub fn to_item_struct(
    name: &str,
    schema: &Schema,
    _: &[either::Either<(String, Schema), String>],
    required: &IndexSet<String>,
    schemas: &IndexMap<String, Schema>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    struct FieldInfo<'a> {
        default: bool,
        description: Option<&'a str>,
        ident: syn::Ident,
        name: &'a str,
        nullable: bool,
        optional: bool,
        public: bool,
        serde_as: Option<String>,
        type_: syn::Type,
    }

    let description = to_description(schema.description.as_deref());
    let derive = to_derive(schema, schemas);
    let vis = public.then_some(quote::quote!(pub));
    let ident = to_ident_pascal(name);

    let mut fields = extract_fields(schema, schemas, false)
        .into_iter()
        .map(|(field_name, (ref_, schema, required))| {
            let public = public
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
                description: schema.description.as_deref(),
                ident: to_ident_snake(field_name),
                name: field_name,
                nullable: is_nullable(schema, schemas),
                optional: !required,
                public,
                serde_as: to_serde_as(schema),
                type_: if let Some(ref_) = ref_ {
                    to_type(
                        &format!("{ref_}.{field_name}"),
                        schema,
                        schemas,
                        public,
                        &mut Vec::new(),
                    )
                } else {
                    to_type(
                        &format!("{name}.{field_name}"),
                        schema,
                        schemas,
                        public,
                        items,
                    )
                },
            }
        })
        .collect::<Vec<_>>();

    // complete missing required fields
    let mut required = required.clone();
    for FieldInfo { name, .. } in &fields {
        required.shift_remove(*name);
    }
    for field_name in &required {
        let schema = crate::Schema {
            description: None,
            nullable: false,
            type_: crate::Type::Any,
        };
        fields.push(FieldInfo {
            default: false,
            description: None,
            ident: to_ident_snake(field_name),
            nullable: false,
            optional: false,
            public: true,
            serde_as: to_serde_as(&schema),
            name: field_name,
            type_: to_type(
                &format!("{name}.{field_name}"),
                &schema,
                schemas,
                true,
                &mut Vec::new(),
            ),
        });
    }

    if fields.iter().all(|FieldInfo { public, .. }| *public) {
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
                let description = to_description(*description);
                let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                    let serde_as = if *nullable || *optional {
                        format!("Option<{serde_as}>")
                    } else {
                        serde_as.clone()
                    };
                    quote::quote!(#[serde_as(as = #serde_as)])
                });
                let attr_serde_rename = quote::quote!(#[serde(rename = #name)]);
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
                    #attr_serde_rename
                    #attr_serde_skip_serializing_if
                    #attr_builder
                    #vis #ident: #type_
                }
            },
        );
        syn::parse_quote! {
            #description
            #derive
            #[serde_with::serde_as]
            #[derive(serde::Deserialize, serde::Serialize)]
            #[derive(typed_builder::TypedBuilder)]
            #vis struct #ident {
                #(#fields),*
            }
        }
    } else {
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
                    let attr_serde_rename = quote::quote!(#[serde(rename = #name)]);
                    let type_ = if *nullable || *optional {
                        quote::quote!(Option<#type_>)
                    } else {
                        quote::quote!(#type_)
                    };
                    quote::quote! {
                        #attr_serde_as
                        #attr_serde_rename
                        #[allow(dead_code)]
                        #ident: #type_
                    }
                },
            );
            let idents_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, public, .. }| public.then_some(ident));
            let field_values_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, public, .. }| public.then_some(ident));
            items.push(syn::parse_quote! {
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
            });
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
                    let attr_serde_rename = quote::quote!(#[serde(rename = #name)]);
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
                        #attr_serde_rename
                        #attr_serde_skip_serializing_if
                        #ident: &'a #type_
                    }
                },
            );
            let idents_outer = fields
                .iter()
                .filter_map(|FieldInfo { ident, public, .. }| public.then_some(ident));
            let field_values_inner = fields.iter().map(|FieldInfo { ident, public, .. }| {
                if *public {
                    quote::quote!(#ident)
                } else {
                    quote::quote!(#ident: &Default::default())
                }
            });
            items.push(syn::parse_quote! {
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
            });
        }

        {
            let fields = fields.iter().filter_map(
                |FieldInfo {
                     default,
                     description,
                     ident,
                     nullable,
                     optional,
                     public,
                     type_,
                     ..
                 }| {
                    let description = to_description(*description);
                    let attr_builder = (*default || *nullable || *optional)
                        .then_some(quote::quote!(#[builder(default)]));
                    let type_ = if *nullable || *optional {
                        quote::quote!(Option<#type_>)
                    } else {
                        quote::quote!(#type_)
                    };
                    public.then_some(quote::quote! {
                        #description
                        #attr_builder
                        #vis #ident: #type_
                    })
                },
            );
            syn::parse_quote! {
                #description
                #derive
                #[derive(typed_builder::TypedBuilder)]
                #vis struct #ident {
                    #(#fields),*
                }
            }
        }
    }
}
