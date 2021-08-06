#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]
// ==============================================================
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]
// ==============================================================
#![doc(html_root_url = "https://docs.rs/cli-toolbox/0.5.1")]

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
pub use verbosity::Verbosity;

#[cfg(feature = "debug")]
mod debug;
#[cfg(feature = "eval")]
mod eval;
#[cfg(feature = "release")]
mod release;
#[cfg(feature = "report")]
mod report;
#[cfg(any(feature = "report", feature = "release"))]
mod verbosity;

#[cfg(test)]
mod tests;
