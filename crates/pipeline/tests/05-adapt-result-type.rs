// If the pipeline is attached to a function
// which return type is a `Result<T, E>` then also
// then the generate pipeline.run should return a RunResult<T, E>
// instead of a RunResult<Result<T, E>>

use pipeline::pipeline;
use std::{fmt::Display, error::Error};
use schema::{Pipeline, RunResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomError;

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encountered a custom error!")
    }
}

impl Error for CustomError {}

#[pipeline(name => "Foo", retries => 0, retry_delay_secs => 1, cron => "*/5 * * * *")]
fn foo(bar: String) -> std::result::Result<String, CustomError> {
    Ok(bar)
}

fn main() {
    let mut pipeline = Foo::new();
    let args: String = "bar".to_string();
    let actual: RunResult<String, CustomError> = pipeline.run(&args);
    assert!(actual.is_ok());
}
