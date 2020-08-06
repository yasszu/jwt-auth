#[macro_use]
extern crate validator_derive;
extern crate validator;

use validator::Validate;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web::error;
use postgres::{NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use model::*;

mod auth;
mod model;

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

async fn signup(form: web::Json<FormLogin>, _db: web::Data<Pool<PostgresConnectionManager<NoTls>>>) -> Result<HttpResponse, Error> {
    // let _client = db.get().unwrap();
    let data = form.into_inner();
    match data.clone().validate() {
        Ok(_) => {
            let res = format!("{}, {}", data.email, data.password);
            Ok(HttpResponse::Ok().body(res))
        },
        Err(e) => Err(error::ErrorBadRequest(e)),
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    // r2d2 pool
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres password=root".parse().unwrap(), 
        NoTls
    );
    let pool = r2d2::Pool::new(manager)
        .expect("Faild to build postgres connection.");

    HttpServer::new(move || {
        App::new()
        .data(pool.clone()) 
        .wrap(middleware::Logger::default())
        .route("/", web::get().to(index))
        .route("/signup", web::post().to(signup))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}