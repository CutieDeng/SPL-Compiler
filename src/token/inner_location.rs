#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerLocation {
    pub line : usize, 
    pub col : usize, 
}

impl InnerLocation {
    pub fn next_line(self) -> Self {
        Self {
            line: self.line + 1, 
            col: 0, 
        } 
    } 
    pub fn next_col(self) -> Self {
        Self {
            line: self.line, 
            col: self.col + 1, 
        }
    } 
    pub fn new() -> Self {
        Self {
            line: 0, 
            col: 0, 
        } 
    }
} 
