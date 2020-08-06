use actix_web::error;
use actix_web::{web, Error, HttpResponse};
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use validator::Validate;

use crate::auth::*;
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
    let res = web::block(move || {
        let mut conn = db.get().unwrap();
        let hash = get_hash(&data.password).to_owned();
        conn.execute(
            "INSERT INTO accounts (email, password) VALUES ($1, $2)",
            &[&data.email, &hash],
        ).unwrap();

        conn.query_one("SELECT password FROM accounts WHERE email = $1", &[&data.email])
    })
    .await
    .map(|row| {
        let password: String = row.get("password");
        HttpResponse::Ok().body(password)
    })
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
