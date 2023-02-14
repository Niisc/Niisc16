use core::panic;
use std::string::ToString;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

#[derive(Display, EnumIter, Debug, PartialEq, Eq, PartialOrd)]
pub enum TokenType {
    EOF = -1,
	NEWLINE = 0,
	NUMBER = 1,
	IDENT = 2,
	STRING = 3,
	// Keywords.
	LABEL = 101,
	GOTO = 102,
	PRINT = 103,
	INPUT = 104,
	LET = 105,
	IF = 106,
	THEN = 107,
	ENDIF = 108,
	WHILE = 109,
	REPEAT = 110,
	ENDWHILE = 111,
	// Operators.
	EQ = 201,
	PLUS = 202,
	MINUS = 203,
	ASTERISK = 204,
	SLASH = 205,
	EQEQ = 206,
	NOTEQ = 207,
	LT = 208,
	LTEQ = 209,
	GT = 210,
	GTEQ = 211,
}

#[derive(Debug)]
pub struct Token {
	pub data: String,
	pub token: TokenType,
}

pub fn get_tokens(str: &String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
	let mut iter = str.chars().enumerate();
    
	loop {
		let (mut i,mut c_char) = iter.next().unwrap();

		match c_char {

			'#' => while iter.next().unwrap().1 != '\n'{}, //skip comments

			' ' | '\r' | '\t' => continue, //skip whitespaces

            '+' => tokens.push(Token { data: c_char.to_string(), token: TokenType::PLUS }),

			'-' => tokens.push(Token { data: c_char.to_string(), token: TokenType::MINUS }),

			'*' => tokens.push(Token { data: c_char.to_string(), token: TokenType::ASTERISK }),

			'/' => tokens.push(Token { data: c_char.to_string(), token: TokenType::SLASH }),

			'=' => if peek(str, i) == '=' {
					tokens.push(Token { data: c_char.to_string(), token: TokenType::EQEQ });
					iter.next();
				}else{
					tokens.push(Token { data: c_char.to_string(), token: TokenType::EQ });
				}

			'>' => if peek(str, i) == '=' {
					tokens.push(Token { data: c_char.to_string(), token: TokenType::GTEQ });
					iter.next();
				}else{
					tokens.push(Token { data: c_char.to_string(), token: TokenType::GT });	
				}

			'<' => if peek(str, i) == '=' {
					tokens.push(Token { data: c_char.to_string(), token: TokenType::LTEQ });
					iter.next();
				}else{
					tokens.push(Token { data: c_char.to_string(), token: TokenType::LT });
				}

			'!' => if peek(str, i) == '=' {
				tokens.push(Token { data: c_char.to_string(), token: TokenType::NOTEQ });
				} else {
					panic!("expected !=, got ! and {}", peek(str, i));
				}

			'\"' => {
					(i,c_char) = iter.next().unwrap();
					let start_index = i;
					while c_char != '\"' {
						if c_char == '\r' || c_char == '\n' || c_char == '\t' || c_char == '\\' || c_char == '%' {
							panic!("unallowed char");
						}
						(i,c_char) = iter.next().unwrap();
					}
					tokens.push(Token { data: str[start_index..i].to_owned(), token: TokenType::STRING });
				}
				
			'0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
				let start_index = i;

				while peek(str, i).is_numeric() {
					(i, _) = iter.next().unwrap(); 
				}

				if peek(str, i) == '.'{
					panic!("no decimal/floating point numbers allowed");
				}
				tokens.push(Token { data: str[start_index..i].to_owned(), token: TokenType::NUMBER });

			}
			
			'\n' => tokens.push(Token { data: c_char.to_string(), token: TokenType::NEWLINE }),
			
			'\0' => panic!("got null terminator?"),

            _ => {
				if !c_char.is_alphabetic() {
					panic!("bruh, unexpected error occured");
				}

				let start_index = i;

				while peek(str, i).is_alphanumeric() {
					(i, _) = iter.next().unwrap(); 
				}

				let tok_text = str[start_index..i+1].to_owned();
				let a = check_if_keyword(&tok_text);

				tokens.push(Token { data: tok_text, token: a });


			}
			
        }
		if i+1 == str.chars().count() 
		{
			tokens.push(Token { data: String::from(""), token: TokenType::EOF });
			break; 
		}
    }
    tokens
}

fn check_if_keyword(token_text: &String) -> TokenType {
	for token in TokenType::iter() {
		if token.to_string() == *token_text && token >= TokenType::LABEL && token <= TokenType::ENDWHILE   {
			return  token;
		}
	}
	TokenType::IDENT
}

fn peek(str: &String, index: usize) -> char {
	if index + 1 >= str.len() {
		return '\0';
	}
	str.chars().nth(index+1).expect("error in peek function")
}