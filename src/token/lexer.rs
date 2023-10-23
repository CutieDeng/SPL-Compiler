use crate::{yield_now, token::TokenType};

use super::{Token, Location};

pub struct Lexer; 

impl Lexer {
    pub async fn lexer<'a, 'l> (&self, content: &'a str, start_location: Location<'l>) -> (Vec<Token<'l, 'a>>, bool) where 'a : 'l { 
        let origin_content = content; 
        let mut count = 0; 
        let mut tokens = Vec::new(); 
        let mut location = start_location; 
        let content: Vec<_> = content.char_indices().collect(); 
        let mut buffer_idx = None::<usize>; 
        #[derive(Debug, Clone, PartialEq)]
        enum Comment {
            Line, 
            Block, 
            None, 
        }
        let mut is_number = false; 
        let mut comment = Comment::None; 
        let mut is_single_quote = false; 
        let mut ignore = 0; 
        let mut _macro_handle = false; 
        let mut last_location; 
        let mut now_location = None; 
        for idx in 0..content.len() { 
            count += 1; 
            if count == 1000 {
                count = 0; 
                yield_now().await; 
            }
            // Get the current character 
            let (i, c) = content[idx]; 
            // Adjust the location 
            last_location = now_location; 
            now_location = Some(location.location.clone()); 
            location.location = if c == '\n' {
                location.location.next_line()
            } else {
                location.location.next_col() 
            };
            // Ignore any content, with the highest priority ignoreing rule 
            if ignore > 0 {
                ignore -= 1; 
                continue ; 
            } 
            match comment {
                Comment::Line => {
                    if c == '\n' {
                        comment = Comment::None; 
                    }
                    continue ;  
                }
                Comment::Block => {
                    if c == '*' && content.get(idx + 1).map(|(_, c)| *c) == Some('/') {
                        comment = Comment::None; 
                        ignore = 1; 
                    }
                    continue ;  
                }
                Comment::None => (), 
            }
            if c == '\'' {
                if is_single_quote {
                    is_single_quote = false; 
                    let bidx = buffer_idx.unwrap(); 
                    let l = content[bidx].0;
                    let r = content[idx].0; 
                    let content = &origin_content[l..r]; 
                    tokens.push(Token {
                        location: Location {
                            file: location.file.clone(), 
                            location: now_location.clone().unwrap(), 
                        }, 
                        token_type: Some(TokenType::Char), 
                        content: content, 
                    }); 
                } 
                continue ; 
            }
            if c == '#' && now_location.as_ref().unwrap().col == 0 {
                _macro_handle = true; 
                continue ; 
            }
            // remove the buffer ~ 
            if c.is_ascii_punctuation() || c.is_whitespace() {
                if let Some(bidx) = buffer_idx {
                    if is_number && c == '.' {
                        continue ; 
                    } 
                    if c == '_' {
                        continue ; 
                    }
                    let l = content[bidx].0;
                    let content = &origin_content[l..i]; 
                    let token_type = match content {
                        | "int" | "float" | "char" => Some(TokenType::Type), 
                        | "struct" => Some(TokenType::Struct), 
                        | "if" => Some(TokenType::If), 
                        | "else" => Some(TokenType::Else), 
                        | "while" => Some(TokenType::While), 
                        | "return" => Some(TokenType::Return), 
                        _ if is_number => {
                            if content.contains('.') {
                                Some(TokenType::Float)
                            } else {
                                Some(TokenType::Int)
                            } 
                        }, 
                        _ => Some(TokenType::Id),
                    }; 
                    tokens.push(Token {
                        location: Location {
                            file: location.file.clone(), 
                            location: last_location.clone().unwrap(), 
                        }, 
                        token_type, 
                        content,
                    }); 
                    buffer_idx = None;  
                }
            } else {
                if buffer_idx.is_none() {
                    is_number = c.is_ascii_digit(); 
                    buffer_idx = Some(idx); 
                } 
                continue ; 
            }
            let mut lo = Location {
                file: location.file.clone(), 
                location: now_location.clone().unwrap(), 
            }; 
            match c {
                '/' => {
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('/') {
                        comment = Comment::Line; 
                        ignore = 1; 
                        continue ; 
                    } else if content.get(idx + 1).map(|(_, c)| *c) == Some('*') {
                        comment = Comment::Block; 
                        ignore = 1; 
                        continue ; 
                    } 
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Div), 
                        content: "/", 
                    }; 
                    tokens.push(token); 
                }, 
                '*' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Mul), 
                        content: "*", 
                    }; 
                    tokens.push(token); 
                }, 
                '+' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Plus), 
                        content: "+", 
                    }; 
                    tokens.push(token); 
                }, 
                '-' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Minus), 
                        content: "-", 
                    }; 
                    tokens.push(token); 
                }, 
                '=' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('=') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Eq), 
                            content: "==", 
                        }; 
                        ignore = 1; 
                    } else {
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Assign), 
                            content: "=", 
                        };  
                    } 
                    tokens.push(token); 
                },                 
                '!' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('=') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Ne), 
                            content: "!=", 
                        }; 
                        ignore = 1; 
                    } else {
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Not), 
                            content: "!", 
                        };  
                    } 
                    tokens.push(token); 
                }, 
                '<' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('=') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Le), 
                            content: "<=", 
                        }; 
                        ignore = 1; 
                    } else {
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Lt), 
                            content: "<", 
                        };  
                    } 
                    tokens.push(token); 
                }, 
                '>' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('=') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Ge), 
                            content: ">=", 
                        }; 
                        ignore = 1; 
                    } else {
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Gt), 
                            content: ">", 
                        };  
                    } 
                    tokens.push(token); 
                }, 
                '.' => {
                    if is_number {
                        continue ; 
                    } 
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Dot), 
                        content: ".", 
                    }; 
                    tokens.push(token); 
                }, 
                ';' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Semi), 
                        content: ";", 
                    }; 
                    tokens.push(token); 
                }, 
                ',' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Comma), 
                        content: ",", 
                    }; 
                    tokens.push(token); 
                },
                '&' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('&') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::And), 
                            content: "&&", 
                        }; 
                        ignore = 1; 
                        tokens.push(token); 
                    } 
                }, 
                '|' => {
                    let token ; 
                    if content.get(idx + 1).map(|(_, c)| *c) == Some('|') {
                        lo.location = lo.location.next_col(); 
                        token = Token {
                            location: lo, 
                            token_type: Some(TokenType::Or), 
                            content: "||", 
                        }; 
                        ignore = 1; 
                        tokens.push(token); 
                    } 
                }, 
                '(' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Lp), 
                        content: "(", 
                    }; 
                    tokens.push(token); 
                }, 
                ')' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Rp), 
                        content: ")", 
                    }; 
                    tokens.push(token); 
                }, 
                '[' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Lb), 
                        content: "[", 
                    }; 
                    tokens.push(token); 
                }, 
                ']' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Rb), 
                        content: "]", 
                    }; 
                    tokens.push(token); 
                }, 
                '{' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Lc), 
                        content: "{", 
                    }; 
                    tokens.push(token); 
                }, 
                '}' => {
                    let token = Token {
                        location: lo, 
                        token_type: Some(TokenType::Rc), 
                        content: "}", 
                    }; 
                    tokens.push(token); 
                }, 
                _ if c.is_ascii_punctuation() && c != '_' => {
                    let token = Token {
                        location: lo, 
                        token_type: None, 
                        content: &origin_content[i..i+1], 
                    }; 
                    tokens.push(token); 
                }, 
                _ => (), 
            }
        }
        if let Some(bidx) = buffer_idx {
            let l = content[bidx].0;
            let content = &origin_content[l..]; 
            let token_type = match content {
                | "int" | "float" | "char" => Some(TokenType::Type), 
                | "struct" => Some(TokenType::Struct), 
                | "if" => Some(TokenType::If), 
                | "else" => Some(TokenType::Else), 
                | "while" => Some(TokenType::While), 
                | "return" => Some(TokenType::Return), 
                _ => None, 
            }; 
            tokens.push(Token {
                location: Location {
                    file: location.file.clone(), 
                    location: now_location.clone().unwrap(), 
                }, 
                token_type, 
                content,
            }); 
        }
        (tokens, comment != Comment::Block)
    } 
} 