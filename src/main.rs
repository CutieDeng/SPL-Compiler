use std::env;
use std::process::exit;
use std::sync::Arc;

use spl::token::error::lex_error_detected;
use spl::token::{Location, VirtualFile, InnerLocation};
use spl::token::lexer::Lexer;

pub fn main() {
    let fname = env::args().nth(1).expect("no file given");
    let path = std::path::Path::new(&fname); 
    let file = VirtualFile::FilePath(path.into()); 
    let content = std::fs::read_to_string(path).expect("could not read file"); 
    let l = Lexer.lexer(&content, Location { file: Arc::new(file), location: InnerLocation::new() }) ; 
    let last = l.0.last().map(|x| x.location.location.line).unwrap_or(0); 
    if !l.1 {
        eprintln!("Error type A at Line #{}: Missing multi line comment end", last); 
        exit(1); 
    }
    let l = l.0; 
    for li in &l {
        println!("{:?}", li); 
    }
    let lexer_error = lex_error_detected(&l); 
    for e in lexer_error.iter() {
        if let Some(e) = e {
            eprintln!("Error type A at Line #{}: {:?}", e.token.location.location.line, e.error_type); 
        }
    }
}