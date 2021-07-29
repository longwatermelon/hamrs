mod lexer;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(args[1].as_str()).expect("Couldn't open file");

    let mut lexer = lexer::init_lexer(String::from(contents.as_str()));
    let mut dummy = lexer::lexer_collect_next_token(&mut lexer);

    println!("started");

    loop {
        if dummy.token_type == lexer::TokenType::Eof {
            break;
        }

        println!("type: {}, value: {}", dummy.token_type as u32, dummy.value.as_str());
        dummy = lexer::lexer_collect_next_token(&mut lexer);
    }

    println!("Finished");
}
