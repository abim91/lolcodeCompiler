use std::fs::File;
use std::io::Write;

use crate::lexer::LolLexer;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::syntax_analyzer::{AST, LolCodeSyntaxAnalyzer, SyntaxAnalyzer};
pub trait Compiler {
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str);

    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String;

    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self);

    /// Get the current token being processed.
    fn current_token(&self) -> String;

    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String);
}
pub struct LolCompiler {
    lexer: LolLexer,
    current: String,
    tokens: Vec<String>,
    parser: Option<LolCodeSyntaxAnalyzer>,
    tree: Vec<AST>,
    srcFileName: String,
}

impl LolCompiler {
    pub fn new(source: &str, sourceTitle: String) -> Self {
        Self {
            lexer: LolLexer::new(&source),
            current: String::new(),
            tokens: Vec::new(),
            parser: None,
            tree: Vec::new(),
            srcFileName: sourceTitle,
        }
    }
}
impl Compiler for LolCompiler {
    /// Begin the compilation process (entry point).
    fn compile(&mut self, source: &str) {
        self.lexer = LolLexer::new(&source);
        self.tokens.clear();

        let mut tk = self.next_token();
        while tk != "EOF" {
            self.tokens.push(tk.clone());
            tk = self.next_token();
        }

        let parser = LolCodeSyntaxAnalyzer {
            tokens: self.tokens.clone(),
            position: 0,
            ast: Vec::new(),
        };
        println!("{:#?}", parser.ast);
        self.parser = Some(parser);
        self.parse();
    

        let mut sem_analyzer = SemanticAnalyzer::new();
        let mut html = String::new();
        if let Some(ast) = self.tree.first() {
            sem_analyzer.check_program(ast); // exits on first semantic error
            html = sem_analyzer.generate(ast);
            print!("{}", html);
        }
        let mut file: File = File::create(self.srcFileName.clone()).expect("Failed to create file");

        file.write_all(html.as_bytes())
            .expect("Failed to write to file");
    }

    /// Get the next token from the lexical analyzer.
    fn next_token(&mut self) -> String {
        let token = self.lexer.next_token();
        self.current = token.clone();
        return token;
    }

    /// Run the syntax analyzer starting from <lolcode>.
    fn parse(&mut self) {
        if let Some(p) = self.parser.as_mut() {
            p.parse_lolcode(); // exits on syntax error
            self.tree = p.ast.clone();
        }
    }

    /// Get the current token being processed.
    fn current_token(&self) -> String {
        return self.current.clone();
    }

    /// Set the current token (typically used internally).
    fn set_current_token(&mut self, tok: String) {
        self.current = tok;
    }
}
