extern crate dotenv;

use actix_web::error;
use actix_web::error::ErrorUnauthorized;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;
use validator::Validate;

use crate::accounts::*;
use crate::auth::*;
use crate::jwt::*;
use crate::model::*;

pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("OK"))
}

pub async fn signup(
    form: web::Json<FormLogin>,
    db: web::Data<Pool<PostgresConnectionManager<NoTls>>>,
) -> Result<HttpResponse, Error> {
    let data = form.into_inner();
    match data.clone().validate() {
        Ok(_) => (),
        Err(e) => return Err(error::ErrorBadRequest(e)),
    }
    let res = web::block(move || create_account(&data.email, &data.password, db))
        .await
        .map(|row| match row {
            Some(row) => {
                let account_id: i32 = row.get("account_id");
                let email: String = row.get("email");
                let _token = jwt_sign(account_id, email);
                let _cookie = get_cookie(_token.clone());
                let _res = ResultToken {
                    success: true,
                    token: _token,
                    error: "".to_owned(),
                };
                HttpResponse::Ok().cookie(_cookie).json(_res)
            }
            None => {
                let _res = ResultToken {
                    success: false,
                    token: "".to_owned(),
                    error: format!("{}", "Faild to sighup."),
                };
                HttpResponse::Ok().json(_res)
            }
        })
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
    let res = web::block(move || find_account(&email, db))
        .await
        .map(|row| match row {
            Some(row) => {
                let _account_id: i32 = row.get("account_id");
                let _email: String = row.get("email");
                let _password: String = row.get("password");
                let _hash = get_hash(password.as_str());
                if _password == _hash {
                    let _token = jwt_sign(_account_id, _email);
                    let _cookie = get_cookie(_token.clone());
                    let _res = ResultToken {
                        success: true,
                        token: _token,
                        error: "".to_owned(),
                    };
                    HttpResponse::Ok().cookie(_cookie).json(_res)
                } else {
                    let _res = ResultToken {
                        success: false,
                        token: "".to_owned(),
                        error: format!("{}", "Password is invalid"),
                    };
                    HttpResponse::Ok().json(_res)
                }
            }
            None => {
                let _res = ResultToken {
                    success: false,
                    token: "".to_owned(),
                    error: format!("{}", "This email doesn't exist."),
                };
                HttpResponse::Ok().json(_res)
            }
        })
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

pub async fn verify(req: HttpRequest) -> Result<HttpResponse, Error> {
    let res = match req.headers().get("Authorization") {
        Some(auth) => {
            let _token: Vec<&str> = auth.to_str().unwrap().split("Bearer").collect();
            let token = _token[1].trim();
            match jwt_verify(token) {
                Some(claims) => ResponseAccount {
                    account_id: claims.sub,
                    email: claims.email,
                },
                None => return Err(ErrorUnauthorized("invalid token!")),
            }
        }
        None => return Err(ErrorUnauthorized("invalid token!")),
    };
    Ok(HttpResponse::Ok().json(res))
}
