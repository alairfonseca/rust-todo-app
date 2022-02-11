use anyhow::Error;

pub trait UseCase<T, R> {
    fn execute(&self, payload: T) -> Result<R, Error>;
}
