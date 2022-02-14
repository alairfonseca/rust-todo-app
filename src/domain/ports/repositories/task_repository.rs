use anyhow::Error;

use serde::{Deserialize, Serialize};
use crate::schema::tasks;
use crate::domain::ports::repositories::board_repository::Board;

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, AsChangeset, Associations)]
#[belongs_to(Board)]
pub struct Task {
    pub id: i32,
    pub board_id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="tasks"]
pub struct NewTask {
    pub board_id: i32,
    pub title: String,
    pub description: String,
}

pub trait TaskRepository {
    fn create_task(&self, payload: NewTask) -> Result<Task, Error>;
    //fn update_task(&self, payload: Task) -> Result<Task, Error>;
}
