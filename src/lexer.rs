pub struct Lexer {
    pub current_char: char,
    pub index: u32,
    pub contents: String,
    pub line_num: u32
}

pub fn init_lexer(string: String) -> Lexer {
    return Lexer{ current_char: ' ', index: 0, contents: string, line_num: 1 };
}

