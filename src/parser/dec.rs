use crate::token::Token;

use super::{var_dec::VarDec, exp::Exp};

#[derive(Debug, PartialEq, Clone)] 
pub struct Dec<'a> (pub Box<VarDec<'a>>, pub Option<Box<Exp<'a>>>, pub &'a [Token<'a>]); 