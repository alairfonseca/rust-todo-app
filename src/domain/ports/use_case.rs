pub trait UseCase<T, R> {
    fn execute(&self, payload: T) -> R;
}
