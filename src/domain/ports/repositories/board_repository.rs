// use serde::{Serialize, Deserialize};
use diesel::PgConnection;
use crate::adapters::db::orms::board::Board;

// #[derive(Serialize, Deserialize, Debug)]
pub struct CreateBoardPayload {
    pub name: String,
}

pub trait BoardRepository {
    fn create_board(&self, payload: CreateBoardPayload, db_connection: &PgConnection) -> Result<Board, diesel::result::Error>;
}
