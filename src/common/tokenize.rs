use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::common::Message;

impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let report = self.build_message(false);

        tokens.extend(report);
    }
}
