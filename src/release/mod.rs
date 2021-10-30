use syn::Expr;
use verbosity::Verbosity;

mod parse;
mod tokenize;

pub struct Release {
    expr: Expr,
    verbosity: Verbosity
}