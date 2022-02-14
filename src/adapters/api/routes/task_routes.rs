use actix_web::web;
use super::path::Path;

use super::super::handlers::task_handler::*;

pub fn task_routes_setup(config: &mut web::ServiceConfig) {
    let base_path: Path = Path { prefix: String::from("/task") };

    config
        .route(&base_path.define(String::from("/")), web::post().to(create));
}
