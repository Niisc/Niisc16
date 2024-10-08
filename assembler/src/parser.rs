 use core::panic;
use std::{arch::x86_64, iter::{Enumerate, Peekable}, str::Chars};

use crate::{lexer::{Token, get_token, TokenType}, emitter::Emitter};
use crate::OutputLineFormat;

#[derive(Debug)]
pub struct Label {
    pub name: String,
    pub declared_line: Option<i16>
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
    current_line: i16,
    //program specific stuff
    current_token_num: u16,
    start_defined: bool,
    line_format: OutputLineFormat,
}

impl<'a> Parser<'a> {
    pub fn init(code: &'a String, ite: &'a mut Peekable<Enumerate<Chars<'a>>>,  format: OutputLineFormat) -> Parser<'a> {
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
            line_format: format
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
        self.all_tokens_iter.as_mut().unwrap().next();
        self.all_tokens_iter.as_mut().unwrap().next();
 

        //check sections once
        match self.check_sections() {
            Ok(_) =>{},
            Err(x) => return Err(x),
        }

        //skip the first few lines
        while  self.check_token(TokenType::NEWLINE) {
            self.next_token()
        }

        //maybe remove this?
        self.current_tokens.clear();
        self.current_tokens.push(self.current_token.clone());
        

        while !self.check_token(TokenType::EOF) {
            self.section();
        }

        if self.all_tokens.len() == 0 {
            return Err("Size of all_tokens equal to zero?")
        }

        Ok((&self.current_tokens, &self.all_labels))

        //need to check here that _start, .text and .data were defined.

    }


    fn section(&mut self) {

        loop {

            if !self.check_token(TokenType::SECTION) {
                panic!("Expected a section, Found: {}, {:?}", self.current_token.token.to_string(), self.peek_token);
            }
            
            self.next_token();

            if self.check_token(TokenType::TEXT) {
                self.next_token();
                self.new_line();
                self.current_line=0; //reset because of previous use of new_line()
                while !self.check_token(TokenType::SECTION) && !self.check_token(TokenType::EOF) {
                    self.instruction();
                }
            }
            
            if self.check_token(TokenType::DATA) {
                //maybe put newline
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
                        //we only have 14 bits for number
                        if self.current_token.data.parse::<u16>().expect("Can not parse number after imm") > 16383 {
                            panic!("The number provided after imm is too large. Max: {}. Found: {}", 16383, self.current_token.data.parse::<u16>().expect("Can not parse number after imm"));
                        }
                    }
                    TokenType::IDENT => {
                        //TODO: have to replace with number

                        if !self.label_exists(&self.current_token.data) {
                            self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                        }

                    }
                    _ => {
                        panic!("A number/label is needed after imm. Found: \"{}\" \"{}\"",self.current_token.token, self.current_token.data)
                    }
                }
                
                self.next_token();
            }
            //syntax add regA, regB
            //add
            //regA
            //,
            //regB/Imm
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

                if !self.check_token_numeral() {
                    panic!("A register or a number is needed as second argument of: {}", c_token)
                }
                if  self.check_token(TokenType::IDENT) {
                    // add warning for over writing CX
                    //so essentialy we add an "imm number" before the instruction, and then substitute the number with cx
                    if !self.label_exists(&self.current_token.data) {
                        self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                    }
                    self.add_imm(false);
                    /*
                    let element = self.current_tokens.pop().unwrap();
                    self.current_tokens.insert(self.current_tokens.len()-4, Token{token: TokenType::IMM, data: "".to_string()});
                    self.current_tokens.insert(self.current_tokens.len()-4, element);
                    self.current_tokens.push(Token{token: TokenType::CX, data: "cx".to_string()});
                    */
                }
                
                //TODO: we dont need EX here?
                if self.check_token(TokenType::NUMBER) {
                    self.add_imm(true);
                }

                self.next_token();
            }

            //Syntax: JZ arg1, arg2
            //Jump to value stored in arg1 if arg2 is zero
            TokenType::JNZ | TokenType::JZ => {
                self.next_token();
                if !self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to jump to when using jz/jnz. Found: {}", self.current_token.token);
                }

                if self.check_token(TokenType::IDENT) {
                    if !self.label_exists(&self.current_token.data) {
                        self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                    }
                    self.add_imm(true);
                }

                if self.check_token(TokenType::NUMBER) {
                    self.add_imm(true);

                }
                //do nothing if register
                
                self.next_token();
                if !self.check_token(TokenType::COMMA) {
                    panic!("Need to add a comma in between arguments when using jz/jnz");
                }
                self.next_token();
                if !self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to compare when using jz/jnz. Found: {}", self.current_token.token);
                }
                
                if self.check_token(TokenType::IDENT) {
                    if !self.label_exists(&self.current_token.data) {
                        self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: None })
                    }
                    self.add_imm( false);
                }

                if self.check_token(TokenType::NUMBER) {
                    self.add_imm( false);
                }

                self.next_token();

            }

            TokenType::JMP => {
                self.next_token();
                if self.check_token(TokenType::IDENT) {

                    if !self.label_exists(&self.current_token.data) {
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
                        if self.line_format == OutputLineFormat::Separated8Bit {
                            x.declared_line = Some(self.current_line*2);
                        }else {
                            x.declared_line = Some(self.current_line);
                        }
                    }
                    None => {
                        if self.line_format == OutputLineFormat::Separated8Bit {
                            self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: Some(self.current_line*2) })
                        }else {
                            self.all_labels.push(Label { name: self.current_token.data.clone(), declared_line: Some(self.current_line) })
                        }
                        
                    }
                    
                };

                self.next_token();
                if !self.check_token(TokenType::COLON) {
                    panic!("Expected colon \":\" after each label");
                }
                self.current_line -=1;
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

    //use this when
    fn add_imm(&mut self, use_EX: bool ) {
        let element = self.current_tokens.pop().unwrap();

        if use_EX {
            self.current_tokens.push(Token{token: TokenType::EX, data: "".to_string()}); 
        }else {
            self.current_tokens.push(Token{token: TokenType::CX, data: "".to_string()});
        }

        for i in (0..self.current_tokens.len()).rev() {
            if  TokenType::ADD <= self.current_tokens.get(i).unwrap().token && self.current_tokens.get(i).unwrap().token <= TokenType::MOV {
                
                if use_EX {
                    self.current_tokens.insert(i, Token{token: TokenType::CX, data: "".to_string()});
                    self.current_tokens.insert(i, Token{token: TokenType::COMMA, data: "".to_string()});
                    self.current_tokens.insert(i, Token{token: TokenType::EX, data: "".to_string()});
                    self.current_tokens.insert(i, Token{token: TokenType::MOV, data: "".to_string()});
                    self.current_line += 1;

                }
                self.current_tokens.insert(i, element.clone());
                self.current_tokens.insert(i, Token{token: TokenType::IMM, data: "".to_string()});
                self.current_line += 1;
                break;
            }
        }
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
        if self.current_token.token  != TokenType::NEWLINE {
            panic!("expected {} found {}", TokenType::NEWLINE, self.current_token.token);
        }
        while self.current_token.token  == TokenType::NEWLINE {
            self.current_tokens.pop();
            self.next_token();
        }
        self.current_line+=1;
        //self.current_tokens.clear();
        //self.current_tokens.push(self.current_token.clone());
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
            
            self.peek_token = self.all_tokens_iter.as_mut().unwrap().next().expect("Failed to get next token").1;
        }
        self.current_tokens.push(self.current_token.clone());
        
    }

    //checks if something containing a number was passed
    fn check_token_numeral(&mut self) -> bool {
        match self.current_token.token {

            TokenType::NUMBER => {
                if self.current_token.data.parse::<u16>().expect("Can not parse number after imm") > 16383 {
                    panic!("The number provided after imm is too large. Max: {}. Found: {}", 16383, self.current_token.data.parse::<u16>().expect("Can not parse number after imm"));
                }
                true
                
            },
            TokenType::IDENT  => {true},
            TokenType::LABEL  => {true},
            _ =>{
                if self.check_token_register() {
                    return true
                }
                false
            },
        }
    }

    fn label_exists(&self, token_data: &str) -> bool {
        self.all_labels.iter().any(|x| x.name == token_data)
    }
}