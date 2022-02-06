mod app_state;

pub use app_state::AppState;
use crate::{
    adapters::db::BoardRepositoryImpl,
    domain::use_cases::CreateBoardUseCase
};

pub fn app_state_factory() -> AppState {
    let board_repository = BoardRepositoryImpl::new();

    let create_board_use_case = CreateBoardUseCase::new(board_repository);

    let app_state = AppState {
        create_board_use_case,
    };

    app_state
}
