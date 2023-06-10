use proc_macro::TokenStream;
use syn::{
    DeriveInput, Meta, Lit, Data, Fields, Type, TypePath, PathArguments, Token,
    spanned::Spanned, Expr
};
use quote::quote;

#[proc_macro_derive(RwebResponse, attributes(response))]
pub fn derive_rweb_response_fn(input: TokenStream) -> TokenStream {
    #[derive(Default, Debug)]
    struct RwebResponse {
        description: Option<String>,
        content: Option<String>,
        status: Option<String>,
        error: Option<String>,
    }
    let mut rweb_response = RwebResponse::default();
    let input: DeriveInput = syn::parse(input).expect("Failed to parse");
    let DeriveInput {
        attrs, ident, data, ..
    } = input;
    for attr in &attrs {
        if attr.meta.path().is_ident("response") {
            if let Meta::List(metalist) = &attr.meta {
                metalist.parse_nested_meta(
                    |meta| {
                        if let Some(ident) = meta.path.get_ident() {
                            let ident = ident.to_string();
                            if let Expr::Lit(lit) = meta.value()?.parse::<Expr>()? {
                                if let Lit::Str(lit) = lit.lit {
                                    let lit = Some(lit.value());
                                    match ident.as_str() {
                                        "description" => rweb_response.description = lit,
                                        "content" => rweb_response.content = lit,
                                        "status" => rweb_response.status = lit,
                                        "error" => rweb_response.error = lit,
                                        id => panic!("{} is not a valid key", id),
                                    }
                                }
                            }
                        }
                        Ok(())
                    }
                ).map_err(|e| panic!("encountered error {}", e)).unwrap();
            }
        }
    }
    let mut inner_type: Option<TypePath> = None;
    if let Data::Struct(data_struct) = data {
        if let Fields::Unnamed(fields) = data_struct.fields {
            if let Some(first) = fields.unnamed.first() {
                if let Type::Path(typath) = &first.ty {
                    inner_type = Some(typath.clone());
                }
            }
        }
    }
    let inner_type = inner_type.expect("No inner type");
    let mut inner_type_mod = inner_type.clone();
    if let Some(first) = inner_type_mod.path.segments.first_mut() {
        if let PathArguments::AngleBracketed(args) = &mut first.arguments {
            args.colon2_token = Some(Token![::](args.span()));
        }
    }
    let from_impl = quote! {
        impl From<#inner_type> for #ident {
            fn from(item: #inner_type) -> Self {
                Self(item)
            }
        }
    };
    let content = match rweb_response.content.as_ref().map(String::as_str) {
        Some("html") => Some(quote!{rweb_helper::content_type_trait::ContentTypeHtml}),
        Some("css") => Some(quote!{rweb_helper::content_type_trait::ContentTypeCss}),
        Some("js") => Some(quote!{rweb_helper::content_type_trait::ContentTypeJs}),
        Some(val) => panic!("{} is not a valid content type", val),
        None => None,
    };
    let status = match rweb_response.status.as_ref().map(String::as_str) {
        Some("OK") | Some("200") => Some(quote!{rweb_helper::status_code_trait::StatusCodeOk}),
        Some("CREATED") | Some("201") => Some(quote!{rweb_helper::status_code_trait::StatusCodeCreated}),
        _ => None,
    };
    let content_reply = if let Some(content) = &content {
        quote!{
            use rweb_helper::content_type_trait::ContentTypeTrait;    
            res.headers_mut().insert(
                rweb::http::header::CONTENT_TYPE ,
                rweb::http::HeaderValue::from_static( #content::content_type_header() ) 
            );
        }
    } else {quote!{}};
    let status_reply = if let Some(status) = &status {
        quote!{
            use rweb_helper::status_code_trait::StatusCodeTrait;
            *res.status_mut() = #status::status_code();
        }
    } else {
        quote!{}
    };
    let reply_impl = quote! {
        impl rweb::Reply for #ident {
            fn into_response(self) -> rweb::http::Response<rweb::hyper::Body> {
                let mut res = self.0.into_response();
                #content_reply
                #status_reply
                res
            }
        }
    };
    let entity_impl = quote! {
        impl rweb::openapi::Entity for #ident {
            fn type_name() -> std::borrow::Cow<'static, str> {
                #inner_type_mod::type_name()
            }
            fn describe(comp_d: &mut rweb::openapi::ComponentDescriptor) -> rweb::openapi::ComponentOrInlineSchema {
                #inner_type_mod::describe(comp_d)
            }
        }
    };
    let content_response_entity = if let Some(content) = &content {
        quote!{
            let old_code: std::borrow::Cow<'static, str> = "200".into();
            if let Some(mut old) = resp.get_mut(&old_code) {
                use rweb_helper::content_type_trait::ContentTypeTrait;
                let old_content_type: std::borrow::Cow<'static, str> = "text/plain".into();
                let new_content_type: std::borrow::Cow<'static, str> = #content::content_type().into();
                if let Some(old_content) = old.content.remove(&old_content_type) {
                    old.content.insert(new_content_type, old_content);
                }
            }
        }
    } else {
        quote!{}
    };
    let description_response_entity = if let Some(description) = &rweb_response.description {
        quote!{
            let old_code: std::borrow::Cow<'static, str> = "200".into();
            if let Some(mut old) = resp.get_mut(&old_code) {
                old.description = #description.into();
            }
        }
    } else {
        quote!{}
    };
    let status_response_entity = if let Some(status) = &status {
        quote!{
            use rweb_helper::status_code_trait::StatusCodeTrait;
            let old_code: std::borrow::Cow<'static, str> = "200".into();
            let new_code: std::borrow::Cow<'static, str> = #status::status_code().as_u16().to_string().into();
            if let Some(old) = resp.remove(&old_code) {
                resp.insert(new_code, old);
            }
        }
    } else {
        quote!{}
    };
    let response_entity_impl = quote! {
        impl rweb::openapi::ResponseEntity for #ident {
            fn describe_responses(comp_d: &mut rweb::openapi::ComponentDescriptor) -> rweb::openapi::Responses {
                let mut resp = #inner_type_mod::describe_responses(comp_d);
                #content_response_entity
                #description_response_entity
                #status_response_entity
                resp.sort_keys();
                resp
            }
        }
    };
    let tokens = quote!{
        #from_impl
        #reply_impl
        #entity_impl
        #response_entity_impl
    };
    tokens.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
