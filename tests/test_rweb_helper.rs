use serde::Serialize;
use std::convert::Infallible;

use rweb::{get, Filter, Rejection, Schema};
use rweb_helper_macro::RwebResponse;

use rweb_helper::{html_response::HtmlResponse, json_response::JsonResponse};

#[test]
fn it_works() {
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

    let (spec, test_path) = rweb::openapi::spec().build(|| test_get().or(test_json()));

    println!(
        "{}",
        serde_json::to_string(&spec).expect("Failed to deserialize")
    );

    assert!(false);
}
