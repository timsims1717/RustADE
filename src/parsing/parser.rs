/*
parser.rs
	|_parse(&mut Vec<&str>)		fn->Option<CommandNode>
Created by Tim Sims on 20/3/2017
Edited by Tim Sims on 21/3/2017
Version 0.1.2
*/

use std::io;

use parsing::token::{Token,TokenType,VerbType,ItemType};
use parsing::grammar::{CommandNode,ItemNode};

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
	words.push("end");
	parse(&mut words)
}

/*
Parses a given command (the input Vec) into a parse tree
*/
fn parse(input: &mut Vec<&str>) -> Option<CommandNode> {
	match command(input) {
		Some(c) => match get_token(input) {
			Some(next_token) => match next_token.t_type {
				TokenType::END => Some(c),
				_ => None,
			},
			None => None,
		},
		None => None,
	}
}

/*
Determines if the command is a gamestate command or a verb
*/
fn command(input: &mut Vec<&str>) -> Option<CommandNode> {
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::QUIT => Some(CommandNode::QUIT),
			TokenType::VERB(v) => verb(v, input),
			_ => None,
		},
		None => None,
	}
}

/*
Determines the verb of the command
*/
fn verb(v: VerbType, input: &mut Vec<&str>) -> Option<CommandNode> {
	match v {
		VerbType::LOOK => Some(CommandNode::LOOK),
		VerbType::INVENTORY => Some(CommandNode::INVENTORY),
		VerbType::GO => go(input),
		VerbType::GET => match item(input) {
			Some(i) => Some(CommandNode::GET(i)),
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
Determines if an item was specified
*/
fn item(input: &mut Vec<&str>) -> Option<ItemNode> {
	let lexeme = peek_lexeme(input);
	match get_token(input) {
		Some(next_token) => match next_token.t_type {
			TokenType::ITEM(i) => Some(ItemNode::new(i, lexeme)),
			TokenType::WORD => Some(ItemNode::new(ItemType::UNKNOWN, lexeme)),
			_ => None,
		},
		None => None,
	}
}