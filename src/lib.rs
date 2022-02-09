use functions::{BuiltInFunction, Type};

mod error;
mod functions;
mod interpreter;
mod lexer;
mod node;
mod parser;
mod scope;
mod token;
mod value;

const KEYWORDS: [&str; 1] = ["Int"];
static DEFINED_WORDS: [BuiltInFunction; 7] = [
    BuiltInFunction::new("+", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("-", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("*", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new("/", &[Type::Number, Type::Number], Type::Number),
    BuiltInFunction::new(".", &[Type::Any], Type::None),
    BuiltInFunction::new("?", &[Type::Bool, Type::Any, Type::Any], Type::Any),
    BuiltInFunction::new("==", &[Type::Number, Type::Number], Type::Bool),
];

pub fn run(contents: &str, file: &str) -> Result<(), error::Error> {
    let tokens = lexer::lex(contents, file.to_string());
    println!(
        "{:?}",
        tokens.iter().map(|t| (**t).clone()).collect::<Vec<_>>()
    );
    println!("----------------------------------------------------------------------");
    let ast = parser::parse(&tokens)?;
    println!("----------------------------------------------------------------------");
    println!("{ast}");
    println!("----------------------------------------------------------------------");
    interpreter::interpret(&ast)
}
