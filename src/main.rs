mod lexer;
mod syntaxAnalyzer;

use syntaxAnalyzer::{SyntaxAnalyzer, LolCodeSyntaxAnalyzer};
use crate::lexer::LolLexer;


fn main() {
    let src = "#HAI
		#OBTW This is a LOLCODE Markdown source file #TLDR
		#MAEK HEAD
			#GIMMEH TITLE The Simpsons #MKAY
		#OIC
		#I HAZ
			The Simpsons! #GIMMEH NEWLINE
			#GIMMEH SOUNDZ 
                http://www.televisiontunes.com/themesongs/The%20Simpsons.mp3 
                #MKAY
			#GIMMEH NEWLINE

			The members of the #GIMMEH BOLD Simpson #MKAY family are:
			#MAEK LIST
				#GIMMEH ITEM Homer Simpson #MKAY
				#GIMMEH ITEM Marge Simpson #MKAY
				#GIMMEH ITEM Bart Simpson #MKAY
				#GIMMEH ITEM Lisa Simpson #MKAY
				#GIMMEH ITEM Maggie Simpson #MKAY 
			#OIC
			#GIMMEH NEWLINE
			Lets watch now: #GIMMEH NEWLINE
			#GIMMEH VIDZ http://www.youtube.com/embed/zoO0s1ukcqQ #MKAY
		#OIC
	#KTHXBYE
";

    let src2 = "#HAI
#OBTW This test case assess that your compiler resolves a simple variable. #TLDR
#MAEK HEAD 
	#GIMMEH TITLE Test 6 #MKAY
#OIC

#MAEK PARAGRAF
	#I HAZ answer #IT IZ 42 #MKAY
	#GIMMEH ITALICS What is the meaning of life. #MKAY 
	The meaning of #GIMMEH BOLD life #MKAY is #LEMME SEE answer #MKAY
#OIC
#KTHXBYE
";

    let src3 = " 
					#HAI
					
					";

    let mut c = LolLexer::new(src3);
	let mut parser = LolCodeSyntaxAnalyzer::collectTokens(c);
	parser.parse_lolcode();
	println!("compelted");
    // loop {
    //     let t = c.next_token();
    //     println!("{t}");
    //     if t == "EOF" {
    //         break;
    //     }
    // }
}
