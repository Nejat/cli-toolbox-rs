use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::tokenize::tokenize_expression;
use crate::common::trace_expansion;
use crate::eval_macro::Eval;

impl ToTokens for Eval {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion(tokenize_expression(&self.terse, &self.verbose)));
    }
}
