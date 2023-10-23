use std::sync::Arc;

use super::{VirtualFile, InnerLocation};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location <'a> {
    pub file: Arc<VirtualFile<'a>>, 
    pub location: InnerLocation, 
}
