use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::report::{ReportLnMacro, ReportMacro, ReportMessage};

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

impl ToTokens for ReportMessage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let report = self.message.build_message(self.std_err);
        // todo: this is incorrect, see eval or release macros for correct output
        let is_verbosity = Ident::new(&format!("is_{}", self.verbosity), Span::call_site());

        tokens.extend(quote! { if Verbosity::#is_verbosity() { #report; } });
    }
}

fn tokenize_report_macro(
    tokens: &mut TokenStream, terse: &Option<ReportMessage>, verbose: &Option<ReportMessage>,
) {
    match (terse, verbose) {
        (Some(terse), None) =>
            terse.to_tokens(tokens),
        (None, Some(verbose)) =>
            verbose.to_tokens(tokens),
        (Some(terse), Some(verbose)) => {
            let terse = terse.message.build_message(terse.std_err);
            let verbose = verbose.message.build_message(verbose.std_err);
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

    #[cfg(all(debug_assertions, feature = "trace"))]
    println!("EXPANSION: {}", tokens);
}
