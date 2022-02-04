use std::fmt;

use crate::{
    error::{Error, ErrorType},
    functions::UserDefinedFunction,
    lexer::{Token, TokenType},
    parser::Node,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    None,
}

impl Value {
    fn from_token(token: &Token) -> Value {
        match **token {
            TokenType::Number(n) => Value::Number(n),
            _ => panic!("Invalid token type for value"),
        }
    }

    fn get_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            _ => panic!("Invalid value type for number"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::None => write!(f, "()"),
        }
    }
}

pub fn interpret(ast: &Node) -> Result<(), Error> {
    inner_interpret(ast, &mut vec![]).map(|_| ())
}

fn inner_interpret(ast: &Node, functions: &mut Vec<UserDefinedFunction>) -> Result<Value, Error> {
    match ast {
        Node::Number(n) => Ok(Value::from_token(n)),
        Node::Call(func, args, _) => {
            let args = args
                .iter()
                .map(|a| inner_interpret(a, functions))
                .collect::<Vec<_>>();
            let mut new_args = Vec::with_capacity(args.capacity());
            for arg in args {
                new_args.push(arg?);
            }
            let ret = if let Some(func) = functions.iter().find(|f| f.name() == func) {
                let mut ret = Value::None;
                for statement in &func.body().clone() {
                    ret = inner_interpret(statement, functions)?;
                }
                ret
            } else {
                get_func(func)(new_args.as_slice())?
            };
            Ok(ret)
        }
        Node::Define(f) => {
            functions.push(f.clone());
            Ok(Value::None)
        }
        Node::Statements(statements, _) => {
            let mut ret = Value::None;
            for statement in statements {
                ret = inner_interpret(statement, functions)?;
            }
            Ok(ret)
        }
    }
}

fn get_func(name: &Token) -> Box<dyn FnOnce(&[Value]) -> Result<Value, Error>> {
    let pos = name.position().clone();
    match **name {
        TokenType::Word(ref func) => match func.as_str() {
            "+" => Box::new(|a| {
                let (a, b) = match a {
                    [a, b] => (a, b),
                    _ => unreachable!(),
                };
                Ok(Value::Number(a.get_number() + b.get_number()))
            }),
            "-" => Box::new(|a| {
                let (a, b) = match a {
                    [a, b] => (a, b),
                    _ => unreachable!(),
                };
                Ok(Value::Number(a.get_number() - b.get_number()))
            }),
            "*" => Box::new(|a| {
                let (a, b) = match a {
                    [a, b] => (a, b),
                    _ => unreachable!(),
                };
                Ok(Value::Number(a.get_number() * b.get_number()))
            }),
            "/" => Box::new(move |a| {
                let (a, b) = match a {
                    [a, b] => (a, b),
                    _ => unreachable!(),
                };
                if b.get_number() == 0. {
                    Err(Error::new(
                        ErrorType::DivisionByZero,
                        pos,
                        "Cannot divide by zero".to_string(),
                    ))
                } else {
                    Ok(Value::Number(a.get_number() / b.get_number()))
                }
            }),
            "." => Box::new(|a| {
                println!(
                    "{}",
                    match a {
                        [a] => a,
                        _ => unreachable!(),
                    }
                );
                Ok(Value::None)
            }),
            _ => unreachable!("Function : {name} not implemented"),
        },
        _ => unreachable!(),
    }
}
