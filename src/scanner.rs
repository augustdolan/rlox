use std::iter::Peekable;

use crate::token::token_type::TokenType;
use crate::token::{self, Literal};

pub struct Scanner<'a, F: FnMut(u32, &str)> {
    source: String,
    char_iter: Peekable<std::str::Chars<'a>>,
    char_len: u32,
    tokens: Vec<token::Token>,
    start: u32,
    current: u32,
    line: u32,
    error_handler: F,
}

impl<F: FnMut(u32, &str)> Scanner<'_, F> {
    pub fn new(source: &String, error_handler: F) -> Scanner<F> {
        return Scanner {
            error_handler,
            source: source.to_string(),
            char_len: source.chars().count() as u32,
            char_iter: source.chars().peekable(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        };
    }

    // NOTE: consumes lexer on use
    pub fn scan_tokens(mut self) -> Vec<token::Token> {
        // probably worthwhile to swap this out to a simple None check?
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(token::Token::new(
            TokenType::Eof,
            String::from(""),
            Literal::None,
            self.line,
        ));
        return self.tokens;
    }
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Single character tokens
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

            // operators
            Some('!') => {
                let lexeme = if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(lexeme);
            }
            Some('=') => {
                let lexeme = if self.match_char('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(lexeme);
            }
            Some('<') => {
                let lexeme = if self.match_char('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(lexeme);
            }
            Some('>') => {
                let lexeme = if self.match_char('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(lexeme);
            }
            Some('/') => {
                if self.match_char('/') {
                    // A comment goes until the end of the line
                    // Not sure if this is the best way to iterate to end
                    let mut next = match self.char_iter.peek() {
                        Some(&c) => Some(c),
                        None => None,
                    };

                    while next != None && next != Some('\n') {
                        self.advance();
                        next = match self.char_iter.peek() {
                            Some(&c) => Some(c),
                            None => None,
                        };
                    }
                } else if self.match_char('*') {
                    self.block_comment();
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            Some('"') => {
                self.string();
            }
            Some(x) if x.is_whitespace() && x != '\n' => {
                // Ignore whitespace.
                return;
            }
            Some('\n') => {
                self.line += 1;
            }
            Some(x) if x.to_digit(10).is_some() => self.digit(),
            Some(x) if x == '_' || x.is_alphabetic() => self.identifier(),
            None => return,
            _ => (self.error_handler)(self.line, "Unexpected character"),
        }
    }

    fn block_comment(&mut self) {
        let mut nested_count = 1;
        while nested_count > 0 {
            let current = self.advance();
            if current == Some('*') {
                if self.char_iter.peek() == Some(&'/') {
                    self.advance();
                    nested_count -= 1;
                }
            } else if current == Some('/') {
                if self.char_iter.peek() == Some(&'*') {
                    self.advance();
                    nested_count += 1;
                }
            } else if current == None {
                (self.error_handler)(self.line, "Unterminated Block Comment");
                break;
            }
        }
    }
    fn identifier(&mut self) {
        let mut current = self.char_iter.peek();
        while let Some(x) = current {
            if x.is_alphanumeric() || *x == '_' {
                self.advance();
                current = self.char_iter.peek();
            } else {
                break;
            }
        }
        let token_type = match &self
            .source
            .chars()
            .skip(self.start as usize)
            .take(self.current as usize - self.start as usize)
            .collect::<String>() as &str
        {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(token_type);
    }

    fn digit(&mut self) {
        let mut current = self.char_iter.peek();
        let mut decimal_found = false;
        while let Some(x) = current {
            if x.to_digit(10).is_some() {
                self.advance();
                current = self.char_iter.peek();
            } else if *x == '.' && !decimal_found {
                decimal_found = true;
                if let Some(y) = self.source.chars().nth(self.current as usize + 1) {
                    if y.to_digit(10).is_some() {
                        self.advance();
                        current = self.char_iter.peek();
                    }
                }
            } else {
                break;
            }
        }
        self.add_token_with_literal(
            TokenType::Number,
            self.start as usize,
            self.current as usize,
            Literal::Number(
                self.source
                    .chars()
                    .skip(self.start as usize)
                    .take(self.current as usize - self.start as usize)
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap(), // should be safe since I guarantee is a valid float char by char
            ),
        );
    }
    fn string(&mut self) {
        // start at the next character
        let mut current = self.advance();
        while current != None && current != Some('"') {
            if current == Some('\n') {
                self.line += 1
            }
            current = self.advance();
        }

        if current == None {
            (self.error_handler)(self.line, "Unterminated String!")
        } else {
            // Trim the quotes
            self.add_token_with_index_control(
                TokenType::String,
                (self.start + 1) as usize, // BUG: this may cause a panic on 64 bit arch
                (self.current - 1) as usize, // BUG: this may cause a panic
            )
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        let next = self.char_iter.peek();
        match next {
            Some(&c) => {
                if c == expected {
                    self.advance(); // consume the peeked character to create the lexeme
                    return true;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    fn add_token_with_index_control(&mut self, kind: TokenType, start: usize, end: usize) {
        self.add_token_with_literal(kind, start, end, Literal::None);
    }
    fn add_token_with_literal(
        &mut self,
        kind: TokenType,
        start: usize,
        end: usize,
        literal: Literal,
    ) {
        let text = self.source.chars().skip(start).take(end - start).collect();

        self.tokens.push(token::Token::new(
            kind, text, literal, // this needs to change in the near future
            self.line,
        ));
    }
    fn add_token(&mut self, kind: TokenType) {
        self.add_token_with_index_control(kind, self.start as usize, self.current as usize);
    }

    fn advance(&mut self) -> Option<char> {
        let next = self.char_iter.next();

        // only advance current counter if not past end of line
        if next != None {
            self.current += 1;
        };

        return next;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.char_len as u32;
    }
}
