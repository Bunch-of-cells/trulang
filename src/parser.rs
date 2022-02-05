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
                    self.define_function(scope)
                } else {
                    self.get_function(scope, token)
                }
            }
            TokenType::LBracket => self.define_function(scope),
            TokenType::Pipe => {
                self.advance();
                let (statements, ret) = self.statements(scope, TokenType::Pipe)?;
                Ok(Node::Statements(statements, ret))
            }
            _ => Err(Error::new(
                ErrorType::SyntaxError,
                self.current.position().clone(),
                format!("Unexpected token: {}", self.current),
            )),
        }
    }

    fn define_function(&mut self, scope: &mut Scope) -> ParseResult {
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
        Ok(Node::Define(
            token,
            UserDefinedFunction::new(params, ret, stmts),
        ))
    }

    fn get_function(&mut self, scope: &mut Scope, token: Token) -> ParseResult {
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
        if *self.current == TokenType::Bang {
            Ok(Node::Function(token, params, ret))
        } else {
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
}

pub fn parse(tokens: &[Token]) -> ParseResult {
    let mut parser = Parser::new(tokens);
    let mut scope = Scope::new();
    let (stmts, ty) = parser.statements(&mut scope, TokenType::Eof)?;
    Ok(Node::Statements(stmts, ty))
}
