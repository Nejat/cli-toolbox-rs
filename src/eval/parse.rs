use syn::parse::{Parse, ParseStream};

use crate::common::parse::{parse_expression, parse_verbosity};
use crate::eval::Eval;

impl Parse for Eval {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            verbosity: parse_verbosity(input)?,
            expr: parse_expression(input, "eval")?,
        })
    }
}