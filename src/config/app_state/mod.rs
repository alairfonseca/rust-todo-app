mod app_state;

pub use app_state::AppState;
use diesel::{pg::PgConnection, Connection};
use crate::{
    adapters::db::BoardRepositoryImpl,
    domain::use_cases::{CreateBoardUseCase, UpdateBoardUseCase}
};
use std::sync::Arc;

use std::env;

pub fn app_state_factory() -> AppState {
    let database_url = env::var("DATABASE_URL").unwrap();

    println!("instantiating modules...");

    let db_connection = PgConnection::establish(&database_url).expect("Error connecting to database");

    // repositories
    let board_repository = Box::new(BoardRepositoryImpl::new(db_connection));

    // use cases
    let create_board_use_case = CreateBoardUseCase::new(Box::new(BoardRepositoryImpl::new(PgConnection::establish(&database_url).expect("Error connecting to database"))));
    let update_board_use_case = UpdateBoardUseCase::new(Box::new(BoardRepositoryImpl::new(PgConnection::establish(&database_url).expect("Error connecting to database"))));


    let app_state = AppState {
        create_board_use_case,
        update_board_use_case,
    };

    app_state
}
