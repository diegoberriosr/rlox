#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    // Single char
    LParenthesis,
    RParenthesis,
    LCurlyBrace,
    RCurlyBrace,
    Comma,
    Dot,
    Semicomma,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Bang,
    Greater,
    Less,

    // Multi character tokens
    BangEqual,
    EqualEqual,
    GreaterEqual,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
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

    // Special tokens
    Eof,
    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
}
