use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Expr;
use verbosity::Verbosity;

use crate::common::Message;

impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let report = self.build_message(false);

        tokens.extend(report);
    }
}

pub fn tokenize_verbosity_checked_expr(verbosity: Verbosity, expr: &Expr) -> TokenStream {
    let verbosity_check = if verbosity == Verbosity::Terse {
        quote! { Verbosity::is_terse() }
    } else {
        quote! { Verbosity::is_verbose() }
    };

    quote! { if #verbosity_check { #expr; } }
}
