/// testing macro for capturing stdout
#[doc(hidden)]
#[macro_export]
macro_rules! out {
    ($test:stmt) => {{
        let mut out = BufferRedirect::stdout().expect("redirected stdout required for test");

        $test

        let mut actual = String::new();

        out.read_to_string(&mut actual).unwrap();

        actual
    }}
}