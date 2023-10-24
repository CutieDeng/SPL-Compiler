use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind, 
    pub reason: Cow<'static, str>, 
}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    UnexpectedTerminals, 
    UnexpectedEof, 
}