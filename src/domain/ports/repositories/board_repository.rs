use anyhow::Error;

use serde::{Deserialize, Serialize};
use crate::schema::boards;

#[derive(Debug, Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Board {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="boards"]
pub struct NewBoard {
    pub name: String,
}

pub trait BoardRepository {
    fn create_board(&self, payload: NewBoard) -> Result<Board, Error>;
    fn update_board(&self, payload: Board) -> Result<Board, Error>;
}
