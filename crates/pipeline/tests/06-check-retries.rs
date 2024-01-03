// This test checks whether the
// pipeline run will return a RunResult::Failure if
// the number of retries is 0
// and a RunResult::Retry if the number of retries is bigger
// than 0
use pipeline::pipeline;
use schema::Pipeline;
use std::io::Error;

#[pipeline(name => "OneRetryPipeline", retries => 1, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn one_retry() -> std::result::Result<(), Error> {
    let io_err: Error = Error::other("oh no!");
    Err(io_err)
}
fn test_one_retry() {
    let mut pipeline = OneRetryPipeline::new();
    let args = ();
    let actual = pipeline.run(&args);
    assert!(actual.is_retry());

    // pipeline should stop retrying after retries have been exhausted
    let actual = pipeline.run(&args);
    assert!(!actual.is_retry());
    assert!(actual.is_err());
}

#[pipeline(name => "NoMoreRetries", retries => 0, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn no_more_retries() -> std::result::Result<(), Error> {
    let io_err: Error = Error::other("oh no!");
    Err(io_err)
}

fn test_no_more_retries() {
    let mut pipeline = NoMoreRetries::new();
    let args = ();
    let actual = pipeline.run(&args);
    assert!(!actual.is_retry());
    assert!(actual.is_err());
}

#[pipeline(name => "WorksOneSecondTry", retries => 1, retry_delay_secs => 120, cron => "*/5 * * * *")]
fn works_one_second_try(try_count: usize) -> std::result::Result<(), Error> {
    if try_count >= 2 {
        Ok(())
    } else {
        Err(Error::other("oh no!"))
    }
}

fn test_pipeline_works_after_retry() {
    let mut pipeline = WorksOneSecondTry::new();
    let args: usize = 1;
    let actual = pipeline.run(&args);
    assert!(actual.is_retry());

    let args: usize = 2;
    let actual = pipeline.run(&args);
    assert!(!actual.is_retry());
    assert!(actual.is_ok());
}

fn main() {
    test_no_more_retries();
    test_one_retry();
    test_pipeline_works_after_retry();
}
