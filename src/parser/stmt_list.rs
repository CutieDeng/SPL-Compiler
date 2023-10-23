use crate::token::Token;

use super::stmt::Stmt;

#[derive(Debug, PartialEq, Clone)] 
pub struct StmtList <'a> ( pub Vec<Stmt<'a>>, pub &'a [Token<'a, 'a>] );