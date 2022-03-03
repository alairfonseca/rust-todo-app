use std::rc::Rc;

use crate::domain::ports::use_case::UseCase;
use crate::domain::ports::repositories::list_repository::{
    ListRepository, List, NewList
};
use anyhow::Error;

pub struct CreateListUseCase {
    list_repository: Rc<dyn ListRepository>,
}

impl CreateListUseCase {
    pub fn new(list_repository: Rc<dyn ListRepository>) -> Self {
        Self {
            list_repository,
        }
    }
}

impl UseCase<NewList, List> for CreateListUseCase {
    fn execute(&self, payload: NewList) -> Result<List, Error> {
        let insert_result = self.list_repository.create_list(payload);

        match insert_result {
            Ok(value) => Ok(value),
            Err(err) => Err(err),
        }
    }
}
