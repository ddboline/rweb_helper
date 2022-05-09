use serde::{Deserialize, Serialize};
use std::{borrow::Cow, convert::Infallible};

use rweb::{
    filters::BoxedFilter,
    get,
    openapi::{
        self, ComponentDescriptor, ComponentOrInlineSchema, Entity, ResponseEntity, Responses,
    },
    reject::Reject,
    Filter, Rejection, Reply, Schema,
};

use rweb_helper::{
    derive_rweb_schema, derive_rweb_test, html_response::HtmlResponse, json_response::JsonResponse,
    RwebResponse,
};

#[test]
fn basic_example() {
    #[derive(RwebResponse)]
    #[response(description = "0", content = "html", status = "OK")]
    struct TestResponse(HtmlResponse<&'static str, Infallible>);

    #[get("/")]
    async fn test_get() -> Result<TestResponse, Rejection> {
        Ok(HtmlResponse::new("test").into())
    }

    #[derive(Serialize, Schema)]
    struct TestJson {
        field: String,
    }

    #[derive(RwebResponse)]
    #[response(description = "json test", status = "CREATED")]
    struct TestJsonResponse(JsonResponse<TestJson, Infallible>);

    #[get("/test_json")]
    async fn test_json() -> Result<TestJsonResponse, Rejection> {
        let test = TestJson {
            field: "test_field".into(),
        };
        Ok(JsonResponse::new(test).into())
    }

    let (spec, _) = rweb::openapi::spec().build(|| test_get().or(test_json()));

    let expected = r#"{"openapi":"3.0.1","info":{"title":"","version":""},"paths":{"/":{"get":{"responses":{"200":{"description":"0","content":{"text/html":{"schema":{"type":"string"}}}}}}},"/test_json":{"get":{"responses":{"201":{"description":"json test","content":{"application/json":{"schema":{"properties":{"field":{"type":"string"}},"type":"object","required":["field"]}}}}}}}},"components":{}}"#;
    let observed = serde_json::to_string(&spec).expect("Failed to deserialize");
    println!("{}", observed);

    assert_eq!(expected, observed);
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Test0 {
    a: u8,
    b: u8,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Test01(Test0);

derive_rweb_schema!(Test01, Test1);

#[allow(dead_code)]
#[derive(Schema)]
struct Test1 {
    #[schema(description = "fieldA")]
    a: u8,
    #[schema(description = "fieldB")]
    b: u8,
}

#[derive(Debug)]
struct TestError;

impl Reject for TestError {}

impl Entity for TestError {
    fn type_name() -> Cow<'static, str> {
        rweb::http::Error::type_name()
    }
    fn describe(comp_d: &mut ComponentDescriptor) -> ComponentOrInlineSchema {
        rweb::http::Error::describe(comp_d)
    }
}

impl ResponseEntity for TestError {
    fn describe_responses(_: &mut ComponentDescriptor) -> Responses {
        use indexmap::IndexMap;
        use rweb::{http::StatusCode, openapi::Response};
        let mut map = IndexMap::new();

        let error_responses = [
            (StatusCode::NOT_FOUND, "Not Found"),
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
            (StatusCode::BAD_REQUEST, "Bad Request"),
            (StatusCode::METHOD_NOT_ALLOWED, "Method not allowed"),
        ];

        for (code, msg) in &error_responses {
            map.insert(
                Cow::Owned(code.as_str().into()),
                Response {
                    description: Cow::Borrowed(*msg),
                    ..Response::default()
                },
            );
        }

        map
    }
}

#[derive(RwebResponse)]
#[response(description = "Test Description")]
struct TestResponse(JsonResponse<Test01, TestError>);

#[get("/")]
async fn test_response() -> Result<TestResponse, Rejection> {
    let x = Test01(Test0 { a: 2, b: 3 });
    Ok(JsonResponse::new(x).into())
}

#[test]
fn test_derive_rweb_test() {
    derive_rweb_test!(Test0, Test1);
}

#[test]
fn test_api_spec() {
    fn get_api_path() -> BoxedFilter<(impl Reply,)> {
        test_response().boxed()
    }

    let (spec, _) = openapi::spec().build(|| get_api_path());
    let spec_json = serde_json::to_string_pretty(&spec).unwrap();
    println!("{}", spec_json);
    let expected = include_str!("test_schema.json");
    assert_eq!(&spec_json, expected);
}
