mod token;
mod lexer;
mod parser;
mod json_syntax;
mod tests;

use lexer::Lexer;
use parser::Parser;
use json_syntax::*;

use std::io::{Read};
use std::fs::{File};

fn main() {
    let mut handle = File::open("examples/file1.json").unwrap();
    let mut content = String::new();
    handle.read_to_string(&mut content).expect("Something went wrong while reading the file");
    // lexe and parse
    let result = Parser::new( Lexer::new(&content).tokenize().unwrap() ).parse().unwrap();
    for item in result["phone numbers"].unwrap_list() {
        println!("{:?}", item["number"]);
    }
}
