use crate::token::Token;

use super::ext_def::ExtDef;

#[derive(Debug, PartialEq, Clone)] 
pub struct ExtDefList <'a> {
    pub list: Vec<ExtDef<'a>>, 
    pub span: &'a [Token<'a, 'a>], 
}