#[macro_use]
extern crate validator_derive;
extern crate dotenv;
extern crate validator;

use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use dotenv::dotenv;
use postgres::NoTls;
use r2d2_postgres::PostgresConnectionManager;
use std::env;

mod accounts;
mod handler;
mod hash;
mod jwt;
mod model;

async fn index(id: Identity) -> Result<HttpResponse, Error> {
        if let Some(id) = id.identity() {
            println!("jwt: {}", id);
            Ok(HttpResponse::Ok().body("Authorized"))
        } else {
            Ok(HttpResponse::Ok().body("Not authorized"))
        }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    dotenv().ok();

    // r2d2 pool
    let pool = r2d2::Pool::new(get_postgre_manager()).expect("Faild to build postgres connection.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                      .name("token")
                      .path("/")
                      .max_age_time(chrono::Duration::days(365))
                      .secure(false)))
            .route("/", web::get().to(index))
            .route("/signup", web::post().to(handler::signup))
            .route("/login", web::post().to(handler::login))
            .route("/verify", web::post().to(handler::verify))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

fn get_postgre_manager() -> PostgresConnectionManager<NoTls> {
    let host = env::var("POSTGRES_HOST").unwrap();
    let user = env::var("POSTGRES_USER").unwrap();
    let password = env::var("POSTGRES_PASSWORD").unwrap();
    let config = format!("host={} user={} password={}", host, user, password);
    PostgresConnectionManager::new(config.parse().unwrap(), NoTls)
}
