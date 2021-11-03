use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::{Message, trace_expansion};
use crate::debug_macro::{DebugLnMacro, DebugMacro};

impl ToTokens for DebugLnMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion(tokenize_debug_message_macro(&self.message)));
    }
}

impl ToTokens for DebugMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion(
            match self {
                Self::Message(message) =>
                    tokenize_debug_message_macro(message),
                Self::Expr(expr) =>
                    quote! {
                    #[cfg(debug_assertions)]
                    { #expr; }
                }
            }
        ));
    }
}

fn tokenize_debug_message_macro(message: &Message) -> TokenStream {
    let message = message.build_message(false);

    quote! {
        #[cfg(debug_assertions)]
        { #message; }
    }
}