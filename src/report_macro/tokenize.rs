use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;

use crate::common::trace_expansion;
use crate::report_macro::{ReportLnMacro, ReportMacro, ReportMessage};

impl ToTokens for ReportMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion(tokenize_report_macro(&self.terse, &self.verbose)));
    }
}

impl ToTokens for ReportLnMacro {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(trace_expansion(tokenize_report_macro(&self.terse, &self.verbose)));
    }
}

impl ToTokens for ReportMessage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let report = self.message.build_message(self.std_err);
        let is_verbosity = Ident::new(&format!("is_{}", self.verbosity), Span::call_site());

        tokens.extend(quote! { if verbosity::Verbosity::#is_verbosity() { #report; } });
    }
}

fn tokenize_report_macro(
    terse: &Option<ReportMessage>, verbose: &Option<ReportMessage>,
) -> TokenStream {
    match (terse, verbose) {
        (Some(terse), None) =>
            quote! { #terse },
        (None, Some(verbose)) =>
            quote! { #verbose },
        (Some(terse), Some(verbose)) => {
            let terse = terse.message.build_message(terse.std_err);
            let verbose = verbose.message.build_message(verbose.std_err);

            quote! {
                match verbosity::Verbosity::level() {
                    verbosity::Verbosity::Terse => #terse,
                    verbosity::Verbosity::Verbose => #verbose,
                    verbosity::Verbosity::Quite => {}
                }
            }
        }
        (None, None) => TokenStream::new()
    }
}
