use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct VarDec <'a> {
    pub inner: VarDecImpl<'a>,
    pub span: &'a [Token<'a>], 
}

#[derive(Debug, PartialEq, Clone)]
pub enum VarDecImpl <'a> {
    Id(&'a Token<'a>),
    VarDec(Box<VarDec<'a>>, &'a Token<'a>), 
}