#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.compile_fail("tests/02-missing-attribute.rs");
    t.pass("tests/03-generate-pipeline.rs");
}
