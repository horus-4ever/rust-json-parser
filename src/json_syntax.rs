use std::collections::{HashMap};
use std::iter::{Iterator};
use std::ops::{Index};
use std::slice::{SliceIndex};

#[derive(Debug)]
pub enum JSONSyntax {
    Container(HashMap<String, JSONSyntax>),
    Str(String),
    Num(u64),
    Bool(bool),
    Null,
    List(Vec<JSONSyntax>)
}

impl Index<&str> for JSONSyntax {
    type Output = JSONSyntax;
    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).unwrap()
    }
}

impl Index<usize> for JSONSyntax {
    type Output = JSONSyntax;
    fn index(&self, key: usize) -> &Self::Output {
        self.at(key).unwrap()
    }
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

    pub fn unwrap_list(&self) -> &Vec<JSONSyntax> {
        match self {
            Self::List(ref lst) => lst,
            _ => panic!("Not a list")
        }
    }

    pub fn unwrap_map(&self) -> &HashMap<String, JSONSyntax> {
        match self {
            Self::Container(ref map) => map,
            _ => panic!("Not a hashmap")
        }
    }
}