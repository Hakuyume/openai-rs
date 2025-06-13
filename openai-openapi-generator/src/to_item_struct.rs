use crate::{
    Field, Schema, Type, is_copy, is_default, to_description, to_ident_pascal, to_ident_snake,
    to_serde_as, to_type,
};
use indexmap::IndexMap;

pub fn to_item_struct(
    name: &str,
    schema: &Schema<'_>,
    fields: &Vec<Field>,
    schemas: &IndexMap<&str, Schema<'_>>,
    public: bool,
    items: &mut Vec<syn::Item>,
) -> syn::Item {
    struct FieldInfo<'a> {
        default: bool,
        description: Option<&'a str>,
        ident: syn::Ident,
        name: Option<&'a str>,
        option: bool,
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

    let mut items_inner = Vec::new();
    let fields = fields
        .iter()
        .map(|field| match field {
            Field::Property {
                name: field_name,
                schema,
                required,
            } => {
                let option = schema.nullable || !required;
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
                    description: schema.description,
                    ident: to_ident_snake(field_name),
                    name: Some(field_name),
                    option,
                    public,
                    serde_as: {
                        let serde_as = to_serde_as(schema);
                        if option {
                            serde_as.map(|serde_as| format!("Option<{serde_as}>"))
                        } else {
                            serde_as
                        }
                    },
                    type_: {
                        let type_ = to_type(
                            &format!("{name}.{field_name}"),
                            schema,
                            schemas,
                            public,
                            if public { items } else { &mut items_inner },
                        );

                        if option {
                            syn::parse_quote!(Option<#type_>)
                        } else {
                            type_
                        }
                    },
                }
            }
            Field::Ref(ref_) => FieldInfo {
                default: is_default(schemas.get(ref_).unwrap(), schemas),
                description: None,
                ident: to_ident_snake(ref_),
                name: None,
                option: false,
                public: true,
                serde_as: to_serde_as(schema),
                type_: {
                    let type_ = to_ident_pascal(ref_);
                    syn::parse_quote!(#type_)
                },
            },
        })
        .collect::<Vec<_>>();

    {
        let fields_inner = fields.iter().map(
            |FieldInfo {
                 ident,
                 name,
                 serde_as,
                 type_,
                 ..
             }| {
                let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                    quote::quote!(#[serde_as(as = #serde_as)]
                    )
                });
                let attr_serde_rename = name.map_or_else(
                    || quote::quote!(#[serde(flatten)]),
                    |name| quote::quote!(#[serde(rename = #name)]),
                );
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
                    #(#items_inner)*
                    #[serde_with::serde_as]
                    #[derive(serde::Deserialize)]
                    struct _D {
                        #(#fields_inner),*
                    }
                    let _D {
                        #(#idents_outer,)*
                        ..
                    } = _D::deserialize(deserializer)?;
                    Ok(#ident {
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
                 option,
                 serde_as,
                 type_,
                 ..
             }| {
                let attr_serde_as = serde_as.as_ref().map(|serde_as| {
                    quote::quote!(#[serde_as(as = #serde_as)]
                    )
                });

                let attr_serde_rename = name.map_or_else(
                    || quote::quote!(#[serde(flatten)]),
                    |name| quote::quote!(#[serde(rename = #name)]),
                );
                let attr_serde_skip_serializing_if = option
                    .then_some(quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]));
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
                    #(#items_inner)*
                    #[serde_with::serde_as]
                    #[derive(serde::Serialize)]
                    struct _S<'a> {
                        #(#fields_inner),*
                    }
                    let #ident {
                        #(#idents_outer),*
                    } = self;
                    _S { #(#field_values_inner),* }.serialize(serializer)
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
                 option,
                 public,
                 type_,
                 ..
             }| {
                let description = to_description(*description);
                let attr_builder =
                    (*option || *default).then_some(quote::quote!(#[builder(default)]));
                public.then_some(quote::quote! {
                    #description
                    #attr_builder
                    pub #ident: #type_
                })
            },
        );
        syn::parse_quote! {
            #description
            #derive
            #derive_copy
            #derive_default
            #[derive(typed_builder::TypedBuilder)]
            #vis struct #ident {
                #(#fields),*
            }
        }
    }
}
