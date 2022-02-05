use actix_web::web;

mod path;
mod board;

pub fn routes_factory(app: &mut web::ServiceConfig) {
    board::board_routes_factory(app);
}
