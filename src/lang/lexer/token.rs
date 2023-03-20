#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    Comment,
    Str,
    Symbol,
    Operator,
    Identifier,
    Keyword,
    EndOfFile,
    Error,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
}

impl Token {
    pub fn new(token_type: TokenType, content: String) -> Token {
        Token {
            token_type,
            content,
        }
    }
}
