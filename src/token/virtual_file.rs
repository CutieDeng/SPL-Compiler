use std::{borrow::Cow, path::Path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VirtualFile <'a> {
    Stdin, 
    FilePath(Cow<'a, Path>), 
    Rename(Cow<'a, str>), 
}
