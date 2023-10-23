use crate::token::Token;

use super::dec::Dec;

#[derive(Debug, PartialEq, Clone)] 
pub struct DecList <'a> (pub Vec<Dec<'a>>, pub &'a [Token<'a, 'a>]); 