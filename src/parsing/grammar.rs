/*

*/

use parsing::{GameStateType,PrepositionType,DirectionType,OtherType};

#[derive(Clone, PartialEq)]
pub struct ItemNode {
	// pub desc: DescType,
	pub subject: String,
	pub subject_lexeme: String,
}

/*
FOR verbs like put or break later on
pub struct ItemNode {
	pub desc: DescType,
	pub subject: String,
	pub preposition: PrepositionType,
	pub obj_desc: DescType,
	pub object: String,
}
*/

/*
FOR TAKE/DROP later on
pub struct ItemListNode {
	// Vector of ItemNodes
}
*/

impl ItemNode {
	/*
	Creates a new ItemNode using an item token.
	*/
	pub fn new(item_type: &str, item_lexeme: &str) -> ItemNode {
		ItemNode {
			subject: item_type.to_string(),
			subject_lexeme: item_lexeme.to_string(),
		}
	}
}

#[derive(Clone, PartialEq)]
pub struct PrepositionNode {
	pub prep: PrepositionType,
	pub item: ItemNode,
}

impl PrepositionNode {
	/*
	Creates a new PrepositionNode using an PrepositionType and an ItemType.
	*/
	pub fn new(p: PrepositionType, i: ItemNode) -> PrepositionNode {
		PrepositionNode {
			prep: p,
			item: i,
		}
	}
}