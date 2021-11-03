#![allow(unused_mut)] // for testing purposes

use verbosity::Verbosity;

use cli_toolbox::eval;

#[test]
fn when_quite_verbosity_should_suppress_default() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    eval! { actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_quite_verbosity_should_suppress_terse() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    eval! { @terse actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_quite_verbosity_should_suppress_verbose() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    eval! { @verbose actual = 42 }

    assert_eq!(expected, actual)
}

#[test]
fn when_quite_verbosity_should_suppress_terse_verbose() {
    Verbosity::Quite.set_as_global();

    let expected = 0;
    let mut actual = 0;

    eval! {
        @terse actual = 21 * 2
        @verbose actual = 42
    }

    assert_eq!(expected, actual)
}
