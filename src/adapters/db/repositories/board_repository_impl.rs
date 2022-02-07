use crate::domain::ports::repositories::board_repository::*;
use diesel::PgConnection;
use super::super::orms::board::{ Board, NewBoard };
use crate::schema::boards;
use diesel::prelude::*;

pub struct BoardRepositoryImpl {

}

impl BoardRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl BoardRepository for BoardRepositoryImpl {
    fn create_board(&self, payload: CreateBoardPayload, db_connection: &PgConnection) -> Result<Board, diesel::result::Error> {
        println!("executando repositorio...");
        
        let new_board = NewBoard {
            name: payload.name.clone(),
        };

        let result = diesel::insert_into(boards::table)
            .values(&new_board)
            .get_result::<Board>(db_connection)?;

        Ok(result)
    }
}
