/// Trait for a simple lolcompiler front-end. 
/// Errors should cause immediate exit inside the implementation.
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

struct lolCompiler{

}

/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {

    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char;

    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char);

    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool;
}

struct LolcodeLexicalAnalyzer{
    input:Vec<char>,//the type `str` cannot be indexed by `usize` when set to string
    index: usize,
    potential_token: String
}
impl LexicalAnalyzer for LolcodeLexicalAnalyzer{
    /// Return the next character from the input.
    /// If input is exhausted, should terminate the program.
    fn get_char(&mut self) -> char{//changed the 
        if self.index < self.input.len(){
            let current_char = self.input[self.index];
            self.index += 1;
            current_char
        }
        else{
            std::process::exit(1);
        }

    }

    /// Add a character to the current potential token.
    fn add_char(&mut self, c: char){
        self.potential_token.push(c);
    }

    /// Lookup a potential token to determine if it is valid.
    /// Returns true if a valid token/lexeme, false otherwise.
    fn lookup(&self, s: &str) -> bool{
        
        return matches!(s,"#HAI" | "#KTHXBYE" | "#OBTW" | "#TLDR" | "#MAEK" | "#OIC" | "#GIMMEH" | "#MKAY" | "HEAD" | "TITLE" | "PARAGRAF" | "BOLD" | "ITALICS" | "LIST" | "ITEM" | "NEWLINE" | "SOUNDZ" | "VIDZ" | "#I HAZ" | "#IT IZ" | "#LEMME SEE");

    }

}



fn main() {
    println!("Hello, world!");
}
