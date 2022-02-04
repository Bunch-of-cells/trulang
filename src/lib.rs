use std::fs;

use functions::{BuiltInFunction, Type};

mod error;
mod functions;
mod interpreter;
mod lexer;
mod parser;

const KEYWORDS: [&str; 1] = ["Int"];
const DEFINED_WORDS: [BuiltInFunction; 5] = [
    BuiltInFunction::new("+", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("-", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("*", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("/", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new(".", &[Type::Number], Type::None),
];

pub fn run(file: &str) -> Result<(), error::Error> {
    let tokens = lexer::lex(&fs::read_to_string(file).unwrap(), file.to_string());
    // println!(
    //     "{:?}",
    //     tokens.iter().map(|t| (**t).clone()).collect::<Vec<_>>()
    // );
    let ast = parser::parse(&tokens)?;
    // println!("{}", ast);
    interpreter::interpret(&ast)
}
