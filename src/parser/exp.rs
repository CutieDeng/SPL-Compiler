use crate::{token::{Token, TokenType}, parser::error::ErrorKind};

use super::{args::Args, error::Error};

#[derive(Debug, PartialEq, Clone)]
pub enum ExpImpl <'a> {
    Bi(Box<Exp<'a>>, Box<Exp<'a>>, &'a Token<'a, 'a>), 
    Uniary(Box<Exp<'a>>, &'a Token<'a, 'a>), 
    Call(&'a Token<'a, 'a>, Option<Box<Args<'a>>>),
    LBRB(Box<Exp<'a>>, Box<Exp<'a>>), 
    Access(Box<Exp<'a>>, &'a Token<'a, 'a>),
    Id(&'a Token<'a, 'a>),
    Int(&'a Token<'a, 'a>),
    Float(&'a Token<'a, 'a>), 
    Char(&'a Token<'a, 'a>), 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Exp <'a> {
    pub inner: ExpImpl<'a>, 
    pub span: &'a [Token<'a, 'a>],  
}

#[derive(Debug, PartialEq, Clone)]
pub enum Precedence {
    /// Parthesis, Bracket, Dot 
    Parthenesis, 
    /// Negative, Exclamation 
    Exclamation, 
    /// Multiplication, Division 
    Multiplication, 
    /// Addition, Subtraction 
    Addition, 
    /// Less, Greater, LessEqual, GreaterEqual 
    Less, 
    /// && 
    LogicalAnd, 
    /// || 
    LogicalOr, 
    /// = 
    Assign, 
}

impl Precedence {
    pub fn upper(&self) -> Option<Self> {
        use Precedence::*; 
        let u = match self {
            Precedence::Parthenesis => return None, 
            Precedence::Exclamation => Parthenesis, 
            Precedence::Multiplication => Exclamation,
            Precedence::Addition => Multiplication, 
            Precedence::Less => Addition, 
            Precedence::LogicalAnd => Less, 
            Precedence::LogicalOr => LogicalAnd, 
            Precedence::Assign => LogicalOr, 
        }; 
        Some(u) 
    }
    pub fn lower(&self) -> Option<Self> {
        use Precedence::*; 
        let u = match self {
            Parthenesis => Exclamation, 
            Exclamation => Multiplication, 
            Multiplication => Addition, 
            Addition => Less, 
            Less => LogicalAnd, 
            LogicalAnd => LogicalOr, 
            LogicalOr => Assign, 
            Assign => return None, 
        }; 
        Some(u) 
    } 
} 

impl <'a> Exp <'a> {
    pub async fn parse(input: &'a [Token<'a, 'a>]) -> Result<(Exp<'a>, &'a [Token<'a, 'a>]), Error> {
        todo!()
        // return Err(Error {} ); 
    } 
    #[async_recursion::async_recursion]
    pub async fn parse_with_precedence(mut input: &'a [Token<'a, 'a>], precedence: Precedence) -> Result<(Exp<'a>, &'a [Token<'a, 'a>]), Error> {
        let p = precedence.upper(); 
        match p {
            Some(p) => {
                let rp = Self::parse_with_precedence(input, p).await; 

            }
            None => {
                // parthenesis 
                let f = input.first().ok_or(Error { kind: ErrorKind::UnexpectedEof, reason: "".into() }); 
                let f = f?; 
                let ft = f.token_type.as_ref().unwrap(); 
                match ft {
                    | TokenType::Id 
                    | TokenType::Int 
                    | TokenType::Float 
                    | TokenType::Char => {

                    }
                    _ => return Error { kind: ErrorKind::UnexpectedTerminals, reason: "".into() }, 
                }
            }
        }
        todo!()
        // return Err(Error {} ); 
    } 
}