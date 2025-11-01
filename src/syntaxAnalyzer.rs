use crate::lexer::LolLexer;
use substring::Substring;

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

pub struct LolCodeSyntaxAnalyzer {//parser
    pub tokens: Vec<String>,
    pub position: usize,
}

impl LolCodeSyntaxAnalyzer {
    //Using the lexer to get all of the valid tokens
    pub fn collectTokens(mut lexer: LolLexer) -> Self{
        let mut tokens: Vec<String> = Vec::new();
        let mut t = lexer.next_token();
        while t != "EOF"{
            tokens.push(t.clone());
            t = lexer.next_token();
        }
        tokens.push(t);
        LolCodeSyntaxAnalyzer{tokens: tokens, position:0}

    }

    fn expect(&mut self, tok: &str) {
        let currentToken = self.current().to_string();
        if currentToken == tok {
            self.position += 1;
        } else {
            self.error(tok,"expect()");
        }
    }

    fn error(&self, excep_token: &str,funcFrom:&str) {
        eprintln!(
            "Hello From {} function. Syntax error at line {}, Expected {} token",
            funcFrom,self.position, excep_token
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
}




impl SyntaxAnalyzer for LolCodeSyntaxAnalyzer{
     fn parse_lolcode(&mut self){
        self.expect("#HAI");
        
        while self.current() == "#OBTW" {
            self.parse_comment();
        }

         if self.current() == "#MAEK" {
            println!("we are here");
            self.parse_head();
         }


        self.expect("#KTHXBYE");
    }

    fn parse_head(&mut self){
        self.expect("#MAEK");
        self.expect("HEAD");
        self.parse_title();
        self.expect("#OIC");
    }        
    fn parse_title(&mut self){
        self.expect("#GIMMEH");
        self.expect("TITLE");
        while self.current().starts_with("TEXT(") && self.current().ends_with(')') {
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

    fn parse_body(&mut self){}           
    fn parse_paragraph(&mut self){}     
    fn parse_inner_paragraph(&mut self){}
    fn parse_inner_text(&mut self){}     
    fn parse_variable_define(&mut self){}
    fn parse_variable_use(&mut self){}
    fn parse_bold(&mut self){}  
    fn parse_italics(&mut self){}        
    fn parse_list(&mut self){}           
    fn parse_list_items(&mut self){}     
    fn parse_inner_list(&mut self){}     
    fn parse_audio(&mut self){}          
    fn parse_video(&mut self){}          
    fn parse_newline(&mut self){}        
    fn parse_text(&mut self){
        let tok = self.current();
        if tok.starts_with("TEXT(") && tok.ends_with(')') {
            
            self.position += 1;
            // let text = tok.substring(5, tok.len()-1);
            // text
        } else {
            self.error("TEXT()","partse_text");
        }
    }  

}
