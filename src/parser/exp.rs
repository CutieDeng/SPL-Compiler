use crate::{token::{Token, TokenType}, parser::error::ErrorKind};

use super::{args::Args, error::Error};

#[derive(Debug, PartialEq, Clone)]
pub enum ExpImpl <'a> {
    Bi(Box<Exp<'a>>, Box<Exp<'a>>, &'a Token<'a>), 
    Uniary(Box<Exp<'a>>, &'a Token<'a>), 
    Call(&'a Token<'a>, Option<Box<Args<'a>>>),
    LBRB(Box<Exp<'a>>, Box<Exp<'a>>), 
    Access(Box<Exp<'a>>, &'a Token<'a>),
    Id(&'a Token<'a>),
    Int(&'a Token<'a>),
    Float(&'a Token<'a>), 
    Char(&'a Token<'a>), 
}

#[derive(Debug, PartialEq, Clone)]
pub struct Exp <'a> {
    pub inner: ExpImpl<'a>, 
    pub span: &'a [Token<'a>],  
}

impl <'a> Exp <'a> {
    pub fn parse(input: &'a [Token<'a>]) -> Result<(Exp<'a>, &'a [Token<'a>]), Error> {
        todo!()
        // return Err(Error {} ); 
    } 
    pub fn parse_single(input: &'a [Token<'a>]) -> Result<(Exp<'a>, &'a [Token<'a>]), Error> {
        match input.first() {
            Some(i) => {
                let tt = i.token_type.as_ref().unwrap(); 
                let r; 
                match tt {
                    crate::token::TokenType::Int => r = ExpImpl::Int(i),
                    crate::token::TokenType::Float => r = ExpImpl::Float(i), 
                    crate::token::TokenType::Char => r = ExpImpl::Char(i), 
                    crate::token::TokenType::Id => r = ExpImpl::Id(i), 
                    _ => return Err(Error { kind: ErrorKind::UnexpectedTerminals, reason: "Expected id, char, float, int (exp expressions) ".into() }), 
                }
                return Ok((Exp { inner: r, span: &input[0..1] }, &input[1..])); 
            },
            None => return Err(Error { kind: ErrorKind::Eof, reason: "Unexpected EOF".into() }), 
        }
    }
    pub fn parse_parenthesis(input: &'a [Token<'a>]) -> Result<(Exp<'a>, &'a [Token<'a>]), Error> {
        let mut error = None; 
        // ID LP, special case 
        match input {
            &[Token { location: _, token_type: Some(TokenType::Id), content: _ }, Token { location: _, token_type: Some(TokenType::Lp), content: _ }, ..] => {
                let mut rest = &input[2..]; 
                let r = Args::parse(rest); 
                let args; 
                let id; 
                id = input.first().unwrap(); 
                match r {
                    Err(e) => {
                        args = None; 
                        error = Some(e); 
                    }
                    Ok(r) => {
                        args = Some(r.0); 
                        rest = r.1; 
                    }
                }
                match rest.first() {
                    Some(Token { location: _, token_type: Some(TokenType::Rp), content: _ }) => {
                        let args = args.map(Box::new); 
                        return Ok((Exp { inner: ExpImpl::Call(id, args), span: &input[0..rest.len()] }, &rest[1..])); 
                    }
                    _ => (), 
                }
            }
            _ => (), 
        }
        match input {
            &[Token { location: _, token_type: Some(TokenType::Lp), content: _ }, ..] => {
                let mut rest = &input[1..]; 
                let r = Exp::parse(rest); 
                let exp; 
                match r {
                    Err(e) => {
                        exp = None; 
                        _ = error; 
                        error = Some(e); 
                    }
                    Ok(r) => {
                        exp = Some(r.0); 
                        rest = r.1; 
                    }
                }
                match rest.first() {
                    Some(Token { location: _, token_type: Some(TokenType::Rp), content: _ }) => {
                        let exp = exp.map(Box::new); 
                        return Ok((Exp { inner: ExpImpl::Uniary(exp.unwrap(), input.first().unwrap()), span: &input[0..rest.len()] }, &rest[1..])); 
                    }
                    _ => {
                        _ = error; 
                        error = Some(Error { kind: ErrorKind::UnexpectedTerminals, reason: "Expected )".into() }); 
                    }
                }
            }
            _ => (),  
        }
        
        todo!()
    }
}
