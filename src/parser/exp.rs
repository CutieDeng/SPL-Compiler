use crate::token::Token;

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

impl <'a> Exp <'a> {
    pub fn parse(input: &'a [Token<'a, 'a>]) -> Result<(Exp<'a>, &'a [Token<'a, 'a>]), Error> {
        todo!()
        // return Err(Error {} ); 
    } 
}