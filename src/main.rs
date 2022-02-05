mod adapters;
mod domain;

use adapters::api::*;

fn main() -> std::io::Result<()> {
    server::setup()
}
