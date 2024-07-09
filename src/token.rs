use core::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

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
    Eof
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: u8,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<String>, line: u8) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}