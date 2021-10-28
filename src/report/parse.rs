use syn::{Error, Expr, Lit};
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use verbosity::Verbosity;

use crate::report::{Message, QUITE_ERR, ReportLnMacro, ReportMacro};

mod kw {
    custom_keyword!(err);
    custom_keyword!(terse);
    custom_keyword!(verbose);
}

impl Parse for Message {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (std_err, verbosity) = parse_verbosity(input)?;

        Ok(Self {
            std_err,
            verbosity,
            ln_brk: true,
            fmt: parse_format(input)?,
            args: parse_args(input)?,
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

fn parse_verbosity(input: ParseStream) -> syn::Result<(bool, Verbosity)> {
    let mut std_err = false;
    let verbosity;
    let span = input.span();

    if input.peek(Token![@]) && verbosity_keyword_peek2(input) {
        <Token![@]>::parse(input)?;

        if input.peek(kw::err) {
            <kw::err>::parse(input)?;
            <Token![-]>::parse(input)?;

            std_err = true;
        }

        if input.peek(kw::terse) {
            <kw::terse>::parse(input)?;

            verbosity = Verbosity::Terse;
        } else if input.peek(kw::verbose) {
            <kw::verbose>::parse(input)?;

            verbosity = Verbosity::Verbose;
        } else {
            return Err(Error::new(
                span,
                "invalid verbosity designation, use @terse, @verbose, @err-terse, @err-verbose or leave blank for default level",
            ));
        }
    } else if input.is_empty() {
        return Err(Error::new(span, "expecting a string literal or a valid verbosity designation"));
    } else {
        verbosity = Verbosity::Terse;
    }

    Ok((std_err, verbosity))
}

fn verbosity_keyword_peek2(input: ParseStream) -> bool {
    input.peek2(kw::err) ||
        input.peek2(kw::terse) ||
        input.peek2(kw::verbose)
}

impl Parse for ReportMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        cfg_if! {
            if #[cfg(all(debug_assertions, feature = "trace"))] {
                let result = parse_report_macro(input, false, |terse, verbose| Self { terse, verbose });

                if let Ok(result) = &result {
                    println!("PARSED: {}", result);
                }

                result
            } else {
                parse_report_macro(input, false, |terse, verbose| Self { terse, verbose })
            }
        }
    }
}

impl Parse for ReportLnMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        cfg_if! {
            if #[cfg(all(debug_assertions, feature = "trace"))] {
                let result = parse_report_macro(input, true, |terse, verbose| Self { terse, verbose });

                if let Ok(result) = &result {
                    println!("PARSED: {}", result);
                }

                result
            } else {
                parse_report_macro(input, true, |terse, verbose| Self { terse, verbose })
            }
        }
    }
}

fn parse_report_macro<T>(
    input: ParseStream, ln_brk: bool, builder: impl Fn(Option<Message>, Option<Message>) -> T,
) -> syn::Result<T> {
    let mut message = Message::parse(input)?;
    let error_span = input.span();

    message.ln_brk = ln_brk;

    match &message.verbosity {
        Verbosity::Quite =>
            unreachable!(QUITE_ERR),
        Verbosity::Terse => {
            if let Ok(mut verbose) = Message::parse(input) {
                verbose.ln_brk = ln_brk;

                match &verbose.verbosity {
                    Verbosity::Quite =>
                        unreachable!(QUITE_ERR),
                    Verbosity::Terse =>
                        Err(Error::new(error_span, "do not duplicate verbosity")),
                    Verbosity::Verbose =>
                        Ok(builder(Some(message), Some(verbose)))
                }
            } else {
                Ok(builder(Some(message), None))
            }
        }
        Verbosity::Verbose =>
            if input.is_empty() {
                Ok(builder(None, Some(message)))
            } else {
                let error_span = input.span();

                match parse_verbosity(input) {
                    Ok((_, verbosity)) => {
                        match verbosity {
                            Verbosity::Quite =>
                                unreachable!(QUITE_ERR),
                            Verbosity::Terse =>
                                Err(Error::new(error_span, "define terse reporting before verbose reporting")),
                            Verbosity::Verbose =>
                                Err(Error::new(error_span, "do not duplicate verbosity"))
                        }
                    }
                    Err(err) => {
                        Err(Error::new(error_span, format!("expected verbose message: {:?}", err)))
                    }
                }
            }
    }
}
