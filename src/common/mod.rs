#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{Display, Formatter};
#[cfg(all(debug_assertions, feature = "trace"))]
use std::string::ToString;

use proc_macro2::TokenStream;
#[cfg(all(debug_assertions, feature = "trace"))]
use quote::ToTokens;
use syn::{Expr, Lit};

pub mod parse;
mod tokenize;

pub struct Message {
    pub args: Option<Vec<Expr>>,
    pub fmt: Lit,
    pub ln_brk: bool,
}

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


#[cfg(all(debug_assertions, feature = "trace"))]
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
            fmt,
            "{{ args: {}, fmt: {}, ln_brk: {} }}",
            args,
            self.fmt.to_token_stream(),
            self.ln_brk
        )
    }
}
