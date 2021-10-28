use test_toolbox::capture;
use verbosity::Verbosity;

use cli_toolbox::{report, reportln};

const EXPECTED_BLANK_STD_ERR: &str = "";
const EXPECTED_BLANK_STD_OUT: &str = "";

#[test]
fn when_quite_verbosity_should_suppress_err_terse_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! { @err-terse "error terse message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @err-terse "error terse message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_err_terse_err_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @err-terse "error terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @err-terse "error terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_err_terse_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @err-terse "error terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @err-terse "error terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_err_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! { @err-verbose "error verbose message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @err-verbose "error verbose message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_terse_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! { @terse "terse message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @terse "terse message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_terse_err_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @terse "terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @terse "terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_terse_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @terse "terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @terse "terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_quite_verbosity_should_suppress_verbose_reporting() {
    Verbosity::Quite.set_as_global();

    assert_eq!(Verbosity::Quite, Verbosity::level());

    let (actual_stdout, actual_std_err) = capture! {
        report! { @verbose "verbose message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @verbose "verbose message" }
    };

    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}
