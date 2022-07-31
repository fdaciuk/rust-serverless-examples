use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  let func = service_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
  let (event, _context) = event.into_parts();
  let first_name = event["firstName"].as_str().unwrap_or("world");
  Ok(json!({ "message": format!("Hello, {}", first_name) }))
}
