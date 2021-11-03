use proc_macro2::TokenStream;
use quote::ToTokens;
use crate::common::Message;

use crate::debug_macro::{DebugLnMacro, DebugMacro};

impl ToTokens for DebugLnMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let result = tokenize_debug_message_macro(&self.message);

        #[cfg(all(debug_assertions, feature = "trace"))]
        println!("EXPANSION: {}", result);

        tokens.extend(result);
    }
}

impl ToTokens for DebugMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let result = match self {
            Self::Message(message) =>
                tokenize_debug_message_macro(message),
            Self::Expr(expr) =>
                quote! {
                    #[cfg(debug_assertions)]
                    { #expr; }
                }
        };

        #[cfg(all(debug_assertions, feature = "trace"))]
        println!("EXPANSION: {}", result);

        tokens.extend(result);
    }
}

fn tokenize_debug_message_macro(message: &Message) -> TokenStream{
    let message = message.build_message(false);

    quote! {
        #[cfg(debug_assertions)]
        { #message; }
    }
}