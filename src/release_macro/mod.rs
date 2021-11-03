use syn::Expr;

mod parse;
mod tokenize;

pub struct Release {
    terse: Option<Expr>,
    verbose: Option<Expr>,
}