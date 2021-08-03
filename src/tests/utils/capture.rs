/// testing macro for capturing stdout
#[doc(hidden)]
#[macro_export]
macro_rules! capture {
    ($test:stmt) => {{
        let mut out = BufferRedirect::stdout().expect("redirected stdout required for test");
        let mut err = BufferRedirect::stderr().expect("redirected stderr required for test");

        $test

        let mut stdout = String::new();
        let mut stderr = String::new();

        out.read_to_string(&mut stdout).unwrap();
        err.read_to_string(&mut stderr).unwrap();

        (stdout, stderr)
    }}
}