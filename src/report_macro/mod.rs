#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{self, Display, Formatter};
#[cfg(all(debug_assertions, feature = "trace"))]
use std::string::ToString;

use verbosity::Verbosity;

use crate::common::Message;

mod parse;
mod tokenize;

struct ReportMessage {
    message: Message,
    std_err: bool,
    verbosity: Verbosity,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for ReportMessage {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "{{  message: {},  std_err: {}}}", self.message, self.std_err)
    }
}

pub struct ReportMacro {
    terse: Option<ReportMessage>,
    verbose: Option<ReportMessage>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for ReportMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        format_report_macro(fmt, self.terse.as_ref(), self.verbose.as_ref(), false)
    }
}

pub struct ReportLnMacro {
    terse: Option<ReportMessage>,
    verbose: Option<ReportMessage>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for ReportLnMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        format_report_macro(fmt, self.terse.as_ref(), self.verbose.as_ref(), true)
    }
}

#[cfg(all(debug_assertions, feature = "trace"))]
fn format_report_macro(
    fmt: &mut Formatter, terse: Option<&ReportMessage>, verbose: Option<&ReportMessage>, ln: bool,
) -> fmt::Result {
    write!(
        fmt, "report{}! {{\n  terse: {}\n  verbose: {}\n}}",
        if ln { "ln" } else { "" },
        terse.map_or_else(|| "None".to_string(), ToString::to_string),
        verbose.map_or_else(|| "None".to_string(), ToString::to_string),
    )
}
