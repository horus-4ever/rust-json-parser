use std::collections::{HashMap};
use std::ops::Index;

#[derive(Debug)]
pub enum JSONSyntax {
    Container(HashMap<String, JSONSyntax>),
    Str(String),
    Num(u64),
    Bool(bool),
    Null,
    List(Vec<JSONSyntax>)
}

impl JSONSyntax {
    pub fn get(&self, key: &str) -> Option<&JSONSyntax> {
        match self {
            Self::Container(ref map) => map.get(key),
            _ => None
        }
    }

    pub fn at(&self, key: usize) -> Option<&JSONSyntax> {
        match self {
            Self::List(ref lst) => lst.get(key),
            _ => None
        }
    }
}