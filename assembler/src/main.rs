mod lexer;
mod parser;
mod emitter;
use std::{io::ErrorKind, fs::{File, self}};
use crate::emitter::*;
use crate::lexer::*;
use crate::parser::*;

fn main() {
    let file_name = String::from("/Users/nico/Documents/Code stuff/Niisc16/assembler/fibonacci.nasm");

    let out_filename = String::from("/Users/nico/Documents/Code stuff/Niisc16/assembler/main.out");

    let code = fs::read_to_string(file_name).expect("error with opening / finding the file");

    if code.is_empty() {
        panic!("Empty file was provided");
    }

    let mut iter = code.chars().enumerate().peekable();

    let mut parser = Parser::init(&code, &mut iter);

    match parser.program() {
        Ok((all_tokens, all_labels)) => {
            let mut emitter = Emitter::init(&out_filename, all_labels, all_tokens);
        },
        Err(x) => panic!("{}",x),
    }
    
    
}

/*
    loop {
        let a = get_token(&code, &mut iter);
        println!("{:?}", a);
        if a.token == TokenType::EOF {
            break;
        }
    }
    */