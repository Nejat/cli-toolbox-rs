use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::tokenize::tokenize_expression;
use crate::common::trace_expansion;
use crate::release_macro::Release;

impl ToTokens for Release {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion({
            let expr = tokenize_expression(&self.terse, &self.verbose);

            quote! {
                #[cfg(not(debug_assertions))]
                #expr
            }
        }));
    }
}