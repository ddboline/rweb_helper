use rweb::{
    http::header::SET_COOKIE,
    hyper::{Body, Response},
    openapi::{self, Entity, ResponseEntity, Responses},
    Json, Reply,
};
use serde::Serialize;
use std::{borrow::Cow, marker::PhantomData};

use crate::{
    response_description_trait::{DefaultDescription, ResponseDescriptionTrait},
    status_code_trait::{StatusCodeOk, StatusCodeTrait},
};

pub struct JsonResponse<T, E, S = StatusCodeOk, D = DefaultDescription>
where
    T: Serialize + Entity + Send,
    S: StatusCodeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    data: T,
    cookie: Option<String>,
    phantom_s: PhantomData<S>,
    phantom_d: PhantomData<D>,
    phantom_e: PhantomData<E>,
}

impl<T, E, S, D> JsonResponse<T, E, S, D>
where
    T: Serialize + Entity + Send,
    S: StatusCodeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cookie: None,
            phantom_s: PhantomData,
            phantom_d: PhantomData,
            phantom_e: PhantomData,
        }
    }
    pub fn with_cookie(mut self, cookie: String) -> Self {
        self.cookie = Some(cookie);
        self
    }
}

impl<T, E, S, D> Reply for JsonResponse<T, E, S, D>
where
    T: Serialize + Entity + Send,
    S: StatusCodeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn into_response(self) -> Response<Body> {
        let reply = rweb::reply::json(&self.data);
        let reply = rweb::reply::with_status(reply, S::status_code());
        #[allow(clippy::option_if_let_else)]
        if let Some(header) = self.cookie {
            let reply = rweb::reply::with_header(reply, SET_COOKIE, header);
            reply.into_response()
        } else {
            reply.into_response()
        }
    }
}

impl<T, E, S, D> Entity for JsonResponse<T, E, S, D>
where
    T: Serialize + Entity + Send,
    S: StatusCodeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn describe() -> openapi::Schema {
        Result::<T, E>::describe()
    }
}

impl<T, E, S, D> ResponseEntity for JsonResponse<T, E, S, D>
where
    T: Serialize + Entity + Send,
    S: StatusCodeTrait,
    D: ResponseDescriptionTrait,
    E: ResponseEntity + Send,
{
    fn describe_responses() -> Responses {
        let mut responses = Result::<Json<T>, E>::describe_responses();
        let old_code: Cow<'static, str> = "200".into();
        let new_code: Cow<'static, str> = S::status_code().as_u16().to_string().into();
        if let Some(mut old) = responses.remove(&old_code) {
            old.description = D::description().into();
            responses.insert(new_code, old);
        }
        responses.sort_keys();
        responses
    }
}
