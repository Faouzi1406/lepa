pub mod lexer {
    use crate::cursor::cursor::{Cursor, CursorIter};

    /// The tokeentypes that are used in the lexer
    #[derive(Debug, PartialEq, Clone)]
    pub enum TokenType {
        Identifier,
        Number,
        String,
        Operator(Operators),
        Keyword(KeyWords),
        Dot,
        Comma,
        Space,
        SemiColon,
        Invalid,
        Min,
        Plush,
        OpenBrace,
        CloseBrace,
        OpenBracket,
        CloseBracket,
        OpenCurlyBracket,
        CloseCurlyBracket,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum KeyWords {
        Let,
        If,
        Else,
        Fn,
        Bool,
    }

    /// All the operators
    #[derive(Debug, PartialEq, Clone)]
    pub enum Operators {
        EqEq,
        Eq,
        Less,
        LessEq,
        More,
        MoreEq,
        Invalid(Box<[char]>),
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Token {
        pub token_type: TokenType,
        pub value: String,
    }

    impl Token {
        pub fn new(token_type: TokenType, value: impl AsRef<str>) -> Token {
            return Token {
                token_type,
                value: value.as_ref().to_owned(),
            };
        }
    }

    pub trait Lexer {
        fn lex(input: String) -> Vec<Token>;
    }

    pub trait Tokenizer {
        fn eq_token(&mut self) -> Token;
        fn less_token(&mut self) -> Token;
        fn more_token(&mut self) -> Token;
        fn string_token(&mut self) -> Token;
        fn number_token(&mut self) -> Token;
        fn identifier_token(&mut self) -> Token;
        fn keyword_token(t: impl AsRef<str>) -> Option<Token>;
    }

    impl Tokenizer for Cursor {
        /// My language will not support === cause that is for js
        fn eq_token(&mut self) -> Token {
            // If there is no char next it must only be a single =
            // Therefore it is a single equal
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Eq), "=");
            };

            let peak = &peak[0..1];
            println!("{:#?}", peak);
            match peak {
                ['='] => {
                    // Advance the position by one to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::EqEq), "==")
                }
                [' '] => Token::new(TokenType::Operator(Operators::Eq), "="),
                _ => {
                    return Token::new(TokenType::Operator(Operators::Eq), "=");
                }
            }
        }
        fn less_token(&mut self) -> Token {
            // If there is no char next it must only be a single <
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Less), "<");
            };

            match peak[0..1] {
                ['='] => {
                    // Advance the position by one to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::LessEq), "<=")
                }
                [' '] => Token::new(TokenType::Operator(Operators::Less), "<"),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid less operator",
                ),
            }
        }
        fn more_token(&mut self) -> Token {
            // If there is no char next it must only be a single >
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::More),  ">");
            };

            match peak[0..1] {
                ['='] => {
                    // Advance the position by 1 to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::MoreEq), "==")
                }
                [' '] => Token::new(TokenType::Operator(Operators::More), ">"),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid more operator",
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

            // This should techicly never fail
            let prev = self.previous().unwrap();
            number.push(prev);

            while let Some(char) = self.next() {
                if char.is_numeric() {
                    number.push(char);
                    continue;
                }
                self.advance_back(1);
                return Token::new(TokenType::Number, number);
            }
            // Return a invalid token if we did not find a space, this would mean the number is
            // going on forever/ the file ended
            return Token::new(TokenType::Invalid, "Found a non ending number");
        }

        // Matches the identifier to a keyword
        // If it is not a keyword it returns the identifier otherwise it returns the keyword
        fn identifier_token(&mut self) -> Token {
            // The identifier is the value that gets assigned to a variable
            let mut identifier = String::new();

            // Push the current char to the identifier
            let prev = self.previous().unwrap();
            identifier.push(prev);

            while let Some(char) = self.next() {
                match char {
                    ' ' => {
                        if let Some(token) = Self::keyword_token(&identifier) {
                            return token;
                        }
                        return Token::new(TokenType::Identifier, identifier);
                    }
                    '(' | ')' | '{' | '}' | '[' | ']' | '.' | ',' | '=' | '\n' => {
                        self.advance_back(1);
                        if let Some(token) = Self::keyword_token(&identifier) {
                            return token;
                        }
                        return Token::new(TokenType::Identifier, identifier);
                    }
                    _ => identifier.push(char),
                }
            }

            // Return a invalid token if we did not find a space, this would mean the identifier is
            // going on forever/the file ended
            return Token::new(TokenType::Invalid, "Found a non ending identifier");
        }

        /// Returns a token if the token is in the existing field of tokens
        fn keyword_token(t: impl AsRef<str>) -> Option<Token> {
            match t.as_ref() {
                "let" => Some(Token::new(TokenType::Keyword(KeyWords::Let), "let")),
                "fn" => Some(Token::new(TokenType::Keyword(KeyWords::Fn), "fn")),
                "if" => Some(Token::new(TokenType::Keyword(KeyWords::If), "if")),
                "else" => Some(Token::new(TokenType::Keyword(KeyWords::Else), "else")),
                value if value == "true" || value == "false" => {
                    Some(Token::new(TokenType::Keyword(KeyWords::Bool), value))
                }
                _ => None,
            }
        }
    }

    impl Lexer for Token {
        fn lex(input: String) -> Vec<Token> {
            let mut vec = Vec::new();
            let mut cursor = Cursor::new(input);
            while let Some(token) = cursor.next() {
                match token {
                    ' ' | '\n' | '\t' => continue,
                    ',' => vec.push(Token::new(TokenType::Comma, ",")),
                    '.' => vec.push(Token::new(TokenType::Dot, ".")),
                    '}' => vec.push(Token::new(TokenType::CloseBrace, "}")),
                    '{' => vec.push(Token::new(TokenType::OpenBrace, "{")),
                    '(' => vec.push(Token::new(TokenType::OpenBrace, "(")),
                    ')' => vec.push(Token::new(TokenType::CloseBrace, ")")),
                    '=' => vec.push(cursor.eq_token()),
                    '>' => vec.push(cursor.more_token()),
                    '<' => vec.push(cursor.less_token()),
                    '0'..='9' => vec.push(cursor.number_token()),
                    '"' => vec.push(cursor.string_token()),
                    '[' => vec.push(Token::new(TokenType::OpenBracket, "[")),
                    ']' => vec.push(Token::new(TokenType::CloseBracket, "]")),
                    ';' => vec.push(Token::new(TokenType::SemiColon, ";")),
                    'A'..='Z' | 'a'..='z' => vec.push(cursor.identifier_token()),
                    _ => vec.push(Token::new(TokenType::Invalid, "Invalid token")),
                }
            }
            return vec;
        }
    }
}
