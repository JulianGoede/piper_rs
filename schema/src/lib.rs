use std::error::Error;

#[derive(Debug)]
pub enum RunResult<T, E = Box<dyn Error>> {
    Ok(T),
    Retry(E),
    Err(E),
}

impl<T, E, U> From<Result<T, U>> for RunResult<T, E> where U: Into<E> {
    fn from(value: Result<T, U>) -> Self {
        match value {
            Ok(v) => RunResult::Ok(v),
            Err(e) => RunResult::Retry(e.into()),
        }
    }
}

impl<T, E, U> Into<Result<T, U>> for RunResult<T, E> where E: Into<U>{
    fn into(self) -> Result<T, U> {
        match self {
            RunResult::Ok(t) => Result::Ok(t),
            RunResult::Retry(e) => Result::Err(e.into()),
            RunResult::Err(e) => Result::Err(e.into()),
        }
    }
}

impl<T, E> From<T> for RunResult<T, E> {
    fn from(value: T) -> Self {
        RunResult::Ok(value)
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

    pub fn unwrap_err(self) -> E {
        match self {
            RunResult::Ok(_) => panic!("Expected an Err but was Ok"),
            RunResult::Retry(e) => e,
            RunResult::Err(e) => e,
        }
    }
}

pub trait Pipeline<T, E = Box<dyn Error>> {
    fn new() -> Self;

    fn run(&mut self, args: &dyn std::any::Any) -> RunResult<T, E>;
}
