use actix_web::web;
use super::path::Path;
use crate::api::controllers::board_controller::*;

pub fn board_routes_factory(app: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/board") };

    app
        .route(&base_path.define(String::from("/")), web::post().to(create))
        .route(&base_path.define(String::from("/")), web::get().to(get));
}
