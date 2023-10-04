use crate::error::*;
use crate::token::*;

pub struct Scanner {
    line: u64,
    current: u64,
    start: u64,
    code: Vec<char>,
}

impl Scanner {
    pub fn new(code: &String) -> Self {
        Scanner {
            code: code.chars().collect(),
            line: 1,
            current: 0,
            start: 0,
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

    fn add_token(&mut self, token_type: TokenType) {}

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: String) {}

    fn scan_token(&mut self) -> Result<Token, ErrorType> {
        let c = self.advance();
        match c {
            '(' => Ok(Token {
                token_type: TokenType::LeftParenthesis,
                lexeme: "(".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            ')' => Ok(Token {
                token_type: TokenType::RightParenthesis,
                lexeme: ")".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '{' => Ok(Token {
                token_type: TokenType::LeftBrace,
                lexeme: "{".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '}' => Ok(Token {
                token_type: TokenType::RightBrace,
                lexeme: "}".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            ',' => Ok(Token {
                token_type: TokenType::Comma,
                lexeme: ",".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '.' => Ok(Token {
                token_type: TokenType::Dot,
                lexeme: ".".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '-' => Ok(Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '+' => Ok(Token {
                token_type: TokenType::Plus,
                lexeme: "+".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            ';' => Ok(Token {
                token_type: TokenType::Semicolon,
                lexeme: ";".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            '*' => Ok(Token {
                token_type: TokenType::Star,
                lexeme: "*".to_string(),
                literal: "".to_string(),
                line: self.line,
            }),
            _ => Err(ErrorType::UnknownTokenError(
                self.line,
                "Encountered an unknown token".to_string(),
            )),
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, ErrorType> {
        self.line = 1;
        self.start = 0;
        self.current = 0;
        let mut tokens: Vec<Token> = vec![];

        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            tokens.push(token);
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: String::new(),
            literal: String::new(),
            line: self.line,
        });

        Ok(vec![])
    }
}
