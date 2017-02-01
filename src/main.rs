mod lexer;

fn main() {
    let tokens = lexer::tokenize("12 3+4+ -843\n");
    println!("Tokens: {:?}", tokens)
}

#[cfg(test)]
mod tests {
    use super::lexer;
    use super::lexer::Token::*;

    #[test]
    fn all_whitespace() {
        assert!(lexer::tokenize("   ").is_empty());
    }

    #[test]
    fn parse_expr(){
        let parsed = lexer::tokenize("1 + 24 - 16");
        let expected = vec![Integer(1), PlusSign, Integer(24), MinusSign, Integer(16)];
        assert_eq!(parsed.as_slice(), expected.as_slice());
    }

    #[test]
    fn empty_input(){
        let parsed = lexer::tokenize("");
        let expected = vec![];
        assert_eq!(parsed.as_slice(), expected.as_slice());
    }

    #[test]
    fn parse_bignum(){
        let parsed = lexer::tokenize("98765432100");
        let expected = vec![Integer(98765432100)];
        assert_eq!(parsed.as_slice(), expected.as_slice());
    }
}
