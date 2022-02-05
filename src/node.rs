use std::fmt;

use crate::{
    functions::{Type, UserDefinedFunction},
    token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Statements(Vec<Node>, Type),
    Number(Token),
    Call(Token, Vec<Node>, Type),
    Define(Token, UserDefinedFunction),
    Function(Token, Vec<Type>, Type),
}

impl Node {
    pub fn get_type(&self) -> Type {
        match self {
            Node::Function(_, p, r) => Type::Function(p.clone(), Box::new(r.clone())),
            Node::Number(_) => Type::Number,
            Node::Call(_, _, ret) => ret.clone(),
            Node::Statements(_, t) => t.clone(),
            Node::Define(..) => Type::None,
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Number(t) => write!(f, "Number[{}]", t),
            Node::Call(t, args, _) => write!(
                f,
                "Call[{}][{}]",
                t,
                args.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Node::Statements(nodes, _) => write!(
                f,
                "{}",
                nodes
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(";\n")
            ),
            Node::Define(t, func) => write!(
                f,
                "Define[{}][{}] ~> [{}] [{}]",
                t,
                func.params()
                    .iter()
                    .map(|(t, n)| format!("[{}] {}", n, t))
                    .collect::<Vec<String>>()
                    .join(", "),
                func.ret(),
                func.body()
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(";\n")
            ),
            Node::Function(t, p, r) => write!(
                f,
                "Function[{}][{}] ~> [{}]",
                t,
                p.iter()
                    .map(|t| format!("[{}]", t))
                    .collect::<Vec<String>>()
                    .join(", "),
                r,
            ),
        }
    }
}
