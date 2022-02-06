use crate::domain::ports::repositories::board_repository::*;

pub struct BoardRepositoryImpl {

}

impl BoardRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl BoardRepository for BoardRepositoryImpl {
    fn create_board(&self, payload: CreateBoardPayload) -> String {
        println!("executando repositorio...");
        payload.name
    }
}

