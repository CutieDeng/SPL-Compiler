use std::process::exit;
use std::sync::Arc;

use futures::{select, FutureExt};
use futures::executor::block_on;

use spl::token::error::lex_error_detected;
use spl::token::{Location, VirtualFile, InnerLocation};
use spl::token::lexer::Lexer;
use spl::yield_now;

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

    let mut a = Box::pin(async {
        println!("Hello, world!");
        yield_now().await;
        println!("Hello, world again from a!");
        yield_now().await;
        Ok::<(), ()>(()) // 成功
    }.fuse());

    let mut b = Box::pin(async {
        println!("Hello, world");
        yield_now().await;
        println!("Hello, world again from b!");
        Err::<(), ()>(()) // 失败
    }.fuse());

    block_on(async {
        loop {
            select! {
                result = &mut a => match result {
                    Ok(_) => println!("a succeeded"),
                    Err(_) => {} // do nothing
                },
                result = &mut b => match result {
                    Ok(_) => println!("b succeeded"),
                    Err(_) => {} // do nothing
                },
                default => {
                    println!("neither a nor b is ready"); 
                }
                complete => {
                    println!("both a and b completed");
                    break;
                }
            }
        }
    });

    exit(0);

    let p = block_on(Lexer.lexer("int struct take_2 many 123b k90 r   ft a1,2 never! handle problem-cast float", Location { file: Arc::new(VirtualFile::Stdin), location: InnerLocation::new() })); 
    let p2 = lex_error_detected(&p.0); 
    dbg!(p);
    dbg!(p2); 
}