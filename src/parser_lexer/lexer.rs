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
        Colon,
        Invalid,
        Min,
        Plus,
        OpenBrace,
        CloseBrace,
        OpenBracket,
        CloseBracket,
        OpenCurlyBracket,
        CloseCurlyBracket,
        Comment,
        Slash,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub enum KeyWords {
        Let,
        If,
        Else,
        Fn,
        Bool,
        While,
        For,
        Return,
        Number,
        String,
        Use,
        Const,
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
        pub line: usize,
    }

    impl Token {
        pub fn new(token_type: TokenType, value: impl AsRef<str>, line: usize) -> Token {
            return Token {
                token_type,
                value: value.as_ref().to_owned(),
                line,
            };
        }
    }

    pub trait Lexer {
        fn lex(input: String) -> Vec<Token>;
    }

    pub trait Tokenizer {
        fn eq_token(&mut self, l: usize) -> Token;
        fn less_token(&mut self, l: usize) -> Token;
        fn more_token(&mut self, l: usize) -> Token;
        fn string_token(&mut self, l: usize) -> Token;
        fn comment_token(&mut self, l: usize) -> Token;
        fn number_token(&mut self, l: usize) -> Token;
        fn identifier_token(&mut self, l: usize) -> Token;
        fn keyword_token(t: impl AsRef<str>, l: usize) -> Option<Token>;
    }

    impl Tokenizer for Cursor {
        /// My language will not support === cause that is for js
        fn eq_token(&mut self, l: usize) -> Token {
            // If there is no char next it must only be a single =
            // Therefore it is a single equal
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Eq), "=", l);
            };

            let peak = &peak[0..1];
            match peak {
                ['='] => {
                    // Advance the position by one to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::EqEq), "==", l)
                }
                [' '] => Token::new(TokenType::Operator(Operators::Eq), "=", l),
                _ => {
                    return Token::new(TokenType::Operator(Operators::Eq), "=", l);
                }
            }
        }
        fn less_token(&mut self, l: usize) -> Token {
            // If there is no char next it must only be a single <
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::Less), "<", l);
            };

            match peak[0..1] {
                ['='] => {
                    // Advance the position by one to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::LessEq), "<=", l)
                }
                [' '] => Token::new(TokenType::Operator(Operators::Less), "<", l),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid less operator",
                    l,
                ),
            }
        }
        fn more_token(&mut self, l: usize) -> Token {
            // If there is no char next it must only be a single >
            let Some(peak) = self.peak_nth(1) else {
                return Token::new(TokenType::Operator(Operators::More),  ">",l);
            };

            match peak[0..1] {
                ['='] => {
                    // Advance the position by 1 to consume the next char
                    self.advance_pos(1);
                    Token::new(TokenType::Operator(Operators::MoreEq), "==", l)
                }
                [' '] => Token::new(TokenType::Operator(Operators::More), ">", l),
                _ => Token::new(
                    TokenType::Operator(Operators::Invalid(Box::from(peak))),
                    "Invalid more operator",
                    l,
                ),
            }
        }
        /// Returns the token type and the value
        fn string_token(&mut self, l: usize) -> Token {
            let mut string = String::new();
            // Advance until we find a "
            while let Some(char) = self.next() {
                match char {
                    '"' => return Token::new(TokenType::String, string, l),
                    _ => string.push(char),
                }
            }
            // If we get here it means we did not find a closing quote
            // This would be considered an error
            return Token::new(
                TokenType::Invalid,
                "Found a string without a closing quote",
                l,
            );
        }
        /// Returns the comment token type and the comment
        /// Could also just return a slash if the /  isn't followed by two slashes
        fn comment_token(&mut self, l: usize) -> Token {
            let mut comment = String::new();
            let Some(peak_next) = self.peak_nth(1) else {
                return Token::new(TokenType::Slash, "/", l);
            };

            match peak_next[0..1] {
                ['/'] => {
                    self.next();
                    while let Some(char) = self.next() {
                        match char {
                            '\n' => return Token::new(TokenType::Comment, comment, l),
                            _ => comment.push(char),
                        };
                    }
                    return Token::new(TokenType::Comment, comment, l);
                }
                _ => {
                    return Token::new(TokenType::Slash, "/", l);
                }
            }
        }
        /// Returns the token type and the value
        fn number_token(&mut self, l: usize) -> Token {
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
                return Token::new(TokenType::Number, number, l);
            }
            // Return a invalid token if we did not find a space, this would mean the number is
            // going on forever/ the file ended
            return Token::new(TokenType::Invalid, "Found a non ending number", l);
        }

        // Matches the identifier to a keyword
        // If it is not a keyword it returns the identifier otherwise it returns the keyword
        fn identifier_token(&mut self, l: usize) -> Token {
            // The identifier is the value that gets assigned to a variable
            let mut identifier = String::new();

            // Push the current char to the identifier
            let prev = self.previous().unwrap();
            identifier.push(prev);

            while let Some(char) = self.next() {
                match char {
                    ' ' => {
                        if let Some(token) = Self::keyword_token(&identifier, l) {
                            return token;
                        }
                        return Token::new(TokenType::Identifier, identifier, l);
                    }
                    '(' | ')' | '{' | '}' | '[' | ']' | '.' | ',' | '=' | '\n' | ';' => {
                        self.advance_back(1);
                        if let Some(token) = Self::keyword_token(&identifier, l) {
                            return token;
                        }
                        return Token::new(TokenType::Identifier, identifier, l);
                    }
                    _ => identifier.push(char),
                }
            }

            // Return a invalid token if we did not find a space, this would mean the identifier is
            // going on forever/the file ended
            return Token::new(TokenType::Invalid, "Found a non ending identifier", l);
        }

        /// Returns a token if the token is in the existing field of tokens
        fn keyword_token(t: impl AsRef<str>, l: usize) -> Option<Token> {
            match t.as_ref() {
                "let" => Some(Token::new(TokenType::Keyword(KeyWords::Let), "let", l)),
                "fn" => Some(Token::new(TokenType::Keyword(KeyWords::Fn), "fn", l)),
                "if" => Some(Token::new(TokenType::Keyword(KeyWords::If), "if", l)),
                "else" => Some(Token::new(TokenType::Keyword(KeyWords::Else), "else", l)),
                "while" => Some(Token::new(TokenType::Keyword(KeyWords::While), "while", l)),
                "for" => Some(Token::new(TokenType::Keyword(KeyWords::For), "for", l)),
                "number" => Some(Token::new(
                    TokenType::Keyword(KeyWords::Number),
                    "number",
                    l,
                )),
                "string" => Some(Token::new(
                    TokenType::Keyword(KeyWords::String),
                    "string",
                    l,
                )),
                "return" => Some(Token::new(
                    TokenType::Keyword(KeyWords::Return),
                    "return",
                    l,
                )),
                "use" => Some(Token::new(TokenType::Keyword(KeyWords::Use), "use", l)),
                "const" => Some(Token::new(TokenType::Keyword(KeyWords::Const), "const", l)),
                value if value == "true" || value == "false" => {
                    Some(Token::new(TokenType::Keyword(KeyWords::Bool), value, l))
                }
                _ => None,
            }
        }
    }

    impl Lexer for Token {
        fn lex(input: String) -> Vec<Token> {
            let mut vec = Vec::new();
            let mut cursor = Cursor::new(input);
            let mut line = 1;
            while let Some(token) = cursor.next() {
                match token {
                    '\n' => {
                        line += 1;
                        continue;
                    }
                    ' ' | '\t' => continue,
                    ',' => vec.push(Token::new(TokenType::Comma, ",", line)),
                    '+' => vec.push(Token::new(TokenType::Plus, "+", line)),
                    '-' => vec.push(Token::new(TokenType::Min, "-", line)),
                    '/' => vec.push(cursor.comment_token(line)),
                    '.' => vec.push(Token::new(TokenType::Dot, ".", line)),
                    '}' => vec.push(Token::new(TokenType::CloseCurlyBracket, "}", line)),
                    '{' => vec.push(Token::new(TokenType::OpenCurlyBracket, "{", line)),
                    '(' => vec.push(Token::new(TokenType::OpenBrace, "(", line)),
                    ')' => vec.push(Token::new(TokenType::CloseBrace, ")", line)),
                    ':' => vec.push(Token::new(TokenType::Colon, ":", line)),
                    '=' => vec.push(cursor.eq_token(line)),
                    '>' => vec.push(cursor.more_token(line)),
                    '<' => vec.push(cursor.less_token(line)),
                    '0'..='9' => vec.push(cursor.number_token(line)),
                    '"' => vec.push(cursor.string_token(line)),
                    '[' => vec.push(Token::new(TokenType::OpenBracket, "[", line)),
                    ']' => vec.push(Token::new(TokenType::CloseBracket, "]", line)),
                    ';' => vec.push(Token::new(TokenType::SemiColon, ";", line)),
                    'A'..='Z' | 'a'..='z' => vec.push(cursor.identifier_token(line)),
                    _ => vec.push(Token::new(TokenType::Invalid, "Invalid token", line)),
                }
            }
            return vec;
        }
    }
}
