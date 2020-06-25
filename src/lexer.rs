use crate::token::*;
use std::iter::{Peekable};

pub struct Lexer<'a> {
    iter: Peekable<std::str::Chars<'a>>,
    current: char
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut iter = source.chars();
        Self { iter: iter.peekable(), current: '\0' }
    }

    fn advance(&mut self) -> Option<()> {
        self.current = self.iter.next()?;
        Some(())
    }

    fn find_digit(&mut self) -> Option<u64> {
        let mut result = String::new();
        while let Some(chr) = self.iter.peek() {
            result.push(self.current);
            if !chr.is_digit(10) { break }
            self.advance()?;
        }
        Some(result.parse().unwrap())
    }

    fn find_keyword(&mut self) -> Option<TokenKind> {
        let mut result = String::new();
        while let Some(chr) = self.iter.peek() {
            result.push(self.current);
            if !chr.is_alphabetic() { break }
            self.advance()?;
        }
        match result.as_str() {
            "null" => Some(TokenKind::Type(TypeKind::Null)),
            "true" => Some(TokenKind::Type(TypeKind::Boolean(true))),
            "false" => Some(TokenKind::Type(TypeKind::Boolean(false))),
            _ => None
        }
    }

    fn find_string(&mut self) -> Option<String> {
        let mut result = String::new();
        self.advance()?;
        while self.current != '"' {
            if self.current == '\\' {
                match self.iter.peek()? {
                    'n' => { result.push('\n') }
                    't' => { result.push('\t') }
                    '"' => { result.push('"') }
                    '\\' => {result.push('\\')}
                    _ => return None
                }
                self.advance()?;
            } else {
                result.push(self.current);
            }
            self.advance()?;
        }
        Some(result)
    }

    pub fn tokenize(mut self) -> Option<Vec<Token>> {
        let mut result: Vec<Token> = Vec::new();
        while let Some(_) = self.advance() {
            match self.current {
                c if c.is_ascii_whitespace() => continue,
                c if c.is_digit(10) => {
                    result.push( Token {
                        kind: TokenKind::Type(TypeKind::Numeric(self.find_digit()?))
                    } )
                }
                c if c.is_alphabetic() => {
                    result.push( Token {
                        kind: self.find_keyword()?
                    } )
                }
                '"' => {
                    result.push( Token {
                        kind: TokenKind::Type(TypeKind::Str(self.find_string()?))
                    } )
                } // '{' | '}' | '[' | ']' | ',' | ':'
                c => {
                    result.push( Token {
                        kind: TokenKind::Syntax(SyntaxKind::try_from(c).ok()?)
                    } )
                }
            }
        }
        Some(result)
    }
}