use std::rc::Rc;

use crate::domain::ports::use_case::UseCase;
use crate::domain::ports::repositories::board_repository::{
    BoardRepository, Board
};
use anyhow::Error;

pub struct UpdateBoardUseCase {
    board_repository: Rc<dyn BoardRepository>,
}

impl UpdateBoardUseCase {
    pub fn new(board_repository: Rc<dyn BoardRepository>) -> Self {
        Self {
            board_repository,
        }
    }
}

impl UseCase<Board, Board> for UpdateBoardUseCase {
    fn execute(&self, payload: Board) -> Result<Board, Error> {
        let update_result = self.board_repository.update_board(payload);

        match update_result {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
