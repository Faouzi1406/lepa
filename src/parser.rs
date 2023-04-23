use crate::{
    ast::{Ast, Type, TypeVar},
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
}

pub trait Parse {
    fn parse(&mut self) -> Ast;
    fn parse_variable(&mut self) -> Ast;
}

#[derive(PartialEq)]
pub enum ParseVarType {
    Id,
    Value,
}

impl Parse for Parser {
    fn parse(&mut self) -> Ast {
        let mut ast = Ast::new(Type::Program);
        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Keyword(KeyWords::Let) => {
                    ast.body.push(self.parse_variable());
                }
                _ => {}
            }
        }
        ast
    }

    fn parse_variable(&mut self) -> Ast {
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
                        let Some(var_name) = ast.var_name() else {panic!("Found a variable without a name")};

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
        ast
    }
}
