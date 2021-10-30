use trybuild::TestCases;

#[test]
fn test_result_macro() {
    let tests = TestCases::new();

    tests.compile_fail("tests/eval_compile_fails/*.rs");
    tests.compile_fail("tests/release_compile_fails/*.rs");
    tests.compile_fail("tests/report_compile_fails/*.rs");
}