//use std::sync::Arc;

use super::super::ports::use_case::UseCase;
use crate::domain::ports::repositories::board_repository::{BoardRepository, CreateBoardPayload};

pub struct CreateBoardUseCase<T>
    where T: BoardRepository
{
    //board_repository: String
    board_repository: T
}

impl<T> CreateBoardUseCase<T>
    where T: BoardRepository
{
    //pub fn new(board_repository: impl BoardRepository) -> Self {
    pub fn new(board_repository: T) -> Self {
        Self {
            board_repository,
        }
    }
}

impl<T> UseCase<String, String> for CreateBoardUseCase<T>
    where T: BoardRepository
{
    fn execute(&self, payload: String) -> String {
        let pl = CreateBoardPayload {
            name: "Quadro 1".to_string(),
        };

        self.board_repository.create_board(pl);
        payload
    }
}

