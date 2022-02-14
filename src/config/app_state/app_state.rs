use crate::domain::use_cases::{CreateBoardUseCase, UpdateBoardUseCase};

pub struct AppState {
    pub create_board_use_case: CreateBoardUseCase,
    pub update_board_use_case: UpdateBoardUseCase
}
