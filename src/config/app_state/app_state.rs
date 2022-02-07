use crate::domain::use_cases::CreateBoardUseCase;
use crate::adapters::db::BoardRepositoryImpl;
use diesel::pg::PgConnection;

pub struct AppState {
    pub create_board_use_case: CreateBoardUseCase<BoardRepositoryImpl>,
    pub db_connection: PgConnection,
}
