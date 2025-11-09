use std::fs::File;
use std::io::Write;

use crate::lexer::LolLexer;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::syntax_analyzer::{AST, LolCodeSyntaxAnalyzer, SyntaxAnalyzer};
pub trait Compiler {
    fn compile(&mut self, source: &str);
    fn next_token(&mut self) -> String;
    fn parse(&mut self);
    fn current_token(&self) -> String;
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

    ///This function creates Syntax Analyzer and Semantics analyzer.
    /// It calls on the lexer to scan the src. If successful, runs the parser
    ///then runs the semantics analysis.
    
    fn compile(&mut self, source: &str) {
       
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
        self.parser = Some(parser);
        self.parse();
    

        let mut sem_analyzer = SemanticAnalyzer::new();
        let mut html = String::new();
        if let Some(ast) = self.tree.first() {
            sem_analyzer.check_program(ast); //The parse tree created using teh Parser is passed for semantics analysis
            html = sem_analyzer.generate(ast);
        }
        //Creates and output the HTML file.
        let mut file: File = File::create(self.srcFileName.clone()).expect("Failed to create file");

        file.write_all(html.as_bytes())
            .expect("Failed to write to file");
    }

    fn next_token(&mut self) -> String {//initates the lexer
        let token = self.lexer.next_token();
        self.current = token.clone();
        return token;
    }

    /// Runs the syntax analyzer starting from <lolcode>.
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
