use core::panic;
use std::{iter::{Peekable, Enumerate}, str::Chars};

use crate::{lexer::{Token, get_token, TokenType}, emitter::Emitter};

pub struct Label {
    name: String,
    declared_line: Option<u16>
}

pub struct Parser<'a> {
    current_token: Token,
    peek_token: Token,
    symbols: Vec<String>,
    str: &'a String,
    iter: &'a mut Peekable<Enumerate<Chars<'a>>>,
    current_tokens: Vec<Token>,
    all_tokens: Vec<Token>,
    all_tokens_iter: Option<Enumerate<std::vec::IntoIter<Token>>>,
    all_labels: Vec<Label>,
    current_line: u16,
    //program specific stuff
    current_token_num: u16,
    start_defined: bool,
}

impl<'a> Parser<'a> {
    pub fn init(code: &'a String, ite: &'a mut Peekable<Enumerate<Chars<'a>>>) -> Parser<'a> {
        Parser { 
            current_token: get_token(&code, ite),
            peek_token: get_token(&code, ite), 
            symbols: Vec::new(), // variables in .data
            str: code,
            iter: ite,
            current_tokens: Vec::new(),
            all_tokens: Vec::new(),
            all_tokens_iter: None,
            all_labels: Vec::new(),
            current_line: 0,
            
            current_token_num: 0,
            start_defined: false,
        }
    }

    pub fn program(&'a mut self) -> Result<(&Vec<Token>, &Vec<Label>), &'static str> {

        //push all tokens to a vector for easy access
        self.all_tokens.push(self.current_token.clone());
        self.all_tokens.push(self.peek_token.clone());
        loop {
            self.all_tokens.push(get_token(self.str, self.iter));
            if self.all_tokens.last().unwrap().token == TokenType::EOF {
                break;
            }
        }

        self.all_tokens_iter = Some(self.all_tokens.clone().into_iter().enumerate());

        //check sections once
        match self.check_sections() {
            Ok(_) =>{},
            Err(x) => return Err(x),
        }

        //skip the first few lines
        while  self.check_token(TokenType::NEWLINE) {
            self.next_token()
        }


        self.current_tokens.clear();
        self.current_tokens.push(self.current_token.clone());


        while !self.check_token(TokenType::EOF) {
            self.section();
        }

        Ok((&self.all_tokens, &self.all_labels))

        //need to check here that _start, .text and .data were defined.

    }


    fn section(&mut self) {
        
        loop {

            if !self.check_token(TokenType::SECTION) {
                panic!("Expected a section, Found: {}", self.current_token.token.to_string());
            }

            self.next_token();

            if self.check_token(TokenType::TEXT) {
                self.next_token();
                while !self.check_token(TokenType::SECTION) && !self.check_token(TokenType::EOF) {
                    self.instruction();
                }
            }
            
            if self.check_token(TokenType::DATA) {

                self.next_token();

                while !self.check_token(TokenType::SECTION) && !self.check_token(TokenType::EOF) {
                    //need to add data intructions
                    self.next_token();
                }
            }

            

            if self.check_token(TokenType::EOF) { break; }
        }

    }

    // need to add a warning for when a 16 bit and 8 bit register are being used
    // need to check that instructions are under text and variables are under data
    fn instruction(&mut self) {
        match self.current_token.token {

            // make it so it can be on the same line (use continue)

            //add imm label
            TokenType::IMM => {
                self.next_token();
                match self.current_token.token {
                    TokenType::NUMBER => {
                        if self.current_token.data.parse::<u16>().expect("Can not parse number after imm") > u16::MAX {
                            panic!("The number provided after imm is too large. Max: {}. Found: {}", u16::MAX, self.current_token.data.parse::<u16>().expect("Can not parse number after imm"));
                        }
                    }
                    TokenType::LABEL => {
                        //have to replace with number
                        let c_label = self.all_labels.iter().position(|x| x.name == self.current_token.data);
                        if c_label.is_none() {
                            self.all_labels.push(Label{name: self.current_token.data.clone(), declared_line: None})
                        }
                    }
                    _ => {
                        panic!("A number/label is needed after imm. Found: \"{}\" \"{}\"",self.current_token.token, self.current_token.data)
                    }
                }
                
                self.next_token();
            }

            TokenType::ADD | TokenType::SUB | TokenType::AND | TokenType::OR | TokenType::MULU | TokenType::MULSW | TokenType::MULSB | TokenType::XOR | TokenType::DIVU | TokenType::DIVSB | TokenType::DIVSW | TokenType::SHL | TokenType::SHR | TokenType::CMP | TokenType::MOV => {
                let c_token =  self.current_token.token.to_string();
                self.next_token();
                if !self.check_token_register() {
                    panic!("A register is needed as first argument of: {}", c_token)
                }
                self.next_token();
                if !self.check_token(TokenType::COMMA) {
                    panic!("A comma is needed between arguments when using: {}", c_token)

                }
                self.next_token();
                if !self.check_token_register() && !self.check_token(TokenType::NUMBER) {
                    panic!("A register or a number is needed as second argument of: {}", c_token)
                }
                if self.check_token(TokenType::NUMBER) {
                    // add warning for over writing CX
                    //emitter.emit_line(&[Token{token: TokenType::IMM, data: "imm".to_string()}, self.current_token.clone()]);
                    self.current_tokens.pop(); // remove the number
                    self.current_tokens.push(Token{token: TokenType::CX, data: "cx".to_string()});
                }
                //add register
                //emitter.emit_line(&self.current_tokens);
                self.next_token();
            }

            

            TokenType::JNZ | TokenType::JZ => {
                self.next_token();
                if !self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to jump to when using jz/jnz. Found: {}", self.current_token.token);
                
                }
                if self.check_token(TokenType::IDENT) {
                    
                    for x in self.all_labels.iter() {
                        if *x.name == self.current_token.data {
                            return;
                        }
                    }
                    //TODO: make this more memory "performant"
                    self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                }

                self.next_token();
                if !self.check_token(TokenType::COMMA) {
                    panic!("Need to add a comma in between arguments when using jz/jnz");
                }
                self.next_token();
                if !self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to compare when using jz/jnz. Found: {}", self.current_token.token);
                }
                self.next_token();

            }

            TokenType::JMP => {
                self.next_token();
                if self.check_token(TokenType::IDENT) {
                    let mut found: bool = false;
                    for x in self.all_labels.iter() {
                        if *x.name == self.current_token.data {
                            found = true;
                            break;
                        }
                    }
                    //TODO: make this more memory "performant"
                    if !found {
                        self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                    }
                }
                self.next_token();

            }

            //here we add the tokens we dont have to handle
            TokenType::IDENT =>{
                if self.current_token.data.to_lowercase() == "_start" {
                    if self.start_defined {
                        //learn to handle errors lol
                        panic!("Found multiple _starts");
                    }
                    self.start_defined = true;
                }
                //assume its a label
                match self.all_labels.iter_mut().find(|x| x.name == self.current_token.data) {
                    Some(x) => {
                        if x.declared_line.is_some() {
                            panic!("Label {} was declared more than once", x.name);
                        }
                        x.declared_line = Some(self.current_line);
                    }
                    None => {
                        self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: Some(self.current_line) })
                    }
                };

                self.next_token();
                if !self.check_token(TokenType::COLON) {
                    panic!("Expected colon \":\" after each label");
                }
                //find index of the label

                self.next_token();
            },

            _ => panic!("Token type not handled in instruction fn? Found: {}", self.current_token.token.to_string())
        }
        if self.current_token.token == TokenType::EOF {
            return;
        }
        self.new_line();
    }

    fn find_label(&mut self) -> Result<u16, &'static str> {
        let mut found_token: Option<u16> = None;
        let mut current_section: Option<&Token> = None;
        Ok(1)

    }

    fn check_sections(&mut self) -> Result<(), &'static str>{

        let mut iter: Enumerate<std::slice::Iter<'_, Token>> = self.all_tokens.iter().enumerate();
        let mut text_defined: bool  = false;
        let mut data_defined: bool  = false;        

        loop {
            let (_,mut x) = iter.next().unwrap();
            if x.token == TokenType::SECTION {
                (_, x) = iter.next().unwrap();

                match x.token {
                    TokenType::DATA => {
                        if data_defined {
                            return Err("Data already defined");
                        }
                        data_defined = true;
                    }

                    TokenType::TEXT => {
                        if text_defined {
                            return Err("Text already defined");
                        }
                        text_defined = true;
                    }
                    
                    _ => return Err("Found unexpected token after SECTION."),
                }

            }

            if x.token == TokenType::EOF {
                break;
            }

        }
        if !text_defined {
            return Err("Could not find \"section .text\"")
        }
        Ok(())
    }

    fn check_token(&mut self, token: TokenType) -> bool {
        self.current_token.token  == token
    }
    
    fn check_token_register(&mut self) -> bool {
        TokenType::AX <= self.current_token.token && self.current_token.token <= TokenType::IO
    }

    fn check_peek(&mut self, token: TokenType) -> bool {
        self.peek_token.token  == token
    }

    fn new_line(&mut self) {
        self.match_token(TokenType::NEWLINE);
        while self.current_token.token  == TokenType::NEWLINE {
            self.next_token();
        }
        self.current_line+=1;
        self.current_tokens.clear();
        self.current_tokens.push(self.current_token.clone());
    }
    
    fn match_token(&mut self, token: TokenType) {
        if self.current_token.token  != token {
            panic!("expected {} found {}", token, self.current_token.token  );
        }
        self.next_token();
    }

    fn next_token(&mut self) {
        if self.peek_token.token == TokenType::EOF {
            self.current_token = self.peek_token.clone();
        }else {
            self.current_token = self.peek_token.clone();
            self.current_tokens.push(self.current_token.clone());
            self.peek_token = self.all_tokens_iter.as_mut().unwrap().next().expect("Failed to get next token").1;
        }
        
    }

    //checks if something containing a number was passed
    fn check_token_numeral(&mut self) -> bool {
        match self.current_token.token {

            TokenType::NUMBER =>{true},
            TokenType::IDENT=>{true},
            _ =>{
                if self.check_token_register() {
                    return true
                }
                false
            },
        }
    }
}
