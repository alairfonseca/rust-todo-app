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

    println!("instantiating modules...");

    let db_connection = PgConnection::establish(&database_url).expect("Error connecting to database");

    let board_repository = BoardRepositoryImpl::new(db_connection);

    let create_board_use_case = CreateBoardUseCase::new(Box::new(board_repository));

    let app_state = AppState {
        create_board_use_case,
    };

    app_state
}
