use test_toolbox::capture;
use verbosity::Verbosity;

use cli_toolbox::{report, reportln};

const EXPECTED_BLANK_STD_ERR: &str = "";
const EXPECTED_BLANK_STD_OUT: &str = "";

#[test]
fn when_terse_verbosity_should_report_err_terse() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "error terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! { @err-terse "error terse message" }
    };

    assert_eq!(expected, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @err-terse "error terse message" }
    };

    let expected = "error terse message\n";

    assert_eq!(expected, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_terse_verbosity_should_report_err_terse_not_err_verbose() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "error terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @err-terse "error terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(expected, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let expected = "error terse message\n";

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @err-terse "error terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(expected, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_terse_verbosity_should_report_err_terse_not_verbose() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "error terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @err-terse "error terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(expected, actual_std_err, "report!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "report!");

    let expected = "error terse message\n";

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @err-terse "error terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(expected, actual_std_err, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_OUT, actual_stdout, "reportln!");
}

#[test]
fn when_terse_verbosity_should_suppress_err_verbose_reporting() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

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
fn when_terse_verbosity_should_report_terse() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! { @terse "terse message" }
    };

    assert_eq!(expected, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let expected = "terse message\n";

    let (actual_stdout, actual_std_err) = capture! {
        reportln! { @terse "terse message" }
    };

    assert_eq!(expected, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_terse_verbosity_should_report_terse_not_err_verbose() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @terse "terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(expected, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let expected = "terse message\n";

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @terse "terse message";
            @err-verbose "error verbose message"
        }
    };

    assert_eq!(expected, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_terse_verbosity_should_report_terse_not_verbose() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

    let expected = "terse message";

    let (actual_stdout, actual_std_err) = capture! {
        report! {
            @terse "terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(expected, actual_stdout, "report!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "report!");

    let expected = "terse message\n";

    let (actual_stdout, actual_std_err) = capture! {
        reportln! {
            @terse "terse message";
            @verbose "verbose message"
        }
    };

    assert_eq!(expected, actual_stdout, "reportln!");
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_std_err, "reportln!");
}

#[test]
fn when_terse_verbosity_should_suppress_verbose_reporting() {
    Verbosity::Terse.set_as_global();

    assert_eq!(Verbosity::Terse, Verbosity::level());

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

