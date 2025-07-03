use crate::{
    Operation, Schema, Type, openapi, to_description, to_ident_pascal, to_ident_snake, to_serde_as,
    to_type,
};
use indexmap::{IndexMap, IndexSet};

pub fn to_item_fn(
    operation: &Operation,
    schemas: &IndexMap<String, Schema>,
    items: &mut Vec<syn::Item>,
) {
    let method = match operation.method {
        openapi::Method::Delete => quote::quote!(http::Method::DELETE),
        openapi::Method::Get => quote::quote!(http::Method::GET),
        openapi::Method::Post => quote::quote!(http::Method::POST),
    };

    let (params, path) = if let Some(parameters) = &operation.parameters {
        let params = to_type(
            &format!("{}.params", operation.id),
            &Schema {
                description: None,
                nullable: false,
                type_: Type::Struct {
                    fields: Vec::new(),
                    required: IndexSet::new(),
                },
            },
            schemas,
            true,
            None,
        );
        let path = &operation.path;
        let params_path = parameters
            .iter()
            .filter(|parameter| matches!(parameter.in_, openapi::In::Path))
            .map(|parameter| {
                let name = quote::format_ident!("{}", parameter.name);
                let ident = to_ident_snake(&parameter.name);
                quote::quote!(#name = params.#ident)
            });
        let fields_query = parameters
            .iter()
            .filter(|parameter| matches!(parameter.in_, openapi::In::Query))
            .map(|parameter| {
                let attr_serde_as = to_serde_as(&parameter.schema).map(|serde_as| {
                    let serde_as = if parameter.required {
                        serde_as
                    } else {
                        format!("Option<{serde_as}>")
                    };
                    quote::quote!(#[serde_as(as = #serde_as)])
                });
                let name = &parameter.name;
                let attr_serde_skip_serializing_if = (!parameter.required)
                    .then_some(quote::quote!(#[serde(skip_serializing_if = "Option::is_none")]));
                let ident = to_ident_snake(&parameter.name);
                let type_ = {
                    let type_ = to_type(
                        &format!("{}.params.{}", operation.id, parameter.name),
                        &parameter.schema,
                        schemas,
                        false,
                        None,
                    );
                    if parameter.required {
                        quote::quote!(#type_)
                    } else {
                        quote::quote!(Option<#type_>)
                    }
                };
                quote::quote! {
                    #attr_serde_as
                    #[serde(rename = #name)]
                    #attr_serde_skip_serializing_if
                    #ident: &'a #type_
                }
            });
        let idents_query = parameters
            .iter()
            .filter(|parameter| matches!(parameter.in_, openapi::In::Query))
            .map(|parameter| to_ident_snake(&parameter.name));
        let field_values_query = parameters
            .iter()
            .filter(|parameter| matches!(parameter.in_, openapi::In::Query))
            .map(|parameter| to_ident_snake(&parameter.name));
        (
            Some(quote::quote!(params: &#params,)),
            quote::quote! {
                let path = {
                    #[serde_with::serde_as]
                    #[derive(serde::Serialize)]
                    struct Query<'a> {
                        #(#fields_query,)*
                        #[serde(skip_serializing)]
                        _phantom: std::marker::PhantomData<&'a ()>,
                    }

                    #[allow(clippy::useless_format)]
                    let mut path = format!(#path, #(#params_path),*);
                    let #params {
                        #(#idents_query,)*
                        ..
                    } = params;
                    let query = serde_urlencoded::to_string(
                        Query {
                            #(#field_values_query,)*
                            _phantom: std::marker::PhantomData,
                        }
                    )?;
                    if !query.is_empty() {
                        path.push('?');
                        path.push_str(&query);
                    }
                    path
                };
            },
        )
    } else {
        let path = &operation.path;
        (
            None,
            quote::quote! {
                let path = #path;
            },
        )
    };

    let (request, content_type, body) = match operation.requests.as_deref() {
        Some([(openapi::ContentType::ApplicationJson, request, true)]) => {
            let request = to_type(
                &format!("{}.request", operation.id),
                request,
                schemas,
                true,
                None,
            );
            (
                Some(quote::quote!(request: &#request,)),
                Some(quote::quote!(.header(http::header::CONTENT_TYPE, "application/json"))),
                quote::quote! {
                    let body = serde_json::to_string(request)?;
                },
            )
        }
        None => (
            None,
            None,
            quote::quote! {
                let body = String::new();
            },
        ),
        _ => return,
    };

    for (status, response) in &operation.responses {
        if let (200..300, Some((openapi::ContentType::ApplicationJson, response))) =
            (status, response)
        {
            let description = to_description(Some(&operation.description));
            let ident_fn = to_ident_snake(&operation.id);
            let ident_future = to_ident_pascal(&operation.id);
            let response = to_type(
                &format!("{}.response", operation.id),
                response,
                schemas,
                true,
                None,
            );

            items.push(syn::parse_quote! {
                #description
                pub fn #ident_fn<S, Fut, B, E>(
                    service: S,
                    #params
                    #request
                ) -> #ident_future<Fut, B, E>
                where
                S: FnOnce(http::Request<String>) -> Fut,
                Fut: Future<Output = Result<http::Response<B>, E>>,
                B: http_body::Body,
                {
                    #ident_future(
                        futures::TryFutureExt::and_then(
                            crate::__combinators::Send::new(
                                service,
                                || {
                                    #path
                                    #body
                                    Ok(
                                        http::Request::builder()
                                        .method(#method)
                                        .uri(path)
                                        .header(http::header::CONTENT_LENGTH, body.len())
                                        #content_type
                                        .body(body)?
                                    )
                                },
                                http::StatusCode::from_u16(#status).unwrap(),
                            ),
                            crate::__combinators::Json::new,
                        )
                    )
                }
            });

            items.push(syn::parse_quote! {
                future!(
                    #ident_future,
                    futures::future::AndThen<
                    crate::__combinators::Send<Fut, B, E>,
                    crate::__combinators::Json<B, E, #response>,
                    fn(B) -> crate::__combinators::Json<B, E, #response>,
                    >,
                    #response
                );
            });
        }

        if let (200..300, Some((openapi::ContentType::TextEventStream, response))) =
            (status, response)
        {
            let description = to_description(Some(&operation.description));
            let ident_fn = to_ident_snake(&format!("{}.stream", operation.id));
            let ident_future = to_ident_pascal(&format!("{}.stream", operation.id));
            let response = to_type(
                &format!("{}response", operation.id),
                response,
                schemas,
                true,
                None,
            );

            items.push(syn::parse_quote! {
                #description
                pub fn #ident_fn<S, Fut, B, E>(
                    service: S,
                    #params
                    #request
                ) -> #ident_future<Fut, B, E>
                where
                S: FnOnce(http::Request<String>) -> Fut,
                Fut: Future<Output = Result<http::Response<B>, E>>,
                B: http_body::Body,
                {
                    #ident_future(
                        futures::TryFutureExt::map_ok(
                            crate::__combinators::Send::new(
                                service,
                                || {
                                    #path
                                    #body
                                    Ok(
                                        http::Request::builder()
                                        .method(#method)
                                        .uri(path)
                                        .header(http::header::CONTENT_LENGTH, body.len())
                                        #content_type
                                        .body(body)?
                                    )
                                },
                                http::StatusCode::from_u16(#status).unwrap(),
                            ),
                            crate::__combinators::EventStream::new,
                        )
                    )
                }
            });

            items.push(syn::parse_quote! {
                future!(
                    #ident_future,
                    futures::future::MapOk<
                    crate::__combinators::Send<Fut, B, E>,
                    fn(B) -> crate::__combinators::EventStream<B, #response>,
                    >,
                    crate::__combinators::EventStream<B, #response>
                );
            });
        }
    }
}
