use crate::{token::Token, yield_now};

use super::{exp::Exp, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct Args <'a> (pub Vec<Exp<'a>>, pub &'a [Token<'a, 'a>]);

impl <'a> Args <'a> {
    pub async fn parse(input: &'a [Token<'a, 'a>]) -> Result<(Args<'a>, &'a [Token<'a, 'a>]), Error> {
        // let mut args_v = Vec::new(); 
        loop {
            let r = Exp::parse(input).await; 
            

        }

        todo!()
    }
}