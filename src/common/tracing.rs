#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::Display;

use syn::parse::ParseStream;

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

#[cfg(feature = "trace")]
#[inline]
pub fn trace_source(input: ParseStream) -> ParseStream {
    println!("SOURCE: {}", input);
    input
}

#[cfg(not(feature = "trace"))]
#[inline]
pub const fn trace_source(input: ParseStream) -> ParseStream { input }
