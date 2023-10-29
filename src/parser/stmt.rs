use crate::token::Token;

use super::{compst::CompSt, exp::Exp};

#[derive(Debug, PartialEq, Clone)]
pub enum StmtImpl <'a> {
    Exp(Box<Exp<'a>>),
    CompSt(Box<CompSt<'a>>),
    Return(Box<Exp<'a>>),
    If(Box<Exp<'a>>, Box<Stmt<'a>>, Option<Box<Stmt<'a>>>), 
    While(Box<Exp<'a>>, Box<Stmt<'a>>), 
}

#[derive(Debug, PartialEq, Clone)] 
pub struct Stmt <'a> {
    pub inner: StmtImpl<'a>,
    pub span: &'a [Token<'a>], 
}

