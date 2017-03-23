

use obj::item::Item;

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
			"You are carrying nothing.\n".to_string()
		} else {
			let mut inventory = String::new();

			inventory.push_str("You are carrying:\n");

			for item in &self.inventory {
				inventory.push_str("\t");
				inventory.push_str(&item.name);
				inventory.push_str("\n");
			}
			inventory
		}
	}
}