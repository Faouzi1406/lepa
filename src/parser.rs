use crate::{
    ast::{Ast, Type, TypeVar, Variable},
    lexer::lexer::{KeyWords, Operators, Token, TokenType},
};

#[derive(Debug)]
pub struct Parser {
    current_position: usize,
    input: Vec<Token>,
    previous: Option<Token>,
}

impl Iterator for Parser {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.input.get(self.current_position);
        self.current_position += 1;
        self.previous = token.cloned();
        token.cloned()
    }
}

impl Parser {
    pub fn new(input: Vec<Token>) -> Self {
        Parser {
            current_position: 0,
            input,
            previous: None,
        }
    }
    pub fn advance_back(&mut self, n:usize) {
        self.previous = self.input.get(self.current_position - n  - 1).cloned();
        self.current_position -= n;
    }
}

pub trait Parse {
    fn parse(&mut self) -> Result<Ast, String>;
    fn parse_variable(&mut self) -> Result<Ast, String>;
    fn parse_func(&mut self) -> Result<Ast, String>;
    fn parse_args(&mut self) -> Result<Vec<Variable>, String>;
    fn parse_block(&mut self) -> Result<Ast, String>;
}

#[derive(PartialEq)]
pub enum ParseVarType {
    Id,
    Value,
}

impl Parse for Parser {
    fn parse(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new(Type::Program);
        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Keyword(KeyWords::Let) => {
                    let var = self.parse_variable()?;
                    ast.body.push(var);
                }
                TokenType::Identifier => {
                    let func = self.parse_func()?;
                    ast.body.push(func);
                }
                TokenType::Keyword(KeyWords::Fn) => {
                    let func = self.parse_func()?;
                    ast.body.push(func);
                }
                _ => {}
            }
        }
        Ok(ast)
    }

    fn parse_variable(&mut self) -> Result<Ast, String>{
        let mut ast = Ast::new(Type::Variable(crate::ast::Variable {
            name: String::new(),
            type_: TypeVar::None,
        }));

        let mut current = ParseVarType::Id;
        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Identifier => {
                    if current == ParseVarType::Id {
                        current = ParseVarType::Value;
                        ast = Ast::new(Type::Variable(crate::ast::Variable {
                            name: token.value,
                            type_:TypeVar::None
                        }));
                    }
                }
                TokenType::Operator(Operators::Eq) => {
                    current = ParseVarType::Value;
                }
                TokenType::String => {
                    if current == ParseVarType::Value {
                        // Todo: Return a error and not panic
                        let Some(var_name) = ast.var_name() else { 
                            return Err("Couldn't parse variable, found a variable with a value but without a name".to_string());
                        };

                        ast = Ast::new(Type::Variable(crate::ast::Variable {
                            name: var_name,
                            type_: TypeVar::String(token.value),
                        }));
                    }
                }
                TokenType::Number => {
                    if current == ParseVarType::Value {
                        // Todo: Return a error and not panic
                        let Some(var_name) = ast.var_name() else {panic!("Found a variable without a name")};

                        ast = Ast::new(Type::Variable(crate::ast::Variable {
                            name: var_name,
                            type_: TypeVar::parse_number(token.value),
                        }));
                    }
                }
                TokenType::SemiColon => {
                    break;
                }
                _ => {}
            }
        }
        Ok(ast)
    }

    fn parse_func(&mut self) -> Result<Ast, String> {
        // We can unwrap here since there has to be a token for us to even execute parse_func()
         match self.previous.clone().unwrap().token_type {
            TokenType::Keyword(KeyWords::Fn) => {
                let Some(token) = self.next() else {return Err("Found a function without a identifier".into())};
                match token.token_type {
                    TokenType::Identifier => {
                        let Some(has_body) = self.next() else {
                            self.advance_back(1);
                            return Ok(Ast::new(Type::Function(crate::ast::Func { name: token.value, args: self.parse_args()?, body: None })))
                        };
                        match has_body.token_type {
                            TokenType::OpenBrace => { 
                                self.advance_back(1);
                                let args = self.parse_args()?;
                                let parse_block = self.parse_block()?;
                                self.next();
                                return Ok(Ast::new(Type::Function(crate::ast::Func { name: token.value, args, body: Some(Box::from(parse_block)) })))
                            }
                            _ => {
                                return Ok(Ast::new(Type::Function(crate::ast::Func { name: token.value, args: self.parse_args()?, body: None })))
                            }
                        }
                    },
                    _ => return Err("Invalid function sequence.".into())
                }
            }
            TokenType::Identifier => {
                    let Some(has_body) = self.next() else {
                        self.advance_back(1);
                        return Ok(Ast::new(Type::Function(crate::ast::Func { name: self.previous.clone().unwrap().value, args: self.parse_args()?, body: None })));
                    };
                    self.advance_back(1);
                    match has_body.token_type {
                        TokenType::OpenBracket => { 
                            println!("I am being called{:#?}", self);
                            let name =  self.previous.clone().unwrap().value;
                            let args = self.parse_args()?;
                            self.next();
                            let parse_block = self.parse_block()?;
                            return Ok(Ast::new(Type::Function(crate::ast::Func { name, args, body: Some(Box::from(parse_block)) })));
                        }
                        _ => {
                            return Ok(Ast::new(Type::Function(crate::ast::Func { name: self.previous.clone().unwrap().value, args: self.parse_args()?, body: None })));
                        }
                    }
            }
            val => return Err(format!("Invalid function: {:#?}", val))
        };
    }

    fn parse_args(&mut self) -> Result<Vec<Variable>, String> {
        let mut args = Vec::new();
        // This is the state of the args
        // If a ( is not found it will not parse arguments
        // If it is never valid this but however a ) is found this will lead to and error Result 
        // Otherwise all tokens will be consumed since the function was closed but never opened.
        let mut valid = false;
        // The current argument, used to make sure there are komma's inbetween the arguments to
        // make sure parsing the arguments goes correctly
        let mut current:Vec<Token> = Vec::new();
        while let Some(arg) =  self.next() {
            match arg.token_type {
                TokenType::CloseBrace => {
                    if valid  {
                        let Some(curr) = current.pop() else {
                            break;
                        };
                        match curr.token_type {
                            TokenType::Identifier => {
                                args.push(Variable { name: curr.value, type_: TypeVar::None })
                            }
                            TokenType::String => {
                                args.push(Variable { name: "".to_string(), type_: TypeVar::String(curr.value) })
                            }
                            TokenType::Number => {
                                args.push(Variable { name: "".to_string(), type_: TypeVar::parse_number(curr.value) })
                            }
                            _ => return Err(format!("Invalid argument {:#?},", curr))
                        }
                        break;
                    }
                    else {
                        return Err("Function was opened but it was never closed.".into());
                    }
                }
                TokenType::OpenBrace => {
                    if valid == false {
                        valid = true;
                    }else {
                        return Err("Functions inside functions are currently not supported.".into());
                    }
                }
                TokenType::Identifier =>  {
                    if valid {
                        if current.len() > 0 {
                            return Err("Found multiple arguments without comma's inbetween them. ".into())
                        }
                        current.push(arg);
                    }
                }
                TokenType::Comma => {
                    if valid {
                        let Some(curr) = current.pop() else {
                            return Err("Found more comma's than arguments.".into())
                        };
                        match curr.token_type {
                            TokenType::Identifier => {
                                args.push(Variable { name: curr.value, type_: TypeVar::None })
                            }
                            TokenType::String => {
                                args.push(Variable { name: "".to_string(), type_: TypeVar::String(curr.value) })
                            }
                            TokenType::Number => {
                                args.push(Variable { name: "".to_string(), type_: TypeVar::parse_number(curr.value) })
                            }
                            _ => return Err(format!("Invalid argument {:#?},", curr))
                        }
                    }
                }
                TokenType::String => {
                    if valid {
                        if current.len() > 0 {
                            return Err("Found multiple arguments without comma's inbetween them. ".into())
                        }
                        current.push(arg);
                    }
                }
                TokenType::Number => {
                    if valid {
                        if current.len() > 0 {
                            return Err("Found multiple arguments without comma's inbetween them. ".into())
                        }
                        current.push(arg);
                    }
                }
                TokenType::Keyword(KeyWords::Fn) => (),
                _ => ()
            }
        }
        return Ok(args)
    }

    fn parse_block(&mut self) -> Result<Ast, String> {
        let mut ast = Ast::new(Type::Block);  

        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Keyword(KeyWords::Let) => {
                    let var = self.parse_variable()?;
                    ast.body.push(var);
                }
                TokenType::Identifier => {
                    let func = self.parse_func()?;
                    ast.body.push(func);
                }
                TokenType::Keyword(KeyWords::Fn) => {
                    let func = self.parse_func()?;
                    ast.body.push(func);
                }
                TokenType::OpenBrace => {
                    let parse = self.parse_block()?;
                    ast.body.push(parse);
                }
                TokenType::CloseBrace => {
                    return Ok(ast);
                }
                _ => {}
            }
        } 
        return Ok(ast);
    }
}
