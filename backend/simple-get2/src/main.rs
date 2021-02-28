use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

fn main() {
    println!(">>>");
    env_logger::init();
    println!("<<<");
    lambda!(handler)
}

fn handler(_: Request, _: Context) -> Result<impl IntoResponse, HandlerError> {
    log::debug!("debug!");
    log::info!("info!");
    log::warn!("warn!");
    log::error!("error!");
    Ok(json!({
        "message": "AWS Lambda on Rust simple-get2"
    }))
}
