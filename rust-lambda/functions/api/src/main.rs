use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde_json::{json, Value};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  setup_logs();
  let func = service_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

fn setup_logs() {
  match is_local() {
    true => tracing_subscriber::fmt::init(),
    false => tracing_subscriber::fmt().with_ansi(false).init(),
  }
}

fn is_local() -> bool {
  match std::env::var("ENVIRONMENT") {
    Ok(value) => value == "local",
    Err(_) => false,
  }
}

async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
  let (event, _context) = event.into_parts();
  let first_name = event["firstName"].as_str().unwrap_or("world");
  info!("The first_name is {}", first_name);
  Ok(json!({ "message": format!("Hello, {}", first_name) }))
}
