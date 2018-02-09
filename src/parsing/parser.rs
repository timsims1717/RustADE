/*

*/

use std::io;

use parsing::{TokenType,GameStateType,VerbType,OtherType,CommandNode};
use parsing::token::Token;
use parsing::grammar::{ItemNode,PrepositionNode};

/*
If a Token is still in input, it is removed from input and returned
*/
fn get_token(input: &mut Vec<&str>) -> Option<Token> {
	if input.len() == 0 {
		None
	} else {
		Some(Token::new(input.remove(0)))
	}
}

/*
If a Token is still in input, it is returned, leaving the input
unchanged
*/
#[allow(dead_code)]
fn peek_token(input: &Vec<&str>) -> Option<Token> {
	if input.len() == 0 {
		None
	} else {
		Some(Token::new(input[0]))
	}
}

/*
If a Token is still in input, return the lexeme, leaving the input
unchanged
*/
fn peek_lexeme<'a>(input: &Vec<&'a str>) -> &'a str {
	if input.len() == 0 {
		""
	} else {
		input[0]
	}
}

/*
Retrieves the next input by the user and parses it, returning
the parse tree
*/
pub fn get_next_command() -> Option<CommandNode> {
	let mut input = String::new();
	io::stdin().read_line(&mut input)
			.expect("Failed to read line");

	let mut words = input.split_whitespace().collect::<Vec<&str>>();
	parse(&mut words)
}

/*
Parses a given command (the input Vec) into a parse tree
*/
fn parse(input: &mut Vec<&str>) -> Option<CommandNode> {
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::GAMESTATE(s) => gamestate(s, input),
			TokenType::VERB(v) => verb(v, input),
			TokenType::OTHER(o) => other(o, input),
			TokenType::DIRECTION(d) => Some(CommandNode::GO(d)),
			_ => None,
		},
		None => None,
	}
}

/*
Determines the game state command
*/
#[allow(unused_variables)]
fn gamestate(s: GameStateType, input: &mut Vec<&str>) -> Option<CommandNode> {
	match s {
		GameStateType::QUIT => Some(CommandNode::GAMESTATE(s)),
	}
}

/*
Determines the verb of the command
*/
fn verb(v: VerbType, input: &mut Vec<&str>) -> Option<CommandNode> {
	match v {
		VerbType::LOOK => Some(CommandNode::LOOK(preposition(input))),
		VerbType::INVENTORY => Some(CommandNode::INVENTORY),
		VerbType::GO => go(input),
		VerbType::TAKE => match item(input) {
			Some(i) => Some(CommandNode::TAKE(i)),
			None => None,
		},
		VerbType::DROP => match item(input) {
			Some(i) => Some(CommandNode::DROP(i)),
			None => None,
		},
		VerbType::EXAMINE => match item(input) {
			Some(i) => Some(CommandNode::EXAMINE(i)),
			None => None,
		},
		VerbType::USE => match item(input) {
			Some(i) => Some(CommandNode::USE(i)),
			None => None,
		},
		_ => None,
	}
}

/*
Determines if a direction was specified
*/
fn go(input: &mut Vec<&str>) -> Option<CommandNode> {
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::DIRECTION(d) => Some(CommandNode::GO(d)),
			_ => None,
		},
		None => None,
	}
}

/*
Determines if a preposition was specified
*/
fn preposition(input: &mut Vec<&str>) -> Option<PrepositionNode> {
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::PREPOSITION(p) => match item(input) {
				Some(i) => Some(PrepositionNode::new(p, i)),
				None => None,
			},
			_ => None,
		},
		None => None,
	}
}

/*
Determines if an item was specified
*/
#[allow(unused_variables)]
fn item(input: &mut Vec<&str>) -> Option<ItemNode> {
	let lexeme = peek_lexeme(input);
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::ITEM(i) => Some(ItemNode::new(i.as_str(), lexeme)),
			TokenType::WORD => Some(ItemNode::new("UNKNOWN", lexeme)),
			_ => None,
		},
		None => None,
	}
}

/*
Other token types
*/
#[allow(unused_variables)]
fn other(o: OtherType, input: &mut Vec<&str>) -> Option<CommandNode> {
	Some(CommandNode::OTHER(o))
	// match o {
	// 	OtherType::YES => Some(CommandNode::OTHER(o)),
	// 	OtherType::NO => Some(CommandNode::OTHER(o)),
	// }
}