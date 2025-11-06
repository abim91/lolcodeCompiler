mod compiler;
mod lexer;
mod semantic_analyzer;
mod syntax_analyzer;
use crate::compiler::{Compiler, LolCompiler};
use std::env;
use std::fs;
use std::process;
use syntax_analyzer::{LolCodeSyntaxAnalyzer, SyntaxAnalyzer};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please Use: lolcompiler.exe  <filename>");
        process::exit(1);
    }
    let filename = &args[1];
    if !filename.ends_with(".lol") {
        eprintln!("The file must use a lol extension(ie filename.lol");
        process::exit(1);
    }

    let outputFileName = format!("{}.html", filename.trim_end_matches(".lol"));
    println!("{}", outputFileName);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let mut c = LolCompiler::new(&contents, outputFileName.clone());
    c.compile(&contents);
    

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let _ = Command::new("open")
            .arg("-a")
            .arg("Google Chrome")
            .arg(format!("./{}", outputFileName))
            .status();
    }
}
