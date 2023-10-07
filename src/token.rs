#[derive(Debug, PartialEq, PartialOrd)]
pub enum TokenType {
    // Single-character tokens.
    LeftParenthesis,  // '('
    RightParenthesis, // ')'
    LeftBrace,        // '{'
    RightBrace,       // '}'
    Comma,            // ','
    Dot,              // '.'
    Minus,            // '-'
    Plus,             // '+'
    Semicolon,        // ';'
    Slash,            // '/'
    Star,             // '*'

    // One or two character tokens.
    Bang,         // '!'
    BangEqual,    // '!='
    Equal,        // '='
    EqualEqual,   // '=='
    Greater,      // '>'
    GreaterEqual, // '>='
    Less,         // '<'
    LessEqual,    // '<='

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),
    Boolean(bool),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u64,
}
