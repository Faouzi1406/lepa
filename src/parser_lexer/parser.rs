use crate::{
    ast::{
        ast::{
            Arg, Ast, Case, Logic, Return, ReturnTypes, Type, TypeVar, TypesArg, VarBuilder,
            Variable,
        },
        function::Func,
        use_::Use,
    },
    errors::{
        error::{BuildError, ErrorBuilder},
        error_messages::{
            invalid_arr_no_end, invalid_function_body_syntax, invalid_function_call,
            invalid_function_syntax_missing_id, invalid_if_statement_body, invalid_return_no_end,
            invalid_use, invalid_var_syntax_token, non_ending_variable,
        },
    },
    parser_lexer::lexer::lexer::{KeyWords, Operators, Token, TokenType},
};

/// Parser struct
///
/// Consumes the stream of tokens.
/// It is used for turning the tokens into a ast
#[derive(Debug)]
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
    ///  use lepa::parser_lexer::lexer::lexer::{Token, Lexer, TokenType};
    ///  use lepa::parser_lexer::parser::Parser;
    ///  use lepa::parser_lexer::parser::WalkParser;
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
    ///  use lepa::parser_lexer::lexer::lexer::{Token, Lexer, TokenType, KeyWords};
    ///  use lepa::parser_lexer::parser::Parser;
    ///  use lepa::parser_lexer::parser::WalkParser;
    ///
    ///  let mut parser = Token::lex(include_str!("../../sample_code/main.lp").to_string());
    ///  let mut parser:Parser = Parser {
    ///  current_position:0,
    ///  tokens:parser,
    ///  prev_token:None
    ///  };
    ///  let token:Option<Vec<Token>> = parser.peak_nth_all(2);
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
    ///  use lepa::parser_lexer::lexer::lexer::{Token, Lexer, TokenType, KeyWords};
    ///  use lepa::parser_lexer::parser::Parser;
    ///  use lepa::parser_lexer::parser::WalkParser;
    ///
    ///  let mut parser = Token::lex(include_str!("../../sample_code/main.lp").to_string());
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

pub trait CaseLogic {
    fn get_case(&mut self) -> Result<Case, ErrorBuilder>;
}

impl CaseLogic for Parser {
    fn get_case(&mut self) -> Result<Case, ErrorBuilder> {
        let mut type_var_1 = TypeVar::None;
        let mut type_var_2 = TypeVar::None;
        let mut case = Case::None;

        let val_1 = self.next().unwrap();
        match val_1.token_type {
            TokenType::Number => {
                type_var_1 =  TypeVar::parse_number(val_1.value);
            }
            TokenType::Identifier => {
                type_var_1 =  TypeVar::Identifier(val_1.value);
            }
            _ => return Err(ErrorBuilder::new()
                .message("The first value of this if statement is either not supported yet or incorrect.")
                .line(val_1.line)
                .file_name("todo")
                .build_error()),
        };

        let case_token = self.next().unwrap();
        match case_token.token_type {
            TokenType::Operator(op) => {
                case = Case::from_op(op)?;
            }
            _ => {
                return Err(ErrorBuilder::new()
                    .message("Pleas replace your current case with a valid if case")
                    .line(val_1.line)
                    .file_name("todo")
                    .build_error())
            }
        };

        let val_2 = self.next().unwrap();
        match val_2.token_type {
            TokenType::Number => {
                type_var_2 =  TypeVar::parse_number(val_2.value);
            }
            TokenType::Identifier => {
                type_var_2 =  TypeVar::Identifier(val_2.value);
            }
            _ => return Err(ErrorBuilder::new()
                .message("The first value of this if statement is either not supported yet or incorrect.")
                .line(val_2.line)
                .file_name("todo")
                .build_error()),
        };

        let assign = case.assign(type_var_1, type_var_2);
        Ok(assign)
    }
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
    /// Parsing variables,
    ///
    /// Currently it parses:
    ///
    /// It doesn't need the let token and expects it to not be there, this is because the main
    /// parse function consumes it.
    ///
    /// It also expects there to be a semicolon at the end of every new variable.
    ///
    /// - some = "wow";
    /// - some = 1;
    ///
    /// It doesn't support:
    ///
    /// - some = some;
    fn parse_var(&mut self) -> Result<Variable, ErrorBuilder>;
    /// Parsing blocks
    ///
    /// Blocks can be considered as anything that starts with a '{' and end withs a '}'.
    /// Nested blocks are also supported.
    ///
    /// # Example
    ///
    /// {
    ///  let hello ="world";
    ///  {
    ///   let number = 1;
    ///  }
    /// }
    fn parse_block(&mut self) -> Result<Ast, ErrorBuilder>;
    /// Parsing functions
    ///
    /// It will parse any valid function:
    ///
    /// # Example
    ///
    ///
    /// fn some(arg, arg) {
    ///  let hello_world = "wow";
    /// }
    ///
    /// fn other() {
    /// let wowo
    /// }
    fn parse_fn(&mut self) -> Result<Ast, ErrorBuilder>;
    /// Parsing arguments
    ///
    /// This could be anything inbetween a OpenBrace and CloseBrace:
    ///
    /// # Examples
    ///
    /// fn some ( arg1, arg2, arg3 )
    ///
    /// some( arg1, arg2, arg3 )
    ///
    /// ( arg1, arg2, arg3 )
    fn parse_args(&mut self) -> Result<Vec<Arg>, ErrorBuilder>;
    /// Parsing arrays
    ///
    /// # Examples
    ///
    /// ```rust
    /// [1,2,3,4,5];
    ///
    /// ["wow", "hello world", "googbye world"];
    /// ```
    fn parse_array(&mut self) -> Result<TypeVar, ErrorBuilder>;
    /// Parsing function calls:
    ///
    /// # Examples
    ///
    /// ```rust
    /// fn hello_world() {
    /// println!("hello world!");
    /// }
    ///
    /// hello_world() // this would be a function call
    /// ```
    fn parse_fn_call(&mut self) -> Result<Ast, ErrorBuilder>;
    /// Parsing returns:
    ///
    /// # Example
    ///
    /// fn goodbye_world(string number) number {
    ///  return number; // It parses this part.
    /// }
    fn parse_return(&mut self) -> Result<Return, ErrorBuilder>;
    fn parse_use(&mut self) -> Result<Use, ErrorBuilder>;
    // Parsing statements
    //
    // # Example
    //
    // ```
    // if 1 == 2 {
    //  println!("?");
    // }
    // else if 1 == 1 {
    //  println!("looks good to me");
    // }
    // else {
    //  println!("?");
    // }
    // ```
    fn parse_statement(&mut self) -> Result<Logic, ErrorBuilder>;
}

impl Parse for Parser {
    fn parse(&mut self) -> Result<Ast, ErrorBuilder> {
        let mut ast = Ast::new(Type::Program);
        while let Some(token) = self.next() {
            match token.token_type {
                // Parsing variables starting with let
                TokenType::Keyword(KeyWords::Use) => {
                    let use_ = Ast::new(Type::Use(self.parse_use()?));
                    ast.body.push(use_);
                }
                TokenType::Keyword(KeyWords::Let) => {
                    let var = self.parse_var()?;
                    ast.body.push(Ast::new(Type::Variable(var)));
                }
                TokenType::Keyword(KeyWords::Fn) => {
                    ast.body.push(self.parse_fn()?);
                }
                TokenType::OpenCurlyBracket => {
                    ast.body.push(self.parse_block()?);
                }
                TokenType::Identifier => {
                    ast.body.push(self.parse_fn_call()?);
                }
                TokenType::Comment => {
                    continue;
                }
                token => todo!("Haven't added parsing for these tokens yet {token:#?}"),
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
                let mut parser = Parser::new(tokens);
                while let Some(token) = parser.next() {
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
                        TokenType::OpenBracket => {
                            var.type_(parser.parse_array()?)?;
                        }
                        TokenType::Operator(Operators::Eq) => {
                            // Ofcourse we shouldn't  just think that it wil always be as string or
                            // a number for now we do but this will change
                        }
                        TokenType::SemiColon => {
                            return Ok(var);
                        }
                        // If we find any token that shouldn't be there we return and error
                        _ => return Err(invalid_var_syntax_token(token)),
                    }
                }
                return Ok(var);
            }
            None => Err(non_ending_variable(prev.value, prev.line)),
        }
    }
    fn parse_block(&mut self) -> Result<Ast, ErrorBuilder> {
        let mut ast = Ast::new(Type::Block);
        let mut line = 0;
        while let Some(token) = self.next() {
            line += token.line;
            match token.token_type {
                TokenType::Keyword(KeyWords::Use) => {
                    let use_ = Ast::new(Type::Use(self.parse_use()?));
                    ast.body.push(use_);
                }
                TokenType::Keyword(KeyWords::Let) => {
                    let ast_var = Ast::new(Type::Variable(self.parse_var()?));
                    ast.body.push(ast_var);
                }
                TokenType::Keyword(KeyWords::Fn) => {
                    ast.body.push(self.parse_fn()?);
                }
                TokenType::Identifier => {
                    ast.body.push(self.parse_fn_call()?);
                }
                TokenType::OpenCurlyBracket => {
                    ast.body.push(self.parse_block()?);
                }
                TokenType::Keyword(KeyWords::Return) => {
                    let return_ = Ast::new(Type::Return(self.parse_return()?));
                    ast.body.push(return_);
                }
                TokenType::Keyword(KeyWords::If) => {
                    let if_ = Ast::new(Type::Logic(self.parse_statement()?));
                    ast.body.push(if_);
                }
                TokenType::CloseCurlyBracket => {
                    return Ok(ast);
                }
                TokenType::Comment => {
                    continue;
                }
                token => todo!("Add parsing for these tokens {:#?}", token),
            }
        }
        return Err(invalid_function_body_syntax("".to_string(), line));
    }
    fn parse_args(&mut self) -> Result<Vec<Arg>, ErrorBuilder> {
        let prev = self.prev_token.clone().unwrap();

        let Some(tokens_until_close) = self.up_until_token(TokenType::CloseBrace) else {
            return Err(invalid_function_syntax_missing_id(prev.line))
        };

        let mut args = Vec::new();
        let mut current_arg = Arg::new();

        for token in tokens_until_close {
            match token.token_type {
                TokenType::Comma => {
                    args.push(current_arg.clone());
                    current_arg.clear_type();
                    current_arg.clear_value();
                }
                TokenType::Keyword(keyword) => {
                    match keyword {
                        KeyWords::Number => {
                            let ass_type = current_arg.assign_type(TypesArg::Number);
                            // Todo: Add better error for this case
                            if ass_type.is_err() {
                                return Err(invalid_var_syntax_token(token));
                            }
                        }

                        KeyWords::String => {
                            let ass_type = current_arg.assign_type(TypesArg::String);
                            if ass_type.is_err() {
                                return Err(invalid_var_syntax_token(token));
                            }
                        }
                        _ => todo!("Add a good error message for this case"),
                    }
                }
                TokenType::Identifier => {
                    let val = current_arg.assign_value(token.value.clone());
                    if current_arg.type_ == TypesArg::None {
                        // Todo: Add better error for this case
                        return Err(invalid_var_syntax_token(token));
                    }
                    if val.is_err() {
                        // Todo: Add better error for this case
                        return Err(invalid_var_syntax_token(token));
                    }
                }
                TokenType::String => {
                    let val = current_arg.assign_value(token.value.clone());
                    if val.is_err() {
                        // Todo: Add better error for this case
                        return Err(invalid_var_syntax_token(token));
                    }
                    let ass_type = current_arg.assign_type(TypesArg::String);
                    if ass_type.is_err() {
                        return Err(invalid_var_syntax_token(token));
                    }
                }
                TokenType::Number => {
                    let val = current_arg.assign_value(token.value.clone());
                    if val.is_err() {
                        // Todo: Add better error for this case
                        return Err(invalid_var_syntax_token(token));
                    }
                    let ass_type = current_arg.assign_type(TypesArg::Number);
                    if ass_type.is_err() {
                        return Err(invalid_var_syntax_token(token));
                    }
                }
                TokenType::CloseBrace => {
                    if current_arg.type_ != TypesArg::None && current_arg.value != "" {
                        args.push(current_arg.clone());
                        current_arg.clear_type();
                        current_arg.clear_value();
                    }
                    return Ok(args);
                }
                TokenType::OpenBrace => {
                    continue;
                }
                TokenType::Comment => {
                    continue;
                }
                // Todo: Invalid argument token error
                _ => {
                    return Err(invalid_function_syntax_missing_id(prev.line));
                }
            }
        }
        return Ok(args);
    }
    fn parse_fn(&mut self) -> Result<Ast, ErrorBuilder> {
        let prev = self.prev_token.clone().unwrap();

        let Some(next) = self.next() else {
            return Err(invalid_function_syntax_missing_id(prev.line));
        };
        if next.token_type != TokenType::Identifier {
            return Err(invalid_function_syntax_missing_id(prev.line));
        }

        let args = self.parse_args()?;
        let Some(body) = self.next() else {
            return Err(invalid_function_syntax_missing_id(prev.line));
        };

        let mut return_type = ReturnTypes::None;

        match body.token_type {
            TokenType::OpenCurlyBracket => {}
            TokenType::Keyword(KeyWords::Number) => {
                return_type = ReturnTypes::Number;
                let Some(next) = self.next() else {
                    return Err(invalid_function_body_syntax(next.value, prev.line));
                };
                if next.token_type != TokenType::OpenCurlyBracket {
                    return Err(invalid_function_body_syntax(next.value, prev.line));
                }
            }
            TokenType::Keyword(KeyWords::String) => {
                return_type = ReturnTypes::String;
                let Some(next) = self.next() else {
                    return Err(invalid_function_body_syntax(next.value, prev.line));
                };
                if next.token_type != TokenType::OpenCurlyBracket {
                    return Err(invalid_function_body_syntax(next.value, prev.line));
                }
            }
            _ => return Err(invalid_function_body_syntax(next.value, prev.line)),
        }

        let body = Some(Box::from(self.parse_block()?));

        let ast = Ast::new(Type::Function(Func {
            name: next.value,
            args,
            body,
            return_type,
        }));
        return Ok(ast);
    }
    fn parse_array(&mut self) -> Result<TypeVar, ErrorBuilder> {
        let mut current_var = String::new();
        let mut values = Vec::new();

        let mut line = 0;
        while let Some(token) = self.next() {
            line = token.line;
            match token.token_type.clone() {
                TokenType::Comma => {
                    let num: Result<i32, _> = current_var.parse();
                    if num.is_ok() {
                        values.push(TypeVar::Number(num.unwrap()));
                        current_var = "".into();
                        continue;
                    }
                    values.push(TypeVar::Number(num.unwrap()));
                    current_var = "".into();
                }
                TokenType::Identifier => {
                    current_var = token.value;
                }
                TokenType::CloseBracket => {
                    return Ok(TypeVar::Arr { values });
                }
                TokenType::Number => {
                    if current_var != "" {
                        return Err(invalid_var_syntax_token(token));
                    }
                    current_var = token.value;
                }
                TokenType::Comment => {
                    continue;
                }
                TokenType::String => {
                    if current_var != "" {
                        return Err(invalid_var_syntax_token(token));
                    }
                    current_var = token.value;
                }
                _ => return Err(invalid_var_syntax_token(token)),
            }
        }
        return Err(invalid_arr_no_end(line));
    }
    fn parse_fn_call(&mut self) -> Result<Ast, ErrorBuilder> {
        let prev = self.prev_token.clone().unwrap();
        let next = self.peak_nth(0);

        match next {
            Some(token) => {
                if token.token_type != TokenType::OpenBrace {
                    return Err(invalid_function_call(prev.value, prev.line));
                }
            }
            None => return Err(invalid_function_call(prev.value, prev.line)),
        }

        let func = Ast::new(Type::FunctionCall(Func {
            name: prev.value.clone(),
            args: self.parse_args()?,
            body: None,
            return_type: ReturnTypes::None,
        }));
        let Some(close) =self.next() else {
            return Err(non_ending_variable(prev.value, prev.line));
        };

        if close.token_type != TokenType::SemiColon {
            return Err(non_ending_variable(prev.value, prev.line));
        }

        return Ok(func);
    }
    fn parse_return(&mut self) -> Result<Return, ErrorBuilder> {
        let prev = &self.prev_token.clone().unwrap();
        let Some(up_until) = &self.up_until_token(TokenType::SemiColon) else {
            return Err(invalid_return_no_end(prev.line));
        };
        for token in up_until {
            match token.token_type {
                TokenType::Number => {
                    return Ok(Return {
                        value: token.value.clone(),
                        type_: ReturnTypes::Number,
                    });
                }
                TokenType::String => {
                    return Ok(Return {
                        value: token.value.clone(),
                        type_: ReturnTypes::String,
                    });
                }
                TokenType::Identifier => {
                    return Ok(Return {
                        value: token.value.clone(),
                        type_: ReturnTypes::Identifier,
                    });
                }
                TokenType::SemiColon => {
                    return Ok(Return {
                        value: "void".to_string(),
                        type_: ReturnTypes::None,
                    });
                }
                _ => todo!("This is currently not supported yet, might get added in the future..."),
            }
        }

        return Err(invalid_return_no_end(prev.line));
    }
    fn parse_statement(&mut self) -> Result<Logic, ErrorBuilder> {
        let prev = self.prev_token.clone().unwrap();
        let case = self.get_case()?;
        let block = self.next().unwrap();
        match block.token_type {
            TokenType::OpenCurlyBracket => {
                let body = &self.parse_block()?;
                let Some(check_else) = self.next() else {
                    return Ok(Logic::new(case,None, body.clone()));
                };
                match check_else.token_type {
                    TokenType::Keyword(KeyWords::Else) => match self.next() {
                        Some(val) => match val.token_type {
                            TokenType::OpenCurlyBracket => {
                                let body = &self.parse_block()?;
                                return Ok(Logic::new(
                                    case,
                                    Some(Box::from(body.clone())),
                                    body.clone(),
                                ));
                            }
                            _ => return Err(invalid_if_statement_body(prev.line)),
                        },
                        _ => return Err(invalid_if_statement_body(prev.line)),
                    },
                    _ => {
                        self.advance_back(1);
                        return Ok(Logic::new(case, None, body.clone()));
                    }
                }
            }
            _ => return Err(invalid_if_statement_body(prev.line)),
        }
    }
    fn parse_use(&mut self) -> Result<Use, ErrorBuilder> {
        let tokens = &self.next();
        let prev = &self.prev_token.clone().unwrap();
        match tokens {
            Some(file) => match file.token_type {
                TokenType::String => {
                    let Some(end_use) = self.next() else {
                                return Err(invalid_use(Some(file.value.clone()), prev.line))
                    };
                    if end_use.token_type != TokenType::SemiColon {
                        return Err(invalid_use(Some(file.value.clone()), prev.line));
                    }
                    return Ok(Use::new(file.value.clone()));
                }
                _ => return Err(invalid_use(None, prev.line)),
            },
            None => return Err(invalid_use(None, prev.line)),
        }
    }
}
