/// The `report!` macro provides progress and error reporting
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