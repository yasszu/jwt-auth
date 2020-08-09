#[macro_use]
extern crate validator_derive;
extern crate validator;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;

mod handler;
mod hash;
mod model;
mod jwt;

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("OK"))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    // r2d2 pool
    let manager = PostgresConnectionManager::new(
        "host=localhost user=postgres password=root"
            .parse()
            .unwrap(),
        NoTls,
    );
    let pool = r2d2::Pool::new(manager).expect("Faild to build postgres connection.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/signup", web::post().to(handler::signup))
            .route("/login", web::post().to(handler::login))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
