use std::collections::HashMap;
use crate::syntax_analyzer::AST;
pub struct SemanticAnalyzer {
    scopes: Vec<HashMap<String, String>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            scopes: Vec::new(),
        }
    }

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }
    // 
    pub fn check_program(&mut self, root: &AST) {
        self.push_scope();
        self.visit(root);
        self.pop_scope();
    }

    fn define(&mut self, name: &str, value: &str) {
        if let Some(top) = self.scopes.last_mut() {
            top.insert(name.to_string(), value.to_string());
        }
    }

    fn lookup(&self, name: &str) -> Option<String> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.clone());
            }
        }
        None
    }
//Each element is appended to its' subarray which represents a scope.
    fn visit(&mut self, node: &AST) {
        match node {
            AST::Program { parts } => {
                for p in parts {
                    self.visit(p);
                }
            }

            AST::Paragraph { items }
            | AST::List { items }
            | AST::ListItem { items } => {
                self.push_scope();
                for item in items {
                    self.visit(item);
                }
                self.pop_scope();
            }

            AST::List { items } => {
                self.push_scope();
                for item in items {
                    self.visit(item);
                }
                self.pop_scope();
            }

            AST::ListItem { items } => {
                self.push_scope();
                for item in items {
                    self.visit(item);
                }
                self.pop_scope();
            }

            AST::VarDefine { name, value } => {
                self.define(name, value);
            }
            //For each variable use found it will check that it exists within its' own block/scope,
            //then parent scope and so on.
           AST::VarUse(name) => {
                if self.lookup(name).is_none() {
                    eprintln!(
                        "Static scope error: variable '{}' used before it was defined (or out of scope).",
                        name
                    );
                    std::process::exit(1);
                }
            }

            AST::Head { .. } => {}
            AST::Comment(_) => {}
            AST::Text(_) => {}
            AST::Bold(_) => {}
            AST::Italics(_) => {}
            AST::Audio(_) => {}
            AST::Video(_) => {}
            AST::Newline => {}
        }
    }
    /*Generates HTML code based on the Parse Tree. For each lolcode element,
    it will append an equivalent HTML elements to a output string.
    */
    fn parse_lolcode_html(&mut self, node: &AST, out: &mut String) {
        match node {
            AST::Program { parts } => {
                out.push_str("<html>\n");
                for p in parts {
                    self.parse_lolcode_html(p, out);
                }
                out.push_str("</html>\n");
            }

            AST::Comment(txt) => {
                out.push_str("<!-- ");
                out.push_str(txt);
                out.push_str(" -->\n");
            }

            AST::Head { title } => {
                out.push_str("<head>\n<title>");
                out.push_str(title);
                out.push_str("</title>\n</head>\n");
            }

            AST::Paragraph { items } => {
                out.push_str("<p>");
                self.push_scope();
                for it in items {
                    self.parse_lolcode_html(it, out);
                }
                self.pop_scope();
                out.push_str("</p>\n");
            }
            AST::List { items } => {
                out.push_str("<ul>\n");
                self.push_scope();
                for it in items {
                    self.parse_lolcode_html(it, out);
                }
                self.pop_scope();
                out.push_str("</ul>\n");
            }
            AST::ListItem { items } => {
                out.push_str("<li>");
                self.push_scope();
                for it in items {
                    self.parse_lolcode_html(it, out);
                }
                self.pop_scope();
                out.push_str("</li>\n");
            }

            AST::Bold(txt) => {
                out.push_str("<b>");
                out.push_str(txt);
                out.push_str("</b>");
            }
            AST::Italics(txt) => {
                out.push_str("<i>");
                out.push_str(txt);
                out.push_str("</i>");
            }
            AST::Audio(url) => {
                out.push_str("<audio controls>\n<source src=\"");
                out.push_str(url);
                out.push_str("\">\n</audio>");
            }
            AST::Video(url) => {
                out.push_str("<iframe src=\"");
                out.push_str(url);
                out.push_str("\"></iframe>\n");
            }
            AST::Newline => out.push_str("<br>\n"),

            AST::Text(t) => {
                out.push_str(t);
                if !t.is_empty() {
                    out.push(' ');
                }
            }

            AST::VarDefine { name, value } => {
                self.define(name, value);
            }

            AST::VarUse(name) => {
                if let Some(v) = self.lookup(name) {
                    out.push_str(&v);
                }
            }
        }
    }

    pub fn generate(&mut self, ast: &AST) -> String {
        let mut html = String::new();
        self.push_scope();                
        self.parse_lolcode_html(ast, &mut html);
        self.pop_scope();
        html
    }
}