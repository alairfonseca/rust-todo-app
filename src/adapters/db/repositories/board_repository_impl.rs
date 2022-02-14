use crate::domain::ports::repositories::board_repository::{BoardRepository, NewBoard, Board};
use diesel::PgConnection;
use crate::schema::boards;
use diesel::prelude::*;
use anyhow::Error;

pub struct BoardRepositoryImpl {
    db_connection: PgConnection,
}

impl BoardRepositoryImpl {
    pub fn new(db_connection: PgConnection) -> Self {
        Self {
            db_connection,
        }
    }
}

impl BoardRepository for BoardRepositoryImpl {
    fn create_board(&self, payload: NewBoard) -> Result<Board, Error> {
        let result = diesel::insert_into(boards::table)
            .values(&payload)
            .get_result::<Board>(&self.db_connection)?;

        Ok(result)
    }

    fn update_board(&self, payload: Board) -> Result<Board, Error> {
        let result = diesel::update(boards::table)
            .filter(boards::id.eq_all(payload.id))
            .set(&payload)
            .get_result::<Board>(&self.db_connection)?;

        Ok(result)
    }
}
