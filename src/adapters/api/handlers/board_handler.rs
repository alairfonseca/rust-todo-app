use actix_web::{web, HttpResponse, Responder};
use crate::domain::ports::repositories::board_repository::NewBoard;
use crate::domain::ports::use_case::UseCase;
use crate::config::AppState;

pub async fn create(service: web::Data<AppState>, body: web::Json<NewBoard>) -> impl Responder {
    let new_board = NewBoard {
        name: body.name.clone(),
    };

    let result = service.create_board_use_case.execute(new_board);

    match result {
        Ok(value) => Ok(HttpResponse::Ok().json(value)),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}

pub async fn get() -> String {
    "lista de quadros".to_string()
}
