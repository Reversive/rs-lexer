pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    String,
    Number,
    EOF,
}

pub enum Literal<'a> {
    S(&'a str),
    N(u32),
    F(f32),
}

pub struct Token<'a> {
    token_type: TokenType,
    lexeme: &'a str,
    line: usize,
    literal: Literal<'a>,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal: Literal::N(0),
        }
    }

    pub fn new_literal(
        token_type: TokenType,
        lexeme: &'a str,
        line: usize,
        literal: Literal<'a>,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            literal,
        }
    }

    pub fn lexeme(&self) -> &'a str {
        self.lexeme
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn literal(&self) -> &Literal {
        &self.literal
    }
}
