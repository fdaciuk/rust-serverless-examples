use lambda_web::actix_web::{
  get,
  post,
  web,
  App,
  HttpResponse,
  HttpServer,
  Responder,
};
use lambda_web::{run_actix_on_lambda, LambdaError};
use serde::{Deserialize, Serialize};
use tracing::info;
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
  }
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
  HttpResponse::Ok().json(&json)
}
