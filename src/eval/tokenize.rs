use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::tokenize::tokenize_verbosity_checked_expr;
use crate::eval::Eval;

impl ToTokens for Eval {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(tokenize_verbosity_checked_expr(self.verbosity, &self.expr));
    }
}
