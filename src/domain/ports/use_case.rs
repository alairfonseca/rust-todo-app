use actix_web::web;
use crate::adapters::db::orms::board::NewBoard;
use diesel::PgConnection;
use anyhow::Error;

pub trait UseCase<T, R> {
    fn execute(&self, payload: web::Json<NewBoard>, db_connection: &PgConnection) -> Result<R, Error>;
}
