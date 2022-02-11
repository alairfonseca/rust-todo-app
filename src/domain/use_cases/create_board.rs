use crate::domain::ports::use_case::UseCase;
use crate::domain::ports::repositories::board_repository::{
    BoardRepository, Board, NewBoard
};
use anyhow::Error;

pub struct CreateBoardUseCase {
    board_repository: Box<dyn BoardRepository>,
}

impl CreateBoardUseCase {
    pub fn new(board_repository: Box<dyn BoardRepository>) -> Self {
        Self {
            board_repository,
        }
    }
}

impl UseCase<NewBoard, Board> for CreateBoardUseCase {
    fn execute(&self, payload: NewBoard) -> Result<Board, Error> {
        let insert_result = self.board_repository.create_board(payload);

        match insert_result {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
