use crate::{functions::Type, token::Token, DEFINED_WORDS};

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    defined: Vec<(Token, Vec<Type>, Type)>,
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

    pub fn define(&mut self, function: (Token, Vec<Type>, Type)) {
        self.defined.push(function);
    }

    pub fn find(&self, function: &Token) -> Option<(&[Type], &Type)> {
        match self.defined.iter().rev().find(|(f, ..)| f == function) {
            Some((_, p, r)) => Some((p, r)),
            None => DEFINED_WORDS
                .iter()
                .find(|f| **function == f.name())
                .map(|f| (f.params(), f.ret())),
        }
    }
}
