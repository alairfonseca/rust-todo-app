use diesel::PgConnection;

pub trait UseCase<T, R> {
    fn execute(&self, payload: T, db_connection: &PgConnection) -> R;
}
