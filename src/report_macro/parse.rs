use syn::Error;
use syn::parse::{Parse, ParseStream};
use verbosity::Verbosity;

use crate::common::{DUPE_VERBOSITY_ERR, kw, QUITE_ERR, VERBOSITY_ORDER_ERR};
use crate::common::tracing::{trace_parsed, trace_source};
use crate::report_macro::{Message, ReportLnMacro, ReportMacro, ReportMessage};

impl Parse for ReportLnMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        trace_parsed(parse_report_macro(
            trace_source(input), true, |terse, verbose| Self { terse, verbose },
        ))
    }
}

impl Parse for ReportMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        trace_parsed(parse_report_macro(
            trace_source(input), false, |terse, verbose| Self { terse, verbose },
        ))
    }
}

#[allow(clippy::shadow_unrelated)] // intention of code is clear
fn parse_report_macro<T>(
    input: ParseStream, ln_brk: bool, builder: impl Fn(Option<ReportMessage>, Option<ReportMessage>) -> T,
) -> syn::Result<T> {
    let (std_err, verbosity) = parse_verbosity(input)?;
    let message = ReportMessage {
        message: Message::parse(input, ln_brk)?,
        std_err,
        verbosity,
    };
    let error_span = input.span();

    match verbosity {
        Verbosity::Quite =>
            unreachable!("{}", QUITE_ERR),
        Verbosity::Terse => {
            // check if a second message is provided
            // a second message requires an intended verbosity level
            let (std_err, verbosity) =
                if let Ok(parsed_values) = parse_verbosity(input) {
                    parsed_values
                } else {
                    return Ok(builder(Some(message), None));
                };

            match verbosity {
                Verbosity::Quite =>
                    unreachable!("{}", QUITE_ERR),
                Verbosity::Terse =>
                    Err(Error::new(error_span, DUPE_VERBOSITY_ERR)),
                Verbosity::Verbose => {
                    // only accept a second message that is intended for verbose output
                    let verbose = ReportMessage {
                        message: Message::parse(input, ln_brk)?,
                        std_err,
                        verbosity,
                    };

                    Ok(builder(Some(message), Some(verbose)))
                }
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
                                unreachable!("{}", QUITE_ERR),
                            Verbosity::Terse =>
                                Err(Error::new(error_span, VERBOSITY_ORDER_ERR)),
                            Verbosity::Verbose =>
                                Err(Error::new(error_span, DUPE_VERBOSITY_ERR))
                        }
                    }
                    Err(err) => Err(err)
                }
            }
    }
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
