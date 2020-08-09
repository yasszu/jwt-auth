extern crate dotenv;

use actix_web::web;
use postgres::error::Error;
use postgres::row::Row;
use postgres::NoTls;
use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use crate::hash::*;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub fn create_account(
    email: &String,
    password: &String,
    db: web::Data<ConnectionPool>,
) -> Result<Option<Row>, Error> {
    let mut conn = db.get().unwrap();
    let hash = get_hash(password).to_owned();
    conn.execute(
        "INSERT INTO accounts (email, password) VALUES ($1, $2)",
        &[email, &hash],
    )?;
    conn.query_opt(
        "SELECT id, email, password FROM accounts WHERE email = $1",
        &[email],
    )
}

pub fn find_account(email: &String, db: web::Data<ConnectionPool>) -> Result<Option<Row>, Error> {
    let mut conn = db.get().unwrap();
    conn.query_opt(
        "SELECT id, email, password FROM accounts WHERE email = $1",
        &[&email],
    )
}
