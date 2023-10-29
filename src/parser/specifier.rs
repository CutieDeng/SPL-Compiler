use crate::token::Token;

#[derive(Debug, PartialEq, Clone)] 
pub struct Specifier <'a> {
    pub inner: SpecifierInner<'a>,
    pub span: &'a [Token<'a>],
}

#[derive(Debug, PartialEq, Clone)] 
pub enum SpecifierInner <'a> {
    Type(Token<'a>),
    StructSpecifier(&'a !),
}