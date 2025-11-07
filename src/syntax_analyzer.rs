use crate::lexer::LolLexer;
#[derive(Debug, Clone)]
pub enum AST {
    Program { parts: Vec<AST> },
    Comment(String),
    Head { title: String },
    Text(String),
    Paragraph { items: Vec<AST> },
    Bold(String),
    Italics(String),
    List { items: Vec<AST> },
    ListItem { items: Vec<AST> },
    Audio(String),
    Video(String),
    Newline,
    VarDefine { name: String, value: String },
    VarUse(String),
}

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

pub struct LolCodeSyntaxAnalyzer {
    pub tokens: Vec<String>,
    pub position: usize,
    pub ast: Vec<AST>,
}

impl LolCodeSyntaxAnalyzer {
    //Using the lexer to get all of the valid tokens
    pub fn new(mut lexer: LolLexer) -> Self {
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
            ast: Vec::new(),
        }
    }

    fn expect(&mut self, tok: &str) {
        let current_token = self.current().to_string();
        if current_token == tok {
            self.position += 1;
        } else {
            self.error(tok, "expect()");
        }
    }

    fn error(&self, excep_token: &str, func_from: &str) {
        eprintln!(
            "Syntax error near position {}. Expected {} token but found {}",
            self.position,
            excep_token,
            if self.position < self.tokens.len() {
                &self.tokens[self.position]
            } else {
                "EOF"
            }
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

    fn text_content(tok: &str) -> String {
        tok.trim_start_matches("TEXT(")
            .trim_end_matches(')')
            .to_string()
    }
}

impl SyntaxAnalyzer for LolCodeSyntaxAnalyzer {
    fn parse_lolcode(&mut self) {
        // remember where this program starts in AST
        let start_len = self.ast.len();

        self.expect("#HAI");

        while self.current() == "#OBTW" {
            self.parse_comment();
        }

        if self.current() == "#MAEK" {
            self.parse_head();
        }

        self.parse_body();
        self.expect("#KTHXBYE");
        self.expect("EOF");
        let parts: Vec<AST> = self.ast.drain(start_len..).collect();
        self.ast.push(AST::Program { parts });
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

        let mut parts: Vec<String> = Vec::new();
        while self.is_text(self.current()) {
            let tok = self.current().to_string();
            let txt = Self::text_content(&tok);
            self.position += 1;
            parts.push(txt);
        }

        self.expect("#MKAY");

        let title = parts.join(" ");
        self.ast.push(AST::Head { title });
    }

    fn parse_comment(&mut self) {
        self.expect("#OBTW");

        let mut parts: Vec<String> = Vec::new();
        while self.is_text(self.current()) {
            let tok = self.current().to_string();
            let txt = Self::text_content(&tok);
            parts.push(txt);
            self.position += 1;
        }

        self.expect("#TLDR");

        let comment_text = parts.join(" ");
        self.ast.push(AST::Comment(comment_text));
    }

    fn parse_body(&mut self) {
        while self.position < self.tokens.len() {
            let current = self.current();
            let next_token = if self.position + 1 < self.tokens.len() {
                &self.tokens[self.position + 1]
            } else {
                "EOF"
            };

            match (current, next_token) {
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
                ("#I HAZ", _) => self.parse_variable_define(),
                ("#LEMME SEE", _) => self.parse_variable_use(),

                (t, _) if self.is_text(t) => self.parse_text(),

                ("#OBTW", _) => self.parse_comment(),

                _ => break,
            }
        }
    }

    fn parse_paragraph(&mut self) {
        // paragraph will collect its children
        let start_len = self.ast.len();
        self.expect("#MAEK");
        self.expect("PARAGRAF");
        if self.current() == "#I HAZ" {
            self.parse_variable_define();
        }
        self.parse_inner_paragraph();
        self.expect("#OIC");

        let items: Vec<AST> = self.ast.drain(start_len..).collect();
        self.ast.push(AST::Paragraph { items });
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
                ("#GIMMEH", "SOUNDZ") => self.parse_audio(),
                ("#GIMMEH", "VIDZ") => self.parse_video(),
                ("TEXT()", _) => self.parse_inner_text(),
                ("#MAEK", "LIST") => self.parse_list(),

                _ => break,
            }
        }
    }
    fn parse_inner_text(&mut self) {
        self.parse_text();
    }

    fn parse_variable_define(&mut self) {
        self.expect("#I HAZ");
        self.parse_text();
        self.expect("#IT IZ");
        self.parse_text();
        self.expect("#MKAY");

        let value_node = self.ast.pop().unwrap();
        let name_node = self.ast.pop().unwrap();

        let value = match value_node {
            AST::Text(s) => s,
            _ => "<bad value>".to_string(),
        };
        let name = match name_node {
            AST::Text(s) => s,
            _ => "<bad name>".to_string(),
        };

        self.ast.push(AST::VarDefine { name, value });
    }

    fn parse_variable_use(&mut self) {
        self.expect("#LEMME SEE");
        self.parse_text();
        self.expect("#MKAY");

        let name_node = self.ast.pop().unwrap();
        let name = match name_node {
            AST::Text(s) => s,
            _ => "<bad var>".to_string(),
        };
        self.ast.push(AST::VarUse(name));
    }

    fn parse_bold(&mut self) {
        self.expect("#GIMMEH");
        self.expect("BOLD");

        let mut parts: Vec<String> = Vec::new();
        while self.is_text(self.current()) {
            let tok = self.current().to_string();
            let txt = Self::text_content(&tok);
            self.position += 1;
            parts.push(txt);
        }

        self.expect("#MKAY");

        let inner = parts.join(" ");
        self.ast.push(AST::Bold(inner));
    }

    fn parse_italics(&mut self) {
        self.expect("#GIMMEH");
        self.expect("ITALICS");

        let mut parts = Vec::new();
        while self.is_text(self.current()) {
            let tok = self.current().to_string();
            let txt = Self::text_content(&tok);
            self.position += 1;
            parts.push(txt);
        }

        self.expect("#MKAY");

        let inner = parts.join(" ");
        self.ast.push(AST::Italics(inner));
    }

    fn parse_list(&mut self) {
        let start_len = self.ast.len();

        self.expect("#MAEK");
        self.expect("LIST");
        self.parse_list_items();
        self.expect("#OIC");

        let items = self.ast.drain(start_len..).collect();
        self.ast.push(AST::List { items });
    }

    fn parse_list_items(&mut self) {
        while self.current() == "#GIMMEH" && self.tokens[self.position + 1].as_str() == "ITEM" {
            let start_len = self.ast.len();
            self.expect("#GIMMEH");
            self.expect("ITEM");
            self.parse_inner_list();
            self.expect("#MKAY");
            let kids = self.ast.drain(start_len..).collect();
            self.ast.push(AST::ListItem { items: kids });
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

        let url_node = self.ast.pop().unwrap();
        let url = match url_node {
            AST::Text(s) => s,
            _ => "".to_string(),
        };
        self.ast.push(AST::Audio(url));
    }

    fn parse_video(&mut self) {
        self.expect("#GIMMEH");
        self.expect("VIDZ");
        self.parse_text();
        self.expect("#MKAY");

        let url_node = self.ast.pop().unwrap();
        let url = match url_node {
            AST::Text(s) => s,
            _ => "".to_string(),
        };
        self.ast.push(AST::Video(url));
    }

    fn parse_newline(&mut self) {
        self.expect("#GIMMEH");
        self.expect("NEWLINE");
        self.ast.push(AST::Newline);
    }

    fn parse_text(&mut self) {
        let tok = self.current();
        if tok.starts_with("TEXT(") && tok.ends_with(')') {
            let txt = Self::text_content(tok);
            self.position += 1;
            self.ast.push(AST::Text(txt));
        } else {
            self.error("TEXT()", "parse_text");
        }
    }
}
