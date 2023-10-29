use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct StructSpecifier <'a> {
    pub id: Token<'a>,
    pub def_list: Option<Box<&'a !>>,
    pub span: &'a [Token<'a>], 
}