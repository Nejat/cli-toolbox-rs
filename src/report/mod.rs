#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{self, Display, Formatter};
#[cfg(all(debug_assertions, feature = "trace"))]
use std::string::ToString;

use proc_macro2::TokenStream;
#[cfg(all(debug_assertions, feature = "trace"))]
use quote::ToTokens;
use syn::{Expr, Lit};
use verbosity::Verbosity;

mod parse;
mod tokenize;

const QUITE_ERR: &str = "quite should not be able to be specified";

struct Message {
    pub args: Option<Vec<Expr>>,
    pub fmt: Lit,
    pub ln_brk: bool,
    pub std_err: bool,
    pub verbosity: Verbosity,
}

impl Message {
    pub(crate) fn build_message(&self) -> TokenStream {
        let report = if self.std_err {
            if self.ln_brk { quote! { eprintln! } } else { quote! { eprint! } }
        } else if self.ln_brk { quote! { println! } } else {
            quote! { print! }
        };
        let fmt = &self.fmt;
        let mut args = TokenStream::new();

        if let Some(message_args) = &self.args {
            for arg in message_args {
                args.extend(quote! { , #arg });
            }
        }

        quote! { #report(#fmt #args) }
    }
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for Message {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let args = if let Some(args) = &self.args {
            format!(
                "{:?}",
                args.iter().map(|v| format!("{}", v.to_token_stream())).collect::<Vec<String>>()
            )
        } else {
            "None".to_string()
        };

        write!(
            fmt,
            "{{ args: {}, fmt: {}, ln: {}, err: {}, verbosity: {} }}",
            args,
            self.fmt.to_token_stream(),
            self.ln_brk,
            self.std_err,
            self.verbosity
        )
    }
}

pub struct ReportMacro {
    terse: Option<Message>,
    verbose: Option<Message>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for ReportMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        format_report_macro(fmt, self.terse.as_ref(), self.verbose.as_ref())
    }
}

pub struct ReportLnMacro {
    terse: Option<Message>,
    verbose: Option<Message>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for ReportLnMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        format_report_macro(fmt, self.terse.as_ref(), self.verbose.as_ref())
    }
}

#[cfg(all(debug_assertions, feature = "trace"))]
fn format_report_macro(
    fmt: &mut Formatter, terse: Option<&Message>, verbose: Option<&Message>,
) -> fmt::Result {
    write!(
        fmt, "{{\n  terse: {}\n  verbose: {}\n}}",
        terse.map_or_else(|| "None".to_string(), ToString::to_string),
        verbose.map_or_else(|| "None".to_string(), ToString::to_string),
    )
}
