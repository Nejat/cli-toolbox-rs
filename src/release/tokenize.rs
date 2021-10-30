use proc_macro2::TokenStream;
use quote::ToTokens;
use crate::common::tokenize::tokenize_verbosity_checked_expr;

use crate::release::Release;

impl ToTokens for Release {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = tokenize_verbosity_checked_expr(self.verbosity, &self.expr);

        tokens.extend(
            quote! {
                #[cfg(not(debug_assertions))]
                #expr
            }
        );
    }
}