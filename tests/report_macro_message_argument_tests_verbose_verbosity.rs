use test_toolbox::capture;
use verbosity::Verbosity;

use cli_toolbox::report;

const EXPECTED_BLANK_STD_ERR: &str = "";

#[test]
fn when_message_with_array_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: [42, 42, 42]";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", [42, 42, 42] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_block_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", { 42 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_call_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    fn method_to_call() -> usize { 42 }
}

#[test]
fn when_message_with_cast_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", 42_isize as usize }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_field_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    struct Foo {
        bar: usize,
    }

    let expected_stdout = "verbose output: 42";
    let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", sut.bar }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_if_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", if true { 42 } else { 0 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_index_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";
    let sut = [42; 3];

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", sut[1] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_literal_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", 42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_macro_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", format!("{}", 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_match_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";
    let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", match sut { 42 => sut, _ => unreachable!() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_method_to_call_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    struct Foo {
        bar: usize,
    }

    impl Foo {
        fn method_to_call(&self) -> usize { self.bar }
    }

    let expected_stdout = "verbose output: 42";
    let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", sut.method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_paren_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", (21 + 21) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_path_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    mod foo { pub mod bar { pub const VALUE: usize = 42; } }

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", foo::bar::VALUE }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_range_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: ..42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", ..42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_reference_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";
    let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {}", &sut }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_repeat_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: [42, 42, 42]";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", [42; 3] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_try_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let actual_stdout = inner().unwrap();

    assert_eq!(expected_stdout, actual_stdout);

    fn inner() -> Result<String, ()> {
        let (actual_stdout, actual_stderr) = capture! {
            report! { @verbose "verbose output: {:?}", method_to_try()? }
        };

        assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

        return Ok(actual_stdout);

        fn method_to_try() -> Result<usize, ()> { Ok(42) }
    }
}

#[test]
fn when_message_with_tuple_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: (42, 42, 42)";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", (42, 42, 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unary_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", -(-42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unsafe_arg_should_output() {
    Verbosity::Verbose.set_as_global();

    let expected_stdout = "verbose output: 42";

    let (actual_stdout, actual_stderr) = capture! {
        report! { @verbose "verbose output: {:?}", unsafe { unsafe_method_to_call() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    unsafe fn unsafe_method_to_call() -> usize { 42 }
}
