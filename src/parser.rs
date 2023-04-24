use crate::lexer::lexer::Token;

/// Parser struct
///
/// Consumes the stream of tokens.
/// It is used for turning the tokens into a ast
pub struct Parser {
    current_position: usize,
    tokens: Vec<Token>,
    prev_token: Option<Token>,
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
    ///  let parser = Parser {
    ///     current_position:0,
    ///     tokens: vec![..,.., Token { .., token_type:TokenType::OpenBrace }],
    ///     prev_token:None
    ///  }
    ///  let token:Option<Token> = parser.peak_nth(2);
    ///  assert_eq!(token, Some(Token{ .., TokenType::OpenBrace}));
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
    ///
    ///  let parser = Parser {
    ///     current_position: 0,
    ///     tokens: vec![Token {  token_type:TokenType::Identifier }, Token {  token_type:TokenType::OpenBrace}, Token { .., token_type:TokenType::CloseBrace }],
    ///     prev_token:None
    ///  }
    ///  let token:Option<Token> = parser.peak_nth(2);
    ///  assert_eq!(token, Some(vec![ Token{.. , TokenType::Identifier }, Token{.. , TokenType::OpenBrace} ]));
    ///  
    /// ```
    ///
    /// **This wont advance the current_position therefore not "consuming" the tokens**
    fn peak_nth_all(&mut self, n: usize) -> Option<Vec<Token>>;
    /// Advace back the current position, alows walking back into the token stream.
    ///
    /// # Example
    ///
    /// ```Rust
    /// let parser = Parser::new(tokens);
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
    /// // Turns out we need to 
    /// ```
    ///
    /// **This function will panic if you try to advance the token back to a negative number since
    /// n is a usize and current_position is a usize**
    /// 
    fn advance_back(&mut self, n:usize);
}

impl WalkParser for Parser {
    fn peak_nth(&mut self, i: usize) -> Option<Token> {
        Some(self.tokens.get(self.current_position + i)?.clone())
    }
    fn peak_nth_all(&mut self, n: usize) -> Option<Vec<Token>> {
        return Some(self.tokens[self.current_position..(self.current_position + n)].to_vec());
    }
    fn advance_back(&mut self, n:usize) {
        self.current_position -= n;
    }
}
