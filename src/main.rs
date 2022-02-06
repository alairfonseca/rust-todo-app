mod adapters;
mod domain;
mod config;

use adapters::api::*;

fn main() -> std::io::Result<()> {
    server::setup()
}
