

use obj::item::Item;
use parsing::grammar::ItemNode;

pub struct Player {
	pub inventory: Vec<Item>,
	pub location: usize,
}

impl Player {
	/*
	Creates a new Player.
	*/
	pub fn new(l: usize) -> Player {
		Player {
			inventory: Vec::new(),
			location: l,
		}
	}

	pub fn display_inventory(&self) -> String {
		if self.inventory.len() == 0 {
			"You are carrying nothing.".to_string()
		} else {
			let mut inventory = String::new();

			inventory.push_str("You are carrying:");

			for item in &self.inventory {
				inventory.push_str("\n\t");
				inventory.push_str(&item.name);
				inventory.push_str("");
			}
			inventory
		}
	}

	pub fn find_item(&self, i_node: &ItemNode) -> Option<Item> {
		match self.inventory.iter().position(|ref n| n.i_type == i_node.subject) {
			Some(i) => Some(self.inventory[i].clone()),
			None => None,
		}
	}

	pub fn remove_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.inventory.iter().position(|ref n| n.i_type == i_node.subject) {
			Some(i) => Some(self.inventory.remove(i)),
			None => None,
		}
	}

	pub fn add_item(&mut self, item: Item) {
		self.inventory.push(item);
	}
}