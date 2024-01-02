// It is quite common to redefine a Result type
// inside the crate to avoid tediously providing
// the error type. Therefore, it would be very
// handy if the pipeline proc-macro can also
// infer the error type for this cases
use pipeline::pipeline;
use schema::{Pipeline, RunResult};
use std::{error::Error, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encountered a custom error!")
    }
}

impl Error for CustomError {}

pub type Result<T> = std::result::Result<T, CustomError>;

#[pipeline(name => "Bar", retries => 0, retry_delay_secs => 1, cron => "*/5 * * * *")]
fn bar(_baz: String) -> Result<()> {
    Err(CustomError)
}

fn main() {
    let pipeline = Bar::new();
    let args: String = "baz".to_string();
    let actual: RunResult<(), CustomError> = pipeline.run(&args);
    assert!(!actual.is_ok());
    let err: CustomError = actual.unwrap_err();
    let expected = CustomError {};
    assert_eq!(err, expected);
}
