// TODO: turn this into a proper error including
// implementing std::error::Error
// and std::convert::from
// and std::fmt::Display
#[derive(Debug)]
pub struct DefaultError {}

pub enum RunResult<T, E = DefaultError> {
    Ok(T),
    Retry(E),
    Err(E),
}

impl<T, E> From<Result<T, E>> for RunResult<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => RunResult::Ok(v),
            Err(e) => RunResult::Retry(e),
        }
    }
}

impl<T> From<T> for RunResult<T> {
    fn from(value: T) -> Self {
        return RunResult::Ok(value);
    }
}

impl<T, E> RunResult<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, Self::Ok(_))
    }
    pub fn is_retry(&self) -> bool {
        matches!(self, Self::Retry(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, Self::Err(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            RunResult::Ok(value) => value,
            RunResult::Retry(_) => panic!("Cannot unwrap from an Retry"),
            RunResult::Err(_) => panic!("Cannot unwrap from an Err"),
        }
    }
}

pub trait Pipeline<T, E = DefaultError> {
    fn new() -> Self;

    fn run(&self, args: &dyn std::any::Any) -> RunResult<T, E>;
}
