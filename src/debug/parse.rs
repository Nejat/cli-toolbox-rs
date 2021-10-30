use syn::parse::{Parse, ParseStream};

use crate::common::Message;
use crate::common::parse::parse_expression;
use crate::debug::{DebugLnMacro, DebugMacro};

impl Parse for DebugLnMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            message: Message::parse(input, true)?
        })
    }
}

impl Parse for DebugMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(if let Ok(message) = Message::parse(input, false) {
            Self::Message(message)
        } else {
            Self::Expr(Box::new(parse_expression(input, "debug")?))
        })
    }
}