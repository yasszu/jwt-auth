use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web::error;
use postgres::{NoTls};
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use validator::Validate;

use crate::model::FormLogin;

pub async fn signup(
    form: web::Json<FormLogin>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<HttpResponse, Error> {
    let data = form.into_inner();
    match data.clone().validate() {
        Ok(_) => (),
        Err(e) => return Err(error::ErrorBadRequest(e)),
    }
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.execute(
            "INSERT INTO accounts (email, password) VALUES ($1, $2)",
            &[&data.email, &data.password],
        )
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().body("token"))
}