#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cli-toolbox/0.5.3")]

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
//! ### Conditional Code
//!
//! * `eval!` - conditional code execution according to verbosity level - \[`debug`|`release`\]
//!
//! * `release!` - conditional code execution according to verbosity level - \[`release`\]

#[cfg(feature = "report")]
#[macro_use]
extern crate cfg_if;
#[cfg(feature = "report")]
#[macro_use]
extern crate quote;
#[cfg(feature = "report")]
#[macro_use]
extern crate syn;

#[cfg(feature = "report")]
use proc_macro::TokenStream;

#[cfg(feature = "report")]
use quote::ToTokens;

#[cfg(feature = "report")]
mod common;
// #[cfg(feature = "debug")]
// mod debug;
// #[cfg(feature = "eval")]
// mod eval;
// #[cfg(feature = "release")]
// mod release;
#[cfg(feature = "report")]
mod report;

#[cfg(test)]
mod tests;

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
    parse_macro_input!(input as report::ReportMacro).into_token_stream().into()
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
    parse_macro_input!(input as report::ReportLnMacro).into_token_stream().into()
}
