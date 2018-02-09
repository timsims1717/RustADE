

use obj::DamageType;
use obj::item::Item;
use obj::location::Location;
use obj::player::Player;
use parsing::grammar::ItemNode;
use parsing::DirectionType;

pub struct GameState {
	pub break_control: bool,
	player: Player,
	locations: Vec<Location>,
}

impl GameState {
	/*
	Creates a GameState instance.
	*/
	pub fn new(l: usize, loc: Vec<Location>) -> GameState {
		GameState {
			break_control: false,
			player: Player::new(l),
			locations: loc,
		}
	}

	/*
	
	todo: make it so only the current locations display strings
	*/
	pub fn update(&mut self) -> String {
		let mut display = String::new();
		for item in &mut self.player.inventory {
			item.update(&mut display);
		}
		for location in &mut self.locations {
			location.update(&mut display);
		}
		display
	}

	pub fn update_player(&self) -> bool {
		let idol = ItemNode {
			subject: "idol".to_string(),
			subject_lexeme: "idol".to_string(),
		};
		match self.player.find_item(&idol) {
			Some(_) => true,
			None => false,
		}
	}

	/*
	Returns a pointer to the player's current location
	*/
	pub fn current_location(&self) -> &Location {
		&self.locations[self.player.location]
	}

	/*
	Returns a pointer to the player
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

	pub fn has_item(&self, i_node: &ItemNode) -> Option<Item> {
		match self.player.find_item(i_node) {
			Some(i) => Some(i),
			None => self.locations[self.player.location].find_item(i_node),
		}
	}

	/*
	Attempts to move an item from the current location to the player's
	inventory. If the item is not fixed, it succeeds. The Location.remove_item()
	method does not remove it from it's list if the item is fixed.
	*/
	pub fn get_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.locations[self.player.location].remove_item(&i_node) {
			Some(i) => {
				if !i.is_fixed {
					self.player.add_item(i.clone());
				} else {
					self.locations[self.player.location].add_item(i.clone());
				}
				Some(i)
			},
			None => None,
		}
	}

	/*
	Attempts to move an item from the player's inventory to the current
	location.
	*/
	pub fn drop_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.player.remove_item(&i_node) {
			Some(i) => {
				self.locations[self.player.location].add_item(i.clone());
				Some(i)
			},
			None => None,
		}
	}

	/*
	
	todo: add damage_amount as a third parameter
	*/
	pub fn damage_first_item(&mut self, damage_type: Option<DamageType>) -> Option<Item> {
		match damage_type {
			Some(dt) => match self.locations[self.player.location].remove_first_item_by_damage_type(dt) {
				Some(mut item) => {
					item.damage(1);
					self.locations[self.player.location].add_item(item.clone());
					Some(item)
				},
				None => None,
			},
			None => None,
		}
	}

	/*

	*/
	pub fn attach_first_item(&mut self, i_node: &ItemNode, sub_item: Item) -> Option<Item> {
		if sub_item.can_attach {
			match self.locations[self.player.location].remove_first_attachable_item() {
				Some(mut item) => {
					item.attach_item(sub_item);
					self.locations[self.player.location].add_item(item.clone());
					self.player.remove_item(i_node);
					Some(item)
				},
				None => None,
			}
		} else {
			None
		}
	}


	pub fn turn_on_item(&mut self, i_node: &ItemNode) -> Option<bool> {
		match self.player.remove_item(&i_node) {
			Some(mut item) => {
				let result = item.toggle_on();
				self.player.add_item(item);
				result
			},
			None => match self.locations[self.player.location].remove_item(&i_node) {
				Some(mut item) => {
					let result = item.toggle_on();
					self.locations[self.player.location].add_item(item);
					result
				},
				None => None,
			},
		}
	}
}