use crate::{Schema, openapi, parse, to_description, to_ident_pascal, to_ident_snake, to_type};
use anyhow::Context;
use indexmap::IndexMap;

pub fn to_items_fn(
    document: &openapi::Document,
    schemas: &IndexMap<String, Schema>,
) -> anyhow::Result<Vec<syn::Item>> {
    let mut items = Vec::new();

    for (path, operations) in &document.paths {
        for (method, operation) in operations {
            let method_expr = match method.as_str() {
                "delete" => Ok(quote::quote!(http::Method::DELETE)),
                "get" => Ok(quote::quote!(http::Method::GET)),
                "post" => Ok(quote::quote!(http::Method::POST)),
                _ => Err(anyhow::format_err!("unhandled: {method:#?}")),
            }
            .with_context(|| path.clone())?;

            if let (
                None,
                Some(openapi::RequestBody {
                    content,
                    required: Some(true),
                }),
            ) = (&operation.parameters, &operation.request_body)
                && let Some((request_body_content_type, request_body_content)) =
                    content.get_key_value("application/json")
            {
                for (status, response) in &operation.responses {
                    if let Some(content) = &response.content
                        && let Some((response_content_type, response_content)) =
                            content.get_key_value("text/event-stream")
                    {
                        let description = to_description(Some(&operation.summary));
                        let ident_fn =
                            to_ident_snake(&format!("{}.stream", operation.operation_id));
                        let ident_type =
                            to_ident_pascal(&format!("{}.stream", operation.operation_id));

                        let request_body_type = {
                            let schema = parse::parse(
                                &request_body_content.schema,
                                &document.components.schemas,
                            )
                            .context("schema")
                            .with_context(|| format!("content[{request_body_content_type:?}]"))
                            .context("requestBody")
                            .with_context(|| method.clone())
                            .with_context(|| path.clone())?;
                            to_type(&operation.operation_id, &schema, schemas, false, None)
                        };
                        let response_type = {
                            let schema = parse::parse(
                                &response_content.schema,
                                &document.components.schemas,
                            )
                            .context("schema")
                            .with_context(|| format!("content[{response_content_type:?}]"))
                            .with_context(|| format!("responses[{status:?}]"))
                            .with_context(|| method.clone())
                            .with_context(|| path.clone())?;
                            to_type(&operation.operation_id, &schema, schemas, false, None)
                        };

                        items.push(syn::parse_quote! {
                            #description
                            pub fn #ident_fn<S, Fut, B, E>(
                                service: S,
                                body: &crate::types::#request_body_type,
                            ) -> #ident_type<Fut, B, E>
                            where
                            S: FnOnce(http::Request<String>) -> Fut,
                            Fut: Future<Output = Result<http::Response<B>, E>>,
                            B: http_body::Body,
                            {
                                futures::TryFutureExt::map_ok(
                                    crate::SendFuture::new(service, #method_expr, #path, body),
                                    crate::EventStream::new,
                                )
                            }
                        });

                        items.push(syn::parse_quote! {
                            pub type #ident_type<Fut, B, E> = futures::future::MapOk<
                            crate::SendFuture<Fut, B, E>,
                            fn(B) -> crate::EventStream<B, crate::types::#response_type>,
                            >;
                        })
                    }
                }
            }
        }
    }

    Ok(items)
}
