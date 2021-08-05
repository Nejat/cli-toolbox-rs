/// The `eval!` macro provides conditional code evaluation according to [`Verbosity`] level.
/// [`Verbosity`]: crate::verbosity::Verbosity
#[macro_export]
macro_rules! eval {
    // ===============================================================================
    // evaluate if quite
    (QUITE $expr:expr) => {
        if $crate::Verbosity::is_quite() { $expr }
    };
    // ===============================================================================
    // evaluate if terse
    (TERSE $expr:expr) => {
        if $crate::Verbosity::is_terse() { $expr }
    };
    // ===============================================================================
    // evaluate if verbose
    (VERBOSE $expr:expr) => {
        if $crate::Verbosity::is_verbose() { $expr }
    };
    // ===============================================================================
    // evaluate terse or verbose blocks
    (
        TERSE   $terse_block:block
        VERBOSE $verbose_block:block
    ) => {
        match $crate::Verbosity::level() {
            $crate::Verbosity::Terse   => $terse_block
            $crate::Verbosity::Verbose => $verbose_block
            _ => {}
        }
    };
    // evaluate terse or verbose expressions
    (
        TERSE   $terse_expr:expr,
        VERBOSE $verbose_expr:expr
    ) => {
        match $crate::Verbosity::level() {
            $crate::Verbosity::Terse   => $terse_expr,
            $crate::Verbosity::Verbose => $verbose_expr,
            _ => {}
        }
    };
    // evaluate terse block or verbose expression
    (
        TERSE   $terse_block:block
        VERBOSE $verbose_expr:expr
    ) => {
        match $crate::Verbosity::level() {
            $crate::Verbosity::Terse   => $terse_block
            $crate::Verbosity::Verbose => $verbose_expr,
            _ => {}
        }
    };
    // evaluate terse expression or verbose block
    (
        TERSE   $terse_expr:expr,
        VERBOSE $verbose_block:block
    ) => {
        match $crate::Verbosity::level() {
            $crate::Verbosity::Terse   => $terse_expr,
            $crate::Verbosity::Verbose => $verbose_block
            _ => {}
        }
    };
}
