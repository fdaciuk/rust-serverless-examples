use aws_sdk_sqs::Endpoint;
use lambda_web::actix_web::http::Uri;
use tracing_subscriber;

pub fn setup_logs() {
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

pub fn is_running_on_lambda() -> bool {
  std::env::var("OFFLINE").is_err()
}

pub fn localstack_endpoint() -> Endpoint {
  Endpoint::immutable(Uri::from_static("http://localhost:4566"))
}

pub fn use_localstack() -> bool {
  std::env::var("LOCALSTACK").is_ok()
}
