// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
pub struct CreateBoardPayload {
    pub name: String,
}

pub trait BoardRepository {
    fn create_board(&self, payload: CreateBoardPayload) -> String;
}
