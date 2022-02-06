use actix_web::web;

mod path;
mod board_routes;

use board_routes::board_routes_setup;

pub fn routes_setup(config: &mut web::ServiceConfig) {
    board_routes_setup(config);
}
