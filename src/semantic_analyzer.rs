use crate::syntaxAnalyzer::AST;
pub struct ScopeChecker {
    scopes: Vec<Vec<String>>, 
}

impl ScopeChecker {
    pub fn new() -> Self {
        ScopeChecker {
            scopes: Vec::new(),
        }
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

            AST::Head {..} => {}
            AST::Comment(_) => {}
            AST::Text(_) => {}
            AST::Bold(_) => {}
            AST::Italics(_) => {}
            AST::Audio(_) => {}
            AST::Video(_) => {}
            AST::Newline => {}
        }
    }
}


