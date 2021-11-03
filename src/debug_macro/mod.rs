#![allow(dead_code)]

use syn::Expr;

use crate::common::Message;

mod parse;
mod tokenize;

pub enum DebugMacro {
    Message(Message),
    Expr(Box<Expr>),
}

pub struct DebugLnMacro {
    message: Message,
}
