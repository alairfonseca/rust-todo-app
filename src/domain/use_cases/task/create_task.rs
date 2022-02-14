use std::rc::Rc;

use crate::domain::ports::use_case::UseCase;
use crate::domain::ports::repositories::task_repository::{
    TaskRepository, Task, NewTask
};
use anyhow::Error;

pub struct CreateTaskUseCase {
    task_repository: Rc<dyn TaskRepository>,
}

impl CreateTaskUseCase {
    pub fn new(task_repository: Rc<dyn TaskRepository>) -> Self {
        Self {
            task_repository,
        }
    }
}

impl UseCase<NewTask, Task> for CreateTaskUseCase {
    fn execute(&self, payload: NewTask) -> Result<Task, Error> {
        let insert_result = self.task_repository.create_task(payload);

        match insert_result {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
