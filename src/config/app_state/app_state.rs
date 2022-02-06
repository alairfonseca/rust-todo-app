use crate::domain::use_cases::CreateBoardUseCase;
use crate::adapters::db::BoardRepositoryImpl;

pub struct AppState {
    pub create_board_use_case: CreateBoardUseCase<BoardRepositoryImpl>,
}
