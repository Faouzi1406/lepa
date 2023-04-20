pub mod cursor;

pub mod lexer {
    use crate::cursor::cursor::{Cursor, CursorIter};

    /// The tokeentypes that are used in the lexer
    pub enum TokenType {
        Identifier,
        Number,
        String,
        Operator(Operators),
        Keyword,
        Dot,
        Space,
        SemiColon,
        Invalid,
    }

    /// All the operators
    pub enum Operators {
        EqEq,
        Eq,
        Less,
        LessEq,
        More,
        MoreEq,
        Invalid(Box<[char]>),
    }

    pub struct Token {
        pub token_type: TokenType,
        pub value: String,
    }

    impl Token {
        pub fn new(token_type: TokenType, value: impl AsRef<str>) -> Token {
            return Token { token_type, value:value.as_ref().to_owned() };
        }
    }

    trait Lexer {
        fn lex(input: String) -> Vec<Token>;
    }

    pub trait Tokenizer {
        fn eq_token(&mut self) -> Token;
        fn less_token(&mut self) -> Token;
        fn more_token(&mut self) -> Token;
        fn string_token(&mut self) -> Token;
        fn number_token(&mut self) -> Token;
    }

    impl Tokenizer for Cursor {
        /// My language will not support === cause that is for js
        fn eq_token(&mut self) -> Token {
            // If there is no char next it must only be a single =
            // Therefore it is a single equal
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Eq), "=");
            };

            // Advance the position by one to consume the next char
            self.advance_pos(1);
            match peak {
                &['='] => Token::new(TokenType::Operator(Operators::EqEq), "=="),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid equal operator",
                ),
            }
        }
        fn less_token(&mut self) -> Token {
            // If there is no char next it must only be a single <
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Less), "<");
            };

            // Advance the position by one to consume the next char
            self.advance_pos(1);
            match peak {
                &['='] => Token::new(TokenType::Operator(Operators::LessEq), "<="),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid less than operator",
                ),
            }
        }
        fn more_token(&mut self) -> Token {
            // If there is no char next it must only be a single >
            let Some(peak) = &self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::More),  ">");
            };

            // Advance the position by 1 to consume the next char
            self.advance_pos(1);
            match peak {
                &['='] => Token::new(TokenType::Operator(Operators::MoreEq), "=="),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(*peak))),
                    "Invalid more than operator",
                ),
            }
        }
        /// Returns the token type and the value
        fn string_token(&mut self) -> Token {
            // If there is no char next it must only be a single >
            let mut string = String::new();
            // Advance until we find a "
            while let Some(char) = self.next() {
                match char {
                    '"' => return Token::new(TokenType::String, string),
                    _ => string.push(char),
                }
            }
            // If we get here it means we did not find a closing quote
            // This would be considered an error
            return Token::new(TokenType::Invalid, "Found a string without a closing quote");
        }
        /// Returns the token type and the value
        fn number_token(&mut self) -> Token {
            let mut number = String::new();
            // We advance the cursor until we found a space which means there are no more numbers
            // related to this number
            while let Some(char) = self.next() {
                match char {
                    ' ' => return Token::new(TokenType::String, number),
                    _ => number.push(char),
                }
            }
            // Return a invalid token if we did not find a space, this would mean the number is
            // going on forever/ the file ended
            return Token::new(TokenType::Invalid, "Found a non ending number");
        }
    }

    impl Lexer for TokenType {
        fn lex(input: String) -> Vec<Token> {
            let mut vec = Vec::new();
            let mut cursor = Cursor::new(input);
            while let Some(token) = cursor.next() {
                match token {
                    '=' => vec.push(cursor.eq_token()),
                    '>' => vec.push(cursor.more_token()),
                    '<' => vec.push(cursor.less_token()),
                }
            }
            return vec;
        }
    }
}
