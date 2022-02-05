use actix_web::web;
use super::super::super::super::domain::ports::use_case::UseCase;
use super::super::server::AppState;

pub async fn create(service: web::Data<AppState>) -> String {
    println!("PASSOU AQUI!");
    service.create_board_uc.execute("hue".to_string())
}

pub async fn get() -> String {
    "lista de quadros".to_string()
}
