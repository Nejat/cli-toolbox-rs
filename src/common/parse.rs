use syn::{Error, Expr};
#[cfg(any(feature = "debug", feature = "report"))]
use syn::Lit;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
#[cfg(any(feature = "eval", feature = "release"))]
use verbosity::Verbosity;

#[cfg(any(feature = "eval", feature = "release"))]
use crate::common::{DUPE_VERBOSITY_ERR, QUITE_ERR, VERBOSITY_ORDER_ERR};
#[cfg(any(feature = "eval", feature = "release"))]
use crate::common::kw;
#[cfg(any(feature = "debug", feature = "report"))]
use crate::common::Message;

#[cfg(any(feature = "debug", feature = "report"))]
impl Message {
    pub fn parse(input: ParseStream, ln_brk: bool) -> syn::Result<Self> {
        Ok(Self {
            fmt: parse_format(input)?,
            args: parse_args(input)?,
            ln_brk,
        })
    }
}

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
pub fn decode_expr_type(expr: &Expr) -> &'static str {
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
        _ => unimplemented!("{expr:?} is not supported")
    }
}

#[cfg(any(feature = "eval", feature = "release"))]
#[allow(clippy::shadow_unrelated)] // intention of code is clear
pub fn parse_expr_eval<T>(
    input: ParseStream, macro_name: &str, builder: impl Fn(Option<Expr>, Option<Expr>) -> T,
) -> syn::Result<T> {
    let verbosity = parse_verbosity(input, false)?;
    let expr = parse_expression(input, macro_name)?;
    let error_span = input.span();

    match verbosity {
        Some(Verbosity::Quite) =>
            unreachable!("{}", QUITE_ERR),
        Some(Verbosity::Terse) | None => {
            let verbose = if let Ok(Some(verbose)) = parse_verbosity(input, true) {
                verbose
            } else {
                return Ok(builder(Some(expr), None));
            };

            match verbose {
                Verbosity::Quite =>
                    unreachable!("{}", QUITE_ERR),
                Verbosity::Terse =>
                    Err(Error::new(error_span, DUPE_VERBOSITY_ERR)),
                Verbosity::Verbose =>
                // only accept a second expression that is intended for verbose output
                    Ok(builder(Some(expr), Some(parse_expression(input, macro_name)?)))
            }
        }
        Some(Verbosity::Verbose) => {
            if input.is_empty() {
                Ok(builder(None, Some(expr)))
            } else {
                let error_span = input.span();

                match parse_verbosity(input, true) {
                    Ok(verbosity) => {
                        match verbosity {
                            Some(Verbosity::Quite) =>
                                unreachable!("{}", QUITE_ERR),
                            Some(Verbosity::Terse) =>
                                Err(Error::new(error_span, VERBOSITY_ORDER_ERR)),
                            Some(Verbosity::Verbose) =>
                                Err(Error::new(error_span, DUPE_VERBOSITY_ERR)),
                            None =>
                                Err(Error::new(error_span, "unexpected token"))
                        }
                    }
                    Err(err) => Err(err)
                }
            }
        }
    }
}

#[cfg(any(feature = "debug", feature = "eval", feature = "release"))]
pub fn parse_expression(input: ParseStream, macro_name: &str) -> syn::Result<Expr> {
    let expr = <Expr>::parse(input)?;

    match expr {
        Expr::Block(_) | Expr::TryBlock(_) | Expr::Unsafe(_) => {}
        Expr::Array(_) |
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
            parse_optional_semicolon(input, false)?,
        _ =>
            return Err(Error::new(
                expr.span(),
                format!(
                    "{:?} is not a supported {macro_name} expression, try placing it into a code block",
                    decode_expr_type(&expr)
                ),
            ))
    }

    Ok(expr)
}

#[cfg(any(feature = "debug", feature = "eval", feature = "release", feature = "report"))]
pub fn parse_optional_semicolon(input: ParseStream, required: bool) -> syn::Result<()> {
    if input.peek(Token![;]) || (required && input.peek(Token![@])) {
        <Token![;]>::parse(input)?;
    }

    Ok(())
}

#[cfg(any(feature = "eval", feature = "release"))]
pub fn parse_verbosity(input: ParseStream, chk_semicolon: bool) -> syn::Result<Option<Verbosity>> {
    let verbosity;
    let span = input.span();

    if chk_semicolon { parse_optional_semicolon(input, false)?; }

    if input.peek(Token![@]) {
        if verbosity_keyword_peek2(input) {
            <Token![@]>::parse(input)?;

            if input.peek(kw::terse) {
                <kw::terse>::parse(input)?;

                verbosity = Some(Verbosity::Terse);
            } else {
                <kw::verbose>::parse(input)?;

                verbosity = Some(Verbosity::Verbose);
            }
        } else {
            return Err(Error::new(
                span,
                "invalid verbosity designation, use @terse, @verbose or leave blank for default level",
            ));
        }
    } else {
        verbosity = None;
    }

    Ok(verbosity)
}

#[cfg(any(feature = "debug", feature = "report"))]
fn parse_args(input: ParseStream) -> syn::Result<Option<Vec<Expr>>> {
    let mut exprs = Vec::new();

    while input.peek(Token![,]) {
        <Token![,]>::parse(input)?;

        let expr = <Expr>::parse(input)?;

        match expr {
            Expr::Array(_) | Expr::Await(_) |
            Expr::Binary(_) | Expr::Block(_) |
            Expr::Call(_) | Expr::Cast(_) |
            Expr::Field(_) | Expr::Group(_) |
            Expr::If(_) | Expr::Index(_) |
            Expr::Lit(_) | Expr::Macro(_) |
            Expr::Match(_) | Expr::MethodCall(_) |
            Expr::Paren(_) | Expr::Path(_) |
            Expr::Range(_) | Expr::Reference(_) |
            Expr::Repeat(_) | Expr::Try(_) |
            Expr::TryBlock(_) | Expr::Tuple(_) |
            Expr::Unary(_) | Expr::Unsafe(_) => {}
            _ =>
                return Err(Error::new(
                    expr.span(),
                    format!("{:?} is not a supported arg expression", decode_expr_type(&expr)),
                ))
        }

        exprs.push(expr);
    }

    parse_optional_semicolon(input, true)?;

    Ok(if exprs.is_empty() { None } else { Some(exprs) })
}

#[cfg(any(feature = "debug", feature = "report"))]
fn parse_format(input: ParseStream) -> syn::Result<Lit> {
    let literal = <Lit>::parse(input)?;

    match literal {
        Lit::Str(_) | Lit::ByteStr(_) => {}
        _ => return Err(Error::new(literal.span(), "expecting a string literal"))
    }

    Ok(literal)
}

#[cfg(any(feature = "eval", feature = "release"))]
fn verbosity_keyword_peek2(input: ParseStream) -> bool {
    input.peek2(kw::terse) ||
        input.peek2(kw::verbose)
}
