use aide::{axum::{routing::get_with, ApiRouter, IntoApiResponse}, transform::TransformOperation};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub(crate) fn test_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/test", 
            get_with(owo, owo_docs)
            .post_with(echo_body, echo_body_docs)
        )
}

async fn owo() -> impl IntoApiResponse {
    "U~U works"
}

fn owo_docs(op: TransformOperation) -> TransformOperation {
    op.description("Test route, just returns string")
        .tag("Test routes")
        .response::<200, String>()
}

/// A struct for the echo route
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(example = "test_struct_example")]
struct TestStruct {
    /// i32 value, doesn't do anything
    pub id: i32,
    /// String value, doesn't do anything
    pub text: String,
}

fn test_struct_example() -> TestStruct {
    TestStruct {
        id: 23,
        text: "owo".into(),
    }
}

async fn echo_body(
    Json(test): Json<TestStruct>,
) -> impl IntoApiResponse {
    Json(test).into_response()
}

fn echo_body_docs(op: TransformOperation) -> TransformOperation {
    op.description("Echo test route")
        .tag("Test routes")
        .response::<200, Json<TestStruct>>()
}
