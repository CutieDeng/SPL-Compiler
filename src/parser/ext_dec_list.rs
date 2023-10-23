use crate::token::Token;

pub struct ExtDecList <'a> {
    pub var_dec: Vec<&'a !>, 
    pub span: &'a [Token<'a, 'a>], 
}