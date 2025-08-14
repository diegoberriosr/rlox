#[derive(Debug)]
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

pub struct Token {
    pub token_type: TokenType,
}
