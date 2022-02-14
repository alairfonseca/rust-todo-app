mod app_state;

pub use app_state::AppState;
use diesel::{pg::PgConnection, Connection};
use crate::{
    adapters::db::BoardRepositoryImpl,
    domain::use_cases::{CreateBoardUseCase, UpdateBoardUseCase}
};
use std::rc::Rc;

use std::env;

pub fn app_state_factory() -> AppState {
    let database_url = env::var("DATABASE_URL").unwrap();

    println!("instantiating modules...");

    let db_connection = Rc::new(PgConnection::establish(&database_url).expect("Error connecting to database"));

    // repositories
    let board_repository = Rc::new(BoardRepositoryImpl::new(db_connection.clone()));

    // use cases
    let create_board_use_case = CreateBoardUseCase::new(board_repository.clone());
    let update_board_use_case = UpdateBoardUseCase::new(board_repository.clone());

    let app_state = AppState {
        create_board_use_case,
        update_board_use_case,
    };

    app_state
}
