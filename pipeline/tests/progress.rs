#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.compile_fail("tests/02-missing-attribute.rs");
    t.pass("tests/03-generate-pipeline.rs");
    t.pass("tests/04-run-pipeline.rs");
    t.pass("tests/05-adapt-result-type.rs");

    // longterm goals
    // t.pass("tests/??-implicit-error-type.rs");
}
