use crate::token;
use crate::token::token_type::TokenType;
use crate::Lox;

pub struct Scanner<'a> {
    source: String,
    char_iter: std::str::Chars<'a>,
    char_len: u32,
    tokens: Vec<token::Token>,
    start: u32,
    current: u32,
    line: u32,
}

impl Scanner<'_> {
    // NOTE: consumes lexer on use
    pub fn new(source: &String) -> Scanner {
        return Scanner {
            source: source.to_string(),
            char_len: source.chars().count() as u32,
            char_iter: source.chars(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    // should abstract error to a trait
    pub fn scan_tokens(mut self, error: fn(line: u32, message: String)) -> Vec<token::Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(token::Token::new(
            TokenType::Eof,
            String::from(""),
            token::Literal {}, // this needs to change in the near future
            0,                 // hard coded and needs to changed
        ));
        return self.tokens;
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            Some(',') => self.add_token(TokenType::Comma),
            Some('.') => self.add_token(TokenType::Dot),
            Some('-') => self.add_token(TokenType::Minus),
            Some('+') => self.add_token(TokenType::Plus),
            Some(';') => self.add_token(TokenType::Semicolon),
            Some('*') => self.add_token(TokenType::Star),
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('{') => self.add_token(TokenType::LeftBrace),
            Some('}') => self.add_token(TokenType::RightBrace),
            None => return,
            _ => eprintln!("Unexpected character"),
        }
    }

    fn add_token(&mut self, kind: TokenType) {
        let text = self
            .source
            .chars()
            .skip(self.start as usize)
            .take((self.current - self.start) as usize)
            .collect();

        self.tokens.push(token::Token::new(
            kind,
            text,
            token::Literal {}, // this needs to change in the near future
            self.line,
        ));
    }

    fn advance(&mut self) -> Option<char> {
        return self.char_iter.next();
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.char_len as u32;
    }
}
