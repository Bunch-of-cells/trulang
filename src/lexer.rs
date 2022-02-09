use std::rc::Rc;

use crate::{
    token::{Token, TokenType},
    KEYWORDS,
};

pub fn lex(code: &str, file: String) -> Vec<Token> {
    let file = Rc::new(file);
    let mut tokens = Vec::new();
    let mut last_line = 0;
    let mut line = 1;
    let mut chars = code.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            '\n' => {
                last_line = i + 1;
                line += 1;
            }
            ' ' | '\t' | '\r' => {}
            ':' => tokens.push(Token::new(
                TokenType::Colon,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            '|' => tokens.push(Token::new(
                TokenType::Pipe,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            '[' => tokens.push(Token::new(
                TokenType::LBracket,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            ']' => tokens.push(Token::new(
                TokenType::RBracket,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            '!' => tokens.push(Token::new(
                TokenType::Bang,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            '?' => tokens.push(Token::new(
                TokenType::Question,
                line,
                line,
                i + 1 - last_line,
                i + 2 - last_line,
                Rc::clone(&file),
            )),
            '~' if matches!(chars.peek(), Some((_, '>'))) => {
                chars.next();
                tokens.push(Token::new(
                    TokenType::CurlyArrow,
                    line,
                    line,
                    i + 1 - last_line,
                    i + 3 - last_line,
                    Rc::clone(&file),
                ));
            }
            _ => {
                let mut word = c.to_string();
                let start = (i + 1 - last_line, line);
                let mut end = i + 1;
                while let Some(&(i, c)) = chars.peek() {
                    if "[]!:?|\n\t\r ".contains(c) {
                        break;
                    }
                    if c == '~' {
                        chars.next();
                        if matches!(chars.peek(), Some((_, '>'))) {
                            tokens.push(Token::new(
                                TokenType::CurlyArrow,
                                line,
                                line,
                                i + 1 - last_line,
                                i + 1 - last_line,
                                Rc::clone(&file),
                            ));
                            word.clear();
                            break;
                        }
                    }
                    word.push(c);
                    end = i + 1;
                    chars.next();
                }

                if !word.is_empty() {
                    tokens.push(Token::new(
                        match word.parse() {
                            Ok(n) => TokenType::Number(n),
                            _ => {
                                if KEYWORDS.contains(&&*word) {
                                    TokenType::Keyword(word.clone())
                                } else {
                                    TokenType::Word(word.clone())
                                }
                            }
                        },
                        start.1,
                        line,
                        start.0,
                        end + 1 - last_line,
                        Rc::clone(&file),
                    ))
                }
            }
        }
    }
    tokens.push(Token::new(TokenType::Eof, 0, 0, 0, 0, Rc::clone(&file)));
    tokens
}
