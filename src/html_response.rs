use rweb::{
    http::header::{SET_COOKIE},
    hyper::{Body, Response},
    openapi::{self, Entity, ResponseEntity, Responses},
    Reply,
};
use std::{marker::PhantomData};

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
    fn describe() -> openapi::Schema {
        Result::<T, E>::describe()
    }
}

impl<T, E> ResponseEntity for HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    fn describe_responses() -> Responses {
        Result::<T, E>::describe_responses()
    }
}
