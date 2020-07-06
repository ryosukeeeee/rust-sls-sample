use env_logger;
use lambda_runtime::{error::HandlerError, lambda, Context};
use serde_derive::Serialize;
use serde_json::Value;
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    lambda!(handler);
    Ok(())
}

fn handler(_: Value, c: Context) -> Result<CustomOutput, HandlerError> {
    Ok(CustomOutput {
        message: format!("Hello, World"),
    })
}
