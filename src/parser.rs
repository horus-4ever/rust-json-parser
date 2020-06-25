use std::iter::{Peekable};
use crate::token::*;
use crate::json_syntax::*;
use std::collections::{HashMap};

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
    current: Token
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter().peekable();
        let first = iter.next().unwrap();
        Self { tokens: iter, current: first }
    }

    fn advance(&mut self) -> Option<()> {
        self.current = self.tokens.next()?;
        Some(())
    }

    fn skip(&mut self, n: usize) -> Option<()> {
        for _ in 0..n { self.advance()? }
        Some(())
    }

    fn eat(&mut self, t: TokenKind) -> Option<()> {
        match &self.current {
            t => { self.advance(); Some(()) },
            _ => None
        }
    }

    fn dict_entry(&mut self) -> Option<(String, JSONSyntax)> {
        let mut key = String::new();
        match self.anything()? {
            JSONSyntax::Str(value) => { key = value },
            _ => return None
        }
        self.advance()?;
        self.eat(TokenKind::Syntax(SyntaxKind::Colon))?;
        Some((key, self.anything()?))
    }

    fn dict(&mut self) -> Option<HashMap<String, JSONSyntax>> {
        let mut result: HashMap<String, JSONSyntax> = HashMap::new();
        self.advance()?;
        while self.current.kind != TokenKind::Syntax(SyntaxKind::RBracket) {
            let entry = self.dict_entry()?;
            result.insert(entry.0, entry.1);
            self.advance();
            if self.current.kind == TokenKind::Syntax(SyntaxKind::RBracket) {
                break
            }
            else if self.current.kind == TokenKind::Syntax(SyntaxKind::Comma) 
                    && self.tokens.peek()?.kind == TokenKind::Syntax(SyntaxKind::RBracket)
            {
                self.skip(1);
                break
            }
            else
            {
                self.eat(TokenKind::Syntax(SyntaxKind::Comma))?;
            }
        }
        Some(result)
    }

    fn list(&mut self) -> Option<Vec<JSONSyntax>> {
        let mut result: Vec<JSONSyntax> = Vec::new();
        self.advance()?;
        while self.current.kind != TokenKind::Syntax(SyntaxKind::RSBracket) {
            result.push(self.anything()?);
            self.advance()?;
            if self.current.kind == TokenKind::Syntax(SyntaxKind::RSBracket) {
                break
            }
            else if self.current.kind == TokenKind::Syntax(SyntaxKind::Comma) 
                    && self.tokens.peek()?.kind == TokenKind::Syntax(SyntaxKind::RSBracket)
            {
                self.skip(1);
                break
            } 
            else
            {
                self.eat(TokenKind::Syntax(SyntaxKind::Comma));
            }
        }
        Some(result)
    }

    fn anything(&mut self) -> Option<JSONSyntax> {
        match &self.current {
            Token { kind: TokenKind::Syntax(SyntaxKind::LBracket) } => Some(
                JSONSyntax::Container(self.dict()?)
            ),
            Token { kind: TokenKind::Syntax(SyntaxKind::LSBracket) } => Some(
                JSONSyntax::List(self.list()?)
            ),
            Token { kind: TokenKind::Type(TypeKind::Str(value)) } => Some(
                JSONSyntax::Str(value.clone())
            ),
            Token { kind: TokenKind::Type(TypeKind::Numeric(value)) } => Some(
                JSONSyntax::Num(value.clone())
            ),
            Token { kind: TokenKind::Type(TypeKind::Boolean(value)) } => Some(
                JSONSyntax::Bool(value.clone())
            ),
            Token { kind: TokenKind::Type(TypeKind::Null) } => Some(
                JSONSyntax::Null
            ),
            _ => None
        }
    }

    pub fn parse<'a>(mut self) -> Option<JSONSyntax> {
        match self.current {
            Token { kind: TokenKind::Syntax(SyntaxKind::LBracket) } => 
                Some(JSONSyntax::Container(self.dict()?)),
            Token { kind: TokenKind::Syntax(SyntaxKind::LSBracket) } => 
                Some(JSONSyntax::List(self.list()?)),
            _ => return None
        }
    }
}