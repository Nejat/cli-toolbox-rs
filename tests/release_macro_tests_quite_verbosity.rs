#![allow(unused_mut)] // for testing purposes

use verbosity::Verbosity;

use cli_toolbox::release;

#[test]
fn when_quite_verbosity_should_suppress_default_expr_evaluation() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    release! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_quite_verbosity_should_suppress_terse_expr_evaluation() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    release! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_quite_verbosity_should_suppress_verbose_expr_evaluation() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    release! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}
