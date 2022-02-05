pub mod create_board;

//use super::ports::use_case::UseCase;
use create_board::*;

use crate::adapters::api::server::BoardRepositoryImpl;

pub fn use_case_factory(board_respository: BoardRepositoryImpl) -> create_board::CreateBoardUseCase<BoardRepositoryImpl> {
    CreateBoardUseCase::new(board_respository)
}
