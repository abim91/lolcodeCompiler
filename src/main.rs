mod lexer;
mod syntaxAnalyzer;
mod semanticAnalyzer;

use lexer::LolLexer;
use syntaxAnalyzer::{LolCodeSyntaxAnalyzer, SyntaxAnalyzer};
use semanticAnalyzer::ScopeChecker;
// use crate::syntaxAnalyzer::scope_checker;

fn main() {
    let src = "#HAI
		#OBTW This is a LOLCODE Markdown source file #TLDR
		#MAEK HEAD
			#GIMMEH TITLE The Simpsons #MKAY
		#OIC
		#MAEK PARAGRAF
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
		#OBTW This is a LOLCODE Markdown source file #TLDR
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
					#OBTW
					Going to Japan 2025
					#TLDR
					#KTHXBYE
					";
    let src4 = "
	#HAI
#MAEK HEAD 
	#GIMMEH TITLE Test 6 #MKAY
#OIC
	#KTHXBYE
	";

	let test7 = "#HAI
#OBTW This test case assess that your compiler reports an undefined variable. #TLDR
#MAEK HEAD 
	#GIMMEH TITLE Test 7 #MKAY
#OIC

#MAEK PARAGRAF
	#GIMMEH ITALICS What is the meaning of life. #MKAY 
	The meaning of #GIMMEH BOLD life #MKAY is #LEMME SEE answer #MKAY
#OIC
#KTHXBYE
";

let test8 = "#HAI
#OBTW This test case assess that your compiler resolves variable scoping. #TLDR
#MAEK HEAD 
	#GIMMEH TITLE Test 8 #MKAY
#OIC

#I HAZ answer #IT IZ happiness #MKAY

#MAEK PARAGRAF
	#I HAZ answer #IT IZ 42 #MKAY
	#GIMMEH ITALICS What is the meaning of life. #MKAY 
	The meaning of #GIMMEH BOLD life #MKAY is #LEMME SEE answer #MKAY
#OIC

#MAEK PARAGRAF No. The meaning of life is really #LEMME SEE answer #MKAY #OIC

#KTHXBYE
";

let test9 = "#HAI
#OBTW This test case assess that your compiler resolves multiple variable. #TLDR
#MAEK HEAD 
	#GIMMEH TITLE Test 9 #MKAY
#OIC

#I HAZ myanswer #IT IZ happiness #MKAY

#MAEK PARAGRAF
	#I HAZ answer #IT IZ 42 #MKAY
	#GIMMEH ITALICS What is the meaning of life. #MKAY 
	The meaning of #GIMMEH BOLD life #MKAY is #LEMME SEE answer #MKAY
#OIC

#MAEK PARAGRAF No. The meaning of life is really #LEMME SEE myanswer #MKAY #OIC
 
#KTHXBYE

";
    let c = LolLexer::new(test9);
    let mut parser = LolCodeSyntaxAnalyzer::collect_tokens(c);
    parser.parse_lolcode();
	println!("{:#?}", parser.ast);
	println!("Finished Parsing");

	let mut checker = ScopeChecker::new();

	checker.check_program(&parser.ast[0]);
    
	// println!("{:?}",parser.blocks);
    // loop {
    //     let t = c.next_token();
    //     println!("{t}");
    //     if t == "EOF" {
    //         break;
    //     }
    // }
	 
}
