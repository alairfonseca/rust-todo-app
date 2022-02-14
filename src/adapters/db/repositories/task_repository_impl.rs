use std::rc::Rc;
use crate::domain::ports::repositories::task_repository::{TaskRepository, NewTask, Task};
use diesel::PgConnection;
use crate::schema::tasks;
use diesel::prelude::*;
use anyhow::Error;

pub struct TaskRepositoryImpl {
    db_connection: Rc<PgConnection>,
}

impl TaskRepositoryImpl {
    pub fn new(db_connection: Rc<PgConnection>) -> Self {
        Self {
            db_connection,
        }
    }
}

impl TaskRepository for TaskRepositoryImpl {
    fn create_task(&self, payload: NewTask) -> Result<Task, Error> {
        let result = diesel::insert_into(tasks::table)
            .values(&payload)
            .get_result::<Task>(&*self.db_connection)?;

        Ok(result)
    }

    //fn update_task(&self, payload: Task) -> Result<Board, Error> {
    //    let result = diesel::update(tasks::table)
    //        .filter(tasks::id.eq_all(payload.id))
    //        .set(&payload)
    //        .get_result::<Task>(&*self.db_connection)?;

    //    Ok(result)
    //}
}
