use crate::token::Token;
use crate::token::TokenType;

pub struct Lexer<'a> {
    source: &'a String,
    tokens: Vec<Token<'a>>,
    start_lexeme_offset: usize,
    current_lexeme_offset: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn is_at_eof(&self) -> bool {
        self.current_lexeme_offset >= self.source.len()
    }

    pub fn new(source: &'a String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start_lexeme_offset: 0,
            current_lexeme_offset: 0,
            line: 1,
        }
    }

    fn next_char(&mut self) -> char {
        let c = self.source.chars().nth(self.current_lexeme_offset);
        self.current_lexeme_offset += 1;
        c.unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self.source[self.start_lexeme_offset..self.current_lexeme_offset],
            self.line,
        ));
    }

    fn inmediate_next_match(&mut self, expected: char) -> bool {
        if self.is_at_eof() {
            return false;
        }

        if self.source.chars().nth(self.current_lexeme_offset).unwrap() != expected {
            return false;
        }

        self.current_lexeme_offset += 1;
        true
    }

    fn add_token_conditionally(
        &mut self,
        expected: char,
        compound_type: TokenType,
        single_type: TokenType,
    ) {
        let token_type = if self.inmediate_next_match(expected) {
            compound_type
        } else {
            single_type
        };
        self.add_token(token_type);
    }

    fn scan_token(&mut self) {
        let current_char = self.next_char();
        match current_char {
            '(' => self.add_token(TokenType::LeftParenthesis),
            ')' => self.add_token(TokenType::RightParenthesis),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token_conditionally('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_conditionally('=', TokenType::EqualEqual, TokenType::Equal),
            '<' => self.add_token_conditionally('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_token_conditionally('=', TokenType::GreaterEqual, TokenType::Greater),
            _ => println!("Unsupported character at line {}.", self.line),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token<'a>> {
        while !self.is_at_eof() {
            self.start_lexeme_offset = self.current_lexeme_offset;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        return &self.tokens;
    }
}
