use std::fmt::{Debug};
pub use std::convert::{TryFrom};
use std::cmp::{PartialEq};

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Type(TypeKind),
    Syntax(SyntaxKind),
}

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Numeric(u64),
    Str(String),
    Boolean(bool),
    Null
}

#[derive(Debug, PartialEq)]
pub enum SyntaxKind {
    Comma,
    Colon,
    LBracket,
    RBracket,
    LSBracket,
    RSBracket
}

impl TryFrom<char> for SyntaxKind {
    type Error = ();
    fn try_from(chr: char) -> Result<Self, Self::Error> {
        match chr {
            ',' => Ok(Self::Comma),
            ':' => Ok(Self::Colon),
            '{' => Ok(Self::LBracket),
            '}' => Ok(Self::RBracket),
            '[' => Ok(Self::LSBracket),
            ']' => Ok(Self::RSBracket),
            _ => Err(())
        }
    }
}