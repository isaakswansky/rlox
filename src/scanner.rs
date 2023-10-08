use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::error::*;
use crate::token::*;

pub struct Scanner {
    line: u64,
    current: u64,
    start: u64,
    code: Vec<char>,
    pub tokens: Vec<Token>,
}
lazy_static! {
    static ref KEYWORD_MAP: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();
        m.insert("and", Keyword::And);
        m.insert("class", Keyword::Class);
        m.insert("class", Keyword::Class);
        m.insert("else", Keyword::Else);
        m.insert("false", Keyword::False);
        m.insert("for", Keyword::For);
        m.insert("fun", Keyword::Fun);
        m.insert("if", Keyword::If);
        m.insert("nil", Keyword::Nil);
        m.insert("or", Keyword::Or);
        m.insert("print", Keyword::Print);
        m.insert("return", Keyword::Return);
        m.insert("super", Keyword::Super);
        m.insert("this", Keyword::This);
        m.insert("true", Keyword::True);
        m.insert("var", Keyword::Var);
        m.insert("while", Keyword::While);
        m
    };
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
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: self.get_current_text(0, 0),
            line: self.line,
        });
        Ok(())
    }

    fn get_current_text(&self, start_offset: i64, end_offset: i64) -> String {
        let start = (self.start as i64 + start_offset) as usize;
        let current = (self.current as i64 + end_offset) as usize;
        let text = &self.code[start..current];
        text.iter().collect()
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

    fn add_string_literal(&mut self) -> Result<(), ErrorType> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(ErrorType::ScanError(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }
        self.advance();
        let text = self.get_current_text(1, -1);
        self.add_token(TokenType::String(text))
    }

    fn peek_next(&self) -> char {
        if self.current as usize + 1 >= self.code.len() {
            '\0'
        } else {
            self.code[self.current as usize + 1]
        }
    }

    fn add_number_literal(&mut self) -> Result<(), ErrorType> {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }
        let text = self.get_current_text(0, 0);
        if let Ok(number) = text.parse::<f64>() {
            self.add_token(TokenType::Number(number))
        } else {
            Err(ErrorType::ScanError(
                self.line,
                "Invalid number format.".to_string(),
            ))
        }
    }

    fn add_identifier(&mut self) -> Result<(), ErrorType> {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = self.get_current_text(0, 0);
        let token_type = if let Some(keyword) = KEYWORD_MAP.get(text.as_str()) {
            TokenType::Keyword(keyword.clone())
        } else {
            TokenType::Identifier(self.get_current_text(0, 0))
        };
        self.add_token(token_type)
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alphabetic(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alphabetic(c)
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
            '"' => self.add_string_literal(),
            ' ' | '\t' | '\r' => Ok(()), // Skip white spaces
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => {
                if self.is_digit(c) {
                    self.add_number_literal()
                } else if self.is_alphabetic(c) {
                    self.add_identifier()
                } else {
                    Err(ErrorType::ScanError(
                        self.line,
                        "Encountered an unknown token".to_string(),
                    ))
                }
            }
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
            line: self.line,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_keywords() {
        let test_code = "and class else false fun for if nil or print return super this true var while".to_string();
        let mut scanner = Scanner::new(test_code);
        if let Err(_) = scanner.scan() {
            assert!(false);
        }
        let expected:Vec<Token> = vec![
            Token {
                token_type: TokenType::Keyword(Keyword::And),
                lexeme: "and".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Class),
                lexeme: "class".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Else),
                lexeme: "else".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::False),
                lexeme: "false".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Fun),
                lexeme: "fun".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::For),
                lexeme: "for".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::If),
                lexeme: "if".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Nil),
                lexeme: "nil".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Or),
                lexeme: "or".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Print),
                lexeme: "print".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Return),
                lexeme: "return".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Super),
                lexeme: "super".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::This),
                lexeme: "this".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::True),
                lexeme: "true".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::Var),
                lexeme: "var".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::Keyword(Keyword::While),
                lexeme: "while".to_string(),
                line: 1
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }

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
                line: 1,
            },
            Token {
                token_type: TokenType::RightParenthesis,
                lexeme: ")".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Dot,
                lexeme: ".".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Slash,
                lexeme: "/".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Bang,
                lexeme: "!".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Less,
                lexeme: "<".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Greater,
                lexeme: ">".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);

        // scan should always yield the same results
        scanner.scan().unwrap();
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_string_literals() {
        let test_code = r#""hello
sir" "word""#
            .to_string();
        let mut scanner = Scanner::new(test_code);
        scanner.scan().unwrap();
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::String("hello\nsir".to_string()),
                lexeme: r#""hello
sir""#
                    .to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::String("word".to_string()),
                lexeme: r#""word""#.to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 2,
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_number_literal() {
        let test_code = "1.234 1234".to_string();
        let mut scanner = Scanner::new(test_code);
        scanner.scan().unwrap();
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::Number(1.234),
                lexeme: "1.234".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Number(1234.0),
                lexeme: "1234".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_identifiers() {
        let test_code = "my_var1\nthisisa123name".to_string();
        let mut scanner = Scanner::new(test_code);
        scanner.scan().unwrap();
        let expected: Vec<Token> = vec![
            Token {
                token_type: TokenType::Identifier("my_var1".to_string()),
                lexeme: "my_var1".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Identifier("thisisa123name".to_string()),
                lexeme: "thisisa123name".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 2,
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }

    #[test]
    fn scan_unterminated_string_literal_returns_error() {
        let test_code = r#""hello"#.to_string();
        let mut scanner = Scanner::new(test_code);
        if let Ok(_) = scanner.scan() {
            assert!(false);
        }
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
                line: 1,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::LessEqual,
                lexeme: "<=".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::GreaterEqual,
                lexeme: ">=".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
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
                line: 1,
            },
            Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::EqualEqual,
                lexeme: "==".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 4,
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
                line: 1,
            },
            Token {
                token_type: TokenType::RightParenthesis,
                lexeme: ")".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];
        assert_eq!(expected, scanner.tokens);
    }
}
