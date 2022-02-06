use actix_web::web;
use super::path::Path;

use super::super::handlers::*;

pub fn board_routes_setup(config: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/board") };

    config
        .route(&base_path.define(String::from("/")), web::post().to(create))
        .route(&base_path.define(String::from("/")), web::get().to(get));

    // let create_board_use_case = CreateBoardUseCaseImpl::new();
    // let board_controller = BoardController::new(create_board_use_case);

    // app
        // .route(&base_path.define(String::from("/")), web::post().to(|| &board_controller.create()));
}
