use syn::parse::{Parse, ParseStream};

use crate::common::{Message, trace_parsed};
use crate::common::parse::parse_expression;
use crate::debug_macro::{DebugLnMacro, DebugMacro};

impl Parse for DebugLnMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return trace_parsed(parse(input));

        #[inline]
        fn parse(input: ParseStream) -> syn::Result<DebugLnMacro> {
            Ok(DebugLnMacro { message: Message::parse(input, true)? })
        }
    }
}

impl Parse for DebugMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        return trace_parsed(parse(input));

        #[inline]
        fn parse(input: ParseStream) -> syn::Result<DebugMacro> {
            Ok(if let Ok(message) = Message::parse(input, false) {
                DebugMacro::Message(message)
            } else {
                DebugMacro::Expr(Box::new(parse_expression(input, "debug")?))
            })
        }
    }
}