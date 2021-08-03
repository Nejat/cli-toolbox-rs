#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(missing_docs)]

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::items_after_statements)]

#![doc(html_root_url = "https://docs.rs/cli-toolbox/0.1.0")]

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
//! Additionally, this library provides conditionally compiled debugging output intended to be
//! used during application development.
//!
//! All other debugging and telemetry output is most likely better served with a logging library.

mod debug;

#[cfg(test)]
mod tests;