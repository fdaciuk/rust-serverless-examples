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

#[actix_web::main]
async fn main() -> Result<(), LambdaError> {
  let factory = || App::new().service(hello_handler).service(create_user);

  match is_running_on_lambda() {
    true => run_actix_on_lambda(factory).await?,
    false => {
      println!("Server is running on http://localhost:3000");
      HttpServer::new(factory)
        .bind(("127.0.0.1", 3000))?
        .run()
        .await?
    }
  };

  Ok(())
}

fn is_running_on_lambda() -> bool {
  std::env::var("OFFLINE").is_err()
}

#[get("/")]
async fn hello_handler() -> impl Responder {
  HttpResponse::Ok().body("Hello Rust!")
}

#[derive(Serialize, Deserialize)]
struct UserData {
  first_name: String,
  last_name: String,
}

#[post("/user")]
async fn create_user(json: web::Json<UserData>) -> impl Responder {
  HttpResponse::Ok().json(&json)
}
