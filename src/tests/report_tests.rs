use std::io::Read;

use gag::BufferRedirect;

use crate::*;
use crate::verbosity::Verbosity;

const EXPECTED_BLANK: &str = "";

#[test]
fn report_terse () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some user output\n", "some user output\n"],
        &func_2_step(stringify!(report_terse))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { "some user output" }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_formatted_terse () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some formatted user output; 42\n", "some formatted user output; 42\n"],
        &func_2_step(stringify!(report_formatted_terse))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { "some formatted user output; {}", 42 }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, EXPECTED_BLANK, "some user output\n"],
        &func_2_step(stringify!(report_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { VERBOSE "some user output" }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_formatted_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, EXPECTED_BLANK, "some formatted user output; 42\n"],
        &func_2_step(stringify!(report_formatted_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { VERBOSE "some formatted user output; {}", 42 }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_terse () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some error message\n", "some error message\n"],
        &func_2_step(stringify!(report_error_terse))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { ERR "some error message" }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_formatted_terse () {
    let cases = build_cases(
        [
            EXPECTED_BLANK,
            "some formatted error message: -42\n",
            "some formatted error message: -42\n"
        ],
        &func_2_step(stringify!(report_error_formatted_terse))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { ERR "some formatted error message: {}", -42 }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, EXPECTED_BLANK, "some error message\n"],
        &func_2_step(stringify!(report_error_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { ERRV "some error message" }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_formatted_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, EXPECTED_BLANK, "some formatted error message: -42\n"],
        &func_2_step(stringify!(report_error_formatted_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! { ERRV "some formatted error message: {}", -42 }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_terse_or_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some user output\n", "some elaborated user output\n"],
        &func_2_step(stringify!(report_terse_or_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                TERSE   "some user output";
                VERBOSE "some elaborated user output";
            }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_terse_formatted_or_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some formatted user output; 42\n", "some elaborated user output\n"],
        &func_2_step(stringify!(report_terse_formatted_or_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                TERSE   "some formatted user output; {}", 42;
                VERBOSE "some elaborated user output";
            }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_terse_or_verbose_formatted () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some user output\n", "some elaborated formatted user output; +42+\n"],
        &func_2_step(stringify!(report_terse_or_verbose_formatted))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                TERSE   "some user output";
                VERBOSE "some elaborated formatted user output; +{}+", 42;
            }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_terse_formatted_or_verbose_formatted () {
    let cases = build_cases(
        [
            EXPECTED_BLANK,
            "some formatted user output; 42\n",
            "some elaborated formatted user output; +42+\n"
        ],
        &func_2_step(stringify!(report_terse_formatted_or_verbose_formatted))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                TERSE   "some formatted user output; 42";
                VERBOSE "some elaborated formatted user output; +{}+", 42;
            }
        };

        assert_eq!(expected, actual_out, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_err, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_terse_or_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some error message\n", "some elaborated error message\n"],
        &func_2_step(stringify!(report_error_terse_or_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                ERR  "some error message";
                ERRV "some elaborated error message";
            }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_terse_formatted_or_verbose () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some formatted error message: -42\n", "some elaborated error message\n"],
        &func_2_step(stringify!(report_error_terse_formatted_or_verbose))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                ERR  "some formatted error message: {}", -42;
                ERRV "some elaborated error message";
            }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }

}

#[test]
fn report_error_terse_or_verbose_formatted () {
    let cases = build_cases(
        [EXPECTED_BLANK, "some error message\n", "some elaborated formatted error message: -42-\n"],
        &func_2_step(stringify!(report_error_terse_or_verbose_formatted))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                ERR  "some error message";
                ERRV "some elaborated formatted error message: {}-", -42;
            }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

#[test]
fn report_error_terse_formatted_or_verbose_formatted () {
    let cases = build_cases(
        [
            EXPECTED_BLANK,
            "some formatted error message: -42\n",
            "some elaborated formatted error message: -42-\n"
        ],
        &func_2_step(stringify!(report_error_terse_formatted_or_verbose_formatted))
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let (actual_out, actual_err) = capture! {
            // subject under test
            report! {
                ERR  "some formatted error message: {}", -42;
                ERRV "some elaborated formatted error message: {}-", -42;
            }
        };

        assert_eq!(expected, actual_err, "{}", step);
        assert_eq!(EXPECTED_BLANK, actual_out, "{}: alternate io expected to be blank", step);
    }
}

fn build_cases(expected: [&str; 3], step: &str) -> [(Verbosity, String, String); 3] {
    [
        (Verbosity::Quite, expected[0].to_string(), format!("{} when {}", step, Verbosity::Quite)),
        (Verbosity::Terse, expected[1].to_string(), format!("{} when {}", step, Verbosity::Terse)),
        (Verbosity::Verbose, expected[2].to_string(), format!("{} when {}", step, Verbosity::Verbose))
    ]
}

fn func_2_step(func_name: &str) -> String {
    func_name.replace("_", " ")
}

