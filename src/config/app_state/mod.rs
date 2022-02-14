mod app_state;

pub use app_state::AppState;
use diesel::{pg::PgConnection, Connection};
use crate::{
    adapters::db::{BoardRepositoryImpl, TaskRepositoryImpl},
    domain::use_cases::{
        board::{CreateBoardUseCase, UpdateBoardUseCase},
        task::CreateTaskUseCase
    }
};
use std::rc::Rc;

use std::env;

pub fn app_state_factory() -> AppState {
    let database_url = env::var("DATABASE_URL").unwrap();

    println!("instantiating modules...");

    let db_connection = Rc::new(PgConnection::establish(&database_url).expect("Error connecting to database"));

    // repositories
    let board_repository = Rc::new(BoardRepositoryImpl::new(db_connection.clone()));
    let task_repository = Rc::new(TaskRepositoryImpl::new(db_connection.clone()));

    // use cases
    let create_board_use_case = CreateBoardUseCase::new(board_repository.clone());
    let update_board_use_case = UpdateBoardUseCase::new(board_repository.clone());
    let create_task_use_case = CreateTaskUseCase::new(task_repository.clone());

    let app_state = AppState {
        create_board_use_case,
        update_board_use_case,
        create_task_use_case,
    };

    app_state
}
