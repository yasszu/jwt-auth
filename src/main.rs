#[macro_use]
extern crate validator_derive;
extern crate validator;

use actix_web::error;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use model::*;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use validator::Validate;

mod accounts;
mod auth;
mod model;

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

// async fn signup(
//     form: web::Json<FormLogin>,
//     db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
// ) -> Result<HttpResponse, Error> {
//     let data = form.into_inner();
//     match data.clone().validate() {
//         Ok(_) => (),
//         Err(e) => return Err(error::ErrorBadRequest(e)),
//     }
//     web::block(move || {
//         let mut conn = db.get().unwrap();
//         conn.execute(
//             "INSERT INTO accounts (email, password) VALUES ($1, $2)",
//             &[&data.email, &data.password],
//         )
//     })
//     .await
//     .map_err(|_| HttpResponse::InternalServerError())?;
//     Ok(HttpResponse::Ok().body("token"))
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

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
            .route("/signup", web::post().to(accounts::signup))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
