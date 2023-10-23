use crate::token::Token;

use super::{var_dec::VarDec, specifier::Specifier};

pub struct ParamDec <'a> {
    pub specifier: Box<Specifier<'a>>, 
    pub var_dec: Box<VarDec<'a>>,
    pub span: &'a [Token<'a, 'a>], 
}