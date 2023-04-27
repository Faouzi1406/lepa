use crate::{
    ast::{Ast, Type, TypeVar, VarBuilder, Variable},
    errors::{
        error::{BuildError, ErrorBuilder},
        error_messages::non_ending_variable,
    },
    lexer::lexer::{KeyWords, Operators, Token, TokenType},
};

/// Parser struct
///
/// Consumes the stream of tokens.
/// It is used for turning the tokens into a ast
pub struct Parser {
    pub current_position: usize,
    pub tokens: Vec<Token>,
    pub prev_token: Option<Token>,
}

impl Parser {
    /// Create a new parser struct.
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            current_position: 0,
            tokens,
            prev_token: None,
        }
    }
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
    /// **This will advance the current_position therefore "consume" the tokens**
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
        return Some(self.tokens.get(self.current_position + i)?.clone());
    }
    fn peak_nth_all(&mut self, n: usize) -> Option<Vec<Token>> {
        return Some(self.tokens[self.current_position..(self.current_position + n)].to_vec());
    }
    fn advance_back(&mut self, n: usize) {
        self.current_position -= n;
        self.prev_token = Some(self.tokens[self.current_position].clone());
    }
    fn up_until_token(&mut self, t: TokenType) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next() {
            match token {
                token if token.token_type == t => {
                    tokens.push(token);
                    return Some(tokens);
                }
                token => {
                    tokens.push(token);
                }
            }
        }
        return None;
    }
}

/// The parse trait, it uses the parser struct to parse the tokens into a [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
pub trait Parse {
    /// The main parsing function.
    ///
    /// It converts the tokens into a ast
    fn parse(&mut self) -> Result<Ast, ErrorBuilder>;
}

trait ParseTokens {
    fn parse_var(&mut self) -> Result<Variable, ErrorBuilder>;
    fn parse_block(&mut self) -> Result<Ast, ErrorBuilder>;
}

impl Parse for Parser {
    fn parse(&mut self) -> Result<Ast, ErrorBuilder> {
        let mut ast = Ast::new(Type::Program);
        while let Some(token) = self.next() {
            match token.token_type {
                // Parsing variables starting with let
                TokenType::Keyword(KeyWords::Let) => {
                    let var = self.parse_var()?;
                    ast.body.push(Ast::new(Type::Variable(var)));
                }
                TokenType::OpenBrace => {
                    ast.body.push(self.parse_block()?);
                }
                _ => todo!("Haven't added parsing for these tokens yet"),
            }
        }
        return Ok(ast);
    }
}

impl ParseTokens for Parser {
    fn parse_var(&mut self) -> Result<Variable, ErrorBuilder> {
        // This function would never be called before there is a prev_token therefore we can unwrap
        // it
        let prev = self.prev_token.clone().unwrap();
        let mut var = Variable::new();
        // Assigning the line of the variable early so It can be used for errors.
        var.line(prev.line);
        // Retrieve all the tokens up untile the semicolon.
        // Considering the end of every variable must be a SemiColon
        let end_of_var = self.up_until_token(TokenType::SemiColon);
        match end_of_var {
            Some(tokens) => {
                for token in tokens {
                    match token.token_type {
                        TokenType::Identifier => {
                            // We assign the name and use the question mark operator which will
                            // force returning the assign error if there is one.
                            var.name(token.value)?;
                        }
                        TokenType::String => {
                            // We assign the type and use the question mark operator which will
                            // force returning the assign error if there is one.
                            var.type_(TypeVar::String(token.value))?;
                        }
                        TokenType::Number => {
                            // We assign the type and use the question mark operator which will
                            // force returning the assign error if there is one.
                            var.type_(TypeVar::parse_number(token.value))?;
                        }
                        TokenType::Operator(Operators::Eq) => {
                            // Ofcourse we shouldn't  just think that it wil always be as string or
                            // a number for now we do but this will change
                        }
                        TokenType::SemiColon => {
                            return Ok(var);
                        }
                        // If we find any token that shouldn't be there we return and error
                        _ => {
                            return Err(ErrorBuilder::new()
                                .message(format!(
                                    "Invalid syntax found {} while parsing variable",
                                    token.value
                                ))
                                .line(token.line)
                                .file_name("todo:")
                                .build_error())
                        }
                    }
                }
                return Ok(var);
            }
            None => Err(non_ending_variable(prev.value, prev.line)),
        }
    }

    fn parse_block(&mut self) -> Result<Ast, ErrorBuilder> {
        let mut ast = Ast::new(Type::Block);
        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Keyword(KeyWords::Let) => {
                    let ast_var = Ast::new(Type::Variable(self.parse_var()?));
                    ast.body.push(ast_var);
                }
                TokenType::OpenBrace =>  {
                    // Recursion
                    //
                    // Imagine me writing a entire loop right here that does the exact same
                    // as what this function is doing.... pleass don't ever do that. :)
                    ast.body.push(self.parse_block()?);
                }
                TokenType::CloseBrace =>  {
                    return Ok(ast);
                }
                _ => todo!("Add parsing for these tokens")
            }
        }
        Ok(ast)
    }
}
