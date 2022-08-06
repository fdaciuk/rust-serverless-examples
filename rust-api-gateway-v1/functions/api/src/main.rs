use aws_lambda_events::encodings::Body;
use aws_lambda_events::event::apigw::{
  ApiGatewayProxyRequest,
  ApiGatewayProxyResponse,
};
use http::header::HeaderMap;
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
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
  };
}

fn is_local() -> bool {
  match std::env::var("ENVIRONMENT") {
    Ok(value) => value == "local",
    Err(_) => false,
  }
}

async fn handler(
  event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, LambdaError> {
  let (event, _context) = event.into_parts();
  let data = event.body.unwrap_or(String::from("{}"));
  let resp = ApiGatewayProxyResponse {
    status_code: 200,
    headers: HeaderMap::new(),
    multi_value_headers: HeaderMap::new(),
    body: Some(Body::Text(format!("Data: {}", data))),
    is_base64_encoded: Some(false),
  };

  info!("Response: {:?}", resp);
  Ok(resp)
}
