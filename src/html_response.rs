use rweb::{
    http::header::{CONTENT_TYPE, SET_COOKIE},
    hyper::{Body, Response},
    openapi::{self, Entity, ResponseEntity, Responses},
    Reply,
};
use std::{borrow::Cow, marker::PhantomData};

use crate::{
    content_type_trait::{ContentTypeHtml, ContentTypeTrait},
    response_description_trait::{DefaultDescription, ResponseDescriptionTrait},
    status_code_trait::{StatusCodeOk, StatusCodeTrait},
};

pub struct HtmlResponse<T, E, S = StatusCodeOk, C = ContentTypeHtml, D = DefaultDescription>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    S: StatusCodeTrait,
    C: ContentTypeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    data: T,
    cookie: Option<String>,
    phantom_s: PhantomData<S>,
    phantom_c: PhantomData<C>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<T, E, S, C, D> HtmlResponse<T, E, S, C, D>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    S: StatusCodeTrait,
    C: ContentTypeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cookie: None,
            phantom_s: PhantomData,
            phantom_c: PhantomData,
            phantom_d: PhantomData,
            phantom_e: PhantomData,
        }
    }
    pub fn with_cookie(mut self, cookie: &str) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
}

impl<T, E, S, C, D> Reply for HtmlResponse<T, E, S, C, D>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    S: StatusCodeTrait,
    C: ContentTypeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn into_response(self) -> Response<Body> {
        let reply = rweb::reply::html(self.data);
        let reply = rweb::reply::with_status(reply, S::status_code());
        let reply = rweb::reply::with_header(reply, CONTENT_TYPE, C::content_type_header());
        #[allow(clippy::option_if_let_else)]
        if let Some(header) = self.cookie {
            let reply = rweb::reply::with_header(reply, SET_COOKIE, header);
            reply.into_response()
        } else {
            reply.into_response()
        }
    }
}

impl<T, E, S, C, D> Entity for HtmlResponse<T, E, S, C, D>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    S: StatusCodeTrait,
    C: ContentTypeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn describe() -> openapi::Schema {
        Result::<T, E>::describe()
    }
}

impl<T, E, S, C, D> ResponseEntity for HtmlResponse<T, E, S, C, D>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    S: StatusCodeTrait,
    C: ContentTypeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn describe_responses() -> Responses {
        let mut responses = Result::<T, E>::describe_responses();
        let old_code: Cow<'static, str> = "200".into();
        let new_code: Cow<'static, str> = S::status_code().as_u16().to_string().into();
        if let Some(mut old) = responses.remove(&old_code) {
            let old_content_type: Cow<'static, str> = "text/plain".into();
            let new_content_type: Cow<'static, str> = C::content_type().into();
            if let Some(old_content) = old.content.remove(&old_content_type) {
                old.content.insert(new_content_type, old_content);
            }
            old.description = D::description().into();
            responses.insert(new_code, old);
        }
        responses
    }
}
