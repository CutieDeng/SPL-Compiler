use std::borrow::Cow;

use crate::token::{TokenType, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct Error <'a> {
    pub kind: ErrorKind<'a>, 
    pub reason: Cow<'static, str>, 
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind <'a> {
    Eof, 
    UnexpectedTerminals, 
    DesiredTerminal {
        expected: TokenType, 
        position: Token<'a> 
    }
}