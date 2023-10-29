use crate::token::Token;

#[derive(Debug, PartialEq, Clone)]
pub struct FunDec <'a> {
    pub id: Token<'a>, 
    pub var_list: Option<Box<&'a !>>, 
    pub span: &'a [Token<'a>], 
}