mod compiler;
mod lexer;
mod semantic_analyzer;
mod syntax_analyzer;
use crate::compiler::{Compiler, LolCompiler};
use semantic_analyzer::SemanticAnalyzer;
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

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("File contents:\n{}", contents);

    let mut c = LolCompiler::new(&contents);
    c.compile(&contents);
}
