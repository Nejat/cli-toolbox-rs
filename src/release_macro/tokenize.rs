use proc_macro2::TokenStream;
use quote::ToTokens;
use crate::common::tokenize::tokenize_expression;

use crate::release_macro::Release;

impl ToTokens for Release {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = tokenize_expression(&self.terse, &self.verbose);
        let result = quote! {
            #[cfg(not(debug_assertions))]
            #expr
        };

        #[cfg(all(debug_assertions, feature = "trace"))]
        println!("EXPANSION: {}", result);

        tokens.extend(result);
    }
}