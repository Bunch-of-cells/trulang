use crate::{
    error::{Error, ErrorType},
    functions::{Type, UserDefinedFunction},
    node::Node,
    scope::Scope,
    token::{Token, TokenType},
};

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
            TokenType::LBracket => {
                let mut params = vec![];
                while *self.current != TokenType::CurlyArrow {
                    if *self.current != TokenType::LBracket {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            self.current.position().clone(),
                            "Expected '['".to_string(),
                        ));
                    }
                    self.advance();
                    let t = self.make_type()?;
                    if *self.current != TokenType::RBracket {
                        return Err(Error::new(
                            ErrorType::SyntaxError,
                            self.current.position().clone(),
                            "Expected ']'".to_string(),
                        ));
                    }
                    self.advance();
                    params.push(t);
                }
                self.advance();
                Ok(Type::Function(params, Box::new(self.make_type()?)))
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
        let mut new = scope.get_new();
        let mut statements = Vec::new();
        let mut ret = Type::None;
        while *self.current != end_token {
            let expr = self.expression(&mut new)?;
            println!("{expr} {end_token:?}");
            ret = expr.get_type();
            statements.push(expr);
        }
        self.advance();
        scope.add(new);
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
                    self.advance();
                    self.advance();
                    let node = self.expression(scope)?;
                    scope.define((token.clone(), node.get_type()));
                    Ok(Node::Define(token, Box::new(node)))
                } else {
                    let t = match scope.find(&self.current) {
                        Some(t) => t,
                        None => {
                            return Err(Error::new(
                                ErrorType::UndefinedFunction,
                                self.current.position().clone(),
                                format!("Undefined Function : {}", self.current),
                            ))
                        }
                    };
                    match t {
                        Type::Function(params, ret) => {
                            let mut args = Vec::new();
                            self.advance();
                            if *self.current == TokenType::Bang {
                                self.advance();
                                Ok(Node::FuncAccess(token, params, (*ret).clone()))
                            } else {
                                for ty in params {
                                    let expr = self.expression(scope)?;
                                    if expr.get_type() != ty {
                                        return Err(Error::new(
                                            ErrorType::TypeError,
                                            self.current.position().clone(),
                                            format!(
                                                "Expected type {}, but got {}",
                                                ty,
                                                expr.get_type()
                                            ),
                                        ));
                                    }
                                    args.push(expr);
                                }
                                Ok(Node::Call(token, args, *ret))
                            }
                        }
                        _ => {
                            self.advance();
                            Ok(Node::Var(token, t))
                        }
                    }
                }
            }
            TokenType::LBracket => self.define_function(scope),
            TokenType::Pipe => {
                let mut s = self.current.position().clone();
                self.advance();
                let (statements, ret) = self.statements(scope, TokenType::Pipe)?;
                s.merge(self.current.position());
                Ok(Node::Statements(statements, ret, s))
            }
            TokenType::Question => {
                let mut s = self.current.position().clone();
                self.advance();
                let condition = self.expression(scope)?;
                if condition.get_type() != Type::Bool {
                    return Err(Error::new(
                        ErrorType::TypeError,
                        self.current.position().clone(),
                        "Expected bool".to_string(),
                    ));
                }
                let then = self.expression(scope)?;
                let else_ = self.expression(scope)?;
                if then.get_type() != else_.get_type() {
                    return Err(Error::new(
                        ErrorType::TypeError,
                        else_.position().clone(),
                        format!("Branches of an if statement must have same types, expected {}, found {}", then.get_type(), else_.get_type()),
                    ));
                }
                s.merge(self.current.position());
                Ok(Node::If(
                    Box::new(condition),
                    Box::new(then),
                    Box::new(else_),
                    s,
                ))
            }
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                self.current.position().clone(),
                format!("Unexpected token: {}", self.current),
            )),
        }
    }

    fn define_function(&mut self, scope: &mut Scope) -> ParseResult {
        let mut s = self.current.position().clone();
        let mut params = vec![];
        let mut ret = None;
        while *self.current != TokenType::Pipe {
            if *self.current != TokenType::LBracket {
                return Err(Error::new(
                    ErrorType::SyntaxError,
                    self.current.position().clone(),
                    format!("Expected '[', found '{}'", self.current),
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
                        format!("Expected '[', found '{}'", self.current),
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
        for (t, p) in params.clone() {
            scope.define((p, t));
        }
        let (stmts, ty) = self.statements(scope, TokenType::Pipe)?;
        if ty != ret {
            return Err(Error::new(
                ErrorType::TypeError,
                self.current.position().clone(),
                format!("Return type mismatch, expected {}, found {}", ret, ty),
            ));
        }
        s.merge(self.current.position());
        Ok(Node::Function(
            UserDefinedFunction::new(params, ret, stmts),
            s,
        ))
    }
}

pub fn parse(tokens: &[Token]) -> ParseResult {
    let mut parser = Parser::new(tokens);
    let mut scope = Scope::new();
    let mut s = parser.current.position().clone();
    let (stmts, ty) = parser.statements(&mut scope, TokenType::Eof)?;
    s.merge(parser.current.position());
    Ok(Node::Statements(stmts, ty, s))
}
