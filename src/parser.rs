use crate::lexer::lexer::{Token, TokenType};

/// Parser struct
///
/// Consumes the stream of tokens.
/// It is used for turning the tokens into a ast
pub struct Parser {
    pub current_position: usize,
    pub tokens: Vec<Token>,
    pub prev_token: Option<Token>,
}

/// Using the Iterator trait for the parser
/// It will allow for easy iteration over the tokens
impl Iterator for Parser {
    type Item = Token;
    /// Next for the Parser struct
    ///
    /// It will increase the current_position + 1
    ///
    /// Therefore consuming the token.
    ///
    /// ``` Rust
    /// let token = Token::new(tokens);
    ///
    /// while let Some(token) = token.next() {
    ///     // Do stuf with the token..
    ///     println!("token!!!! {:#?}", token);
    /// }
    ///
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.tokens.get(self.current_position)?;
        self.prev_token = Some(next.clone());
        self.current_position += 1;
        next.clone().into()
    }
}

/// # WalkParser
///
/// The walkparser trait, it works a lot like the cursor: [cursor.rs](https://github.com/Faouzi1406/lepa/blob/main/src/cursor.rs),
/// It allows for advancing back over the tokens, peak the next token, peak n amount of tokens, peak token at position I.
pub trait WalkParser {
    /// Peak a token at a certaint position
    /// Peaks the token at current_position + n
    ///
    /// # Example
    ///
    /// ```rust
    ///  use lepa::lexer::lexer::{Token, Lexer, TokenType};
    ///  use lepa::parser::Parser;
    ///  use lepa::parser::WalkParser;
    ///
    ///  let mut parser = Parser {
    ///     current_position:0,
    ///     tokens: vec![Token { value: "".into(), token_type:TokenType::OpenBrace, line:0 }, Token { value: "".into(), token_type:TokenType::OpenBrace, line:0 }, Token { value: "".into(), token_type:TokenType::OpenBrace, line:0 }],
    ///     prev_token:None
    ///  };
    ///  let token:Option<Token> = parser.peak_nth(2);
    ///  assert_eq!(token, Some(Token { value:"".into(),  token_type:TokenType::OpenBrace, line:0}));
    /// ```
    ///
    /// **This wont advance the current_position therefore not "consuming" the tokens**
    fn peak_nth(&mut self, i: usize) -> Option<Token>;
    /// Peaks multiple tokens
    /// Peaks the tokens from current_position + n
    ///
    /// # Example
    ///
    /// ``` rust
    ///  use lepa::lexer::lexer::{Token, Lexer, TokenType, KeyWords};
    ///  use lepa::parser::Parser;
    ///  use lepa::parser::WalkParser;
    ///
    ///  let mut parser = Token::lex(include_str!("../sample_code/main.lp").to_string());
    ///  let mut parser:Parser = Parser {
    ///  current_position:0,
    ///  tokens:parser,
    ///  prev_token:None
    ///  };
    ///  let token:Option<Vec<Token>> = parser.peak_nth_all(2);
    ///  assert_eq!(token, Some(vec![Token { token_type: TokenType::Keyword(KeyWords::Let), value: "let".into(), line: 0 }, Token { token_type: TokenType::Identifier, value: "main".into(), line: 0 }]));
    ///  
    /// ```
    ///
    /// **This wont advance the current_position therefore not "consuming" the tokens**
    fn peak_nth_all(&mut self, n: usize) -> Option<Vec<Token>>;
    /// Retrieves tokens up until a certaint tokentype
    /// It will advance the current_posisition up until that token and return the tokens found up
    /// until that token, if the token is never found it returns None.
    ///
    /// # Example
    ///
    /// ``` rust
    ///  use lepa::lexer::lexer::{Token, Lexer, TokenType, KeyWords};
    ///  use lepa::parser::Parser;
    ///  use lepa::parser::WalkParser;
    ///
    ///  let mut parser = Token::lex(include_str!("../sample_code/main.lp").to_string());
    ///  let mut parser:Parser = Parser {
    ///  current_position:0,
    ///  tokens:parser,
    ///  prev_token:None
    ///  };
    ///  let token:Option<Vec<Token>> = parser.up_until_token(TokenType::OpenBrace);
    /// ```
    ///
    /// **This will advance the current_position therefore not "consume" the tokens**
    fn up_until_token(&mut self, token: TokenType) -> Option<Vec<Token>>;
    /// Advace back the current position, alows walking back into the token stream.
    ///
    /// # Example
    ///
    /// ```Rust
    /// let mut parser = Token::lex(include_str!("../sample_code/main.lp").to_string());
    ///
    /// // We move the cursor one position up and get the token
    /// let next:Option<Token> = parser.next();
    /// match next {
    ///     Some(type) => {
    ///         // do stuff with the token type
    ///         println!("token wow {:#?}", type);
    ///         // Move the cursor back by one therefore not consuming the token
    ///         self.advance_back(1);
    ///     }
    ///     None => {}
    ///}
    /// ```
    ///
    /// **This function will panic if you try to advance the token back to a negative number since
    /// n is a usize and current_position is a usize**
    ///
    fn advance_back(&mut self, n: usize);
}

impl WalkParser for Parser {
    fn peak_nth(&mut self, i: usize) -> Option<Token> {
        Some(self.tokens.get(self.current_position + i)?.clone())
    }
    fn peak_nth_all(&mut self, n: usize) -> Option<Vec<Token>> {
        return Some(self.tokens[self.current_position..(self.current_position + n)].to_vec());
    }
    fn advance_back(&mut self, n: usize) {
        self.current_position -= n;
    }
    fn up_until_token(&mut self, t: TokenType) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next() {
            match token {
                token => {
                    if token.token_type == t {
                        tokens.push(token);
                        return Some(tokens);
                    }
                    tokens.push(token);
                }
            }
        }
        return None;
    }
}
