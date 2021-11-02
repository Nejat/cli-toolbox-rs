use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::tokenize::tokenize_expression;
use crate::eval_macro::Eval;

impl ToTokens for Eval {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let result = tokenize_expression(&self.terse, &self.verbose);

        #[cfg(all(debug_assertions, feature = "trace"))]
        println!("EXPANSION: {}", result);

        tokens.extend(result);
    }
}
