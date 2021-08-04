/// The `report!` macro provides progress and error reporting.
///
/// # Features
///
/// * you can report user output and error messages in two levels of detail,
///   _i.e._ `Terse` _or_ `Verbose`
///
/// * the user can set the level of reporting they want to endure 🙃,
///   _i.e._ `Quite`_,_ `Terse` _or_ `Verbose`
///
/// * all `Terse` output will also output if the level is set to `Verbose`
///
/// * you can also choose different output for `Terse` or `Verbose`
///
/// * you can format output with application state
///
/// # Terse Examples
///
/// These examples output only if [`Verbosity`] level is greater than `Quite`.
///
/// * _basic output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity; // required by report!
///
/// report! { "some user output" }
/// ```
///
/// * _formatted output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { "some formatted user output; {}", 42 }
/// ```
///
/// * _basic output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { ERR "some error message" }
/// ```
///
/// * _formatted output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { ERR "some formatted error message; {}", -42 }
/// ```
///
/// # Verbose Examples
///
/// These examples output only if [`Verbosity`] level is greater than `Terse`.
///
/// * _basic output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { VERBOSE "some detailedd user output" }
/// ```
///
/// * _formatted output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { VERBOSE "some formatted more detailed user output; {}", 42 }
/// ```
///
/// * _basic output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { ERRV "some detailedd error message" }
/// ```
///
/// * _formatted output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// report! { ERRV "some formatted more detailed error message; {}", -42 }
/// ```
///
/// # Terse or Verbose Examples
///
/// These examples conditionally output if [`Verbosity`] level is greater than `Quite`.
///
/// * _output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// // the following will output only one of the two messages, either branch accepts formatting
/// report! {
///     TERSE   "some user output";
///     VERBOSE "some formatted more detailed user output; {}", 42;
/// }
/// ```
///
/// * _output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::report;
/// use cli_toolbox::Verbosity;
///
/// // the following will output only one of the two messages, either branch accepts formatting
/// report! {
///     ERR  "some error message";
///     ERRV "some formatted more detailed error message; {}", -42;
/// }
/// ```
///
/// [`Verbosity`]: crate::verbosity::Verbosity
#[macro_export]
macro_rules! report {
    // ===============================================================================
    // report if terse
    ($msg:expr) => {
        if Verbosity::is_terse() { report! { @ $msg } }
    };
    // report formatted if terse
    ($msg:expr, $($arg:expr),*) => {
        if Verbosity::is_terse() { report! { @ $msg, $($arg),* } }
    };
    // ===============================================================================
    // report if verbose
    (VERBOSE $msg:expr) => {
        if Verbosity::is_verbose() { report! { @ $msg } }
    };
    // report formatted if verbose
    (VERBOSE $msg:expr, $($arg:expr),* ) => {
        if Verbosity::is_verbose() { report! { @ $msg, $($arg),* } }
    };
    // ===============================================================================
    // report error if terse
    (ERR $msg:expr) => {
        if Verbosity::is_terse() { report! { @ERR $msg } }
    };
    // report formatted error if terse
    (ERR $msg:expr, $($arg:expr),*) => {
        if Verbosity::is_terse() { report! { @ERR $msg, $($arg),* } }
    };
    // ===============================================================================
    // report error if verbose
    (ERRV $msg:expr) => {
        if Verbosity::is_verbose() { report! { @ERR $msg } }
    };
    // report formatted error if verbose
    (ERRV $msg:expr, $($arg:expr),*) => {
        if Verbosity::is_verbose() { report! { @ERR $msg, $($arg),* } }
    };
    // ===============================================================================
    // report terse or verbose
    (
        TERSE $terse_msg:expr;
        VERBOSE $verbose_msg:expr;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ $terse_msg } }
            Verbosity::Verbose => { report! { @ $verbose_msg } }
            _ => {}
        }
    };
    // report terse formatted or verbose
    (
        TERSE $terse_msg:expr, $($terse_arg:expr),*;
        VERBOSE $verbose_msg:expr;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ $terse_msg, $($terse_arg),* } }
            Verbosity::Verbose => { report! { @ $verbose_msg } }
            _ => {}
        }
    };
    // report terse or verbose formatted
    (
        TERSE $terse_msg:expr;
        VERBOSE $verbose_msg:expr, $($verbose_arg:expr),*;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ $terse_msg } }
            Verbosity::Verbose => { report! { @ $verbose_msg, $($verbose_arg),* } }
            _ => {}
        }
    };
    // report terse formatted or verbose formatted
    (
        TERSE $terse_msg:expr, $($terse_arg:expr),*;
        VERBOSE $verbose_msg:expr, $($verbose_arg:expr),*;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ $terse_msg, $($terse_arg),* } }
            Verbosity::Verbose => { report! { @ $verbose_msg, $($verbose_arg),* } }
            _ => {}
        }
    };
    // ===============================================================================
    // report error terse or verbose
    (
        ERR $terse_msg:expr;
        ERRV $verbose_msg:expr;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ERR $terse_msg } }
            Verbosity::Verbose => { report! { @ERR $verbose_msg } }
            _ => {}
        }
    };
    // report error terse formatted or verbose
    (
        ERR $terse_msg:expr, $($terse_arg:expr),*;
        ERRV $verbose_msg:expr;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ERR $terse_msg, $($terse_arg),* } }
            Verbosity::Verbose => { report! { @ERR $verbose_msg } }
            _ => {}
        }
    };
    // report error terse or verbose formatted
    (
        ERR $terse_msg:expr;
        ERRV $verbose_msg:expr, $($verbose_arg:expr),*;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ERR $terse_msg } }
            Verbosity::Verbose => { report! { @ERR $verbose_msg, $($verbose_arg),* } }
            _ => {}
        }
    };
    // report error terse formatted or verbose formatted
    (
        ERR $terse_msg:expr, $($terse_arg:expr),*;
        ERRV $verbose_msg:expr, $($verbose_arg:expr),*;
    ) => {
        match Verbosity::level() {
            Verbosity::Terse   => { report! { @ERR $terse_msg, $($terse_arg),* } }
            Verbosity::Verbose => { report! { @ERR $verbose_msg, $($verbose_arg),* } }
            _ => {}
        }
    };
    // ===============================================================================
    // private output message
    (@ $msg:expr) => { println!($msg); };
    // private output formatted message
    (@ $msg:expr, $($val:expr),*) => { println!($msg, $($val),*); };
    // ===============================================================================
    // private output error message
    (@ERR $msg:expr) => { eprintln!($msg); };
    // private output formatted error message
    (@ERR $msg:expr, $($val:expr),*) => { eprintln!($msg, $($val),*); };
}
