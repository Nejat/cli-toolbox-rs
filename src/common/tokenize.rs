use proc_macro2::TokenStream;
#[cfg(any(feature = "debug", feature = "report"))]
use quote::ToTokens;
#[cfg(any(feature = "eval", feature = "release"))]
use syn::Expr;
#[cfg(any(feature = "eval", feature = "release"))]
use verbosity::Verbosity;

#[cfg(any(feature = "debug", feature = "report"))]
use crate::common::Message;

#[cfg(any(feature = "debug", feature = "report"))]
impl ToTokens for Message {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.build_message(false));
    }
}

#[cfg(any(feature = "eval", feature = "release"))]
pub fn tokenize_expression(terse: &Option<Expr>, verbose: &Option<Expr>) -> TokenStream {
    match (terse, verbose) {
        (Some(terse), None) =>
            tokenize_verbosity_expression(Verbosity::Terse, terse),
        (None, Some(verbose)) =>
            tokenize_verbosity_expression(Verbosity::Verbose, verbose),
        (Some(terse), Some(verbose)) => {
            let terse = tokenize_verbosity_expression(Verbosity::Terse, terse);
            let verbose = tokenize_verbosity_expression(Verbosity::Verbose, verbose);

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

#[cfg(any(feature = "eval", feature = "release"))]
pub fn tokenize_verbosity_expression(verbosity: Verbosity, expr: &Expr) -> TokenStream {
    let verbosity_check = if verbosity == Verbosity::Terse {
        quote! { verbosity::Verbosity::is_terse() }
    } else {
        quote! { verbosity::Verbosity::is_verbose() }
    };

    quote! { if #verbosity_check { #expr; } }
}

