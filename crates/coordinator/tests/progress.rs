#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-register-pipeline.rs");
    t.pass("tests/02-heterogenous-pipelines.rs");
}
