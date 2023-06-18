mod lexer;
use std::{io::ErrorKind, fs::{File, self}};
use crate::lexer::*;

fn main() {
    
    let file_name = String::from("/Users/nico/Documents/Code stuff/NiMi-iSC16/compiler/hello.npp");

    let mut code = fs::read_to_string(file_name).expect("error with opening / finding the file");

    code.push('\n'); //add this to make parsing easier

    let tokens: Vec<Token> = get_tokens(&code);
    
    let mut binding: std::iter::Enumerate<std::slice::Iter<Token>> = tokens.iter().enumerate();

    let mut parse: Parser = Parser::init(&mut binding);

    parse.program();
}
