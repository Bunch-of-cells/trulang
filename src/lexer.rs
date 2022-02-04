use std::{borrow::Cow, fmt, rc::Rc};

use crate::{error::Position, KEYWORDS};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(f64),
    Word(String),
    Keyword(String),
    Colon,
    Pipe,
    LBracket,
    RBracket,
    CurlyArrow,
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

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type
    }
}

impl Token {
    fn new(
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

pub fn lex(code: &str, file: String) -> Vec<Token> {
    let file = Rc::new(file);
    let mut tokens = Vec::new();
    let mut new = true;
    let mut last_line = 0;
    let mut line = 1;
    let mut word = String::new();
    let mut loc: Option<(usize, usize, usize, usize)> = None; // ls s le e
    code.chars().enumerate().for_each(|(i, c)| {
        let i = i + 1;
        if c.is_whitespace() {
            if !new {
                let token = match word.parse() {
                    Ok(n) => TokenType::Number(n),
                    Err(_) => match word.as_str() {
                        ":" => TokenType::Colon,
                        "|" => TokenType::Pipe,
                        "[" => TokenType::LBracket,
                        "]" => TokenType::RBracket,
                        "~>" => TokenType::CurlyArrow,
                        _ => {
                            if KEYWORDS.contains(&&*word) {
                                TokenType::Keyword(word.clone())
                            } else {
                                TokenType::Word(word.clone())
                            }
                        }
                    },
                };
                let (s, ls, e, le) = loc.unwrap();
                tokens.push(Token::new(token, ls, le, s, e, Rc::clone(&file)));
                new = true;
                word.clear();
                loc = None;
            }
            if c == '\n' {
                last_line = i;
                line += 1;
            }
        } else {
            new = false;
            word.push(c);
            match loc {
                Some((s1, s2, _, _)) => loc = Some((s1, s2, i + 1 - last_line, line)),
                None => loc = Some((i - last_line, line, i + 1 - last_line, line)),
            }
        }
    });
    if !word.is_empty() {
        let token = match word.parse() {
            Ok(n) => TokenType::Number(n),
            Err(_) => match word.as_str() {
                ":" => TokenType::Colon,
                "|" => TokenType::Pipe,
                "[" => TokenType::LBracket,
                "]" => TokenType::RBracket,
                "~>" => TokenType::CurlyArrow,
                _ => {
                    if KEYWORDS.contains(&&*word) {
                        TokenType::Keyword(word.clone())
                    } else {
                        TokenType::Word(word.clone())
                    }
                }
            },
        };
        let (s, ls, e, le) = loc.unwrap();
        tokens.push(Token::new(token, ls, le, s, e, Rc::clone(&file)));
    }
    tokens.push(Token::new(TokenType::Eof, 0, 0, 0, 0, Rc::clone(&file)));
    tokens
}
