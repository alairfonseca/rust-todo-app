use crate::domain::use_cases::{
    board::{
        CreateBoardUseCase,
        UpdateBoardUseCase
    },
    task::CreateTaskUseCase
};

pub struct AppState {
    pub create_board_use_case: CreateBoardUseCase,
    pub update_board_use_case: UpdateBoardUseCase,
    pub create_task_use_case: CreateTaskUseCase,
}
