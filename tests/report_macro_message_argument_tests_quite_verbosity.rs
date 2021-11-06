use test_toolbox::capture;
use verbosity::Verbosity;

use cli_toolbox::report;

const EXPECTED_BLANK_STD_ERR: &str = "";

#[test]
fn when_message_with_array_arg_should_output() {
    Verbosity::Quite.set_as_global();
    
    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", [42, 42, 42] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_block_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", { 42 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_call_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    fn method_to_call() -> usize { 42 }
}

#[test]
fn when_message_with_cast_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", 42_isize as usize }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_field_arg_should_output() {
    Verbosity::Quite.set_as_global();

    struct Foo { bar: usize }

    let expected_stdout = "";
    let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", sut.bar }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_if_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", if true { 42 } else { 0 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_index_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";
    let sut = [42; 3];

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", sut[1] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_literal_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", 42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_macro_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", format!("{}", 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_match_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";
    let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", match sut { 42 => sut, _ => unreachable!() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_method_to_call_arg_should_output() {
    Verbosity::Quite.set_as_global();

    struct Foo { bar: usize }

    impl Foo {
        fn method_to_call(&self) -> usize { self.bar }
    }

    let expected_stdout = "";
    let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", sut.method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_paren_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", (21 + 21) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_path_arg_should_output() {
    Verbosity::Quite.set_as_global();

    mod foo { pub mod bar { pub const VALUE: usize = 42; }}
    
    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", foo::bar::VALUE }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_range_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", ..42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_reference_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";
    let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {}", &sut }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_repeat_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", [42; 3] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_try_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let actual_stdout = inner().unwrap();

    assert_eq!(expected_stdout, actual_stdout);

    fn inner() -> Result<String, ()> {
        let (actual_stdout, actual_stderr) = capture! {
            report! { "terse output: {:?}", method_to_try()? }
        };

        assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

        return Ok(actual_stdout);

        fn method_to_try() -> Result<usize, ()> { Ok(42) }
    }
}

#[test]
fn when_message_with_tuple_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", (42, 42, 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unary_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", -(-42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unsafe_arg_should_output() {
    Verbosity::Quite.set_as_global();

    let expected_stdout = "";

    let (actual_stdout, actual_stderr) = capture! {
        report! { "terse output: {:?}", unsafe { unsafe_method_to_call() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    unsafe fn unsafe_method_to_call() -> usize { 42 }
}
