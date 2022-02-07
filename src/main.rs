#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod adapters;
mod domain;
mod config;

use dotenv::dotenv;
use adapters::api::*;

fn main() -> std::io::Result<()> {
    dotenv().ok();

    server::setup()
}
