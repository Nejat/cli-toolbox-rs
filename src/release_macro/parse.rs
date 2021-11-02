use syn::parse::{Parse, ParseStream};

use crate::common::parse::parse_expr_eval;
use crate::release_macro::Release;

impl Parse for Release {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        parse_expr_eval(input, "release", |terse, verbose| Self { terse, verbose })
    }
}