use crate::token::Token;

#[derive(Debug, PartialEq, Clone)] 
pub struct ExtDef<'a> (&'a !, pub &'a [Token<'a>]);