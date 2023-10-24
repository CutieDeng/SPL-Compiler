use std::sync::Arc;

use spl::token::error::lex_error_detected;
use spl::token::{Location, VirtualFile, InnerLocation};
use spl::token::lexer::Lexer;

pub fn main() {
    // let a = async {
    //     println!("Hello, world!");
    //     yield_now().await; 
    //     println!("Hello, world!");
    //     return ; 
    // }.fuse(); 
    // let mut a = Box::pin(a); 
    // let b = async {
    //     println!("Hello, world");
    //     yield_now().await; 
    //     println!("Hello, world");
    //     return ; 
    // }.fuse(); 
    // let mut b = Box::pin(b); 
    // block_on(async move {
    //     select! {
    //         _ = a => {
    //             println!("a finished first");  
    //         }
    //         _ = b => {
    //             println!("b finished first"); 
    //         }
    //     }
    // }); 
    let p = Lexer.lexer("int struct take_2 many 123b k90 r   ft a1,2 never! handle problem-cast float", Location { file: Arc::new(VirtualFile::Stdin), location: InnerLocation::new() }) ;
    let p2 = lex_error_detected(&p.0); 
    dbg!(p);
    dbg!(p2); 
}