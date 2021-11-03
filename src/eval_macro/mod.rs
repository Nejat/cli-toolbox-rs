#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{Display, Formatter};
#[cfg(all(debug_assertions, feature = "trace"))]
use std::string::ToString;

#[cfg(all(debug_assertions, feature = "trace"))]
use quote::ToTokens;
use syn::Expr;

mod parse;
mod tokenize;

pub struct Eval {
    terse: Option<Expr>,
    verbose: Option<Expr>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for Eval {
    //noinspection ALL
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt, "eval! {{\n  terse: {}\n  verbose: {}\n}}",
            self.terse.as_ref().map_or_else(
                || "None".to_string(), |e| (*e).to_token_stream().to_string(),
            ),
            self.verbose.as_ref().map_or_else(
                || "None".to_string(), |e| (*e).to_token_stream().to_string(),
            ),
        )
    }
}