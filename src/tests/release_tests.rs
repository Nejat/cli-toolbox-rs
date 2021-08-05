use crate::*;

#[test]
fn release_evaluate_expression() {
    expect! { EXPECTED: u32 => 42, 0 }
    actual! { @rls actual: u32 }

    // subject under test
    release! { actual = 42 }

    assert_eq!(EXPECTED, actual);
}

#[test]
fn release_evaluate_expression_quite() {
    let cases = build_cases(
        [42, 0, 0], &func_2_step(stringify!(release_evaluate_expression_quite)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! { QUITE actual = 42 }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_expression_terse() {
    let cases = build_cases(
        [0, 42, 42], &func_2_step(stringify!(release_evaluate_expression_terse)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! { TERSE actual = 42 }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_expression_verbose() {
    let cases = build_cases(
        [0, 0, 42], &func_2_step(stringify!(release_evaluate_expression_verbose)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! { VERBOSE actual = 42 }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_expression_terse_or_verbose() {
    let cases = build_cases(
        [0, 24, 42], &func_2_step(stringify!(release_evaluate_expression_terse_or_verbose)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! {
            TERSE   actual = 24;
            VERBOSE actual = 42;
        }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_statement() {
    expect! { EXPECTED: u32 => 42, 0 }
    actual! { @rls actual: u32 }

    // subject under test
    release! {{
        let foo  = 22;
        let junk = 20;

        actual = foo + junk;
    }}

    assert_eq!(EXPECTED, actual);
}

#[test]
fn release_evaluate_statement_quite() {
    let cases = build_cases(
        [42, 0, 0], &func_2_step(stringify!(release_evaluate_statement_quite)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! {
            QUITE => {
                let foo  = 22;
                let junk = 20;

                actual = foo + junk;
            }
        }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_statement_terse() {
    let cases = build_cases(
        [0, 42, 42], &func_2_step(stringify!(release_evaluate_statement_terse)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! {
            TERSE => {
                let foo  = 22;
                let junk = 20;

                actual = foo + junk;
            }
        }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_statement_verbose() {
    let cases = build_cases(
        [0, 0, 42], &func_2_step(stringify!(release_evaluate_statement_verbose)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! {
            VERBOSE => {
                let foo  = 22;
                let junk = 20;

                actual = foo + junk;
            }
        }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[test]
fn release_evaluate_statement_terse_or_verbose() {
    let cases = build_cases(
        [0, 24, 42], &func_2_step(stringify!(release_evaluate_statement_terse_or_verbose)),
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        actual! { @rls actual = 0 }

        // subject under test
        release! {
            TERSE => {
                let junk = 12;
                let foo  = 12;

                actual = foo + junk;
            },
            VERBOSE => {
                let foo  = 22;
                let junk = 20;

                actual = foo + junk;
            }
        }

        assert_eq!(expected, actual, "{}", step);
    }
}

#[cfg(not(debug_assertions))]
fn build_cases(_expected: [u32; 3], step: &str) -> [(Verbosity, u32, String); 3] {
    [
        (Verbosity::Quite,   _expected[0], format!("{} when {}", step, Verbosity::Quite)),
        (Verbosity::Terse,   _expected[1], format!("{} when {}", step, Verbosity::Terse)),
        (Verbosity::Verbose, _expected[2], format!("{} when {}", step, Verbosity::Verbose)),
    ]
}

#[cfg(debug_assertions)]
fn build_cases(_expected: [u32; 3], step: &str) -> [(Verbosity, u32, String); 3] {
    [
        (Verbosity::Quite,   0, format!("{} when {}", step, Verbosity::Quite)),
        (Verbosity::Terse,   0, format!("{} when {}", step, Verbosity::Terse)),
        (Verbosity::Verbose, 0, format!("{} when {}", step, Verbosity::Verbose)),
    ]
}

fn func_2_step(func_name: &str) -> String { func_name.replace("_", " ") }
