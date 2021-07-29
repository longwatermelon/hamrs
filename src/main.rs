mod lexer;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].as_str()).expect("Couldn't open file");

    let lexer = lexer::init_lexer(String::from(contents.as_str()));
    println!("{}", lexer.contents);
}
