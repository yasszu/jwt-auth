use actix_web::error;
use actix_web::{web, Error, HttpResponse};
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use validator::Validate;

use crate::hash::*;
use crate::model::*;

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
        )
    })
    .await
    .map(|_| HttpResponse::Ok().body("token"))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

pub async fn login(
    form: web::Json<FormLogin>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<HttpResponse, Error> {
    let data = form.into_inner();
    let email = data.email;
    let password = data.password;
    let res = web::block(move || {
        let mut conn = db.get().unwrap();
        conn.query_opt("SELECT password FROM accounts WHERE email = $1", &[&email])
    })
    .await
    .map(|row| match row {
        Some(row) => {
            let target: String = row.get("password");
            let hash = get_hash(password.as_str());
            let success = target == hash;
            let token = "token".to_owned();
            let error = if success {
                "".to_owned()
            } else {
                format!("{}", "Password is invalid")
            };
            ResultToken {
                success,
                token,
                error,
            }
        }
        None => ResultToken {
            success: false,
            token: "".to_owned(),
            error: format!("{}", "This email doesn't exist."),
        },
    })
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().json(res))
}
