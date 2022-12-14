use lambda_http::{
  service_fn,
  Body,
  Error as LambdaError,
  IntoResponse,
  Request,
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{error, info};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  setup_logs();
  let func = service_fn(handler);
  lambda_http::run(func).await?;
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

async fn handler(request: Request) -> Result<impl IntoResponse, LambdaError> {
  match request.body() {
    Body::Text(value) => match serde_json::from_str::<UserData>(value) {
      Ok(data) => Ok(fn_data(&data)),
      Err(error) => {
        error!("ERROR: {}", error.to_string());
        Ok(json!({
          "error": true,
          "message": format!("Deserialization error: {}", error.to_string())
        }))
      }
    },
    _ => {
      error!("ERROR: Unknown error");
      Ok(json!({ "error": true, "message": "Unknown error" }))
    }
  }
}

#[derive(Debug, Deserialize)]
struct UserData {
  first_name: String,
  last_name: String,
}

fn fn_data(data: &UserData) -> Value {
  info!("INFO: UserData: {:?}", data);
  json!({
    "message": format!("The name is {} {}", data.first_name, data.last_name)
  })
}
