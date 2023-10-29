use crate::token::Token;

use super::ext_def_list::ExtDefList;

pub struct Program <'a> (pub Box<ExtDefList<'a>>, pub &'a [Token<'a>]);