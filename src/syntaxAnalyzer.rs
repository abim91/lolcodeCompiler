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
        let currentToken = self.tokens[self.position].to_string();
        if currentToken == tok {
            self.position += 1;
        } else {
            self.error(tok);
        }
    }
    fn error(&self, excep_token: &str) {
        eprintln!(
            "Syntax error at line {}, Expected {} token",
            self.position, excep_token
        );
        std::process::exit(1);
    }
}

impl SyntaxAnalyzer for LolCodeSyntaxAnalyzer{
     fn parse_lolcode(&mut self){
        self.expect("#HAI");


        self.expect("#KTHXBYE");
    }

     fn parse_head(&mut self){}        
    fn parse_title(&mut self){}         
    fn parse_comment(&mut self){}       
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
    fn parse_text(&mut self){}  

}
