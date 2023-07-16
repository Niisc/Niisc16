use core::panic;
use std::{iter::{Peekable, Enumerate}, str::Chars, collections::hash_map};

use crate::{lexer::{Token, get_token, TokenType}, emitter::Emitter};

pub struct Parser<'a> {
    current_token: Token,
    peek_token: Token,
    label_is_declared: Vec<String>,
    label_is_gotoed: Vec<String>,
    symbols: Vec<String>,
    str: &'a String,
    iter: &'a mut Peekable<Enumerate<Chars<'a>>>,
    current_tokens: Vec<Token>,
    //program specific stuff
    start_defined: bool,
    text_defined: bool,
    data_defined: bool,
    current_section: Option<TokenType>

}

impl<'a> Parser<'a> {
    pub fn init(code: &'a String, ite: &'a mut Peekable<Enumerate<Chars<'a>>>) -> Parser<'a> {
        Parser { 
            current_token: get_token(&code, ite),
            peek_token: get_token(&code, ite), 
            label_is_declared: Vec::new(), // labels declared
            label_is_gotoed: Vec::new(), // jumps to lables using jmp jnz jz
            symbols: Vec::new(), // variables in .data
            str: code,
            iter: ite,
            current_tokens: Vec::new(),

            start_defined: false,
            text_defined: false,
            data_defined: false,
            current_section: None
        }
    }

    pub fn program(&'a mut self, emitter: &mut Emitter) {

        while self.current_token.token == TokenType::NEWLINE {
            self.next_token()
        }

        self.current_tokens.clear();
        self.current_tokens.push(self.current_token.clone());

        while self.current_token.token != TokenType::EOF {
            self.instruction(emitter);
        }

        //need to check here that _start, .text and .data were defined.

    }
    //need to add a warning for when a 16 bit and 8 bit register are being used
    //need to check that instructions are under text and variables are under data
    fn instruction(&mut self, emitter: &mut Emitter) {
        match self.current_token.token {

            TokenType::SECTION => {
                self.next_token();

                match self.current_token.token {
                    TokenType::DATA => {
                        if self.data_defined { panic!(".data section already defined")}
                        self.current_section = Some(TokenType::DATA);
                        self.data_defined = true;

                    }
                    TokenType::TEXT => {
                        if self.text_defined { panic!(".text section already defined")}
                        self.current_section = Some(TokenType::TEXT);
                        self.text_defined = true;

                    }
                    _ => panic!("Only sections allowed are \".data\" and \".text\".\nFound: \"{}\"", self.current_token.token.to_string())
                }
            }
            // make it so it can be on the same line (use continue)
            TokenType::_START => {
                if self.start_defined {
                    panic!("There's already a start defined");
                }
                self.next_token();
                if !self.check_token(TokenType::COLON) {
                    panic!("Need a colon after start label as is the case with all other labels");
                }
                self.start_defined = true;
            }

            TokenType::IDENT => {
                match self.current_section.clone().expect("Can't declare anything before .text or .data segment") {
                    TokenType::DATA => {
                        if self.symbols.contains(&self.current_token.data) {
                            panic!("Multiple variables with the same name found")
                        }
                        self.next_token();
                        if self.check_token(TokenType::COLON) {
                            panic!("Need to add a colon after the variable name definition in .data")
                        }
                        self.symbols.push(self.current_token.data.clone());
                    },
                    TokenType::TEXT => {
                        if self.label_is_declared.contains(&self.current_token.data) {
                            panic!("Multiple labels with the same name found")
                        }
                        self.next_token();
                        if self.check_token(TokenType::COLON) {
                            panic!("Need to add a colon after the label name definition in .data")
                        }
                        self.label_is_declared.push(self.current_token.data.clone());
                    },
                    _ => panic!("Found an unexpected token in ident")
                }
            }

            

            TokenType::IMM => {
                self.next_token();
                if !self.check_token(TokenType::NUMBER) {
                    panic!("A number is needed after imm. Found: \"{}\" \"{}\"",self.current_token.token, self.current_token.data)
                }
                if self.current_token.data.parse::<u16>().expect("Can not parse number after imm") > 16383 {
                    panic!("The number provided after imm is too large. Max: 16383. Found: {}", self.current_token.data.parse::<u16>().expect("Can not parse number after imm"));
                }
                emitter.emit_line(&self.current_tokens);
                self.next_token();
            }

            TokenType::ADD | TokenType::SUB | TokenType::AND | TokenType::OR | TokenType::MULU | TokenType::MULSW | TokenType::MULSB | TokenType::XOR | TokenType::DIVU | TokenType::DIVSB | TokenType::DIVSW | TokenType::SHL | TokenType::SHR | TokenType::CMP => {
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
                    emitter.emit_line(&[Token{token: TokenType::IMM, data: "imm".to_string()}, self.current_token.clone()]);
                    self.current_tokens.pop(); // remove the number
                    self.current_tokens.push(Token{token: TokenType::CX, data: "cx".to_string()});
                }

                emitter.emit_line(&self.current_tokens);
                self.next_token();
            }

            

            TokenType::JNZ | TokenType::JZ => {
                self.next_token();
                if !self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to jump to when using jz/jnz. Found: {}", self.current_token.token);
                }
                if self.check_token(TokenType::IDENT) && !self.check_token(TokenType::NUMBER) && !self.check_token_register() {
                    panic!("Need to add a label, number or register to jump to when using jz/jnz. Found: {}", self.current_token.token);
                }

                //how do we handle the labels
                //make a vector to hold all of the tokens and remove the string at the emitter

                self.label_is_gotoed.push(self.current_token.data.clone());
            }

            TokenType::JMP => {

            }

            _ => panic!("Token type not handled in ident? {}", self.current_token.token.to_string())
        }
        if self.current_token.token == TokenType::EOF {
            return;
        }
        self.new_line();
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
        
        self.current_token = self.peek_token.clone();
        self.current_tokens.push(self.current_token.clone());
        self.peek_token = get_token(self.str, self.iter);
        //can go on forever, need to check
    }
}
