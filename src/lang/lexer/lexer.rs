use std::str::Chars;

use super::*;

#[derive(Debug)]
pub struct Lexer<'a> {
    index: usize,
    source: &'a str,
    iter: Chars<'a>,
    curr_chr: char,
}

impl Lexer<'_> {
    pub fn new(source: &str) -> Lexer {
        let mut lex = Lexer {
            index: 0,
            source: source,
            iter: source.chars(),
            curr_chr: '\x00',
        };
        lex.scan_char();
        // The index starts at zero.
        lex.index = 0;
        lex
    }

    fn ended(&self) -> bool {
        self.source.len() == self.index
    }

    fn scan_char(&mut self) {
        if let Some(chr) = self.iter.next() {
            self.curr_chr = chr;
            self.index += 1;
        } else {
            self.index = self.source.len();
            self.curr_chr = '\x00';
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.ended() {
            return Token::new(TokenType::EndOfFile, "".to_string());
        }
        self.skip_whitespace();

        // Check for two-character ops
        if let Some(next_chr) = self.iter.clone().next() {
            let op = format!("{}{}", self.curr_chr, next_chr);
            let multi_char_tok = match op.as_str() {
                "==" | "!=" | "<=" | ">=" | "||" | "&&" | "++" | "--" | "<<" | ">>" | "//" | "**" | "+=" | "-=" | "/=" | "*=" => {
                    Token::new(TokenType::Operator, op)
                }
                "@{" | "${" => {
                    Token::new(TokenType::Symbol, op)
                }
                _ => Token::new(TokenType::Error, "".to_string()),
            };

            if multi_char_tok.token_type != TokenType::Error {
                for _ in 0..multi_char_tok.content.len() {
                    self.scan_char();
                }
                return multi_char_tok;
            }
        };


        let tok = match self.curr_chr {
            '+' | '-' | '/' | '*' | '%' | '<' | '>' | '^' | '!' | '~' => {
                Token::new(TokenType::Operator, self.curr_chr.to_string())
            }
            ';' | ':' | '{' | '}' | '(' | ')' | '[' | ']' | '.' | ',' | '=' => {
                Token::new(TokenType::Symbol, self.curr_chr.to_string())
            }
            '#' => {
                self.get_comment()
            }
            _ => Token::new(TokenType::Error, "".to_string()),
        };

        if tok.token_type != TokenType::Error {
            self.scan_char();
            tok
        } else if self.curr_chr == 'r'
            && (match self.iter.clone().next() {
                Some(ch) => ch == '\'' || ch == '"',
                None => false,
            })
        {
            self.scan_char();
            self.get_string_literal(true)
        } else if self.curr_chr == '"' || self.curr_chr == '\'' {
            self.get_string_literal(false)
        } else if self.curr_chr.is_alphabetic() || self.curr_chr == '_' || self.curr_chr == '$' {
            self.get_identifier_or_keyword()
        } else if self.curr_chr.is_numeric() {
            self.get_number_literal()
        } else {
            panic!("unknown character/token at position {}", self.index);
        }
    }

    fn skip_whitespace(&mut self) {
        while self.curr_chr == ' '
            || self.curr_chr == '\t'
            || self.curr_chr == '\r'
            || self.curr_chr == '\n'
        {
            self.scan_char()
        }
    }

    fn get_comment(&mut self) -> Token {
        let start = self.index;
        while self.curr_chr != '\n' {
            self.scan_char();
        }
        Token::new(
            TokenType::Comment,
            self.source[start..self.index].to_string(),
        )
    }

    fn get_identifier_or_keyword(&mut self) -> Token {
        let start = self.index;
        while self.curr_chr.is_alphanumeric() || self.curr_chr == '_' || self.curr_chr == '$' {
            self.scan_char();
        }
        let value = &self.source[start..self.index];
        Token::new(
            match value {
                "proc" | "event" | "fn" | "if" | "elif" | "else" | "while" | "forever" | "for" | "in" | "break" | "private" => TokenType::Keyword,
                _ => TokenType::Identifier,
            },
            value.to_string(),
        )
    }

    fn get_number_literal(&mut self) -> Token {
        let start = self.index;
        let mut is_float = false;
        while self.curr_chr.is_numeric() || (!is_float && (self.curr_chr == '.')) {
            if self.curr_chr == '.' {
                is_float = true;
            }
            self.scan_char();
        }
        Token::new(
            TokenType::Number,
            self.source[start..self.index].to_string(),
        )
    }

    fn get_string_literal(&mut self, raw: bool) -> Token {
        let quote_type = self.curr_chr;
        let mut escaped = false;
        let mut string = "".to_string();
        loop {
            self.scan_char();
            assert!(self.curr_chr != '\x00', "unterminated string literal");
            if raw {
                if self.curr_chr == quote_type {
                    break;
                }
                string.push(self.curr_chr);
            } else if escaped {
                let chr = match self.curr_chr {
                    c @ '\\' | c @ '\'' | c @ '"' => c,
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    esc => { panic!("invalid escape literal \\{esc}"); }
                };
                string.push(
                    chr
                );
                escaped = false;
            } else if self.curr_chr == '\\' {
                escaped = true;
            } else if self.curr_chr == quote_type {
                break;
            } else {
                string.push(self.curr_chr);
            }
        }
        self.scan_char(); // Skip the last quotation
        Token::new(TokenType::Str, string)
    }
}

impl<'l> Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<token::Token> {
        let token = self.next_token();

        if token.token_type == TokenType::EndOfFile {
            None
        } else {
            Some(token)
        }
    }
}
