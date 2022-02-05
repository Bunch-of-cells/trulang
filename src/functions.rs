use std::fmt;

use crate::{node::Node, token::Token};

#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedFunction {
    params: Vec<(Type, Token)>,
    ret: Type,
    body: Vec<Node>,
}

impl UserDefinedFunction {
    pub fn new(params: Vec<(Type, Token)>, ret: Type, body: Vec<Node>) -> Self {
        Self { params, ret, body }
    }

    pub fn params(&self) -> &[(Type, Token)] {
        &self.params
    }

    pub fn ret(&self) -> &Type {
        &self.ret
    }

    pub fn body(&self) -> &Vec<Node> {
        &self.body
    }
}

impl fmt::Display for UserDefinedFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
    Function(Vec<Type>, Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
