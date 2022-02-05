use actix_web::{ App, HttpServer };

use crate::domain::ports::repositories::board_repository::{BoardRepository, CreateBoardPayload};

use super::routes::*;
use super::super::super::domain::use_cases::use_case_factory;
use super::super::super::domain::use_cases::create_board::CreateBoardUseCase;

pub struct BoardRepositoryImpl {}
impl BoardRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl BoardRepository for BoardRepositoryImpl {
    fn create_board(&self, payload: CreateBoardPayload) -> String {
        println!("executando repositorio...");
        payload.name
    }
}

pub struct AppState {
    pub create_board_uc: CreateBoardUseCase<BoardRepositoryImpl>,
}

#[actix_web::main]
pub async fn setup() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("starting server...");

        let br = BoardRepositoryImpl::new();
        let create_board_uc = use_case_factory(br);
        let app_state = AppState {
            create_board_uc,
        };

        let app = App::new()
            .data(app_state)
            .configure(|config|
                routes_setup(config),
            );

        return app;
    })
    .bind("127.0.0.1:8080")?
    .workers(2)
    .run()
    .await
}
