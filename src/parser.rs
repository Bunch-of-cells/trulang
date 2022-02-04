use crate::{
    error::{Error, ErrorType},
    functions::{Type, UserDefinedFunction},
    lexer::{Token, TokenType},
    DEFINED_WORDS,
};

use std::fmt::{self, Debug};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Statements(Vec<Node>, Type),
    Number(Token),
    Call(Token, Vec<Node>, Type),
    Define(UserDefinedFunction),
}

impl Node {
    fn get_type(&self) -> Type {
        match self {
            Node::Number(_) => Type::Number,
            Node::Call(_, _, ret) => ret.clone(),
            Node::Statements(_, t) => t.clone(),
            Node::Define(_) => Type::None,
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
            Node::Define(func) => write!(
                f,
                "Define[{}][{}] ~> [{}] [{}]",
                func.name(),
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
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Scope {
    defined: Vec<(Token, Vec<Type>, Type)>,
    scopes: Vec<Scope>,
}

impl Scope {
    fn new() -> Scope {
        Scope {
            defined: Vec::new(),
            scopes: Vec::new(),
        }
    }

    fn define(&mut self, function: (Token, Vec<Type>, Type)) {
        self.defined.push(function);
    }

    fn find(&self, function: &Token) -> Option<(&[Type], &Type)> {
        match self.defined.iter().rev().find(|(f, _, _)| f == function) {
            Some((_, p, r)) => Some((p, r)),
            None => DEFINED_WORDS
                .iter()
                .find(|f| **function == f.name())
                .map(|f| (f.params(), f.ret())),
        }
    }
}

type ParseResult = Result<Node, Error>;

struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    current: Token,
}

impl Parser<'_> {
    fn new(tokens: &[Token]) -> Parser {
        Parser {
            tokens,
            index: 0,
            current: tokens[0].clone(),
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    fn advance(&mut self) {
        self.index += 1;
        if let Some(token) = self.tokens.get(self.index) {
            self.current = token.clone()
        }
    }

    fn make_type(&mut self) -> Result<Type, Error> {
        match *self.current {
            TokenType::Keyword(ref s) => match s.as_str() {
                "Int" => {
                    self.advance();
                    Ok(Type::Number)
                }
                _ => Err(Error::new(
                    ErrorType::SyntaxError,
                    self.current.position().clone(),
                    format!("Expected type, found {}", s),
                )),
            },
            TokenType::RBracket => {
                let mut params = vec![];
                while *self.current != TokenType::CurlyArrow {
                    if *self.current != TokenType::RBracket {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            self.current.position().clone(),
                            "Expected ']'".to_string(),
                        ));
                    }
                    self.advance();
                    let t = self.make_type()?;
                    if *self.current != TokenType::LBracket {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            self.current.position().clone(),
                            "Expected '['".to_string(),
                        ));
                    }
                    self.advance();
                    params.push(t);
                }
                self.advance();
                params.push(self.make_type()?);
                self.advance();
                Ok(Type::Function(params))
            }
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                self.current.position().clone(),
                "Expected parameter type".to_string(),
            )),
        }
    }

    fn statements(
        &mut self,
        scope: &mut Scope,
        end_token: TokenType,
    ) -> Result<(Vec<Node>, Type), Error> {
        let mut statements = Vec::new();
        let mut ret = Type::None;
        while *self.current != end_token {
            let expr = self.expression(scope)?;
            ret = expr.get_type();
            statements.push(expr);
        }
        self.advance();
        Ok((statements, ret))
    }

    fn expression(&mut self, scope: &mut Scope) -> ParseResult {
        let token = self.current.clone();
        match *self.current {
            TokenType::Number(_) => {
                self.advance();
                Ok(Node::Number(token))
            }
            TokenType::Word(_) => {
                if matches!(self.peek(), Some(t) if **t == TokenType::Colon) {
                    let token = self.current.clone();
                    let mut params = vec![];
                    let mut ret = None;
                    self.advance();
                    self.advance();
                    while *self.current != TokenType::Pipe {
                        if *self.current != TokenType::LBracket {
                            return Err(Error::new(
                                ErrorType::SyntaxError,
                                self.current.position().clone(),
                                "Expected '['".to_string(),
                            ));
                        }
                        self.advance();
                        let type_ = self.make_type()?;
                        if *self.current != TokenType::RBracket {
                            return Err(Error::new(
                                ErrorType::SyntaxError,
                                self.current.position().clone(),
                                "Expected ']' after type".to_string(),
                            ));
                        }
                        self.advance();
                        if !matches!(*self.current, TokenType::Word(_)) {
                            if params.is_empty() {
                                if *self.current != TokenType::Pipe {
                                    return Err(Error::new(
                                        ErrorType::SyntaxError,
                                        self.current.position().clone(),
                                        "Expected '|'".to_string(),
                                    ));
                                }
                                ret = Some(type_);
                                break;
                            } else {
                                return Err(Error::new(
                                    ErrorType::SyntaxError,
                                    self.current.position().clone(),
                                    "Expected parameter name".to_string(),
                                ));
                            }
                        }
                        let param = self.current.clone();
                        self.advance();
                        params.push((type_, param));
                        if *self.current == TokenType::CurlyArrow {
                            self.advance();
                            if *self.current != TokenType::LBracket {
                                return Err(Error::new(
                                    ErrorType::SyntaxError,
                                    self.current.position().clone(),
                                    "Expected '['".to_string(),
                                ));
                            }
                            self.advance();
                            ret = Some(self.make_type()?);
                            if *self.current != TokenType::RBracket {
                                return Err(Error::new(
                                    ErrorType::SyntaxError,
                                    self.current.position().clone(),
                                    "Expected ']' after type".to_string(),
                                ));
                            }
                            self.advance();
                            if *self.current != TokenType::Pipe {
                                return Err(Error::new(
                                    ErrorType::SyntaxError,
                                    self.current.position().clone(),
                                    "Expected '|'".to_string(),
                                ));
                            }
                            break;
                        }
                    }
                    let ret = match ret {
                        Some(t) => t,
                        None => {
                            return Err(Error::new(
                                ErrorType::SyntaxError,
                                self.current.position().clone(),
                                "No return Type mentioned".to_string(),
                            ))
                        }
                    };
                    self.advance();
                    scope.define((
                        token.clone(),
                        params.iter().map(|a| a.0.clone()).collect(),
                        ret.clone(),
                    ));
                    let (stmts, ty) = self.statements(scope, TokenType::Pipe)?;
                    if ty != ret {
                        return Err(Error::new(
                            ErrorType::TypeError,
                            self.current.position().clone(),
                            format!("Return type mismatch, expected {}, found {}", ret, ty),
                        ));
                    }
                    Ok(Node::Define(UserDefinedFunction::new(
                        token,
                        params,
                        ret,
                        stmts,
                    )))
                } else {
                    let (params, ret) = match scope.find(&self.current) {
                        Some((p, r)) => (p.to_owned(), r.clone()),
                        None => {
                            return Err(Error::new(
                                ErrorType::UndefinedFunction,
                                self.current.position().clone(),
                                format!("Undefined Function : {}", self.current),
                            ))
                        }
                    };
                    let mut args = Vec::new();
                    self.advance();
                    for ty in params {
                        let expr = self.expression(scope)?;
                        if expr.get_type() != ty {
                            return Err(Error::new(
                                ErrorType::TypeError,
                                self.current.position().clone(),
                                format!("Expected type {}, but got {}", ty, expr.get_type()),
                            ));
                        }
                        args.push(expr);
                    }
                    Ok(Node::Call(token, args, ret))
                }
            }
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                self.current.position().clone(),
                format!("Unexpected token: {}", self.current),
            )),
        }
    }
}

pub fn parse(tokens: &[Token]) -> ParseResult {
    let mut parser = Parser::new(tokens);
    let mut scope = Scope::new();
    let (stmts, ty) = parser.statements(&mut scope, TokenType::Eof)?;
    Ok(Node::Statements(stmts, ty))
}
