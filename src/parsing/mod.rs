pub mod grammar;
pub mod parser;
pub mod token;

use parsing::grammar::{ItemNode,PrepositionNode};

/*
Enums for Token:
	TokenType: the parent type
	GameStateType: for quit, save, and other game state commands
	VerbType: for verbs
	PrepositionType: for prepositions
	DirectionType: for the cardinal directions
	OtherType: any other types
*/

#[derive(Clone, PartialEq)]
pub enum TokenType {
	GAMESTATE(GameStateType),
	VERB(VerbType),
	PREPOSITION(PrepositionType),
	DIRECTION(DirectionType),
	ITEM(String),
	OTHER(OtherType),
	WORD, BAD,
}

#[derive(Clone, Copy, PartialEq)]
pub enum GameStateType {
	QUIT,
}

#[derive(Clone, Copy, PartialEq)]
pub enum VerbType {
	// Location
	GO, LOOK,
	// Inventory
	INVENTORY, TAKE, DROP, EXAMINE,
	// Basic Item
	USE,
	// Item to Item
	CUT, HIT, TIE,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PrepositionType {
	AT, ON, IN, WITH, UNDER, BEHIND, TO,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DirectionType {
	NORTH, SOUTH, EAST, WEST,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OtherType {
	YES, NO,
}

/*
Enum for Commands
*/

#[derive(Clone, PartialEq)]
pub enum CommandNode {
	// Singleton Commands
	LOOK(Option<PrepositionNode>), INVENTORY,
	// Direction Commands
	GO(DirectionType),
	// Item Commands
	TAKE(ItemNode), DROP(ItemNode),
	EXAMINE(ItemNode), USE(ItemNode),
	// Other Commands
	// BAD(String),
	GAMESTATE(GameStateType),
	OTHER(OtherType),
	//ERROR(String),
}