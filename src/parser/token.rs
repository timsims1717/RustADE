/*
token.rs
   |_TokenType		enum
   |_VerbType		enum
   |_DirectionType	enum
   |_Token			struct
Created by Tim Sims on 19/3/2017
Edited by Tim Sims on 19/3/2017
Version 0.1.0
*/

use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
	VERB(VerbType), DIRECTION(DirectionType), WORD,
	BAD, NEWLINE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VerbType {
	GO, GET, DROP, EXAMINE, LOOK, OPEN, CLOSE, USE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DirectionType {
	NORTH, SOUTH, EAST, WEST,
}

/*
Holds the token type and lexeme (the actual string typed by the
player) of each token.
*/
pub struct Token {
	pub t_type: TokenType,
	pub lexeme: String,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TOKEN: {} - {}", self.get_token_type_name(), self.lexeme)
	}
}

impl Token {
	/*
	Formats a small descriptor for the token
		TOKENTYPE[:SUBTYPE]
	*/
	pub fn get_token_type_name(&self) -> String {
		match self.t_type {
			TokenType::VERB(v) => format!("VERB:{}", match v {
				VerbType::GO => "GO".to_string(),
				VerbType::GET => "GET".to_string(),
				VerbType::DROP => "DROP".to_string(),
				VerbType::EXAMINE => "EXAMINE".to_string(),
				VerbType::LOOK => "LOOK".to_string(),
				VerbType::OPEN => "OPEN".to_string(),
				VerbType::CLOSE => "CLOSE".to_string(),
				VerbType::USE => "USE".to_string(),
			}),
			TokenType::DIRECTION(d) => format!("DIRECTION:{}", match d {
				DirectionType::NORTH => "NORTH".to_string(),
				DirectionType::SOUTH => "SOUTH".to_string(),
				DirectionType::EAST => "EAST".to_string(),
				DirectionType::WEST => "WEST".to_string(),
			}),
			TokenType::WORD => "WORD".to_string(),
			TokenType::BAD => "BAD".to_string(),
			TokenType::NEWLINE => "NEWLINE".to_string(),
		}
	}

	/*
	Checks to see if the lexeme matches reserved verbs,
	directions, etc.
	*/
	pub fn check_reserved(&mut self) {
		self.t_type = match self.lexeme.as_str() {
			"go" => TokenType::VERB(VerbType::GO),
			"get" => TokenType::VERB(VerbType::GET),
			"drop" => TokenType::VERB(VerbType::DROP),
			"examine" => TokenType::VERB(VerbType::EXAMINE),
			"look" => TokenType::VERB(VerbType::LOOK),
			"open" => TokenType::VERB(VerbType::OPEN),
			"close" => TokenType::VERB(VerbType::CLOSE),
			"use" => TokenType::VERB(VerbType::USE),
			"north" => TokenType::DIRECTION(DirectionType::NORTH),
			"south" => TokenType::DIRECTION(DirectionType::SOUTH),
			"east" => TokenType::DIRECTION(DirectionType::EAST),
			"west" => TokenType::DIRECTION(DirectionType::WEST),
			_ => self.t_type,
		};
	}
}