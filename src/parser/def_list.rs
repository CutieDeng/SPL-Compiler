use crate::token::Token;

use super::def::Def;

#[derive(Debug, PartialEq, Clone)] 
pub struct DefList <'a> (pub Vec<Def<'a>>, pub &'a [Token<'a, 'a>]); 