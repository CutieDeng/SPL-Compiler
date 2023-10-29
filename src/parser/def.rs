use crate::token::Token;

use super::{specifier::Specifier, dec_list::DecList};

#[derive(Debug, PartialEq, Clone)] 
pub struct Def <'a> {
    pub specifier: Box<Specifier<'a>>, 
    pub dec_list: Box<DecList<'a>>, 
    pub span: &'a [Token<'a>], 
}