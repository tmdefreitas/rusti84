fn main() {
    let tokens = lexer::tokenize("12 3+4+ -843\n");
    println!("Tokens: {:?}", tokens)
}

mod lexer {

    //Valid Tokens
    #[derive(Debug, Eq, PartialEq)]
    pub enum Token {
        Integer(i64),
        PlusSign,
        MinusSign
    }

    pub fn tokenize(s: &str) -> Vec<Token> {
        let mut tokens = Vec::<Token>::new();

        //We need a peekable version of chars, so we can look ahead
        // This will be an ll(1) parser since we can peek one token ahead
        let mut chrs = s.chars().peekable();

        loop {
            let ch = match chrs.next() {
                Some(c) => c,
                None => break
            };

            if ch.is_whitespace() {
                continue;
            }

            // Capture operators
            if ch == '+' {
                tokens.push(Token::PlusSign);
            }

            if ch == '-' {
                tokens.push(Token::MinusSign);
            }

            //Capture Integers (Only support base 10)
            //TODO: make this more idiomatic?
            if ch.is_digit(10) {
                let mut int_value = ch.to_digit(10).expect("is_digit but not to_digit!") as i64;
                loop {
                    match chrs.peek() {
                        None => break,
                        Some(c) => {
                            // Check to see if the next symbol is a digit, if it is, keep going
                            match c.to_digit(10) {
                                Some(d) => int_value = int_value * 10 + (d as i64),
                                None => break
                            }
                        }
                    };
                    //Call next to consume the char
                    chrs.next();
                };
                //Digit is done, push integer as token
                tokens.push(Token::Integer(int_value));
            }

        }

        return tokens;

    }

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
