use crate::token::Token;

pub struct VarList <'a> {
    pub param_decs: Vec<&'a !>, 
    pub span: &'a [Token<'a, 'a>], 
}