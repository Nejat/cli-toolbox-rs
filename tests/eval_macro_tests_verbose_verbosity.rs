#![allow(unused_mut)] // for testing purposes

use verbosity::Verbosity;

use cli_toolbox::eval;

#[test]
fn when_verbose_verbosity_should_evaluate_default_expr() {
    Verbosity::Verbose.set_as_global();

    let expected = 42;
    let mut actual = 0;

    eval! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_verbose_verbosity_should_evaluate_terse_expr() {
    Verbosity::Verbose.set_as_global();

    let expected = 42;
    let mut actual = 0;

    eval! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_verbose_verbosity_should_evaluate_verbose_expr() {
    Verbosity::Verbose.set_as_global();

    let expected = 42;
    let mut actual = 0;

    eval! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}
