use crate::lexer::LolLexer;

pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_audio(&mut self);
    fn parse_video(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

//

pub struct LolCodeSyntaxAnalyzer {
    //The parser which will have collection of tokens
    //and a postition variable to keep track of syntax errors.
    pub tokens: Vec<String>,
    pub position: usize,
}

impl LolCodeSyntaxAnalyzer {
    //Using the lexer to get all of the valid tokens
    pub fn collectTokens(mut lexer: LolLexer) -> Self {
        let mut tokens: Vec<String> = Vec::new();
        let mut t = lexer.next_token();
        while t != "EOF" {
            tokens.push(t.clone());
            t = lexer.next_token();
        }
        tokens.push(t);
        LolCodeSyntaxAnalyzer {
            tokens: tokens,
            position: 0,
        }
    }

    fn expect(&mut self, tok: &str) {
        let currentToken = self.current().to_string();
        if currentToken == tok {
            self.position += 1;
        } else {
            self.error(tok, "expect()");
        }
    }

    fn error(&self, excep_token: &str, funcFrom: &str) {
        eprintln!(
            "Hello From {} function. Syntax error at line {}, Expected {} token",
            funcFrom, self.position, excep_token
        );
        std::process::exit(1);
    }

    fn current(&self) -> &str {
        if self.position < self.tokens.len() {
            &self.tokens[self.position]
        } else {
            "EOF"
        }
    }
    fn is_text(&self, tok: &str) -> bool {
        tok.starts_with("TEXT(") && tok.ends_with(')')
    }
}

impl SyntaxAnalyzer for LolCodeSyntaxAnalyzer {
    fn parse_lolcode(&mut self) {
        self.expect("#HAI");

        while self.current() == "#OBTW" {
            self.parse_comment();
        }

        if self.current() == "#MAEK" {
            println!("we are here");
            self.parse_head();
        }

        self.parse_body();

        self.expect("#KTHXBYE");
    }

    fn parse_head(&mut self) {
        self.expect("#MAEK");
        self.expect("HEAD");
        self.parse_title();
        self.expect("#OIC");
    }
    fn parse_title(&mut self) {
        self.expect("#GIMMEH");
        self.expect("TITLE");
        while self.is_text(self.current()) {
            self.parse_text();
        }
        self.expect("#MKAY");
    }

    //it will read out all the
    fn parse_comment(&mut self) {
        self.expect("#OBTW");

        while self.current().starts_with("TEXT(") && self.current().ends_with(')') {
            self.parse_text();
        }

        self.expect("#TLDR");
    }

    fn parse_body(&mut self) {
        while self.position < self.tokens.len() {
            let current = self.current();
            let nextToken = if self.position + 1 < self.tokens.len() {
                &self.tokens[self.position + 1]
            } else {
                "EOF"
            };

            match (current, nextToken) {
                ("#MAEK", "PARAGRAF") => {
                    self.parse_paragraph();
                }

                ("#MAEK", "LIST") => {
                    self.parse_list();
                }
                ("#GIMMEH", "BOLD") => self.parse_bold(),
                ("#GIMMEH", "ITALICS") => self.parse_italics(),
                ("#GIMMEH", "NEWLINE") => self.parse_newline(),
                ("#GIMMEH", "SOUNDZ") => self.parse_audio(),
                ("#GIMMEH", "VIDZ") => self.parse_video(),
                ("#GIMMEH", "ITEM") => self.parse_list_items(),

                ("#I HAZ", _) => self.parse_variable_define(),
                ("#LEMME SEE", _) => self.parse_variable_use(),

                (t, _) if self.is_text(t) => self.parse_text(),

                ("#OBTW", _) => self.parse_comment(),

                _ => break,
            }
        }
    }

    fn parse_paragraph(&mut self) {
        self.expect("#MAEK");
        self.expect("PARAGRAF");
        self.parse_inner_paragraph();
        self.expect("#OIC");
    }

    fn parse_inner_paragraph(&mut self) {
        while self.position < self.tokens.len() {
            let mut current = self.current();
            let next_token = if self.position + 1 < self.tokens.len() {
                self.tokens[self.position + 1].as_str()
            } else {
                ""
            };

            if self.current().starts_with("TEXT(") && self.current().ends_with(')') {
                current = "TEXT()";
            }
            match (current, next_token) {
                ("#LEMME SEE", _) => self.parse_variable_use(),
                ("#GIMMEH", "BOLD") => self.parse_bold(),
                ("#GIMMEH", "ITALICS") => self.parse_italics(),
                ("#GIMMEH", "NEWLINE") => self.parse_newline(),
                ("#GIMMEH", "ITEM") => self.parse_list_items(),
                ("#GIMMEH", "SOUNDZ") => self.parse_audio(),    
                ("#GIMMEH", "VIDZ") => self.parse_video(),
                ("TEXT()", _) => self.parse_text(),
                ("#MAEK", "LIST") => self.parse_list(),
                ("#I HAZ", _) => self.parse_variable_define(),

                _ => break,
            }
        }
    }
    fn parse_inner_text(&mut self) {
        while self.position < self.tokens.len() {
            let cur = self.tokens[self.position].as_str();

            if cur.starts_with("TEXT(") && cur.ends_with(')') {
                self.parse_text();
            } else if cur == "#GIMMEH" {
                let next = if self.position + 1 < self.tokens.len() {
                    self.tokens[self.position + 1].as_str()
                } else {
                    "EOF"
                };

                if next == "BOLD" {
                    self.parse_bold();
                } else if next == "ITALICS" {
                    self.parse_italics();
                } else if next == "SOUNDZ" {
                    self.parse_audio();
                } else if next == "VIDZ" {
                    self.parse_video();
                } else if next == "NEWLINE" {
                    self.parse_newline();
                } else {
                    break;
                }
            } else if cur == "#LEMME SEE" {
                self.parse_variable_use();
            } else {
                break; // reached end of inner text region
            }
        }
    }

    fn parse_variable_define(&mut self) {
        self.expect("#I HAZ");
        self.parse_text();
        self.expect("#IT IZ");
        self.parse_text();
        self.expect("#MKAY");
    }
    fn parse_variable_use(&mut self) {
        self.expect("#LEMME SEE");
        self.parse_text();
        self.expect("#MKAY");
    }
    fn parse_bold(&mut self) {
        self.expect("#GIMMEH");
        self.expect("BOLD");
        self.parse_text();
        self.expect("#MKAY");
    }
    fn parse_italics(&mut self) {
        self.expect("#GIMMEH");
        self.expect("ITALICS");

        while self.is_text(self.current()) {
            self.parse_text();
        }
        self.expect("#MKAY");
    }
    fn parse_list(&mut self) {
        print!("we in list ");
        self.expect("#MAEK");
        self.expect("LIST");
        self.parse_list_items();
        self.expect("#OIC");
    }
    fn parse_list_items(&mut self) {
        while self.current() == "#GIMMEH" && self.tokens[self.position + 1].as_str() == "ITEM" {
            self.expect("#GIMMEH");
            self.expect("ITEM");
            self.parse_inner_list();
            self.expect("#MKAY");
        }
    }
    fn parse_inner_list(&mut self) {
        while self.position < self.tokens.len() {
            let cur = self.current();
            let next_token = if self.position + 1 < self.tokens.len() {
                self.tokens[self.position + 1].as_str()
            } else {
                ""
            };

            match (cur, next_token) {
                (t, _) if self.is_text(t) => self.parse_text(),
                ("#GIMMEH", "BOLD") => self.parse_bold(),
                ("#GIMMEH", "ITALICS") => self.parse_italics(),
                ("#LEMME SEE", _) => self.parse_variable_use(),
                _ => break,
            }
        }
    }
    fn parse_audio(&mut self) {
        self.expect("#GIMMEH");
        self.expect("SOUNDZ");
        self.parse_text();
        self.expect("#MKAY");
    }
    fn parse_video(&mut self) {
        self.expect("#GIMMEH");
        self.expect("VIDZ");
        self.parse_text();
        self.expect("#MKAY");
    }
    fn parse_newline(&mut self) {
        self.expect("#GIMMEH");
        self.expect("NEWLINE");
    }
    fn parse_text(&mut self) {
        let tok = self.current();
        if tok.starts_with("TEXT(") && tok.ends_with(')') {
            self.position += 1;
        } else {
            self.error("TEXT()", "partse_text");
        }
    }
}
