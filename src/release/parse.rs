use syn::parse::{Parse, ParseStream};

use crate::common::parse::{parse_expression, parse_verbosity};
use crate::release::Release;

impl Parse for Release {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            verbosity: parse_verbosity(input)?,
            expr: parse_expression(input, "release")?,
        })
    }
}