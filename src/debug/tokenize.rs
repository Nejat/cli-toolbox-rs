use proc_macro2::TokenStream;
use quote::ToTokens;
use crate::common::Message;

use crate::debug::{DebugLnMacro, DebugMacro};

impl ToTokens for DebugLnMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokenize_debug_message_macro(tokens, &self.message);
    }
}

impl ToTokens for DebugMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Message(message) =>
                tokenize_debug_message_macro(tokens, message),
            Self::Expr(expr) =>
                tokens.extend(quote! {
                    #[cfg(debug_assertions)]
                    #expr;
                })
        }
    }
}

fn tokenize_debug_message_macro(tokens: &mut TokenStream, message: &Message) {
    let message = message.build_message(false);

    tokens.extend(quote! {
        #[cfg(debug_assertions)]
        #message;
    });
}