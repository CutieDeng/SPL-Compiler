use std::ops::Not;
use std::num::IntErrorKind;
use std::fmt::Display;

use crate::token::TokenType;

use super::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Error <'a> {
    pub error_type: ErrorType, 
    pub token: Token<'a>, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType { 
    OverflowInt,
    OverflowFloat,
    NotAInt, 
    NotAFloat, 
    NotAChar, 
    InvalidId, 
    InvalidType, 
}

impl <'l, 'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} at {:?}", self.error_type, self.token)
    }
}

impl <'l, 'a> std::error::Error for Error<'a> {
}

pub fn lex_error_detected<'l, 'a> (tokens: &[Token<'a>]) -> Box<[Option<Error<'a>>]> {
    let rst : Vec<_> = tokens.iter().map(|token| {
        match token.token_type {
            None => Some(Error { error_type: ErrorType::InvalidType, token: token.clone() }),
            Some(ref t) => {
                match t {
                    TokenType::Int => {
                        // attempt to parse it 
                        let p = token.content.parse::<u32>();
                        if let Err(e) = p {
                            match e.kind() {
                                | IntErrorKind::Empty | IntErrorKind::InvalidDigit => {
                                    Some(Error { error_type: ErrorType::NotAInt, token: token.clone() }) 
                                }
                                | IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                                    Some(Error { error_type: ErrorType::OverflowInt, token: token.clone() })
                                } 
                                _ => unreachable!(),
                            }
                        } else {
                            None
                        } 
                    }
                    TokenType::Float => {
                        // attempt to parse it 
                        let p = token.content.parse::<f32>();
                        if let Err(_) = p {
                            Some(Error { error_type: ErrorType::NotAFloat, token: token.clone() }) 
                        } else {
                            None
                        } 
                    }
                    TokenType::Char => {
                        if token.content.len() != 1 {
                            Some(Error { error_type: ErrorType::NotAChar, token: token.clone() }) 
                        } else {
                            None
                        } 
                    }
                    TokenType::Id => {
                        let t = token.content.chars().all(|c| c.is_alphanumeric() || c == '_').not().then(|| Error { error_type: ErrorType::InvalidId, token: token.clone() }) ;
                        dbg!(&t); 
                        dbg!(&token); 
                        t 
                    }
                    _ => None, 
                }
            }
        }
    }).collect();
    rst.into_boxed_slice() 
}

