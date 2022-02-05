use std::collections::HashMap;

use crate::{
    error::{Error, ErrorType},
    node::Node,
    token::{Token, TokenType},
    value::Value,
};

pub fn interpret(ast: &Node) -> Result<(), Error> {
    inner_interpret(ast, &mut HashMap::new()).map(|_| ())
}

fn inner_interpret(ast: &Node, vars: &mut HashMap<Token, Value>) -> Result<Value, Error> {
    match ast {
        Node::Number(n) => Ok(Value::from_token(n)),
        Node::Call(func, args, _) => {
            let args = args
                .iter()
                .map(|a| inner_interpret(a, vars))
                .collect::<Vec<_>>();
            let mut new_args = Vec::with_capacity(args.capacity());
            for arg in args {
                new_args.push(arg?);
            }
            let ret = if let Some(func) = vars.get(func) {
                let func = if let Value::Function(f) = func {
                    f
                } else {
                    unreachable!();
                };
                let mut ret = Value::None;
                for statement in &func.body().clone() {
                    ret = inner_interpret(statement, vars)?;
                }
                ret
            } else {
                get_func(func)(new_args.as_slice())?
            };
            Ok(ret)
        }
        Node::Define(t, func) => {
            vars.insert(t.clone(), Value::Function(func.clone()));
            Ok(Value::None)
        }
        Node::Statements(statements, _) => {
            let mut ret = Value::None;
            for statement in statements {
                ret = inner_interpret(statement, vars)?;
            }
            Ok(ret)
        }
        Node::Function(_, _, _) => todo!(),
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
