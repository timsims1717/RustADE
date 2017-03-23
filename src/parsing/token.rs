/*
token.rs
	|_TokenType						enum
	|_VerbType						enum
	|_DirectionType					enum
	|_Token							struct
		|_t_type					TokenType
		|_lexeme					String
		|_new(&str)					fn->Token
		|_get_token_type_name()		method->String
		|_check_reserved()			method
		|_fmt::Display				impl
Created by Tim Sims on 19/3/2017
Edited by Tim Sims on 21/3/2017
Version 0.1.2
*/

extern crate regex;

use std::fmt;

use self::regex::Regex;

#[derive(Clone, Copy, PartialEq)]
pub enum TokenType {
	QUIT,
	VERB(VerbType),
	DIRECTION(DirectionType),
	ITEM(ItemType),
	WORD, BAD, END,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VerbType {
	// Singleton
	LOOK, INVENTORY,
	// Direction
	GO,
	// Inventory
	GET, DROP,
	// Basic Item
	EXAMINE, USE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DirectionType {
	NORTH, SOUTH, EAST, WEST,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ItemType {
	KEY, HAMMER, IDOL, TREE, ALTAR,
	UNKNOWN,
}

/*
Holds the token type and lexeme (the actual string typed by the
player) of each token.
*/
pub struct Token {
	pub t_type: TokenType,
	pub lexeme: String,
}

impl Token {
	/*
	Creates a new Token from a lexeme.
	*/
	pub fn new(word: &str) -> Token {
		let token_type: TokenType;
		if is_alpha_string(word) {
			token_type = TokenType::WORD;
		} else {
			token_type = TokenType::BAD;
		}
		let mut token = Token { t_type: token_type, lexeme: word.to_string() };
		token.check_reserved();
		token
	}

	/*
	Formats a small descriptor for the token
		TOKENTYPE[:SUBTYPE]
	*/
	pub fn get_token_type_name(&self) -> String {
		match self.t_type {
			TokenType::QUIT => "QUIT".to_string(),
			TokenType::VERB(v) => format!("VERB:{}", match v {
				VerbType::LOOK => "LOOK".to_string(),
				VerbType::INVENTORY => "INVENTORY".to_string(),
				VerbType::GO => "GO".to_string(),
				VerbType::GET => "GET".to_string(),
				VerbType::DROP => "DROP".to_string(),
				VerbType::EXAMINE => "EXAMINE".to_string(),
				VerbType::USE => "USE".to_string(),
			}),
			TokenType::DIRECTION(d) => format!("DIRECTION:{}", match d {
				DirectionType::NORTH => "NORTH".to_string(),
				DirectionType::SOUTH => "SOUTH".to_string(),
				DirectionType::EAST => "EAST".to_string(),
				DirectionType::WEST => "WEST".to_string(),
			}),
			TokenType::ITEM(_) => "ITEM".to_string(),
			TokenType::WORD => "WORD".to_string(),
			TokenType::BAD => "BAD".to_string(),
			TokenType::END => "END".to_string(),
		}
	}

	/*
	Checks to see if the lexeme matches reserved verbs,
	directions, etc.
	*/
	pub fn check_reserved(&mut self) {
		self.t_type = match self.lexeme.as_str() {
			// Gamestate Tokens
			"quit" => TokenType::QUIT,
			// Singleton Verb Tokens
			"look" => TokenType::VERB(VerbType::LOOK),
			"l" => TokenType::VERB(VerbType::LOOK),
			"inventory" => TokenType::VERB(VerbType::INVENTORY),
			"i" => TokenType::VERB(VerbType::INVENTORY),
			// Other Verb Tokens
			"go" => TokenType::VERB(VerbType::GO),
			"get" => TokenType::VERB(VerbType::GET),
			"drop" => TokenType::VERB(VerbType::DROP),
			"examine" => TokenType::VERB(VerbType::EXAMINE),
			"x" => TokenType::VERB(VerbType::EXAMINE),
			"use" => TokenType::VERB(VerbType::USE),
			// Direction Tokens
			"north" => TokenType::DIRECTION(DirectionType::NORTH),
			"south" => TokenType::DIRECTION(DirectionType::SOUTH),
			"east" => TokenType::DIRECTION(DirectionType::EAST),
			"west" => TokenType::DIRECTION(DirectionType::WEST),
			"n" => TokenType::DIRECTION(DirectionType::NORTH),
			"s" => TokenType::DIRECTION(DirectionType::SOUTH),
			"e" => TokenType::DIRECTION(DirectionType::EAST),
			"w" => TokenType::DIRECTION(DirectionType::WEST),
			// Item Tokens
			"key" => TokenType::ITEM(ItemType::KEY),
			"hammer" => TokenType::ITEM(ItemType::HAMMER),
			"idol" => TokenType::ITEM(ItemType::IDOL),
			"trees" => TokenType::ITEM(ItemType::TREE),
			"altar" => TokenType::ITEM(ItemType::ALTAR),
			// Control Tokens
			"end" => TokenType::END,
			_ => self.t_type,
		};
	}
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "TOKEN: {} - {}", self.get_token_type_name(), self.lexeme)
	}
}

/*
Matches a word to a regex that matches only letters.
Must be at least one letter long.
*/
lazy_static! {
	static ref RE: Regex = Regex::new(r"^[a-zA-Z]+$").unwrap();
}
fn is_alpha_string(word: &str) -> bool {
	RE.is_match(word)
}