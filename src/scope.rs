use crate::{functions::Type, token::Token, DEFINED_WORDS};

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    defined: Vec<(Token, Type)>,
    scopes: Vec<Scope>,
}

impl Scope {
    pub fn new() -> Scope {
        Scope {
            defined: Vec::new(),
            scopes: Vec::new(),
        }
    }

    pub fn get_new(&self) -> Scope {
        let mut new = Self::new();
        new.defined = self.defined.clone();
        new
    }

    pub fn add(&mut self, scope: Self) {
        self.scopes.push(scope);
    }

    pub fn define(&mut self, node: (Token, Type)) {
        self.defined.push(node);
    }

    pub fn find(&self, token: &Token) -> Option<Type> {
        match self.defined.iter().rev().find(|(f, ..)| f == token) {
            Some((_, t)) => Some(t.clone()),
            None => DEFINED_WORDS
                .iter()
                .find(|f| **token == f.name())
                .map(|f| Type::Function(f.params().to_vec(), Box::new(f.ret().clone()))),
        }
    }
}
