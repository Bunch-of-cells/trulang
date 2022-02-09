use std::{
    borrow::Cow,
    fmt,
    hash::{Hash, Hasher},
    rc::Rc,
};

use ordered_float::OrderedFloat;

use crate::error::Position;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum TokenType {
    Number(OrderedFloat<f64>),
    Word(String),
    Keyword(String),
    Colon,
    Pipe,
    LBracket,
    RBracket,
    CurlyArrow,
    Bang,
    Question,
    Eof,
}

impl PartialEq<&str> for TokenType {
    fn eq(&self, other: &&str) -> bool {
        match self {
            TokenType::Word(s) => s == *other,
            TokenType::Keyword(s) => s == *other,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    position: Position,
}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_type.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type
    }
}

impl Eq for Token {}

impl Token {
    pub fn new(
        token_type: TokenType,
        line: usize,
        line_end: usize,
        column: usize,
        column_end: usize,
        file: Rc<String>,
    ) -> Token {
        Token {
            token_type,
            position: Position::new(line, line_end, column, column_end, file),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.token_type {
                TokenType::Number(n) => Cow::Owned(n.to_string()),
                TokenType::Word(ref s) => Cow::Borrowed(&**s),
                TokenType::Keyword(ref s) => Cow::Borrowed(&**s),
                TokenType::Colon => Cow::Borrowed(":"),
                TokenType::Pipe => Cow::Borrowed("|"),
                TokenType::LBracket => Cow::Borrowed("["),
                TokenType::RBracket => Cow::Borrowed("]"),
                TokenType::CurlyArrow => Cow::Borrowed("~>"),
                TokenType::Bang => Cow::Borrowed("!"),
                TokenType::Question => Cow::Borrowed("?"),
                TokenType::Eof => Cow::Borrowed("EOF"),
            }
        )
    }
}

impl std::ops::Deref for Token {
    type Target = TokenType;

    fn deref(&self) -> &Self::Target {
        &self.token_type
    }
}
