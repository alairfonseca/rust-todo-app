use actix_web::{web, HttpResponse, Responder};
use crate::domain::ports::repositories::task_repository::NewTask;
use crate::domain::ports::use_case::UseCase;
use crate::config::AppState;

pub async fn create(service: web::Data<AppState>, body: web::Json<NewTask>) -> impl Responder {
    let new_task = NewTask {
        title: body.title.clone(),
        board_id: body.board_id.clone(),
        description: body.description.clone(),
    };

    let result = service.create_task_use_case.execute(new_task);

    match result {
        Ok(value) => Ok(HttpResponse::Ok().json(value)),
        Err(_) => Err(HttpResponse::InternalServerError()),
    }
}
