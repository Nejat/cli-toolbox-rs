use crate::*;

#[test]
fn evaluate_expression_if_quite() {
    let cases = build_cases([42, 0, 0], stringify!(evaluate_expression_if_quite));

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! { QUITE _actual = 42 }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_expression_if_terse() {
    let cases = build_cases([0, 42, 42], stringify!(evaluate_expression_if_terse));

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! { TERSE _actual = 42 }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_expression_if_verbose() {
    let cases = build_cases([0, 0, 42], stringify!(evaluate_expression_if_verbose));

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! { VERBOSE _actual = 42 }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_terse_block_or_verbose_block_if_verbose_or_verbose() {
    let cases = build_cases([0, 24, 42], stringify!(evaluate_terse_block_or_verbose_block_if_verbose_or_verbose));

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! {
            TERSE   { _actual = 24; }
            VERBOSE { _actual = 42; }
        }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_terse_expression_or_verbose_expression_if_verbose_or_verbose() {
    let cases = build_cases(
        [0, 24, 42],
        stringify!(evaluate_terse_expression_or_verbose_expression_if_verbose_or_verbose)
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! {
            TERSE   _actual = 24,
            VERBOSE _actual = 42
        }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_terse_block_or_verbose_expression_if_verbose_or_verbose() {
    let cases = build_cases(
        [0, 24, 42],
        stringify!(evaluate_terse_block_or_verbose_expression_if_verbose_or_verbose)
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! {
            TERSE   { _actual = 24; }
            VERBOSE _actual = 42
        }

        assert_eq!(expected, _actual, "{}", step);
    }
}

#[test]
fn evaluate_terse_expression_or_verbose_block_if_verbose_or_verbose() {
    let cases = build_cases(
        [0, 24, 42],
        stringify!(evaluate_terse_expression_or_verbose_block_if_verbose_or_verbose)
    );

    for (level, expected, step) in cases {
        level.set_as_global_for_test();

        let mut _actual = 0;

        // subject under test
        eval! {
            TERSE   _actual = 24,
            VERBOSE { _actual = 42; }
        }

        assert_eq!(expected, _actual, "{}", step);
    }
}

fn build_cases(_expected: [u32; 3], func_name: &str) -> [(Verbosity, u32, String); 3] {
    let step = func_name.replace("_", " ");
    [
        (Verbosity::Quite,   _expected[0], format!("{} when {}", step, Verbosity::Quite)),
        (Verbosity::Terse,   _expected[1], format!("{} when {}", step, Verbosity::Terse)),
        (Verbosity::Verbose, _expected[2], format!("{} when {}", step, Verbosity::Verbose)),
    ]
}
