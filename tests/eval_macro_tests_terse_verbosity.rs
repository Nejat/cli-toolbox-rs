#![allow(unused_mut)] // for testing purposes

use verbosity::Verbosity;

use cli_toolbox::eval;

#[test]
fn when_terse_verbosity_should_evaluate_default_expr() {
    Verbosity::Terse.set_as_global();

    let expected = 42;
    let mut actual = 0;

    eval! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_terse_verbosity_should_evaluate_terse_expr() {
    Verbosity::Terse.set_as_global();

    let expected = 42;
    let mut actual = 0;

    eval! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_terse_verbosity_should_suppress_verbose_expr_evaluation() {
    Verbosity::Terse.set_as_global();

    let expected = 0;
    let mut actual = 0;

    eval! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}
