use actix_web::web;
use diesel::PgConnection;
use crate::adapters::db::orms::board::NewBoard;

use super::super::ports::use_case::UseCase;
use crate::{domain::ports::repositories::board_repository::{BoardRepository, CreateBoardPayload}, adapters::db::orms::board::Board};
use anyhow::Error;

pub struct CreateBoardUseCase<T>
    where T: BoardRepository
{
    board_repository: T
}

impl<T> CreateBoardUseCase<T>
    where T: BoardRepository
{
    pub fn new(board_repository: T) -> Self {
        Self {
            board_repository,
        }
    }
}

impl<T> UseCase<String, Board> for CreateBoardUseCase<T>
    where T: BoardRepository
{
    fn execute(&self, payload: web::Json<NewBoard>, db_connection: &PgConnection) -> Result<Board, Error> {
        let pl = CreateBoardPayload {
            name: payload.name.clone(),
        };

        let insert_result = self.board_repository.create_board(pl, db_connection);

        match insert_result {
            Ok(value) => Ok(value),
            Err(err) => Err(Error::new(err)),
        }
    }
}

