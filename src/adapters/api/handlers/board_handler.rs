use actix_web::{web, HttpResponse, Responder};
use crate::adapters::db::orms::board::NewBoard;
use crate::domain::ports::use_case::UseCase;
use crate::config::AppState;

pub async fn create(service: web::Data<AppState>, body: web::Json<NewBoard>) -> impl Responder {
    let result = service.create_board_use_case.execute(body, &service.db_connection);

    match result {
        Ok(value) => Ok(HttpResponse::Ok().json(value)),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

pub async fn get() -> String {
    "lista de quadros".to_string()
}
