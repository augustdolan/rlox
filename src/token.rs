pub mod token_type;

#[derive(Debug)]
pub struct Literal {}

pub struct Token {
    kind: token_type::TokenType,
    lexeme: String,
    literal: Literal,
    line: u32,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {} {:#?}", self.kind, self.lexeme, self.literal)
    }
}

impl Token {
    pub fn new(kind: token_type::TokenType, lexeme: String, literal: Literal, line: u32) -> Token {
        return Token {
            kind,
            lexeme,
            literal,
            line,
        };
    }
}
