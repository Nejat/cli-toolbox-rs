/// The `debug!` macro provides debug only code evaluation and user output.
///
/// # Features
///
/// * you can output basic or formatted debug information to `stdout` or `stderr`
///
/// * you can execute a block of debug code
///
/// * you can operate conditionally based on expressions that evaluate to ```Result<T,E>```
///
///     * conditionally output basic or formatted debug information to `stdout` or `stderr`
///
///       ```Ok``` or ```Err``` _result value automatically appended to output_
///
///       ```Ok``` or ```Err``` _result value can be suppressed by prefixing the_ ```OK```
///       and/or ```ERR``` _keywords with an underscore, i.e._ ```_OK``` or ```_ERR```
///
///       _outputs that append result values expect conditional expressions to evaluate to_
///       ```Result<T: Debug, E: Debug>```
///
///     * conditionally execute a block of debug code -
///       _optionally use result value in code block_
///
/// * prefixes output messages appropriately, i.e. "```DEBUG: ...```" or "```ERROR: ...```"
///
/// # Unconditional Output Examples
///
/// ### ```stdout```
///
/// * _basic output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { "debug output" }
/// ```
/// ```text
/// DEBUG: debug output
/// ```
///
/// * _formatted output to_ `stdout`
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { "debug output with formatted value {}", 42 }
/// ```
/// ```text
/// DEBUG: debug output with formatted value 42
/// ```
///
/// ### ```stderr```
///
/// * _basic output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { "descriptive error message" }
/// ```
/// ```text
/// ERROR: descriptive error message
/// ```
///
/// * _formatted output to_ `stderr`
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { "descriptive error message with formatted value {}", -42 }
/// ```
/// ```text
/// ERROR: descriptive error message with formatted value -42
/// ```
///
/// # Conditional Output Examples
///
/// Using expressions that evaluate to ```Result<T,E>``` you can control debug output.
///
/// ### Expecting ```Ok<T>```
///
/// * _outputs only if_ ```Result<T,E>``` _is_ ```Ok<T>```_, appending the value to the end_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => OK "debug output when foo is OK" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Ok(42) }
/// ```
/// ```text
/// DEBUG: debug output when foo is OK: 42
/// ```
///
/// * _does not output if_ ```Result<T,E>``` _is_ ```Err<E>```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => OK "debug output when foo is OK" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Err(false) }
/// ```
/// ```text
/// ```
///
/// * _the_ ```Ok<T>``` _value can be suppressed by prefixing the_
/// ```OK``` _keyword with an underscore, i.e._ ```_OK```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => _OK "debug output when foo is OK" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Ok(42) }
/// ```
/// ```text
/// DEBUG: debug output when foo is OK
/// ```
///
/// ### Expecting ```ERR<E>```
///
/// * _outputs only if_ ```Result<T,E>``` _is_ ```ERR<E>```_, appending the exception to the end_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => ERR "error message when foo is ERR" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Err(false) }
/// ```
/// ```text
/// ERROR: error message when foo is OK: false
/// ```
///
/// * _does not output if_ ```Result<T,E>``` _is_ ```OK<T>```_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => ERR "error message when foo is ERR" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Ok(42) }
/// ```
/// ```text
/// ```
///
/// * _the_ ```ERR<E>``` _value can be suppressed by prefixing the_
/// ```ERR``` _keyword with an underscore, i.e._ ```_ERR```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! { foo() => _ERR "error message when foo is ERR" }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<i32, bool> { Err(false) }
/// ```
/// ```text
/// ERROR: error message when foo is ERR
/// ```
///
/// # Conditional Output Examples - Exhaustive
///
/// The previous set of conditional examples addressed the simple case of checking
/// for a single outcome of a conditional expression.
///
/// You can also use conditional expressions exhaustively to handle both outcomes.
///
/// All of the features described also apply:
///
/// * result values are appended automatically to end of output
/// * result values can be suppressed
/// * output messages can be formatted
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     foo() =>
///         // with additional formatting and discards ok value
///         _OK "debug output when foo is OK; {}", 42;
///         // no additional formatting and appends err value
///         ERR "error message when foo is ERR";
/// }
///
/// #[cfg(debug_assertions)]
/// fn foo() -> Result<bool, i32> { Ok(true) }
/// ```
/// ```text
/// DEBUG: debug output when foo is OK; 42
/// ```
///
/// # Unconditional Code Block Example
///
/// * _execute a block of code only in debug_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {{
///     let foo = foo();
///
///     if foo != 42 {
///         panic!("run for your lives, the sky is falling!")
///     }
///
///     fn foo() -> i32 { 42 }
/// }}
/// ```
///
/// # Conditional Code Block Examples
///
/// Using expressions that evaluate to ```Result<T,E>``` you can control debug code block execution.
///
/// ### Expecting ```Ok<T>```
///
/// * _executes only if_ ```Result<T,E>``` _is_ ```Ok<T>```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => OK {
///         let foo = foo();
///
///         if foo != 42 {
///             panic!("run for your lives, the sky is falling!")
///         }
///
///         fn foo() -> i32 { 42 }
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Ok(42) }
/// ```
///
/// * _does not execute if_ ```Result<T,E>``` _is_ ```Err<E>```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => OK {
///         panic!("run for your lives, the sky is falling!")
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Err(true) }
/// ```
///
/// * _the_ ```Ok<T>``` _value can be captured by providing a valid identifier_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => OK ok {
///         let foo = foo();
///
///         if ok != 42 || foo != 42 {
///             panic!("run for your lives, the sky is falling!")
///         }
///
///         fn foo() -> i32 { 42 }
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Ok(42) }
/// ```
///
/// ### Expecting ```ERR<E>```
///
/// * _executes only if_ ```Result<T,E>``` _is_ ```ERR<E>```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => ERR {
///         let foo = foo();
///
///         if foo != 42 {
///             panic!("run for your lives, the sky is falling!")
///         }
///
///         fn foo() -> i32 { 42 }
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Err(true) }
/// ```
///
/// * _does not execute if_ ```Result<T,E>``` _is_ ```OK<T>```
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => ERR {
///         panic!("run for your lives, the sky is falling!")
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Ok(42) }
/// ```
///
/// * _the_ ```ERR<E>``` _value can be captured by providing a valid identifier_
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() => ERR err {
///         let foo = foo();
///
///         if !err || foo != 42 {
///             panic!("run for your lives, the sky is falling!")
///         }
///
///         fn foo() -> i32 { 42 }
///     }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Err(true) }
/// ```
///
/// # Conditional Code Blocks Examples - Exhaustive
///
/// Similar to conditional debug output, you can also use conditional expressions
/// exhaustively to handle both outcomes for executing debug code blocks.
///
/// As before, all of the features described for conditional code blocks also apply:
///
/// * result values are suppressed by default
/// * result values can be capture to be used in the executing code block branch
///
/// ```rust
/// # use cli_toolbox::debug;
/// debug! {
///     junk() =>
///         // captures ok value to use in code block
///         OK ok {
///             if ok != 42 {
///                 panic!("just a little, don't be so dramatic: the sky is not falling")
///             }
///         }
///         // defaults to not capturing err value
///         ERR { reset_some_arbitrary_state() }
/// }
///
/// #[cfg(debug_assertions)]
/// fn junk() -> Result<i32, bool> { Ok(42) }
///
/// fn reset_some_arbitrary_state() { /* ... */ }
/// ```
#[macro_export]
macro_rules! debug {
    // ===============================================================================
    // evaluate block
    ($debug:block) => {
        #[cfg(debug_assertions)]
        $debug
    };
    // ===============================================================================
    // output message
    ($debug:expr) => {
        #[cfg(debug_assertions)]
        debug! { @ $debug }
    };
    // output formatted message
    ($debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        debug! { @ $debug, $($val),* }
    };
    // ===============================================================================
    // output error message
    (ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        debug! { @ERR $debug }
    };
    // output formatted error message
    (ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        debug! { @ERR $debug, $($val),* }
    };
    // ===============================================================================
    // evaluates block if result is ok
    ($result:expr => OK $ok:ident $debug:block) => {
        #[cfg(debug_assertions)]
        if let Ok($ok) = $result { $debug }
    };
    // evaluates block if result is ok, discards ok
    ($result:expr => OK $debug:block) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { $debug }
    };
    // ===============================================================================
    // evaluates block if result is err
    ($result:expr => ERR $err:ident $debug:block) => {
        #[cfg(debug_assertions)]
        if let Err($err) = $result { $debug }
    };
    // evaluates block if result is err, discards err
    ($result:expr => ERR $debug:block) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { $debug }
    };
    // ===============================================================================
    // evaluates blocks based on result
    (
        $result:expr =>
            OK $ok:ident $succeeded:block
            ERR $err:ident $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok($ok) => $succeeded
            Err($err) => $failed
        }
    };
    // evaluates blocks based on result, discards ok
    (
        $result:expr =>
            OK $succeeded:block
            ERR $err:ident $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok(_) => $succeeded
            Err($err) => $failed
        }
    };
    // evaluates blocks based on result, discards err
    (
        $result:expr =>
            OK $ok:ident $succeeded:block
            ERR $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok($ok) => $succeeded
            Err(_) => $failed
        }
    };
    // evaluates blocks based on result, discards results
    (
        $result:expr =>
            OK $succeeded:block
            ERR $failed:block
    ) => {
        #[cfg(debug_assertions)]
        match $result {
            Ok(_) => $succeeded
            Err(_) => $failed
        }
    };
    // ===============================================================================
    // output success message if result is ok
    ($result:expr => OK $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Ok(ok) = $result { debug! { @ concat!($debug, ": {:?}"), ok } }
    };
    // output success message if result is ok, discards ok
    ($result:expr => _OK $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { debug! { @ $debug } }
    };
    // output formatted success message if result is ok
    ($result:expr => OK $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Ok(ok) = $result { debug! { @ concat!($debug, ": {:?}"), $($val),*, ok } }
    };
    // output formatted success message if result is ok, discards ok
    ($result:expr => _OK $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Ok(_) = $result { debug! { @ $debug, $($val),* } }
    };
    // ===============================================================================
    // output error message if result is err
    ($result:expr => ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Err(err) = $result { debug! { @ERR concat!($debug, ": {:?}"), err } }
    };
    // output error message if result is err, discards err
    ($result:expr => _ERR $debug:expr) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { debug! { @ERR $debug } }
    };
    // output formatted error message if result is err
    ($result:expr => ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Err(err) = $result { debug! { @ERR concat!($debug, ": {:?}"), $($val),*, err } }
    };
    // output formatted error message if result is err, discards err
    ($result:expr => _ERR $debug:expr, $($val:expr),*) => {
        #[cfg(debug_assertions)]
        if let Err(_) = $result { debug! { @ERR $debug, $($val),* } }
    };
    // ===============================================================================
    // outputs messages based on result
    (
        $result:expr =>
            OK $succeeded:expr;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs messages based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs messages based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(_) => debug! { @ERR  $failed }
        }
    };
    // outputs messages based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // ===============================================================================
    // outputs formatted messages based on result
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted messages based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted messages based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // outputs formatted messages based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // ===============================================================================
    // outputs formatted success message based on result
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs formatted success message based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), err }
        }
    };
    // outputs formatted success message based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), $($ok_val),*, ok },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // outputs formatted success message based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr, $($ok_val:expr),*;
            _ERR $failed:expr;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded, $($ok_val),* },
            Err(_) => debug! { @ERR $failed }
        }
    };
    // ===============================================================================
    // outputs formatted error message based on result
    (
        $result:expr =>
            OK $succeeded:expr;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted error message based on result, discards ok
    (
        $result:expr =>
            _OK $succeeded:expr;
            ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(err) => debug! { @ERR concat!($failed, ": {:?}"), $($err_val),*, err }
        }
    };
    // outputs formatted error message based on result, discards err
    (
        $result:expr =>
            OK $succeeded:expr;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(ok) => debug! { @ concat!($succeeded, ": {:?}"), ok },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // outputs formatted error message based on result, discards results
    (
        $result:expr =>
            _OK $succeeded:expr;
            _ERR $failed:expr, $($err_val:expr),*;
    ) => {
        #[cfg(debug_assertions)]
         match $result {
            Ok(_) => debug! { @ $succeeded },
            Err(_) => debug! { @ERR $failed, $($err_val),* }
        }
    };
    // ===============================================================================
    // private output message
    (@ $debug:expr) => { println!(concat!("DEBUG: ", $debug)); };
    // private output formatted message
    (@ $debug:expr, $($val:expr),*) => { println!(concat!("DEBUG: ", $debug), $($val),*); };
    // ===============================================================================
    // private output error message
    (@ERR $debug:expr) => { eprintln!(concat!("ERROR: ", $debug)); };
    // private output formatted error message
    (@ERR $debug:expr, $($val:expr),*) => { eprintln!(concat!("ERROR: ", $debug), $($val),*); };
}
