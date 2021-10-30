use syn::Expr;
use verbosity::Verbosity;

mod parse;
mod tokenize;

pub struct Eval {
    expr: Expr,
    verbosity: Verbosity
}