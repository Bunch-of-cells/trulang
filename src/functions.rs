use std::fmt;

use crate::{
    lexer::{Token, TokenType},
    parser::Node,
};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFunction {
    name: Token,
    params: Vec<(Type, Token)>,
    ret: Type,
    body: Vec<Node>,
}

impl UserDefinedFunction {
    pub fn new(name: Token, params: Vec<(Type, Token)>, ret: Type, body: Vec<Node>) -> Self {
        Self {
            name,
            params,
            ret,
            body,
        }
    }

    pub fn params(&self) -> &[(Type, Token)] {
        &self.params
    }

    pub fn ret(&self) -> &Type {
        &self.ret
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn body(&self) -> &Vec<Node> {
        &self.body
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltInFunction {
    name: &'static str,
    params: &'static [Type],
    ret: Type,
}

impl BuiltInFunction {
    pub const fn new(name: &'static str, params: &'static [Type], ret: Type) -> Self {
        Self { name, params, ret }
    }

    pub const fn params(&self) -> &[Type] {
        self.params
    }

    pub const fn ret(&self) -> &Type {
        &self.ret
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    None,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Type {
    pub fn from_token(token: &Token) -> Option<Type> {
        match **token {
            TokenType::Keyword(ref s) => match s.as_str() {
                "Int" => Some(Type::Number),
                _ => None,
            },
            _ => panic!("Type::from_token: invalid token"),
        }
    }
}
