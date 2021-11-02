use syn::Expr;

mod parse;
mod tokenize;

pub struct Eval {
    terse: Option<Expr>,
    verbose: Option<Expr>,
}