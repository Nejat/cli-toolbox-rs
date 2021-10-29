use syn::{Error, Expr, Lit};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;

use crate::common::Message;

impl Message {
    pub fn parse(input: ParseStream, ln_brk: bool) -> syn::Result<Self> {
        Ok(Self {
            fmt: parse_format(input)?,
            args: parse_args(input)?,
            ln_brk
        })
    }
}

fn parse_args(input: ParseStream) -> syn::Result<Option<Vec<Expr>>> {
    let mut exprs = Vec::new();

    while input.peek(Token![,]) {
        <Token![,]>::parse(input)?;

        let expr = <Expr>::parse(input)?;

        match expr {
            Expr::Array(_) | Expr::Await(_) |
            Expr::Binary(_) | Expr::Block(_) |
            Expr::Box(_) | Expr::Call(_) |
            Expr::Cast(_) | Expr::Field(_) |
            Expr::If(_) | Expr::Index(_) |
            Expr::Lit(_) | Expr::Macro(_) |
            Expr::Match(_) | Expr::MethodCall(_) |
            Expr::Path(_) | Expr::Reference(_) |
            Expr::Try(_) | Expr::TryBlock(_) |
            Expr::Tuple(_) | Expr::Unary(_) |
            Expr::Unsafe(_) => {}
            _ => return Err(Error::new(expr.span(), "unsupported arg expression"))
        }

        exprs.push(expr);
    }

    if input.peek(Token![;]) || input.peek(Token![@]) {
        <Token![;]>::parse(input)?;
    }

    Ok(if exprs.is_empty() { None } else { Some(exprs) })
}

fn parse_format(input: ParseStream) -> syn::Result<Lit> {
    let literal = <Lit>::parse(input)?;

    match literal {
        Lit::Str(_) | Lit::ByteStr(_) => {}
        _ => return Err(Error::new(literal.span(), "expecting a string literal"))
    }

    Ok(literal)
}
