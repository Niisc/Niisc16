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
    // ALU
	LABEL = 101,
	ADD = 102,
	SUB = 103,
	AND = 104,
	OR = 105,
	NOT = 106,
	XOR = 107,
	MULU = 108,
	DIVU = 109,
	MULS = 110,
	DIVS = 111,
	SHL = 112,
	SHR = 113,
	LNOT = 114,
	CMPW = 115,
	CMPB = 116,

    //IMMEDIATE
	IMM = 117,

    //CONDITIONS
	JMP = 118,

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