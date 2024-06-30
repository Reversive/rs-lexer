pub mod lexer;
mod token;

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Lexer;

    #[test]
    fn token_quantity() {
        let source = String::from("(((");
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.scan_tokens();
        assert_eq!(tokens.len(), 4);
    }

    #[test]
    fn check_braces() {
        let source = String::from("(((");
        let mut lexer = Lexer::new(&source);
        let tokens = lexer.scan_tokens();
        assert_eq!(tokens.get(0).unwrap().lexeme(), "(");
        assert_eq!(tokens.get(1).unwrap().lexeme(), "(");
        assert_eq!(tokens.get(2).unwrap().lexeme(), "(");
    }
}
