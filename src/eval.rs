/// The `eval!` macro provides conditional code evaluation according to [`Verbosity`] level.
///
/// # Features
///
/// * you can evaluate code according to level of verbosity,
///   _i.e._ `Quite`, `Terse` _or_ `Verbose`
///
/// * all `Terse` evaluations will also evaluate if the level is set to `Verbose`
///
/// * you can choose unique evaluations for `Terse` or `Verbose` individually
///
/// # Basic Examples
///
/// * _evaluates if [`Verbosity`] is_ `Quite`
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! { QUITE foo() }
/// # fn foo() { /* ... */ }
/// ```
///
/// * _evaluates if [`Verbosity`] is_ `Terse` or `Verbose`
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! { TERSE foo() }
/// # fn foo() { /* ... */ }
/// ```
///
/// * _evaluates if [`Verbosity`] is_ `Verbose`
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! { VERBOSE foo() }
/// # fn foo() { /* ... */ }
/// ```
///
/// # Terse or Verbose Examples
///
/// These examples evaluate conditionally based on [`Verbosity`] level.
///
/// * _terse or verbose code blocks_
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! {
///     TERSE   { foo(); }
///     VERBOSE { junk(); }
/// }
/// # fn foo() { /* ... */ }
/// # fn junk() { /* ... */ }
/// ```
///
/// * _terse or verbose expressions_
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! {
///     TERSE   foo(),
///     VERBOSE junk()
/// }
/// # fn foo() { /* ... */ }
/// # fn junk() { /* ... */ }
/// ```
///
/// * _terse expression or verbose block_
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! {
///     TERSE   foo(),
///     VERBOSE { junk(); }
/// }
/// # fn foo() { /* ... */ }
/// # fn junk() { /* ... */ }
/// ```
///
/// * _terse block or verbose expression_
///
/// ```rust
/// # use cli_toolbox::eval;
/// eval! {
///     TERSE   { foo(); }
///     VERBOSE junk()
/// }
/// # fn foo() { /* ... */ }
/// # fn junk() { /* ... */ }
/// ```
///
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
