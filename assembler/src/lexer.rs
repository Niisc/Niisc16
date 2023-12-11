use core::panic;
use std::{string::ToString, iter::{Peekable, Enumerate}, str::Chars, usize};
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

#[derive(Display, EnumIter, Debug, PartialEq, Eq, PartialOrd, Clone, Hash)]
pub enum TokenType {
   EOF,
	NEWLINE,
	IDENT,
	STRING,
	NUMBER,
	_START,
	COLON,
	COMMA,
	LABEL,
	VARIABLE,

	// Keywords.
	//registers
	AX,
	AS,
	BX,
	BS,
	CX,
	CS,
	DX,
	DS,
	EX,
	ES,
	DB,
	ME,
	RA,
	SP,
	IP,
	IO,
	TEXT = 95,
	DATA = 96,
	SECTION = 97,
	BYTE = 98,
	WORD = 99,
	// GLOBAL = 100, maybe add later but not really needed
    // ALU
	ADD = 102,
	SUB = 103,
	AND = 104,
	OR = 105,
	MULU = 106,
	MULSW = 107,
	MULSB = 108,
	NOT = 109,
	MULS = 110,
	XOR = 111,
	DIVU = 112,
	DIVSB = 113,
	DIVSW = 114,
	SHL = 115,
	SHR = 116,
	CMP = 117, // not really needed
	LNOT = 118,

    // IMMEDIATE
	IMM = 119,

    // CONDITIONS
	JMP = 120,
	JZ = 121,
	JNZ = 122,

	// MISC
	PUSH = 123,
	POP = 124,
	CALL = 125,
	RET = 126,
	LDR = 127,
	STR = 128,
	MOV = 129,

}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
	pub data: String,
	pub token: TokenType,
}

pub fn get_token(str: &String, iter: &mut Peekable<Enumerate<Chars<'_>>>) -> Token {
	

	loop {
		let ( mut i, mut c_char);
		
		match iter.next() {
			Some((a, c)) => {
				i=a;
				c_char = c;
			},
			None => return Token { data:'\0'.to_string(), token: TokenType::EOF },
		}


		match c_char {

			//skip comments
			'#' => {
				c_char = iter.next().unwrap().1;
				while c_char != '\n' {
					match iter.next() {
						Some((a, c)) => {
							i=a;
							c_char = c;
						},
						None => return Token { data:'\0'.to_string(), token: TokenType::EOF },
					}
				}
			}, 

			' ' | '\r' | '\t' => continue, //skip whitespaces

			'\"' => {
					(i,c_char) = iter.next().unwrap();
					let start_index = i;
					while c_char != '\"' {
						if c_char == '\r' || c_char == '\n' || c_char == '\t' || c_char == '\\' || c_char == '%' {
							panic!("unallowed char found in string at index {}", i);
						}
						(i,c_char) = iter.next().unwrap();
					}
					return Token { data: str[start_index..i].to_owned(), token: TokenType::STRING };
				}
				
			'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
				let start_index = i;

				while peek(iter).is_numeric() {
					(i, _) = iter.next().unwrap(); 
				}

				if peek(iter) == '.'{
					panic!("no decimal/floating point numbers allowed");
				}
				return Token { data: str[start_index..(i+1)].to_owned(), token: TokenType::NUMBER };

			}
			
			'\n' => return Token { data: c_char.to_string(), token: TokenType::NEWLINE },
			
			'\0' => return Token { data: c_char.to_string(), token: TokenType::EOF },

			':' => return Token { data: c_char.to_string(), token: TokenType::COLON },

			',' => return Token { data: c_char.to_string(), token: TokenType::COMMA },

			_ => {

				// dont want labels/variables starting with numbers
				if !c_char.is_alphabetic() && c_char != '_' && c_char != '.' {
					panic!("bruh, unexpected error occured. char was {}", c_char);
				}

				let start_index = i;

				while peek(iter).is_alphanumeric() {
					(i, _) = iter.next().unwrap(); 
				}

				let tok_text = str[start_index..(i+1)].to_owned();
				let a = check_if_keyword(&tok_text);

				return Token { data: tok_text, token: a };


			}
			
		}
		
	}
}


fn check_if_keyword(token_text: &String) -> TokenType {
	for token in TokenType::iter() {
		if token.to_string().to_lowercase() == *token_text.to_ascii_lowercase() && TokenType::AX <= token && token <= TokenType::MOV    {
			return  token;
		}
	}
	match token_text.as_str() {
		".data" => TokenType::DATA,
		".text" => TokenType::TEXT,
		_ => TokenType::IDENT
	}
}

fn peek(iter: &mut std::iter::Peekable<std::iter::Enumerate<std::str::Chars>>) -> char {
    match iter.peek() {
        Some((_, c)) => c.clone(),
        None => '\0',
    }
}