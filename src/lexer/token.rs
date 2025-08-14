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
    Not,

    Unknown,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
}
