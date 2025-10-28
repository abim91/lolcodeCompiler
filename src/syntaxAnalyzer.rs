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
