/// The `release!` macro provides release only expression and statement evaluation
///
/// # Features
///
/// * you can define expressions and statements to evaluate
///
/// * you can apply [`Verbosity`] filters, _i.e._ <`Quite`|`Terse`|`Verbose`>
///
/// # Basic Example
///
/// Unconditionally evaluates statement in release build.
///
/// ```rust
/// # use cli_toolbox::release;
/// # use cfg_if::cfg_if;
/// release! {{
///     do_something_profound();
/// }}
///
/// fn do_something_profound() {
///     cfg_if! {
///         if #[cfg(not(debug_assertions))] {
///             /* i will not explode today */
///         } else {
///             panic!("you didn't wear your peril sensitive glasses, did you?");
///         }
///     }
/// }
///
/// # // what? you thought I was a one trick pony? pft! ☺️
/// ```
///
/// # Conditional Examples
///
/// Conditionally evaluate statement or expression in release build.
///
/// * `Quite`
///
/// ```rust
/// # use cli_toolbox::release;
/// # use cli_toolbox::Verbosity;
/// Verbosity::Terse.set_as_global();
///
/// // will only evaluate in release and level at quite
/// release! {
///     QUITE => {
///         shhhh();
///     }
/// }
///
/// fn shhhh() {
///     panic!("poof!");
/// }
/// ```
///
/// * `Terse`
///
/// ```rust
/// # use cli_toolbox::release;
/// # use cli_toolbox::Verbosity;
/// Verbosity::Quite.set_as_global();
///
/// // will only evaluate in release and level at terse or verbose
/// release! {
///     TERSE => {
///         henny_penny();
///     }
/// }
///
/// fn henny_penny() {
///     panic!("the sky is falling!!!");
/// }
/// ```
///
/// * `Verbose`
///
/// ```rust
/// # use cli_toolbox::release;
/// # use cli_toolbox::Verbosity;
/// Verbosity::Terse.set_as_global();
///
/// // will only evaluate in release and level at verbose
/// release! {
///     VERBOSE => {
///         kaboom();
///     }
/// }
///
/// fn kaboom() {
///     panic!("pop goes the, pop goes the windin of the weasel");
/// }
/// ```
///
/// * `Terse` or `Verbose`
///
/// ```rust
/// # use cli_toolbox::release;
/// # use cli_toolbox::Verbosity;
/// Verbosity::Quite.set_as_global();
///
/// // will only evaluate in release and level at verbose
/// release! {
///     TERSE   => { henny_penny(); },
///     VERBOSE => { kaboom(); }
/// }
///
/// fn henny_penny() {
///     panic!("the sky is falling!!!");
/// }
///
/// fn kaboom() {
///     panic!("pop goes the, pop goes the windin of the weasel");
/// }
/// ```
///
/// [`Verbosity`]: crate::verbosity::Verbosity
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
