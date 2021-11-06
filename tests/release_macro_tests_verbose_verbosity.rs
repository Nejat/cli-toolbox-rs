#![allow(unused_mut)] // for testing purposes

use test_toolbox::expect;
use verbosity::Verbosity;

use cli_toolbox::release;

#[test]
fn when_verbose_verbosity_should_evaluate_default() {
    Verbosity::Verbose.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_verbose_verbosity_should_evaluate_terse() {
    Verbosity::Verbose.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_verbose_verbosity_should_evaluate_verbose() {
    Verbosity::Verbose.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_verbose_verbosity_should_suppress_terse_evaluate_verbose() {
    Verbosity::Verbose.set_as_global();

    expect! { expected = 42, 0 }

    let mut actual = 0;

    release! {
        @terse actual = -21 * 2
        @verbose actual = 42
    }

    assert_eq!(expected, actual)
}
