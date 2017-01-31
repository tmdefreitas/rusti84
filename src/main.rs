fn main() {
    let tokens = lexer::tokenize("12 3+4+ -843\n");
    println!("Tokens: {:?}", tokens)
}

mod lexer {

    //Valid Tokens
    #[derive(Debug)]
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
                let mut int_value = ch.to_digit(10).expect("is_digit but not to_digit!");
                loop {
                    match chrs.peek() {
                        None => break,
                        Some(c) => {
                            // Check to see if the next symbol is a digit, if it is, keep going
                            match c.to_digit(10) {
                                Some(d) => int_value = int_value * 10 + d,
                                None => break
                            }
                        }
                    };
                    //Call next to consume the char
                    chrs.next();
                };
                //Digit is done, push integer as token
                tokens.push(Token::Integer(int_value as i64));
            }

        }

        return tokens;

    }

}
