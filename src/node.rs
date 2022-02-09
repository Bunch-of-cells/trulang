use std::fmt;

use crate::{
    error::Position,
    functions::{Type, UserDefinedFunction},
    token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Statements(Vec<Node>, Type, Position),
    Number(Token),
    Call(Token, Vec<Node>, Type),
    Define(Token, Box<Node>),
    FuncAccess(Token, Vec<Type>, Type),
    Function(UserDefinedFunction, Position),
    Var(Token, Type),
    If(Box<Node>, Box<Node>, Box<Node>, Position),
}

impl Node {
    pub fn get_type(&self) -> Type {
        match self {
            Node::Function(p, _) => Type::Function(
                p.params().iter().map(|a| a.0.clone()).collect(),
                Box::new(p.ret().clone()),
            ),
            Node::Number(_) => Type::Number,
            Node::Call(_, _, ret) => ret.clone(),
            Node::Statements(_, t, _) => t.clone(),
            Node::Define(..) => Type::None,
            Node::Var(_, t) => t.clone(),
            Node::FuncAccess(_, p, r) => Type::Function(p.clone(), Box::new(r.clone())),
            Node::If(_, then, _, _) => then.get_type(),
        }
    }

    pub fn position(&self) -> &Position {
        match self {
            Node::Number(t) => t.position(),
            Node::Call(t, _, _) => t.position(),
            Node::Define(t, _) => t.position(),
            Node::FuncAccess(t, _, _) => t.position(),
            Node::If(.., pos) | Node::Function(_, pos) | Node::Statements(_, _, pos) => pos,
            Node::Var(t, _) => t.position(),
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
            Node::Statements(nodes, _, _) => write!(
                f,
                "| {} |",
                nodes
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(";\n")
            ),
            Node::Define(t, n) => write!(f, "Define[{}][{}]", t, n),
            Node::Function(p, _) => write!(
                f,
                "Function[{}] ~> [{}] | {} |",
                p.params()
                    .iter()
                    .map(|(ty, t)| format!("[{}] {} ", ty, t))
                    .collect::<Vec<String>>()
                    .join(" "),
                p.ret(),
                p.body()
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join("; "),
            ),
            Node::Var(t, _) => write!(f, "Var[{}]", t),
            Node::FuncAccess(t, p, r) => write!(
                f,
                "Function[{}][{}] ~> [{}]",
                t,
                p.iter()
                    .map(|t| format!("[{}]", t))
                    .collect::<Vec<String>>()
                    .join(", "),
                r,
            ),
            Node::If(cond, then, else_, _) => write!(f, "If[{}][{}][{}]", cond, then, else_),
        }
    }
}
