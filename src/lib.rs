#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cli-toolbox/0.8.1")]

//! Utility library for working with ```cli``` output ergonomically.
//!
//! This is not a logging alternative, it's intended to produce output for end user consumption.
//!
//! It handles three levels of verbosity that can be set dynamically at runtime:
//!
//! * Quite - no output
//! * Terse - used to provide minimal user output
//! * Verbose - used to provide elaborated and/or additional user output
//!
//! ### Output Macros
//!
//! * `debug!` - conditionally compiled console debugging output - \[`debug`\]
//!
//! * `report!` - conditional console output according to verbosity level - \[`debug`|`release`\]
//!
//! \* _debug! is intended to be used during application development_
//!
//! \* _all other debugging and telemetry output is most likely better served with a logging library_
//!
//! ### Conditional Code Evaluation
//!
//! * `eval!` - conditional code execution according to verbosity level - \[`debug`|`release`\]
//!
//! * `release!` - conditional code execution according to verbosity level - \[`release`\]

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
#[macro_use]
extern crate quote;
#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
#[macro_use]
extern crate syn;

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
use proc_macro::TokenStream;

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
use quote::ToTokens;

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
mod common;
#[cfg(feature = "debug")]
mod debug_macro;
#[cfg(feature = "eval")]
mod eval_macro;
#[cfg(feature = "release")]
mod release_macro;
#[cfg(feature = "report")]
mod report_macro;

#[cfg(test)]
mod tests;

/// Conditionally prints to `io::stdout` or evaluates an expression when the code
/// is compiled unoptimized with debug assertions, otherwise does not include
/// the message or expression. _for a message a new line is not appended_
///
/// ## Anatomy of the `debug!` macro
///
/// In order to print to `io:stdout`, `debug!` accepts the same input as the `std`
/// library [`print!`] macro. Otherwise it accepts a valid expression.
///
/// ### Examples
/// * Printing to `io::stdout`
///
/// ```no_run
/// # use::cli_toolbox::debug;
/// debug! { "DBG: debugging information - {}", 42 }
/// ```
///
/// * Evaluating Expression
///
/// ```no_run
/// # use::cli_toolbox::debug;
/// debug! { validate_some_important_such_and_such(); }
/// # fn validate_some_important_such_and_such() {}
/// ```
///
/// ## Panics
///
/// Just like the [`print!`] macros used to write the output, this also panics if
/// writing to `io::stdout` fails.
///
/// [`print!`]: <https://doc.rust-lang.org/std/macro.print.html>
#[cfg(feature = "debug")]
#[proc_macro]
pub fn debug(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as debug_macro::DebugMacro).into_token_stream().into()
}

/// Conditionally prints to `io::stdout` when the code is compiled unoptimized
/// with debug assertions, otherwise does not include the message. _a new line is appended_
///
/// ## Anatomy of the `debugln!` macro
///
/// `debug!` accepts the same input as the `std` library [`println!`] macro.
///
/// ### Example
///
/// Printing a line to `io::stdout`
///
/// ```no_run
/// # use::cli_toolbox::debugln;
/// debugln! { "DBG: debugging information - {}", 42 }
/// ```
///
/// ## Panics
///
/// Just like the [`println!`] macros used to write the output, this also panics if
/// writing to `io::stdout` fails.
///
/// [`println!`]: <https://doc.rust-lang.org/std/macro.println.html>
#[cfg(feature = "debug")]
#[proc_macro]
pub fn debugln(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as debug_macro::DebugLnMacro).into_token_stream().into()
}

/// Conditionally evaluates expressions when intended verbosity matches active verbosity.
///
/// The `eval` macro uses the [`Verbosity`] crate to determine when and what to evaluate.
///
/// _\* See the [`Verbosity`] crate to learn how to set the verbosity level._
///
/// ## Anatomy of the `eval!` macro
///
/// Input consists of an optional intended verbosity level, defaulting to `terse`
/// if it is not specifically provided. The remainder of the macro input expects
/// an expression and then an optional semicolon terminator.
///
/// ### Examples
///
/// * Evaluates when `default`, which is `terse`
/// ```no_run
/// # use::cli_toolbox::eval;
/// # let bar = 42;
/// eval! { foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// * Evaluates when `terse`
/// ```no_run
/// # use::cli_toolbox::eval;
/// # let bar = 42;
/// eval! { @terse foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// * Evaluates when `verbose`
/// ```no_run
/// # use::cli_toolbox::eval;
/// # let bar = 42;
/// eval! { @verbose foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// [`Verbosity`]: <https://crates.io/crates/verbosity>
#[cfg(feature = "eval")]
#[proc_macro]
pub fn eval(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as eval_macro::Eval).into_token_stream().into()
}

/// Conditionally evaluates expressions when intended verbosity matches active verbosity
/// and only when the code is compiled optimized.
///
/// The `release` macro uses the [`Verbosity`] crate to determine when and what to evaluate.
///
/// _\* See the [`Verbosity`] crate to learn how to set the verbosity level._
///
/// ## Anatomy of the `release!` macro
///
/// Input consists of an optional intended verbosity level, defaulting to `terse`
/// if it is not specifically provided. The remainder of the macro input expects
/// an expression and then an optional semicolon terminator.
///
/// ### Examples
///
/// * Evaluates when `default`, which is `terse`
/// ```no_run
/// # use::cli_toolbox::release;
/// # let bar = 42;
/// release! { foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// * Evaluates when `terse`
/// ```no_run
/// # use::cli_toolbox::release;
/// # let bar = 42;
/// release! { @terse foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// * Evaluates when `verbose`
/// ```no_run
/// # use::cli_toolbox::release;
/// # let bar = 42;
/// release! { @verbose foo(bar); }
/// # fn foo(_value: usize) {}
/// ```
///
/// [`Verbosity`]: <https://crates.io/crates/verbosity>
#[cfg(feature = "release")]
#[proc_macro]
pub fn release(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as release_macro::Release).into_token_stream().into()
}

/// Conditionally prints to `io::stdout` or `io::stderr` when intended verbosity matches
/// active verbosity,<br/>does not append a new line.
///
#[doc = include_str!("docs/report_macro_doc.md")]
///
/// ## Anatomy of the `report!` macro
///
#[doc = include_str!("docs/report_macro_anatomy_doc.md")]
#[cfg(feature = "report")]
#[proc_macro]
pub fn report(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as report_macro::ReportMacro).into_token_stream().into()
}

/// Conditionally prints to `io::stdout` or `io::stderr` when intended verbosity matches
/// active verbosity,<br/>appends a new line.
///
#[doc = include_str!("docs/report_macro_doc.md")]
///
/// ## Anatomy of the `reportln!` macro
///
#[doc = include_str!("docs/report_macro_anatomy_doc.md")]
#[cfg(feature = "report")]
#[proc_macro]
pub fn reportln(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as report_macro::ReportLnMacro).into_token_stream().into()
}

#[cfg(all(debug_assertions, feature = "trace"))]
fn display<D: ToTokens>(value: &Option<D>) -> String {
    value.as_ref().map_or_else(|| String::from("None"), |val| format!("{}", val.to_token_stream()))
}

#[cfg(all(debug_assertions, feature = "trace"))]
fn displays<D: ToTokens>(value: &Option<Vec<D>>) -> String {
    value.as_ref().map_or_else(
        || String::from("None"),
        |vals| format!(
            "{:?}",
            vals.iter().map(|v| format!("{}", v.to_token_stream())).collect::<Vec<String>>()
        ),
    )
}
