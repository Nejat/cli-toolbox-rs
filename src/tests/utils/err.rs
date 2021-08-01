/// testing macro for capturing stderr
#[doc(hidden)]
#[macro_export]
macro_rules! err {
    ($test:stmt) => {{
        let mut err = BufferRedirect::stderr().expect("redirected stderr required for test");

        $test

        let mut actual = String::new();

        err.read_to_string(&mut actual).unwrap();

        actual
    }}
}