use aws_sdk_sqs::{Client, Endpoint};
use lambda_web::actix_web::{
  get,
  http::Uri,
  post,
  web,
  App,
  HttpResponse,
  HttpServer,
  Responder,
};
use lambda_web::{run_actix_on_lambda, LambdaError};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use tracing_subscriber;

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
  setup_logs();
  let factory = || App::new().service(hello_handler).service(create_user);

  match is_running_on_lambda() {
    true => run_actix_on_lambda(factory).await?,
    false => {
      let port = 3008;
      info!("Server is running on http://localhost:{}", port);
      HttpServer::new(factory)
        .bind(("127.0.0.1", port))?
        .run()
        .await?
    }
  };

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

fn is_running_on_lambda() -> bool {
  std::env::var("OFFLINE").is_err()
}

#[get("/")]
async fn hello_handler() -> impl Responder {
  HttpResponse::Ok().body("Hello Rust!")
}

#[derive(Debug, Serialize, Deserialize)]
struct UserData {
  first_name: String,
  last_name: String,
}

#[post("/user")]
async fn create_user(json: web::Json<UserData>) -> impl Responder {
  info!("CREATE_USER: User data: {:?}", json);
  let client = get_sqs_client().await;
  let queue_url = get_queue_url().await.expect("Queue URL not found");
  let body = format!(
    "{{  \"first_name\": \"{}\", \"last_name\": \"{}\" }}",
    json.first_name, json.last_name
  );
  info!("Body to send: {}", body);
  let response = client
    .send_message()
    .queue_url(queue_url)
    .message_body(body)
    .send()
    .await;

  info!("Message response: {:?}", response);
  HttpResponse::Ok().json(&json)
}

async fn get_queue_url() -> Result<String, Box<dyn std::error::Error>> {
  match std::env::var("SQS_HOOKS_URL") {
    Ok(value) => {
      info!("SQS_HOOKS_URL: {}", value);
      Ok(value)
    }
    _ => get_queue_url_local().await,
  }
}

async fn get_queue_url_local() -> Result<String, Box<dyn std::error::Error>> {
  let client = get_sqs_client().await;
  let queues = match client.list_queues().send().await {
    Ok(v) => v,
    Err(error) => {
      error!("Error when list queues: {}", error);
      return Err("Deu merda pra buscar as filas!")?;
    }
  };
  let queue_urls = queues.queue_urls().unwrap_or_default();
  let hooks_queue = queue_urls
    .into_iter()
    .find(|hooks_queue| hooks_queue.contains("SQSHooks"))
    .expect("Queue URL not found");
  Ok(hooks_queue.to_owned())
}

async fn get_sqs_client() -> Client {
  let shared_config = aws_config::from_env().load().await;
  let mut sqs_config_builder =
    aws_sdk_sqs::config::Builder::from(&shared_config);

  if use_localstack() {
    sqs_config_builder =
      sqs_config_builder.endpoint_resolver(localstack_endpoint())
  }
  Client::from_conf(sqs_config_builder.build())
}

fn localstack_endpoint() -> Endpoint {
  Endpoint::immutable(Uri::from_static("http://localhost:4566"))
}

fn use_localstack() -> bool {
  std::env::var("LOCALSTACK").is_ok()
}
