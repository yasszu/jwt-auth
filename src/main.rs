use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use postgres::{NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Hello world!"))
}

async fn signup(db: web::Data<Pool<PostgresConnectionManager<NoTls>>>) -> Result<HttpResponse, Error> {
    // let _client = db.get().unwrap();
    Ok(HttpResponse::Ok().body("Sign up"))
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
        .route("/signup", web::get().to(signup))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
