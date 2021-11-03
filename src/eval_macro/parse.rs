use syn::parse::{Parse, ParseStream};

use crate::common::parse::parse_expr_eval;
use crate::eval_macro::Eval;

impl Parse for Eval {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_expr_eval(input, "eval", |terse, verbose| Self { terse, verbose })
    }
}