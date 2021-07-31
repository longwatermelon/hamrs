mod lexer;
mod node;
mod parser;
mod visitor;

use std::env;
// use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let contents = fs::read_to_string(args[1].as_str()).expect("Couldn't open file");

    // let mut lexer = lexer::init_lexer(String::from(contents.as_str()));
    // let mut dummy = lexer::collect_next_token(&mut lexer);

    // println!("started");

    // loop {
    //     if dummy.token_type == lexer::TokenType::Eof {
    //         break;
    //     }

    //     println!("type: {}, value: {}", dummy.token_type as u32, dummy.value.as_str());
    //     dummy = lexer::collect_next_token(&mut lexer);
    // }

    // println!("Finished");

    let mut parser = parser::init_parser(args[1].clone());
    let mut visitor = visitor::init_visitor();

    unsafe {
        let root = parser::parse(&mut parser);
        visitor::visit(&mut visitor, root);
        node::cleanup_node(root);

        // println!("{}", (*(*root).compound_value[0]).function_call_name.as_str());
        // println!("{}", (*(*(*root).compound_value[0]).function_call_args[0]).string_value.as_str());
    }
}
