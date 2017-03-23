

use parsing::token::{DirectionType,ItemType};

#[derive(Clone, PartialEq)]
pub enum CommandNode {
	// Game state Commands
	QUIT,
	// Singleton Commands
	LOOK, INVENTORY,
	// Direction Commands
	GO(DirectionType),
	// Item Commands
	GET(ItemNode), DROP(ItemNode),
	EXAMINE(ItemNode), USE(ItemNode),
	// Other Commands
	// BAD(String),
}

#[derive(Clone, PartialEq)]
pub struct ItemNode {
	// pub desc: DescType,
	pub subject: ItemType,
	pub subject_lexeme: String,
}

/*
FOR verbs like put or break later on
pub struct ItemNode {
	pub desc: DescType,
	pub subject: ItemType,
	pub preposition: PrepositionType,
	pub obj_desc: DescType,
	pub object: ItemType,
}
*/

/*
FOR GET/DROP later on
pub struct ItemListNode {
	// Vector of ItemNodes
}
*/

impl ItemNode {
	/*
	Creates a new ItemNode using an item token.
	*/
	pub fn new(item_type: ItemType, item_lexeme: &str) -> ItemNode {
		ItemNode {
			subject: item_type,
			subject_lexeme: item_lexeme.to_string(),
		}
	}
}