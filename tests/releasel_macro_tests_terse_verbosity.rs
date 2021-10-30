#![allow(unused_mut)] // for testing purposes

use test_toolbox::expect;
use verbosity::Verbosity;

use cli_toolbox::release;

#[test]
fn when_terse_verbosity_should_evaluate_default_expr() {
    Verbosity::Terse.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_terse_verbosity_should_evaluate_terse_expr() {
    Verbosity::Terse.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_terse_verbosity_should_suppress_verbose_expr_evaluation() {
    Verbosity::Terse.set_as_global();

    let expected = 0;
    let mut actual = 0;

    release! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}
