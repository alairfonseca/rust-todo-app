use serde::{Deserialize, Serialize};
use crate::schema::boards;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Board {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name="boards"]
pub struct NewBoard {
    pub name: String,
}
