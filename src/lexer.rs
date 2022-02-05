use std::rc::Rc;

use crate::{
    token::{Token, TokenType},
    KEYWORDS,
};

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
                        "!" => TokenType::Bang,
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
                "!" => TokenType::Bang,
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
