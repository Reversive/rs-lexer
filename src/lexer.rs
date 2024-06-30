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

    fn scan_token(&mut self) {
        let current_char = self.next_char();
        match current_char {
            '(' => self.add_token(TokenType::LeftBrace),
            _ => (),
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
