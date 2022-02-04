use std::{error::Error as stdError, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    error: ErrorType,
    position: Position,
    details: String,
}

impl Error {
    pub fn new(error: ErrorType, position: Position, details: String) -> Error {
        Error {
            error,
            position,
            details,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} at {}:{} to {}:{} in {} ~> {}",
            self.error,
            self.position.line,
            self.position.column,
            self.position.line_end,
            self.position.column_end,
            self.position.file,
            self.details
        )
    }
}

impl stdError for Error {}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    SyntaxError,
    UndefinedFunction,
    TypeError,
    DivisionByZero,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    line: usize,
    column: usize,
    line_end: usize,
    column_end: usize,
    file: Rc<String>,
}

impl Position {
    pub fn new(
        line: usize,
        line_end: usize,
        column: usize,
        column_end: usize,
        file: Rc<String>,
    ) -> Position {
        Position {
            line,
            column,
            line_end,
            column_end,
            file,
        }
    }
}
