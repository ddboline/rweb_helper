use rweb::{
    http::header::SET_COOKIE,
    hyper::{Body, Response},
    openapi::{ComponentDescriptor, ComponentOrInlineSchema, Entity, ResponseEntity, Responses},
    Reply,
};
use std::{borrow::Cow, marker::PhantomData};

pub struct HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    data: T,
    cookie: Option<String>,
    phantom_e: PhantomData<E>,
}

impl<T, E> HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cookie: None,
            phantom_e: PhantomData,
        }
    }
    pub fn with_cookie(mut self, cookie: &str) -> Self {
        self.cookie = Some(cookie.into());
        self
    }
}

impl<T, E> Reply for HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    fn into_response(self) -> Response<Body> {
        let reply = rweb::reply::html(self.data);
        #[allow(clippy::option_if_let_else)]
        if let Some(header) = self.cookie {
            let reply = rweb::reply::with_header(reply, SET_COOKIE, header);
            reply.into_response()
        } else {
            reply.into_response()
        }
    }
}

impl<T, E> Entity for HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    fn type_name() -> Cow<'static, str> {
        Result::<T, E>::type_name()
    }
    fn describe(comp_d: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        Result::<T, E>::describe(comp_d)
    }
}

impl<T, E> ResponseEntity for HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    fn describe_responses(comp_d: &mut ComponentDescriptor) -> Responses {
        Result::<T, E>::describe_responses(comp_d)
    }
}
