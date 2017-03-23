

use obj::item::Item;
use obj::location::Location;
use obj::player::Player;
use parsing::grammar::ItemNode;
use parsing::token::DirectionType;

pub struct GameState {
	player: Player,
	locations: Vec<Location>,
}

impl GameState {
	/*
	Creates a GameState instance.
	*/
	pub fn new(l: usize, loc: Vec<Location>) -> GameState {
		GameState {
			player: Player::new(l),
			locations: loc,
		}
	}

	/*
	Returns a pointer to the player's current location
	*/
	pub fn current_location(&self) -> &Location {
		&self.locations[self.player.location]
	}

	/*
	Returns a pointer to the player's inventory
	*/
	pub fn player(&self) -> &Player {
		&self.player
	}

	/*
	Attempts to change the location of the player, returns the old
	location index if successful, otherwise returns None
	*/
	pub fn move_player(&mut self, d: DirectionType) -> Option<usize> {
		match self.locations[self.player.location].find_exit(d) {
			Some(e) => {
				let old_loc = self.player.location;
				self.player.location = e;
				Some(old_loc)
			},
			None => None,
		}
	}

	/*
	Attempts to move an item from the current location to the player's
	inventory. If the item is not fixed, it succeeds. The Location.get_item()
	method does not remove it from it's list if the item is fixed.
	*/
	pub fn get_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.locations[self.player.location].get_item(&i_node) {
			Some(i) => {
				if !i.is_fixed {
					self.player.inventory.push(i.clone());
				}
				Some(i)
			},
			None => None
		}
	}
}