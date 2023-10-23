pub mod token_type;
pub use token_type::TokenType;

pub mod virtual_file; 
pub use virtual_file::VirtualFile; 

pub mod inner_location;
pub use inner_location::InnerLocation; 

pub mod location;
pub use location::Location; 

pub mod lexer;

#[derive(Debug, Clone, PartialEq)]
pub struct Token <'l, 'a> where 'a : 'l {
    pub location: Location<'l>, 
    pub token_type: Option<TokenType>, 
    pub content: &'a str, 
}

pub mod error;
pub use error::Error; 