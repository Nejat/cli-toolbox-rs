use test_toolbox::{actual, capture, expect};

use cli_toolbox::{debug, debugln};

const EXPECTED_BLANK_STD_ERR: &str = "";

#[test]
fn when_optimized_should_evaluate_expression_only_when_unoptimized() {
    expect! { expected = usize::default(), 42 }
    actual! { @dbg actual_value: usize }

    debug! { actual_value = 42 }

    assert_eq!(expected, actual_value);
}

#[test]
fn when_optimized_should_output_debugging_information_only_when_unoptimized() {
    expect! { expected_stdout = "", "DBG: invaluable debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: invaluable debugging output: {}", 42; }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_debugging_should_output_debugging_information_line_only_when_unoptimized() {
    expect! { expected_stdout = "", "DBG: invaluable debugging output: 42\n" }

    let (actual_stdout, actual_stderr) = capture! {
        debugln! { "DBG: invaluable debugging output: {}", 42; }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}
