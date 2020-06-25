mod token;
mod lexer;
mod parser;
mod json_syntax;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let code = r#"{
        "first_name": "Jon",
        "number": 20,
        "boolean": false,
        "other": [
            {
                "nope": 30,
            },
        ],
    }"#;
    let mut lex = Lexer::new(code);
    let result = lex.tokenize().unwrap();
    println!("{:?}", result);
    let parse = Parser::new(result);
    let result = parse.parse().unwrap();
    let lst = result.get("other").unwrap();
    println!("{:?}", lst.at(0))
}