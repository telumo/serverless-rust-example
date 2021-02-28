use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn main() {
    lambda!(handler)
}

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    Ok(json!({
        "message": "AWS Lambda on Rust simple-get"
    }))
}
