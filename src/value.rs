use std::fmt;

use ordered_float::OrderedFloat;

use crate::{
    functions::UserDefinedFunction,
    token::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(OrderedFloat<f64>),
    Function(UserDefinedFunction),
    FuncAccess(Token),
    Bool(bool),
    None,
}

impl Value {
    pub fn from_token(token: &Token) -> Value {
        match **token {
            TokenType::Number(n) => Value::Number(n),
            _ => panic!("Invalid token type for value"),
        }
    }

    pub fn get_number(&self) -> OrderedFloat<f64> {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Invalid value type for number"),
        }
    }

    // pub fn get_type(&self) -> Type {
    //     match self {
    //         Value::Number(_) => Type::Number,
    //         Value::Function(f) => Type::Function(
    //             f.params().iter().map(|a| a.0.clone()).collect(),
    //             Box::new(f.ret().clone()),
    //         ),
    //         Value::None => Type::None,
    //     }
    // }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(n) => write!(f, "{}", n),
            Value::None => write!(f, "()"),
            Value::Function(func) => write!(f, "{}", func),
            Value::FuncAccess(func) => write!(f, "{}", func),
        }
    }
}
