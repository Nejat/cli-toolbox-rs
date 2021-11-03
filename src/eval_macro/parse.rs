use syn::parse::{Parse, ParseStream};

use crate::common::parse::parse_expr_eval;
use crate::common::trace_parsed;
use crate::eval_macro::Eval;

impl Parse for Eval {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        trace_parsed(parse_expr_eval(input, "eval", |terse, verbose| Self { terse, verbose }))
    }
}