use crate::lexer;
use crate::node;

use std::fs;

#[derive(Clone)]
pub struct Parser {
    pub current_token: lexer::Token,
    pub prev_token: lexer::Token,
    pub lexer: lexer::Lexer,
}

pub fn init_parser(file_path: String) -> Parser {
    let contents = fs::read_to_string(file_path.as_str()).expect("Couldn't open file");
    let mut lexer = lexer::init_lexer(String::from(contents.as_str()));

    return Parser{
        current_token: lexer::collect_next_token(&mut lexer),
        prev_token: lexer::Token{ token_type: lexer::TokenType::Id, value: String::new() },
        lexer: lexer
    };
}

pub fn eat(parser: &mut Parser, t: lexer::TokenType) {
    if parser.current_token.token_type == t {
        parser.prev_token = parser.current_token.clone();
        parser.current_token = lexer::collect_next_token(&mut parser.lexer);
    } else {
        panic!("Unexpected token {} at line {}", parser.current_token.value, parser.lexer.line_num);
    }
}

pub unsafe fn parse(parser: &mut Parser) -> *mut node::Node {
    let root = node::init_node(node::NodeType::Compound);
    (*root).compound_value.push(parse_expr(parser));

    while parser.lexer.index < parser.lexer.contents.len() {
        eat(parser, lexer::TokenType::Semi);
        let expr = parse_expr(parser);

        if expr == 0 as *mut node::Node { break; }

        (*root).compound_value.push(expr);
    }

    return root;
}

pub unsafe fn parse_expr(parser: &mut Parser) -> *mut node::Node {
    match parser.current_token.token_type {
        lexer::TokenType::Str => return parse_string(parser),
        lexer::TokenType::Int => return parse_int(parser),
        lexer::TokenType::Id => return parse_id(parser),
        _ => return 0 as *mut node::Node
    }
}

pub unsafe fn parse_string(parser: &mut Parser) -> *mut node::Node {
    let string = node::init_node(node::NodeType::Str);
    (*string).string_value = parser.current_token.value.clone();
    eat(parser, lexer::TokenType::Str);
    return string;
}

pub unsafe fn parse_int(parser: &mut Parser) -> *mut node::Node {
    let integer = node::init_node(node::NodeType::Int);
    (*integer).int_value = parser.current_token.value.parse::<i32>().unwrap();
    eat(parser, lexer::TokenType::Int);
    return integer;
}

pub unsafe fn parse_id(parser: &mut Parser) -> *mut node::Node {
    if parser.current_token.value == String::from("def") {
        return parse_variable_definition(parser);
    } else {
        return parse_variable(parser);
    }
}

pub unsafe fn parse_variable(parser: &mut Parser) -> *mut node::Node {
    let token_value = parser.current_token.value.clone();
    eat(parser, lexer::TokenType::Id);

    if parser.current_token.token_type == lexer::TokenType::Lparen {
        return parse_function_call(parser);
    }

    let var = node::init_node(node::NodeType::Variable);
    (*var).variable_name = token_value;

    return var;
}

pub unsafe fn parse_variable_definition(parser: &mut Parser) -> *mut node::Node {
    eat(parser, lexer::TokenType::Id);
    let name = parser.current_token.value.clone();

    eat(parser, lexer::TokenType::Id);
    eat(parser, lexer::TokenType::Equals);

    let value = parse_expr(parser);
    let var_def = node::init_node(node::NodeType::VariableDefinition);

    (*var_def).variable_definition_name = name;
    (*var_def).variable_definition_value = value;

    return var_def;
}

pub unsafe fn parse_function_call(parser: &mut Parser) -> *mut node::Node {
    let function_call = node::init_node(node::NodeType::FunctionCall);
    (*function_call).function_call_name = parser.prev_token.value.clone();
    eat(parser, lexer::TokenType::Lparen);
    (*function_call).function_call_args.push(parse_expr(parser));

    while parser.current_token.token_type != lexer::TokenType::Rparen {
        eat(parser, lexer::TokenType::Comma);
        (*function_call).function_call_args.push(parse_expr(parser));
    }

    eat(parser, lexer::TokenType::Rparen);
    return function_call;
}

