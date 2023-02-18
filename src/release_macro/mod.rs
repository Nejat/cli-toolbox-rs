#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{Display, Formatter};

use syn::Expr;

#[cfg(all(debug_assertions, feature = "trace"))]
use crate::display;

mod parse;
mod tokenize;

pub struct Release {
    terse: Option<Expr>,
    verbose: Option<Expr>,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for Release {
    //noinspection ALL
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt, "release! {{\n  terse: {}\n  verbose: {}\n}}",
            display(&self.terse), display(&self.verbose),
        )
    }
}