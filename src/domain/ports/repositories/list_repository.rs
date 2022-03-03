use anyhow::Error;

use serde::{Deserialize, Serialize};
use crate::schema::lists;
use crate::domain::ports::repositories::board_repository::Board;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, AsChangeset, Associations)]
#[belongs_to(Board)]
pub struct List {
    pub id: i32,
    pub board_id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="lists"]
pub struct NewList {
    pub title: String,
}

pub trait ListRepository {
    fn create_list(&self, payload: NewList) -> Result<List, Error>;
}
