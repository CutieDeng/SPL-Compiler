use crate::token::Token;

use super::{def_list::DefList, stmt_list::StmtList};

#[derive(Debug, PartialEq, Clone)]
pub struct CompSt <'a> {
    pub def_list: Box<DefList<'a>>, 
    pub stmt_list: Box<StmtList<'a>>, 
    pub span: &'a [Token<'a>], 
}