#![allow(unused)]

use crate::model::ModelController;

pub use self::error::{ Error, Result };

use std::{net::SocketAddr};
use axum::{Router, routing::{get, get_service}, Server, response::{ Html, IntoResponse, Response }, extract::{Query, Path}, middleware};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
  // Initialize ModelController
  let mc = ModelController::new().await?;

  let routes_all = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .nest("/api", web::routes_tickets::routes(mc.clone()))
    .layer(middleware::map_response(main_reponse_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

  let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
  println!("->> LISTENING on {} \n", addr);
  axum::Server::bind(&addr).serve(routes_all.into_make_service())
    .await
    .unwrap();

  Ok(())
}

async fn main_reponse_mapper(res: Response) -> Response {
  println!("->> RES_MAPPER - main_reponse_mapper");
  println!();

  res
}

fn routes_static() -> Router {
  Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router {
  Router::new()
    .route("/hello", get(say_hello))
    .route("/hello2/:name", get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
  name: Option<String>,
}
// e.g. `/hello?name=World`
async fn say_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
  println!("->> {:<12} -  sey_hello - {params:?}", "HANDLER");

  let name = params.name.as_deref().unwrap_or("World");
  Html(format!("Hello <strong>{}!!</strong>", name))
}

// e.g. `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
  println!("->> HANDLER -  sey_hello - {name:?}");

  Html(format!("Hello <strong>{}!!</strong>", name))
}
