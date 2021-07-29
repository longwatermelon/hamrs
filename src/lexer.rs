#[derive(PartialEq)]
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

pub struct Token {
    pub token_type: TokenType,
    pub value: String
}

pub struct Lexer {
    pub current_char: char,
    pub index: usize,
    pub contents: String,
    pub line_num: u32
}


pub fn init_lexer(string: String) -> Lexer {
    return Lexer{ current_char: string.as_bytes()[0] as char, index: 0, contents: string, line_num: 1 };
}

pub fn lexer_advance(lexer: &mut Lexer) {
    if lexer.index < lexer.contents.len() {
        lexer.index += 1;
        lexer.current_char = lexer.contents.as_bytes()[lexer.index] as char;
    }
}

pub fn lexer_collect_string(lexer: &mut Lexer) -> String {
    lexer_advance(lexer);

    let mut ret = String::new();

    while lexer.current_char != '"' && lexer.current_char != '\n' {
        ret.push(lexer.current_char);
        lexer_advance(lexer);
    }

    lexer_advance(lexer);

    return ret;
}

pub fn lexer_collect_int(lexer: &mut Lexer) -> String {
    let mut ret = String::new();

    while lexer.current_char.is_numeric() && lexer.current_char != '\n' {
        ret.push(lexer.current_char);
        lexer_advance(lexer);
    }

    return ret;
}

pub fn lexer_collect_id(lexer: &mut Lexer) -> String {
    let mut ret = String::new();

    while lexer.current_char.is_ascii_alphabetic() && lexer.current_char != '\n' {
        ret.push(lexer.current_char);
        lexer_advance(lexer);
    }

    return ret;
}

pub fn lexer_collect_next_token(lexer: &mut Lexer) -> Token {
    while lexer.index < lexer.contents.len() {
        while lexer.current_char.is_ascii_whitespace() && lexer.current_char != '\n' {
            lexer_advance(lexer);
        }

        if lexer.current_char == '"' {
            return Token{ token_type: TokenType::Str, value: lexer_collect_string(lexer) };
        }

        if lexer.current_char.is_numeric() {
            return Token{ token_type: TokenType::Int, value: lexer_collect_int(lexer) };
        }

        if lexer.current_char.is_ascii_alphanumeric() {
            return Token{ token_type: TokenType::Id, value: lexer_collect_id(lexer) };
        }

        match lexer.current_char {
            ';' => { lexer_advance(lexer); return Token{ token_type: TokenType::Semi, value: String::from(";") } },
            '=' => { lexer_advance(lexer); return Token{ token_type: TokenType::Equals, value: String::from("=") } },
            '(' => { lexer_advance(lexer); return Token{ token_type: TokenType::Lparen, value: String::from("(") } },
            ')' => { lexer_advance(lexer); return Token{ token_type: TokenType::Rparen, value: String::from(")") } },
            ',' => { lexer_advance(lexer); return Token{ token_type: TokenType::Comma, value: String::from(",") } },
            _ => println!("Unrecognized token {}", String::from(lexer.current_char).as_str()),
        }

        lexer_advance(lexer);
        lexer.line_num += 1;
    }

    return Token{ token_type: TokenType::Eof, value: String::from("") };
}

