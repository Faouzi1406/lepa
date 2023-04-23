use crate::{
    ast::{Ast, Type},
    lexer::lexer::{Token, TokenType},
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
        let ast = Ast::new(Type::Program);
        ast
    }

    fn parse_variable(&mut self) -> Ast {
        let ast = Ast::new(Type::Variable(crate::ast::Variable {
            name: String::new(),
            value: String::new(),
        }));

        let mut current = ParseVarType::Id;
        while let Some(token) = self.next() {
            match token.token_type {
                TokenType::Identifier => {
                    if current == ParseVarType::Id {
                        current = ParseVarType::Value;
                        match ast.type_ {
                            Type::Variable(var) => {}
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        ast
    }
}
