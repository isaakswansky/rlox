use crate::error::*;
use crate::token;
use crate::token::*;

pub struct Scanner {
    line: u64,
    current: u64,
    start: u64,
    code: Vec<char>,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(code: String) -> Self {
        Scanner {
            code: code.chars().collect(),
            line: 1,
            current: 0,
            start: 0,
            tokens: vec![],
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len() as u64
    }

    fn advance(&mut self) -> char {
        let c = self.code[self.current as usize];
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<(), ErrorType> {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(
        &mut self,
        token_type: TokenType,
        literal: Option<Literal>,
    ) -> Result<(), ErrorType> {
        let start = self.start as usize;
        let current = self.current as usize;
        let text = &self.code[start..current];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.iter().collect(),
            literal: literal,
            line: self.line,
        });
        Ok(())
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.peek() != expected {
            return false;
        }

        self.advance();
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.code[self.current as usize]
        }
    }

    fn scan_token(&mut self) -> Result<(), ErrorType> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParenthesis),
            ')' => self.add_token(TokenType::RightParenthesis),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '/' => {
                if self.match_next('/') {
                    // This is a comment, ignore everything until newline
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type)
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type)
            }
            ' ' | '\t' | '\r' => Ok(()), // Skip white spaces
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => Err(ErrorType::UnknownTokenError(
                self.line,
                "Encountered an unknown token".to_string(),
            )),
        }
    }

    pub fn scan(&mut self) -> Result<(), ErrorType> {
        self.line = 1;
        self.start = 0;
        self.current = 0;
        self.tokens = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_single_tokens() {
        let test_code = "(){},.-+;*/! =<>// comment".to_string();
        let mut scanner = Scanner::new(test_code);
        let scan_result = scanner.scan();
        if let Err(_) = scan_result {
            assert!(false);
        }
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::LeftParenthesis,
                lexeme: "(".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::RightParenthesis,
                lexeme: ")".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: ".".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Bang,
                lexeme: "!".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                lexeme: "<".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::Greater,
                lexeme: ">".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);

        // scan should always yield the same results
        scanner.scan().unwrap();
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_double_tokens() {
        let test_code = "!= == <= >= //".to_string();
        let mut scanner = Scanner::new(test_code);
        let scan_result = scanner.scan();
        if let Err(_) = scan_result {
            assert!(false);
        }
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::BangEqual,
                lexeme: "!=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::LessEqual,
                lexeme: "<=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: ">=".to_string(),
                literal: None,
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);

        // scan should always yield the same results
        scanner.scan().unwrap();
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_counts_lines() {
        let test_code = "(\n-//some comment\n==\n".to_string();
        let mut scanner = Scanner::new(test_code);
        scanner.scan().unwrap();
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::LeftParenthesis,
                lexeme: "(".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: None,
                line: 2
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                literal: None,
                line: 3
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line: 4
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_ignores_whitespace() {
        let test_code = " ( ) \r { } \t ,  ".to_string();
        let mut scanner = Scanner::new(test_code);
        scanner.scan().unwrap();
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::LeftParenthesis,
                lexeme: "(".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::RightParenthesis,
                lexeme: ")".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                literal: None,
                line: 1
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                literal: None,
                line: 1
            }
        ];
        assert_eq!(expected, scanner.tokens);
    }
}
