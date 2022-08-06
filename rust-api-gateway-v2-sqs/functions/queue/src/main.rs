use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  tracing_subscriber::fmt::init();
  let func = service_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

#[derive(Debug, Deserialize)]
struct Body {
  first_name: String,
}

#[derive(Deserialize)]
struct Record {
  body: String,
}

#[derive(Deserialize)]
struct Event {
  #[serde(rename = "Records")]
  records: Vec<Record>,
}

async fn handler(data: LambdaEvent<Event>) -> Result<Value, LambdaError> {
  let (event, _context) = data.into_parts();
  let body = match serde_json::from_str::<Body>(&event.records[0].body) {
    Ok(body) => body,
    Err(error) => return Err(Box::new(error)),
  };
  info!("BODY: {:?}", body);
  Ok(json!({ "message": format!("Hello, {}", body.first_name) }))
}
