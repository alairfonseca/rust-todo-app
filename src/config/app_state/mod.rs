mod app_state;

pub use app_state::AppState;
use diesel::{pg::PgConnection, Connection};
use crate::{
    adapters::db::BoardRepositoryImpl,
    domain::use_cases::CreateBoardUseCase
};

use std::env;

pub fn app_state_factory() -> AppState {
    let database_url = env::var("DATABASE_URL").unwrap();
    let db_connection = PgConnection::establish(&database_url).expect("Error connecting to database");

    let board_repository = BoardRepositoryImpl::new();

    let create_board_use_case = CreateBoardUseCase::new(board_repository);

    let app_state = AppState {
        create_board_use_case,
        db_connection,
    };

    println!("instanciando repositories e use cases...");

    app_state
}
