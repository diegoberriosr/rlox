pub mod token;

use token::Token;
use token::TokenType;

pub fn generate_tokens(source_code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_index: usize = 0;

    while !is_at_end(current_index, source_code) {
        let current_char: char = source_code.as_bytes()[current_index] as char;

        match current_char {
            '(' => tokens.push(Token {
                token_type: TokenType::LParenthesis,
            }),
            ')' => tokens.push(Token {
                token_type: TokenType::RParenthesis,
            }),
            '{' => tokens.push(Token {
                token_type: TokenType::LCurlyBrace,
            }),
            '}' => tokens.push(Token {
                token_type: TokenType::RCurlyBrace,
            }),
            ',' => tokens.push(Token {
                token_type: TokenType::Comma,
            }),
            '.' => tokens.push(Token {
                token_type: TokenType::Dot,
            }),
            ';' => tokens.push(Token {
                token_type: TokenType::Semicomma,
            }),
            '+' => tokens.push(Token {
                token_type: TokenType::Plus,
            }),
            '-' => tokens.push(Token {
                token_type: TokenType::Minus,
            }),
            '*' => tokens.push(Token {
                token_type: TokenType::Asterisk,
            }),
            '/' => tokens.push(Token {
                token_type: TokenType::Slash,
            }),
            '=' => tokens.push(Token {
                token_type: TokenType::Equals,
            }),
            '!' => tokens.push(Token {
                token_type: TokenType::Not,
            }),
            _ => tokens.push(Token {
                token_type: TokenType::Unknown,
            }),
        }

        current_index += 1;
    }

    tokens
}

fn is_at_end(current_index: usize, source_code: &str) -> bool {
    current_index >= source_code.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tokens_with_single_tokens() {
        let expected_output: Vec<Token> = vec![
            Token {
                token_type: TokenType::LParenthesis,
            },
            Token {
                token_type: TokenType::RParenthesis,
            },
            Token {
                token_type: TokenType::LCurlyBrace,
            },
            Token {
                token_type: TokenType::RCurlyBrace,
            },
            Token {
                token_type: TokenType::Comma,
            },
            Token {
                token_type: TokenType::Dot,
            },
            Token {
                token_type: TokenType::Semicomma,
            },
            Token {
                token_type: TokenType::Plus,
            },
            Token {
                token_type: TokenType::Minus,
            },
            Token {
                token_type: TokenType::Asterisk,
            },
            Token {
                token_type: TokenType::Slash,
            },
            Token {
                token_type: TokenType::Equals,
            },
            Token {
                token_type: TokenType::Not,
            },
        ];

        assert_eq!(expected_output, generate_tokens("(){},.;+-*/=!"));
    }

    #[test]
    fn test_generate_toens_with_not_supported_tokens() {
        let expected_output: Vec<Token> = vec![
            Token {
                token_type: TokenType::Unknown,
            };
            3
        ];

        assert_eq!(expected_output, generate_tokens("~~^"));
    }
}
