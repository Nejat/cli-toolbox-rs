#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{Display, Formatter};
#[cfg(all(debug_assertions, feature = "trace"))]
use std::string::ToString;

#[cfg(any(feature = "debug", feature = "report"))]
use proc_macro2::TokenStream;
#[cfg(all(debug_assertions, feature = "trace"))]
use quote::ToTokens;
#[cfg(any(feature = "debug", feature = "report"))]
use syn::{Expr, Lit};

pub mod parse;
pub mod tokenize;

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
pub mod kw {
    #[cfg(any(feature = "debug", feature = "report"))]
    custom_keyword!(err);
    #[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
    custom_keyword!(terse);
    #[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
    custom_keyword!(verbose);
}

#[cfg(any(feature = "eval", feature = "release", feature = "report"))]
pub const QUITE_ERR: &str = "quite should not be able to be specified";

#[cfg(any(feature = "eval", feature = "release", feature = "report"))]
pub const DUPE_VERBOSITY_ERR: &str = "do not duplicate verbosity";

#[cfg(any(feature = "eval", feature = "release", feature = "report"))]
pub const VERBOSITY_ORDER_ERR: &str = "define terse before verbose";

#[cfg(any(feature = "debug", feature = "report"))]
pub struct Message {
    pub args: Option<Vec<Expr>>,
    pub fmt: Lit,
    pub ln_brk: bool,
}

#[cfg(any(feature = "debug", feature = "report"))]
impl Message {
    pub(crate) fn build_message(&self, std_err: bool) -> TokenStream {
        let report = if std_err {
            if self.ln_brk { quote! { eprintln! } } else { quote! { eprint! } }
        } else if self.ln_brk { quote! { println! } } else {
            quote! { print! }
        };
        let fmt = &self.fmt;
        let mut args = TokenStream::new();

        if let Some(message_args) = &self.args {
            for arg in message_args {
                args.extend(quote! { , #arg });
            }
        }

        quote! { #report(#fmt #args) }
    }
}

#[cfg(all(debug_assertions, any(feature = "debug", feature = "report"), feature = "trace"))]
impl Display for Message {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let args = if let Some(args) = &self.args {
            format!(
                "{:?}",
                args.iter().map(|v| format!("{}", v.to_token_stream())).collect::<Vec<String>>()
            )
        } else {
            "None".to_string()
        };

        write!(
            fmt, "{{ args: {}, fmt: {}, ln_brk: {} }}",
            args, self.fmt.to_token_stream(), self.ln_brk
        )
    }
}

#[cfg(all(debug_assertions, feature = "trace"))]
#[inline]
pub fn trace_expansion<T: Display>(traced: T) -> T {
    println!("EXPANSION: {}", traced);

    traced
}

#[cfg(not(all(debug_assertions, feature = "trace")))]
#[inline]
pub const fn trace_expansion<T>(traced: T) -> T { traced }

#[cfg(all(debug_assertions, feature = "trace"))]
#[inline]
pub fn trace_parsed<T: Display, E: Display>(traced: Result<T, E>) -> Result<T, E> {
    match &traced {
        Ok(ok) =>
            println!("PARSED: {}", ok),
        Err(err) =>
            println!("PARSE-ERR: {}", err)
    }

    traced
}

#[cfg(not(all(debug_assertions, feature = "trace")))]
#[inline]
pub const fn trace_parsed<T>(traced: T) -> T { traced }
