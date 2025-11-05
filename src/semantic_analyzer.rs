use crate::syntax_analyzer::AST;
pub struct SemanticAnalyzer {
    scopes: Vec<Vec<String>>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer { scopes: Vec::new() }
    }

    pub fn check_program(&mut self, root: &AST) {
        self.push_scope();
        self.visit(root);
        self.pop_scope();
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: &str) {
        if let Some(cur) = self.scopes.last_mut() {
            cur.push(name.to_string());
        }
    }

    fn lookup(&self, name: &str) -> bool {
        let mut i = self.scopes.len();
        while i > 0 {
            i -= 1;
            if self.scopes[i].contains(&name.to_string()) {
                return true;
            }
        }
        false
    }

    fn visit(&mut self, node: &AST) {
        match node {
            AST::Program { parts } => {
                for p in parts {
                    self.visit(p);
                }
            }

            AST::Paragraph { items } => {
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

            AST::VarDefine { name, .. } => {
                self.define(name);
            }

            AST::VarUse(name) => {
                if !self.lookup(name) {
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

    fn parse_lolcode_html(node: &AST, out: &mut String) {
        match node {
            AST::Program { parts } => {
                out.push_str("<html>\n");
                for p in parts {
                    Self::parse_lolcode_html(p, out);
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
                for it in items {
                    Self::parse_lolcode_html(it, out);
                }
                out.push_str("</p>\n");
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
            AST::List { items } => {
                out.push_str("<ul>\n");
                for it in items {
                    Self::parse_lolcode_html(it, out);
                }
                out.push_str("</ul>\n");
            }
            AST::ListItem { items } => {
                out.push_str("<li>");
                for it in items {
                    Self::parse_lolcode_html(it, out);
                }
                out.push_str("</li>\n");
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
            AST::VarDefine { .. } | AST::VarUse(_) => {}
        }
    }

    pub fn generate(&self, ast: &AST) -> String {
        let mut h = String::new();
        Self::parse_lolcode_html(ast, &mut h);
        return h;
    }
}
