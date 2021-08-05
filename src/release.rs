/// The `release!` macro provides release only expression and statement evaluation
#[macro_export]
macro_rules! release {
    // ===============================================================================
    // evaluate expression in release
    ($expr:expr) => {
        #[cfg(not(debug_assertions))]
        $expr
    };
    // ===============================================================================
    // evaluate expression in release if verbosity is quite
    (QUITE $expr:expr) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_quite() { $expr }
    };
    // evaluate expression in release if verbosity is terse
    (TERSE $expr:expr) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_terse() { $expr }
    };
    // evaluate expression in release if verbosity is verbose
    (VERBOSE $expr:expr) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_verbose() { $expr }
    };
    // ===============================================================================
    // evaluate terse or verbose expression in release if verbosity matches
    (
        TERSE   $terse_expr:expr;
        VERBOSE $verbose_expr:expr;
    ) => {
        #[cfg(not(debug_assertions))]
        match Verbosity::level() {
            Verbosity::Terse   => { $terse_expr }
            Verbosity::Verbose => { $verbose_expr }
            _ => {}
        }
    };
    // ===============================================================================
    // evaluate statement in release
    ($stmt:stmt) => {
        #[cfg(not(debug_assertions))]
        $stmt
    };
    // ===============================================================================
    // evaluate statement in release if verbosity is quite
    (QUITE => $stmt:stmt) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_quite() { $stmt }
    };
    // evaluate statement in release if verbosity is terse
    (TERSE => $stmt:stmt) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_terse() { $stmt }
    };
    // evaluate statement in release if verbosity is verbose
    (VERBOSE => $stmt:stmt) => {
        #[cfg(not(debug_assertions))]
        if Verbosity::is_verbose() { $stmt }
    };
    // ===============================================================================
    // evaluate terse or verbose statement in release if verbosity matches
    (
        TERSE   => $terse_expr:stmt,
        VERBOSE => $verbose_expr:stmt
    ) => {
        #[cfg(not(debug_assertions))]
        match Verbosity::level() {
            Verbosity::Terse   => { $terse_expr }
            Verbosity::Verbose => { $verbose_expr }
            _ => {}
        }
    };
}
