/// testing macro for capturing stdout
#[doc(hidden)]
#[macro_export]
macro_rules! capture {
    ($test:stmt) => {{
        let mut out = gag::BufferRedirect::stdout().expect("redirected stdout required for test");
        let mut err = gag::BufferRedirect::stderr().expect("redirected stderr required for test");

        $test

        let mut stdout = String::new();
        let mut stderr = String::new();

        std::io::Read::read_to_string(&mut out, &mut stdout).unwrap();
        std::io::Read::read_to_string(&mut err, &mut stderr).unwrap();

        (stdout, stderr)
    }}
}