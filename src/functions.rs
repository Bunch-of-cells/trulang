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
        write!(
            f,
            "{} ~> [{}]",
            self.params
                .iter()
                .map(|a| format!("[{}]", a.0))
                .collect::<Vec<String>>()
                .join(" "),
            self.ret,
        )
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

#[derive(Debug, Clone)]
pub enum Type {
    Number,
    None,
    Any,
    Bool,
    Function(Vec<Type>, Box<Type>),
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, Type::Any)
            | (Type::Any, _)
            | (Type::Number, Type::Number)
            | (Type::Bool, Type::Bool)
            | (Type::None, Type::None) => true,
            (Type::Function(a, b), Type::Function(c, d)) => {
                if a.len() != c.len() {
                    return false;
                }
                for (a, c) in a.iter().zip(c.iter()) {
                    if !a.eq(c) {
                        return false;
                    }
                }
                b.eq(d)
            }
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Number => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::None => write!(f, "None"),
            Type::Any => write!(f, "?"),
            Type::Function(params, ret) => write!(
                f,
                "{} ~> [{}]",
                params
                    .iter()
                    .map(|a| format!("[{}]", a))
                    .collect::<Vec<String>>()
                    .join(" "),
                ret
            ),
        }
    }
}
