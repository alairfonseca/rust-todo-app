use actix_web::web;
use crate::domain::ports::use_case::UseCase;
use crate::config::AppState;

pub async fn create(service: web::Data<AppState>) -> String {
    println!("PASSOU AQUI!");
    service.create_board_use_case.execute("hue".to_string(), &service.db_connection)
}

pub async fn get() -> String {
    "lista de quadros".to_string()
}
