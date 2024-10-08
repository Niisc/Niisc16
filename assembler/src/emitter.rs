use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::Bytes;
use std::iter::Enumerate;
use std::ops::{Shl, BitOr};

use crate::lexer::{TokenType, Token};
use crate::parser::Label;

lazy_static! {
    static ref TOKEN_TO_BYTE: HashMap<TokenType, u8> = {
        let mut m = HashMap::new();
        //ALU
        m.insert(TokenType::ADD,    0b00000000);
        m.insert(TokenType::SUB,    0b00000100);
        m.insert(TokenType::AND,    0b00001000);
        m.insert(TokenType::OR,     0b00001100);
        m.insert(TokenType::MULU,   0b00010000);
        m.insert(TokenType::MULSW,  0b00010100);
        m.insert(TokenType::MULSB,  0b00011000);
        m.insert(TokenType::NOT,    0b00011100);
        m.insert(TokenType::XOR,    0b00100000);
        m.insert(TokenType::DIVU,   0b00100100);
        m.insert(TokenType::DIVSW,  0b00101000);
        m.insert(TokenType::DIVSB,  0b00101100);
        m.insert(TokenType::SHL,    0b00110000);
        m.insert(TokenType::SHR,    0b00110100);
        m.insert(TokenType::CMP,    0b00111000);
        m.insert(TokenType::LNOT,   0b00111100);

        //Imm
        m.insert(TokenType::IMM,    0b01000000);

        //Conditions
        m.insert(TokenType::JMP,    0b10000000);
        m.insert(TokenType::JZ,     0b10000100);
        m.insert(TokenType::JNZ,    0b10001000);

        //Misc
        m.insert(TokenType::PUSH,   0b11000000);
        m.insert(TokenType::POP,    0b11000100);
        m.insert(TokenType::CALL,   0b11001000);
        m.insert(TokenType::RET,    0b11001100);
        m.insert(TokenType::LDR,    0b11010000);
        m.insert(TokenType::STR,    0b11010100);
        m.insert(TokenType::MOV,    0b11011000);


        //Registers
        m.insert(TokenType::AX,  0b0000);
        m.insert(TokenType::AS,  0b0001);
        m.insert(TokenType::BX,  0b0010);
        m.insert(TokenType::BS,  0b0011);
        m.insert(TokenType::CX,  0b0100);
        m.insert(TokenType::CS,  0b0101);
        m.insert(TokenType::DX,  0b0110);
        m.insert(TokenType::DS,  0b0111);
        m.insert(TokenType::EX,  0b1000);
        m.insert(TokenType::ES,  0b1001);
        m.insert(TokenType::DB,  0b1010);
        m.insert(TokenType::ME,  0b1011);
        m.insert(TokenType::RA,  0b1100);
        m.insert(TokenType::SP,  0b1101);
        m.insert(TokenType::IP,  0b1110);
        m.insert(TokenType::IO,  0b1111);

        m
    };
}

pub struct Emitter<'a> {
    code: Vec<u16>,
    filename: &'a String,
    all_labels: &'a Vec<Label>,
    all_tokens: &'a Vec<Token>,
    current_token: Token,
    all_tokens_iter: Enumerate<std::vec::IntoIter<Token>>
}

impl<'a> Emitter<'a> {
    pub fn init(filename: &'a String, labels: &'a Vec<Label>, tokens: &'a Vec<Token>) -> Emitter<'a> {
        Emitter {
            code: Vec::new(),
            filename: filename,
            all_labels: labels,
            all_tokens: tokens,
            current_token: tokens[0].clone(),
            all_tokens_iter: tokens.clone().into_iter().enumerate(),

        }
    }
    //Function to emit all assembly code to machine code
    pub fn emit_all(&mut self) -> &mut Vec<u16> {

        while self.current_token.token != TokenType::EOF {

            //skipp
            if self.current_token.token == TokenType::IDENT || self.current_token.token == TokenType::SECTION || self.current_token.token == TokenType::TEXT || self.current_token.token == TokenType::COLON{
                self.next_token();
                continue;
            }
            //push opcode
            println!("{}", self.current_token.token);
            let mut num: u16 = (*TOKEN_TO_BYTE.get(&self.current_token.token).expect("could not find token in emitter") as u16).shl(8);

            match self.current_token.token {
                // 1: instruction
                //N type
                TokenType::NOP | TokenType::RET => {
                    //do nothing lol
                    println!("1: {:#b}, {}", num,num);
                }

                // 2: instruction argument
                //I type
                TokenType::IMM  => {
                    self.next_token();
                    if self.check_label().unwrap() {
                        //Is label
                        let c_label = self.all_labels.iter().find(|&x| x.name == self.current_token.data).expect("could not find label?");
                        let current_line = c_label.declared_line.unwrap();
                        num = num.bitor(current_line as u16);

                    }else {
                        //Is number
                        println!("{:?}", self.current_token);
                        num = num.bitor(&self.current_token.data.parse::<u16>().expect("Can not parse number after imm"));
                    }
                    println!("2: {:#b}, {}", num,num);
                }


                TokenType::NOT | TokenType::LNOT | TokenType::JMP | TokenType::PUSH | TokenType::POP | TokenType::CALL => {
                    self.next_token();
                    num = num.bitor((*TOKEN_TO_BYTE.get(&self.current_token.token).unwrap() as u16).shl(6));
                    println!("2: {:#b}, {}", num,num);
                }

                // 4: instruction arg comma arg
                //RR type
                TokenType::ADD | TokenType::SUB | TokenType::AND | TokenType::OR | TokenType::MULU | TokenType::MULSW | TokenType::MULSB | TokenType::XOR | TokenType::DIVU | TokenType::DIVSB | TokenType::DIVSW | TokenType::SHL | TokenType::SHR | TokenType::CMP | TokenType::JZ | TokenType::JNZ | TokenType::LDR | TokenType::STR | TokenType::MOV => {
                    self.next_token();
                    num = num.bitor((*TOKEN_TO_BYTE.get(&self.current_token.token).unwrap() as u16).shl(6));
                    //skip 2 because it's a comma
                    self.next_token();
                    //comma
                    self.next_token();

                    num = num.bitor((*TOKEN_TO_BYTE.get(&self.current_token.token).unwrap() as u16).shl(2));

                    println!("3: {:#b}, {}", num,num);
                }


                _ => {panic!("Token not handled? {:?}", &self.current_token.token)}
            }
            self.code.push(num);
            self.next_token();
        }
        return  &mut self.code;
        
    }

    fn check_label(&mut self) -> Result<bool, &str>{
        //is label?
        if self.current_token.token == TokenType::IDENT {
            //was it declared?
            let c_label = self.all_labels.iter().find(|&x| x.name == self.current_token.data).expect("could not find label?");
            if c_label.declared_line.is_some() {
                return Ok(true);
            }
            return Err("Error finding defined line for label");
        }else {
            return Ok(false);
        }
    }
    
    //TODO: need to remove this later
    //this function is old, use emit all
    /*
    fn emit_line(&mut self, line_of_code : &[Token]) {
        match line_of_code.len() {
            //add new line after every call here

            //1: instruction
            1 => {
                let num:u16 = (*TOKEN_TO_BYTE.get(&line_of_code[0].token).unwrap() as u16).shl(8);
                self.code.push_str(num.to_string().as_str());
            }
    
            // 2: instruction arg
            2 => {
                let mut num:u16 = 0;
                if line_of_code[0].token == TokenType::IMM {
                    num = (*TOKEN_TO_BYTE.get(&line_of_code[0].token).unwrap() as u16).shl(8);
                    num = num.bitor(line_of_code[1].data.parse::<u16>().expect("Can not parse number after imm"))
                }else {
                    num = (*TOKEN_TO_BYTE.get(&line_of_code[0].token).unwrap() as u16).shl(8);
                    num = num.bitor((*TOKEN_TO_BYTE.get(&line_of_code[1].token).unwrap() as u16).shl(6));
                }
                println!("2: {:#b}, {}", num,num);
                self.code.push_str(num.to_string().as_str());
            }
    
            // 4: instruction arg comma arg
            4 => {
                let mut num:u16 = (*TOKEN_TO_BYTE.get(&line_of_code[0].token).unwrap() as u16).shl(8);
                num = num.bitor((*TOKEN_TO_BYTE.get(&line_of_code[1].token).unwrap() as u16).shl(6));
                //skip 2 because it's a comma
                num = num.bitor((*TOKEN_TO_BYTE.get(&line_of_code[3].token).unwrap() as u16).shl(2));

                println!("3: {:#b}, {}", num,num);
                self.code.push_str(num.to_string().as_str());
            }
    
            _=> panic!("Received too many/little tokens in emit_line. Amount received: {}", line_of_code.len())
        }
        self.code.push('\n');

    
    }
    

    

    pub fn write_to_file(file_name: &String) {
        
    }
     */

    fn next_token(&mut self) {
        self.current_token = self.all_tokens_iter.next().unwrap().1;
    }

}




