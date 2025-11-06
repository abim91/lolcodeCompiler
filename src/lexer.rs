pub trait LexicalAnalyzer {
    fn get_char(&mut self) -> char;
    fn add_char(&mut self, c: char);
    fn lookup(&self, s: &str) -> bool;
}

pub struct LolLexer {
    input: Vec<char>,
    index: usize,
    pub line: usize,
    pub col: usize,
    potential_token: String,

    hash_annotations: [&'static str; 11],
    plain_annotations: [&'static str; 10],
}

impl LolLexer {
    pub fn new(source: &str) -> Self {
        LolLexer {
            input: source.chars().collect(),
            index: 0, //to give information about potential errors
            line: 1,
            col: 0,
            potential_token: String::new(),

            hash_annotations: [
                "#HAI",
                "#KTHXBYE",
                "#OBTW",
                "#TLDR",
                "#MAEK",
                "#OIC",
                "#GIMMEH",
                "#MKAY",
                "#I HAZ",
                "#IT IZ",
                "#LEMME SEE",
            ],
            plain_annotations: [
                "HEAD", "TITLE", "PARAGRAF", "BOLD", "ITALICS", "LIST", "ITEM", "NEWLINE",
                "SOUNDZ", "VIDZ",
            ],
        }
    }

    //function to return the next valid token
    pub fn next_token(&mut self) -> String {
        self.skip_ws();
        match self.peek() {
            //pattern matching to handle tokens that start with # and everything else
            None => "EOF".to_string(),

            Some('#') => self.read_hash_keyword(),

            Some(_) => {
                let w = self.get_token();
                let up = w.to_ascii_uppercase();
                if self.plain_annotations.contains(&up.as_str()) {
                    return up;
                } else {
                    return format!("TEXT({})", w); //Regular text will be identified using TEXT() wrapper.
                }
            }
        }
    }

    //return next char
    fn peek(&self) -> Option<char> {
        if self.index < self.input.len() {
            return Some(self.input[self.index]);
        } else {
            return None;
        }
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();

        if c != None {
            self.index += 1;
            if c == Some('\n') {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
        return c;
    }

    //a helper function to skip white spaces between tokens
    fn skip_ws(&mut self) {
        let c = self.peek();
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    //returns the next token
    fn get_token(&mut self) -> String {
        self.potential_token.clear();
        while let Some(c) = self.peek() {
            if c.is_whitespace() || c == '#' {
                break;
            }
            let ch = self.get_char();
            self.add_char(ch);
        }
        self.potential_token.clone()
    }

    fn read_hash_keyword(&mut self) -> String {
        self.advance();
        let first_token = self.get_token().to_ascii_uppercase();

        // some tokens such as "#I HAZ" have two parts, so we need to check for them
        let save_i = self.index;
        let save_line = self.line;
        let save_col = self.col;

        if first_token.is_empty() {
            self.error("Expected keyword after '#'");
        }

        self.skip_ws();

        let second_token = self.get_token().to_ascii_uppercase();
        let potentional_FT = format!("#{} {}", first_token, second_token);
        let complete_token = if self.lookup(&potentional_FT) {
            potentional_FT
        } else {
            format!("#{}", first_token)
        };

        if format!("#{}", first_token) == complete_token {
            self.index = save_i;
            self.line = save_line;
            self.col = save_col;
        }

        if !self.lookup(&complete_token) {
            self.error(&format!(
                "'{}' is Not a valid token ",
                complete_token
            ));
        }

        return complete_token;
    }

    fn error(&self, msg: &str) {
        eprintln!(
            "Lexical error at line {}, col {}: {}",
            self.line, self.col, msg
        );
        std::process::exit(1);
    }
}

//implementation of trait functions
impl LexicalAnalyzer for LolLexer {
    fn get_char(&mut self) -> char {
        if let Some(c) = self.advance() {
            c
        } else {
            print!("Unexpected EOF");
            std::process::exit(1);
        }
    }
    fn add_char(&mut self, c: char) {
        self.potential_token.push(c);
    }

    fn lookup(&self, s: &str) -> bool {
        let up = s.to_ascii_uppercase();
        let mut i = 0;
        while i < self.hash_annotations.len() {
            if self.hash_annotations[i] == up {
                return true;
            }
            i += 1;
        }
        false
    }
}
