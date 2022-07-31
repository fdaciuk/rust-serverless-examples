use lambda_http::{
  service_fn,
  Body,
  Error as LambdaError,
  IntoResponse,
  Request,
};
use serde::Deserialize;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
  let func = service_fn(handler);
  lambda_http::run(func).await?;
  Ok(())
}

async fn handler(request: Request) -> Result<impl IntoResponse, LambdaError> {
  match request.body() {
    Body::Text(value) => match serde_json::from_str::<UserData>(value) {
      Ok(data) => Ok(fn_data(&data)),
      Err(error) => Ok(json!({
        "error": true,
        "message": format!("Deserialization error: {}", error.to_string())
      })),
    },
    _ => Ok(json!({ "error": true, "message": "Unknown error" })),
  }
}

#[derive(Debug, Deserialize)]
struct UserData {
  first_name: String,
  last_name: String,
}

fn fn_data(data: &UserData) -> Value {
  json!({
    "message": format!("The name is {} {}", data.first_name, data.last_name)
  })
}
