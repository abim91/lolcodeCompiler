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
}

impl LolCompiler {
    pub fn new(source: &str) -> Self {
        Self {
            lexer: LolLexer::new(&source),
            current: String::new(),
            tokens: Vec::new(),
            parser: None,
            tree: Vec::new(),
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
        self.parser = Some(parser);
        self.parse();

        let mut sem_analyzer = SemanticAnalyzer::new();
        if let Some(ast) = self.tree.first() {
            sem_analyzer.check_program(ast); // exits on first semantic error
            let html = sem_analyzer.generate(ast);
            print!("{}", html);
        }
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
