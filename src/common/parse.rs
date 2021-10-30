use syn::{Error, Expr, Lit};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use verbosity::Verbosity;

use crate::common::kw;
use crate::common::Message;

impl Message {
    pub fn parse(input: ParseStream, ln_brk: bool) -> syn::Result<Self> {
        Ok(Self {
            fmt: parse_format(input)?,
            args: parse_args(input)?,
            ln_brk,
        })
    }
}

#[cfg(feature = "debug")]
fn decode_expr_type(expr: &Expr) -> &'static str {
    match expr {
        Expr::Array(_) => "array",
        Expr::Assign(_) => "assign",
        Expr::AssignOp(_) => "assign-op",
        Expr::Async(_) => "async",
        Expr::Await(_) => "await",
        Expr::Binary(_) => "binary",
        Expr::Block(_) => "block",
        Expr::Box(_) => "box",
        Expr::Break(_) => "break",
        Expr::Call(_) => "call",
        Expr::Cast(_) => "cast",
        Expr::Closure(_) => "closure",
        Expr::Continue(_) => "continue",
        Expr::Field(_) => "field",
        Expr::ForLoop(_) => "for-loop",
        Expr::Group(_) => "group",
        Expr::If(_) => "if",
        Expr::Index(_) => "index",
        Expr::Let(_) => "let",
        Expr::Lit(_) => "lit",
        Expr::Loop(_) => "loop",
        Expr::Macro(_) => "macro",
        Expr::Match(_) => "match",
        Expr::MethodCall(_) => "method call",
        Expr::Paren(_) => "paren",
        Expr::Path(_) => "path",
        Expr::Range(_) => "range",
        Expr::Reference(_) => "reference",
        Expr::Repeat(_) => "repeat",
        Expr::Return(_) => "return",
        Expr::Struct(_) => "struct",
        Expr::Try(_) => "try",
        Expr::TryBlock(_) => "try-block",
        Expr::Tuple(_) => "tuple",
        Expr::Type(_) => "type",
        Expr::Unary(_) => "unary",
        Expr::Unsafe(_) => "unsafe",
        Expr::Verbatim(_) => "verbatim",
        Expr::While(_) => "while",
        Expr::Yield(_) => "yield",
        Expr::__TestExhaustive(_) => unimplemented!()
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


#[cfg(any(feature = "debug", feature = "eval", feature = "release"))]
pub fn parse_expression(input: ParseStream, macro_name: &str) -> syn::Result<Expr> {
    let expr = <Expr>::parse(input)?;

    match expr {
        Expr::Block(_) | Expr::TryBlock(_) | Expr::Unsafe(_) => {}
        Expr::Assign(_) | Expr::AssignOp(_) |
        Expr::Await(_) | Expr::Binary(_) |
        Expr::Box(_) | Expr::Break(_) |
        Expr::Call(_) | Expr::Cast(_) |
        Expr::Closure(_) | Expr::Continue(_) |
        Expr::Field(_) | Expr::Group(_) |
        Expr::If(_) | Expr::Index(_) |
        Expr::Macro(_) | Expr::Match(_) |
        Expr::MethodCall(_) | Expr::Path(_) |
        Expr::Reference(_) | Expr::Repeat(_) |
        Expr::Return(_) | Expr::Try(_) |
        Expr::Tuple(_) | Expr::Unary(_) =>
            parse_optional_semicolon(input)?,
        _ =>
            return Err(Error::new(
                expr.span(),
                format!(
                    "{:?} is not a supported {} expression, try placing it into a code block",
                    decode_expr_type(&expr), macro_name
                ),
            ))
    }

    Ok(expr)
}

fn parse_format(input: ParseStream) -> syn::Result<Lit> {
    let literal = <Lit>::parse(input)?;

    match literal {
        Lit::Str(_) | Lit::ByteStr(_) => {}
        _ => return Err(Error::new(literal.span(), "expecting a string literal"))
    }

    Ok(literal)
}

#[cfg(feature = "debug")]
fn parse_optional_semicolon(input: ParseStream) -> syn::Result<()> {
    if let Some(punct) = input.cursor().punct() {
        if punct.0.as_char() == ';' {
            <Token![;]>::parse(input)?;
        }
    }

    Ok(())
}

#[cfg(any(feature = "eval", feature = "release"))]
pub fn parse_verbosity(input: ParseStream) -> syn::Result<Verbosity> {
    let verbosity;
    let span = input.span();

    if input.peek(Token![@]) {
        if verbosity_keyword_peek2(input) {
            <Token![@]>::parse(input)?;

            if input.peek(kw::terse) {
                <kw::terse>::parse(input)?;

                verbosity = Verbosity::Terse;
            } else {
                <kw::verbose>::parse(input)?;

                verbosity = Verbosity::Verbose;
            }
        } else {
            return Err(Error::new(
                span,
                "invalid verbosity designation, use @terse, @verbose or leave blank for default level",
            ));
        }
    } else {
        verbosity = Verbosity::Terse;
    }

    Ok(verbosity)
}


#[cfg(any(feature = "eval", feature = "release"))]
fn verbosity_keyword_peek2(input: ParseStream) -> bool {
    input.peek2(kw::terse) ||
        input.peek2(kw::verbose)
}
