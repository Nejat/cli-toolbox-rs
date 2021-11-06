use test_toolbox::{capture, expect};

use cli_toolbox::debug;

const EXPECTED_BLANK_STD_ERR: &str = "";

#[test]
fn when_message_with_array_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: [42, 42, 42]" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", [42, 42, 42] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_block_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", { 42 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_call_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    #[cfg(debug_assertions)]
    fn method_to_call() -> usize { 42 }
}

#[test]
fn when_message_with_cast_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", 42_isize as usize }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_field_arg_should_output() {
    #[cfg(debug_assertions)]
    struct Foo {
        bar: usize,
    }

    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    #[cfg(debug_assertions)]
        let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", sut.bar }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_if_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", if true { 42 } else { 0 } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_index_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    #[cfg(debug_assertions)]
        let sut = [42; 3];

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", sut[1] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_literal_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", 42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_macro_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", format!("{}", 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_match_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    #[cfg(debug_assertions)]
        let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", match sut { 42 => sut, _ => unreachable!() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_method_to_call_arg_should_output() {
    #[cfg(debug_assertions)]
    struct Foo {
        bar: usize,
    }

    #[cfg(debug_assertions)]
    impl Foo {
        fn method_to_call(&self) -> usize { self.bar }
    }

    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    #[cfg(debug_assertions)]
        let sut = Foo { bar: 42 };

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", sut.method_to_call() }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_paren_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", (21 + 21) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_path_arg_should_output() {
    #[cfg(debug_assertions)]
    mod foo {
        pub mod bar { pub const VALUE: usize = 42; }
    }

    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", foo::bar::VALUE }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_range_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: ..42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", ..42 }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_reference_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    #[cfg(debug_assertions)]
        let sut = 42;

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {}", &sut }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_repeat_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: [42, 42, 42]" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", [42; 3] }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_try_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let actual_stdout = inner().unwrap();

    assert_eq!(expected_stdout, actual_stdout);

    fn inner() -> Result<String, ()> {
        let (actual_stdout, actual_stderr) = capture! {
            debug! { "DBG: debugging output: {:?}", method_to_try()? }
        };

        assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

        return Ok(actual_stdout);

        #[cfg(debug_assertions)]
        fn method_to_try() -> Result<usize, ()> { Ok(42) }
    }
}

#[test]
fn when_message_with_tuple_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: (42, 42, 42)" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", (42, 42, 42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unary_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", -(-42) }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);
}

#[test]
fn when_message_with_unsafe_arg_should_output() {
    expect! { expected_stdout = "", "DBG: debugging output: 42" }

    let (actual_stdout, actual_stderr) = capture! {
        debug! { "DBG: debugging output: {:?}", unsafe { unsafe_method_to_call() } }
    };

    assert_eq!(expected_stdout, actual_stdout);
    assert_eq!(EXPECTED_BLANK_STD_ERR, actual_stderr);

    #[cfg(debug_assertions)]
    unsafe fn unsafe_method_to_call() -> usize { 42 }
}
