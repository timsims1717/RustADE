/*
Used to convert single words into Tokens
*/

use std::fmt;
use regex::Regex;

use parsing::{TokenType,GameStateType,VerbType,PrepositionType,DirectionType,OtherType};

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
	Formats a small descriptor for the token (for debug)
		TOKENTYPE[:SUBTYPE]
	*/
	pub fn get_token_type_name(&self) -> String {
		match self.t_type {
			TokenType::GAMESTATE(s) => match s {
				GameStateType::QUIT => "QUIT".to_string(),
			},
			TokenType::VERB(v) => format!("VERB:{}", match v {
				VerbType::LOOK => "LOOK".to_string(),
				VerbType::INVENTORY => "INVENTORY".to_string(),
				VerbType::GO => "GO".to_string(),
				VerbType::TAKE => "TAKE".to_string(),
				VerbType::DROP => "DROP".to_string(),
				VerbType::EXAMINE => "EXAMINE".to_string(),
				VerbType::USE => "USE".to_string(),
				VerbType::CUT => "CUT".to_string(),
				VerbType::HIT => "HIT".to_string(),
				VerbType::TIE => "TIE".to_string(),
			}),
			TokenType::PREPOSITION(_) => "PREPOSITION".to_string(),
			TokenType::DIRECTION(d) => format!("DIRECTION:{}", match d {
				DirectionType::NORTH => "NORTH".to_string(),
				DirectionType::SOUTH => "SOUTH".to_string(),
				DirectionType::EAST => "EAST".to_string(),
				DirectionType::WEST => "WEST".to_string(),
			}),
			TokenType::ITEM(_) => "ITEM".to_string(),
			TokenType::OTHER(o) => match o {
				OtherType::YES => "YES".to_string(),
				OtherType::NO => "NO".to_string(),
			},
			TokenType::WORD => "WORD".to_string(),
			TokenType::BAD => "BAD".to_string(),
		}
	}

	/*
	Checks to see if the lexeme matches reserved verbs, directions, etc.
	*/
	pub fn check_reserved(&mut self) {
		self.t_type = match self.lexeme.as_str() {
			// GameState Tokens
			"quit" => TokenType::GAMESTATE(GameStateType::QUIT),
			// Verb Tokens
			"look" => TokenType::VERB(VerbType::LOOK),
			"l" => TokenType::VERB(VerbType::LOOK),
			"inventory" => TokenType::VERB(VerbType::INVENTORY),
			"i" => TokenType::VERB(VerbType::INVENTORY),
			"go" => TokenType::VERB(VerbType::GO),
			"get" => TokenType::VERB(VerbType::TAKE),
			"take" => TokenType::VERB(VerbType::TAKE),
			"drop" => TokenType::VERB(VerbType::DROP),
			"examine" => TokenType::VERB(VerbType::EXAMINE),
			"x" => TokenType::VERB(VerbType::EXAMINE),
			"use" => TokenType::VERB(VerbType::USE),
			"hit" => TokenType::VERB(VerbType::HIT),
			"break" => TokenType::VERB(VerbType::HIT),
			"smash" => TokenType::VERB(VerbType::HIT),
			"cut" => TokenType::VERB(VerbType::CUT),
			"tie" => TokenType::VERB(VerbType::TIE),
			// Preposition Tokens
			"at" => TokenType::PREPOSITION(PrepositionType::AT),
			"to" => TokenType::PREPOSITION(PrepositionType::TO),
			"in" => TokenType::PREPOSITION(PrepositionType::IN),
			"into" => TokenType::PREPOSITION(PrepositionType::IN),
			"on" => TokenType::PREPOSITION(PrepositionType::ON),
			"under" => TokenType::PREPOSITION(PrepositionType::UNDER),
			"behind" => TokenType::PREPOSITION(PrepositionType::BEHIND),
			"with" => TokenType::PREPOSITION(PrepositionType::WITH),
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
			"key" => TokenType::ITEM("key".to_string()),
			"pick" => TokenType::ITEM("pickaxe".to_string()),
			"pickaxe" => TokenType::ITEM("pickaxe".to_string()),
			"rope" => TokenType::ITEM("rope".to_string()),
			"hammer" => TokenType::ITEM("hammer".to_string()),
			"idol" => TokenType::ITEM("idol".to_string()),
			"trees" => TokenType::ITEM("trees".to_string()),
			"altar" => TokenType::ITEM("altar".to_string()),
			"seal" => TokenType::ITEM("seal".to_string()),
			"stones" => TokenType::ITEM("seal".to_string()),
			"pit" => TokenType::ITEM("pit".to_string()),
			"root" => TokenType::ITEM("root".to_string()),
			"lantern" => TokenType::ITEM("lantern".to_string()),
			"lamp" => TokenType::ITEM("lantern".to_string()),
			"machete" => TokenType::ITEM("machete".to_string()),
			"cobwebs" => TokenType::ITEM("cobwebs".to_string()),
			"ceiling" => TokenType::ITEM("ceiling".to_string()),
			// Other Tokens
			"yes" => TokenType::OTHER(OtherType::YES),
			"no" => TokenType::OTHER(OtherType::NO),
			_ => self.t_type.clone(),
		};
	}
}

impl fmt::Debug for Token {
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