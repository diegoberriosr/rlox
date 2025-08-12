mod lexer;

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
    token_type: TokenType,
}
