#![allow(dead_code)]

#[cfg(all(debug_assertions, feature = "trace"))]
use std::fmt::{Display, Formatter};

#[cfg(all(debug_assertions, feature = "trace"))]
use quote::ToTokens;
use syn::Expr;

use crate::common::Message;

mod parse;
mod tokenize;

pub enum DebugMacro {
    Message(Message),
    Expr(Expr),
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for DebugMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Message(msg) => write!(fmt, "debug! {{\n  message: {msg}\n}}"),
            Self::Expr(expr) => write!(fmt, "debug! {{\n  expr: {}\n}}", expr.to_token_stream())
        }
    }
}

pub struct DebugLnMacro {
    message: Message,
}

#[cfg(all(debug_assertions, feature = "trace"))]
impl Display for DebugLnMacro {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "debugln! {{\n  message: {}\n}}", self.message)
    }
}