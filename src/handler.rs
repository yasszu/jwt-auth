extern crate dotenv;

use actix_web::error;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use jwt::Claims;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use std::env;
use validator::Validate;

use crate::hash::*;
use crate::jwt;
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
        )?;
        conn.query_opt(
            "SELECT id, email FROM accounts WHERE email = $1",
            &[&data.email],
        )
    })
    .await
    .map(|row| match row {
        Some(row) => {
            let id: i32 = row.get("id");
            let email: String = row.get("email");
            let token = jwt_sign(id, email);
            ResultToken {
                success: true,
                token: token,
                error: "".to_owned(),
            }
        }
        None => ResultToken {
            success: false,
            token: "".to_owned(),
            error: format!("{}", "Faild to sighup."),
        },
    })
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().json(res))
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
        conn.query_opt(
            "SELECT id, email, password FROM accounts WHERE email = $1",
            &[&email],
        )
    })
    .await
    .map(|row| match row {
        Some(row) => {
            let _id: i32 = row.get("id");
            let _email: String = row.get("email");
            let _password: String = row.get("password");
            let hash = get_hash(password.as_str());
            let success = _password == hash;
            let token = jwt_sign(_id, _email);
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

pub async fn verify(req: HttpRequest) -> Result<HttpResponse, Error> {
    let res = match req.headers().get("Authorization") {
        Some(auth) => {
            let _token: Vec<&str> = auth.to_str().unwrap().split("Bearer").collect();
            let token = _token[1].trim();
            match jwt_verify(token) {
                Some(claims) => ResponseAccount {
                    id: claims.sub,
                    email: claims.email,
                },
                None => return Err(ErrorUnauthorized("invalid token!")),
            }
        }
        None => return Err(ErrorUnauthorized("invalid token!")),
    };
    Ok(HttpResponse::Ok().json(res))
}

fn jwt_sign(id: i32, email: String) -> String {
    let secret = env::var("SECRET").unwrap();
    let claims = jwt::get_claims(id, email);
    jwt::encode_token(claims, &secret)
}

fn jwt_verify(token: &str) -> Option<Claims> {
    let secret = env::var("SECRET").unwrap();
    jwt::decode_token(token.to_owned(), &secret)
}
