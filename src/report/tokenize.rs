use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::report::{Message, ReportLnMacro, ReportMacro};

impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let report = self.build_message();
        let is_verbosity = Ident::new(&format!("is_{}", self.verbosity), Span::call_site());

        tokens.extend(quote! {
            if Verbosity::#is_verbosity() {
                #report;
            }
        });

        #[cfg(all(debug_assertions, feature = "trace"))]
        println!("EXPANSION: {}", tokens);
    }
}

impl ToTokens for ReportMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokenize_report_macro(tokens, &self.terse, &self.verbose);
    }
}

impl ToTokens for ReportLnMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokenize_report_macro(tokens, &self.terse, &self.verbose);
    }
}

fn tokenize_report_macro(
    tokens: &mut TokenStream, terse: &Option<Message>, verbose: &Option<Message>,
) {
    match (terse, verbose) {
        (Some(terse), None) =>
            terse.to_tokens(tokens),
        (None, Some(verbose)) =>
            verbose.to_tokens(tokens),
        (Some(terse), Some(verbose)) => {
            let terse = terse.build_message();
            let verbose = verbose.build_message();
            tokens.extend(
                quote! {
                    match Verbosity::level() {
                        Verbosity::Terse => #terse,
                        Verbosity::Verbose => #verbose,
                        Verbosity::Quite => {}
                    }
                }
            );
        }
        (None, None) => {}
    }
}
