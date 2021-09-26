use rweb::{Reply, http::{HeaderValue, header::SET_COOKIE}, hyper::{Body, Response}, openapi::{ComponentDescriptor, ComponentOrInlineSchema, Entity, ResponseEntity, Responses}};
use std::{borrow::Cow, convert::TryFrom, marker::PhantomData};

pub struct HtmlResponse<T, E>
where
    T: ResponseEntity + Send,
    Body: From<T>,
    E: ResponseEntity + Send,
{
    data: T,
    cookies: Option<Vec<String>>,
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
            cookies: None,
            phantom_e: PhantomData,
        }
    }
    pub fn with_cookie(mut self, cookie: &str) -> Self {
        if let Some(cookies) = self.cookies.as_mut() {
            cookies.push(cookie.into());
        } else {
            self.cookies = Some(vec![cookie.into()]);
        }
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
        let mut res = reply.into_response();
        if let Some(cookies) = self.cookies {
            for cookie in cookies {
                if let Ok(value) = <HeaderValue as TryFrom<String>>::try_from(cookie) {
                    res.headers_mut().append(SET_COOKIE, value);
                }
            }
        }
        res
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
