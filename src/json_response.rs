use rweb::{
    http::header::SET_COOKIE,
    hyper::{Body, Response},
    openapi::{self, Entity, ResponseEntity, Responses},
    Json, Reply,
};
use serde::Serialize;
use std::marker::PhantomData;

pub struct JsonResponse<T, E>
where
    T: Serialize + Entity + Send,
{
    data: T,
    cookie: Option<String>,
    phantom_e: PhantomData<E>,
}

impl<T, E> JsonResponse<T, E>
where
    T: Serialize + Entity + Send,
    E: ResponseEntity + Send,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cookie: None,
            phantom_e: PhantomData,
        }
    }
    pub fn with_cookie(mut self, cookie: String) -> Self {
        self.cookie = Some(cookie);
        self
    }
}

impl<T, E> Reply for JsonResponse<T, E>
where
    T: Serialize + Entity + Send,
    E: ResponseEntity + Send,
{
    fn into_response(self) -> Response<Body> {
        let reply = rweb::reply::json(&self.data);
        #[allow(clippy::option_if_let_else)]
        if let Some(header) = self.cookie {
            let reply = rweb::reply::with_header(reply, SET_COOKIE, header);
            reply.into_response()
        } else {
            reply.into_response()
        }
    }
}

impl<T, E> Entity for JsonResponse<T, E>
where
    T: Serialize + Entity + Send,
    E: ResponseEntity + Send,
{
    fn describe() -> openapi::Schema {
        Result::<T, E>::describe()
    }
}

impl<T, E> ResponseEntity for JsonResponse<T, E>
where
    T: Serialize + Entity + Send,
    E: ResponseEntity + Send,
{
    fn describe_responses() -> Responses {
        Result::<Json<T>, E>::describe_responses()
    }
}
