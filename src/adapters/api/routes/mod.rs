use actix_web::web;

mod path;
mod board;

pub fn routes_setup(config: &mut web::ServiceConfig) {
    board::board_routes_setup(config);
}
