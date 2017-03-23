

use parsing::token::ItemType;

#[derive(Clone, PartialEq)]
pub struct Item {
	pub i_type: ItemType,
	pub name: String,
	pub desc: String,
	pub is_fixed: bool,
	pub is_plural: bool,
}

impl Item {
	/*
	Creates a new Item from an ItemType, description, etc.
	*/
	pub fn new(it: ItemType, n: &str, d: &str, fixed: bool, plural: bool) -> Item {
		match it {
			ItemType::UNKNOWN => panic!("Cannot create an item with type UNKNOWN."),
			_ => Item {
				i_type: it,
				name: n.to_string(),
				desc: d.to_string(),
				is_fixed: fixed,
				is_plural: plural,
			},
		}
	}
}