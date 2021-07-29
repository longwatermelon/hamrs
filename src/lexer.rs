#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    Id,
    Semi,
    Lparen,
    Rparen,
    Equals,
    Str,
    Int,
    Comma,
    Eof,
}

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

#[derive(Clone)]
pub struct Lexer {
    pub current_char: char,
    pub index: usize,
    pub contents: String,
    pub line_num: u32
}


pub fn init_lexer(string: String) -> Lexer {
    return Lexer{ current_char: string.as_bytes()[0] as char, index: 0, contents: string, line_num: 1 };
}

pub fn advance(lexer: &mut Lexer) {
    if lexer.index < lexer.contents.len() {
        lexer.index += 1;
        lexer.current_char = lexer.contents.as_bytes()[lexer.index] as char;
    }
}

pub fn collect_string(lexer: &mut Lexer) -> String {
    advance(lexer);

    let mut ret = String::new();

    while lexer.current_char != '"' && lexer.index < lexer.contents.len() {
        ret.push(lexer.current_char);
        advance(lexer);
    }

    advance(lexer);

    return ret;
}

pub fn collect_int(lexer: &mut Lexer) -> String {
    let mut ret = String::new();

    while lexer.current_char.is_numeric() && lexer.index < lexer.contents.len() {
        ret.push(lexer.current_char);
        advance(lexer);
    }

    return ret;
}

pub fn collect_id(lexer: &mut Lexer) -> String {
    let mut ret = String::new();

    while lexer.current_char.is_ascii_alphabetic() && lexer.index < lexer.contents.len() {
        ret.push(lexer.current_char);
        advance(lexer);
    }

    return ret;
}

pub fn collect_next_token(lexer: &mut Lexer) -> Token {
    while lexer.index < lexer.contents.len() - 1 {
        while lexer.current_char.is_ascii_whitespace() && lexer.current_char != '\n' {
            advance(lexer);
        }

        if lexer.current_char == '"' {
            return Token{ token_type: TokenType::Str, value: collect_string(lexer) };
        }

        if lexer.current_char.is_numeric() {
            return Token{ token_type: TokenType::Int, value: collect_int(lexer) };
        }

        if lexer.current_char.is_ascii_alphanumeric() {
            return Token{ token_type: TokenType::Id, value: collect_id(lexer) };
        }

        match lexer.current_char {
            ';' => { advance(lexer); return Token{ token_type: TokenType::Semi, value: String::from(";") } },
            '=' => { advance(lexer); return Token{ token_type: TokenType::Equals, value: String::from("=") } },
            '(' => { advance(lexer); return Token{ token_type: TokenType::Lparen, value: String::from("(") } },
            ')' => { advance(lexer); return Token{ token_type: TokenType::Rparen, value: String::from(")") } },
            ',' => { advance(lexer); return Token{ token_type: TokenType::Comma, value: String::from(",") } },
            '\n' => { advance(lexer); lexer.line_num += 1; },
            _ => println!("Unrecognized token '{}' on line {}", String::from(lexer.current_char).as_str(), lexer.line_num),
        }
    }

    return Token{ token_type: TokenType::Eof, value: String::from("") };
}
