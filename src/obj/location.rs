/*

*/

use std::fmt;
use uuid::Uuid;

use obj::item::Item;
use obj::{ObjId,DamageType};
use parsing::DirectionType;
use parsing::grammar::ItemNode;

#[derive(Clone, PartialEq)]
pub struct Exit {
	pub dest: usize,
	pub desc: String,
	pub travel_desc: Option<String>, // None gives default according to direction.
	pub blocked_by: Option<ObjId>, // None means not blocked
	pub blocked_desc: String,
}

impl fmt::Display for Exit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.desc)
	}
}

pub struct ExitBuilder {
	dest: usize,
	desc: Option<String>,
	travel_desc: Option<String>,
	blocked_by: Option<ObjId>,
	blocked_desc: Option<String>,
}

impl ExitBuilder {

	pub fn new() -> ExitBuilder {
		ExitBuilder {
			dest: 0,
			desc: None,
			travel_desc: None,
			blocked_by: None,
			blocked_desc: None,
		}
	}

	pub fn set_dest(&mut self, dest: usize) -> &mut ExitBuilder {
		self.dest = dest;
		self
	}

	pub fn set_desc(&mut self, desc: &str) -> &mut ExitBuilder {
		self.desc = Some(desc.to_string());
		self
	}

	pub fn set_travel_desc(&mut self, desc: &str) -> &mut ExitBuilder {
		self.travel_desc = Some(desc.to_string());
		self
	}

	pub fn set_blocked_by_item(&mut self, blocker_id: Uuid) -> &mut ExitBuilder {
		self.blocked_by = Some(ObjId::ITEMID(blocker_id));
		self
	}

	pub fn set_blocked_desc(&mut self, desc: &str) -> &mut ExitBuilder {
		self.blocked_desc = Some(desc.to_string());
		self
	}

	pub fn finalize(&self) -> Exit {
		Exit {
			dest: self.dest,
			desc: match self.desc.clone() {
				Some(s) => s,
				None => "You may go".to_string(),
			},
			travel_desc: self.travel_desc.clone(),
			blocked_by: self.blocked_by.clone(),
			blocked_desc: match self.blocked_desc.clone() {
				Some(s) => s,
				None => "You can't go that way.".to_string(),
			},
		}
	}
}

#[derive(Clone, PartialEq)]
enum ExitExists {
	YES(Exit),
	NO(&'static str),
}

struct Exits {
	pub n: ExitExists,
	pub s: ExitExists,
	pub e: ExitExists,
	pub w: ExitExists,
}

impl Exits {
	/*
	Creates a new Exits struct with no exits using the no_exit &'static str
	*/
	pub fn new(no_exit: &'static str) -> Exits {
		Exits {
			n: ExitExists::NO(no_exit),
			s: ExitExists::NO(no_exit),
			e: ExitExists::NO(no_exit),
			w: ExitExists::NO(no_exit),
		}
	}
}

pub struct Location {
	pub name: String,
	pub desc: String,
	exits: Exits,
	items: Vec<Item>,
	scenery_items: Vec<Item>,
}

impl Location {
	/*
	Creates a new Location from a name and desc.
	The data members exits and items are empty to begin.
	*/
	pub fn new(n: &str, d: &str, no_exit: &'static str) -> Location {
		Location {
			name: n.to_string(),
			desc: d.to_string(),
			exits: Exits::new(no_exit),
			items: Vec::new(),
			scenery_items: Vec::new(),
		}
	}

	pub fn update(&mut self, mut display: &mut String) {
		for item in &mut self.items {
			item.update(&mut display);
		}
	}

	pub fn add_exit(&mut self, d: DirectionType, exit: Exit) {
		let new_exit = ExitExists::YES(exit);
		match d {
			DirectionType::NORTH => self.exits.n = new_exit,
			DirectionType::SOUTH => self.exits.s = new_exit,
			DirectionType::EAST => self.exits.e = new_exit,
			DirectionType::WEST => self.exits.w = new_exit,
		};
	}

	pub fn display_exits(&self) -> String {
		format!("{}{}{}{}", match self.exits.n {
				ExitExists::YES(ref e) => format!("{} to the north. {}\n", e, {
					if self.is_exit_blocked(e) {
						e.blocked_desc.clone()
					} else {
						"".to_string()
					}
				}),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.s {
				ExitExists::YES(ref e) => format!("{} to the south. {}\n", e, {
					if self.is_exit_blocked(e) {
						e.blocked_desc.clone()
					} else {
						"".to_string()
					}
				}),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.e {
				ExitExists::YES(ref e) => format!("{} to the east. {}\n", e, {
					if self.is_exit_blocked(e) {
						e.blocked_desc.clone()
					} else {
						"".to_string()
					}
				}),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.w {
				ExitExists::YES(ref e) => format!("{} to the west. {}\n", e, {
					if self.is_exit_blocked(e) {
						e.blocked_desc.clone()
					} else {
						"".to_string()
					}
				}),
				ExitExists::NO(_) => "".to_string(),
			})
	}

	#[allow(unused_parens)]
	pub fn find_exit(&self, d: DirectionType) -> Option<usize> {
		match (match d {
			DirectionType::NORTH => self.exits.n.clone(),
			DirectionType::SOUTH => self.exits.s.clone(),
			DirectionType::EAST => self.exits.e.clone(),
			DirectionType::WEST => self.exits.w.clone(),
		}) {
			ExitExists::YES(ref e) => {
				if self.is_exit_blocked(e) {
					None
				} else {
					Some(e.dest)
				}
			},
			ExitExists::NO(_) => None,
		}
	}

	/*
	
	*/
	#[allow(unused_parens)]
	pub fn display_exiting(&self, d: DirectionType) -> String {
		format!("{}", match (match d {
			DirectionType::NORTH => self.exits.n.clone(),
			DirectionType::SOUTH => self.exits.s.clone(),
			DirectionType::EAST => self.exits.e.clone(),
			DirectionType::WEST => self.exits.w.clone(),
		}) {
			ExitExists::YES(ref e) => {
				if self.is_exit_blocked(e) {
					e.blocked_desc.clone()
				} else {
					Location::display_exiting_success(d, e.travel_desc.clone())
				}
			},
			ExitExists::NO(s) => s.to_string(),
		})
	}

	fn is_exit_blocked(&self, e: &Exit) -> bool {
		match e.blocked_by.clone() {
			Some(obj) => match obj {
				ObjId::ITEMID(id) => match self.find_item_by_id(id) {
					Some(i) => {
						if i.is_blocking() {
							true
						} else {
							false
						}
					},
					None => false,
				},
				// _ => false,
			},
			None => false,
		}
	}

	fn display_exiting_success(d: DirectionType, travel_desc: Option<String>) -> String {
		match travel_desc {
			Some(ref s) => s.clone(),
			None => match d {
				DirectionType::NORTH => "You go north.".to_string(),
				DirectionType::SOUTH => "You go south.".to_string(),
				DirectionType::EAST => "You go east.".to_string(),
				DirectionType::WEST => "You go west.".to_string(),
			},
		}
	}

	pub fn add_item(&mut self, i: Item) {
		if i.is_scenery {
			self.scenery_items.push(i);
		} else {
			self.items.push(i);
		}
	}

	pub fn find_item(&self, i_node: &ItemNode) -> Option<Item> {
		match self.items.iter().position(|ref n| n.i_type == i_node.subject) {
			Some(i) => Some(self.items[i].clone()),
			None => match self.scenery_items.iter().position(|ref n| n.i_type == i_node.subject) {
				Some(i) => Some(self.scenery_items[i].clone()),
				None => {
					let mut found_item: Option<Item> = None;
					for item in &self.scenery_items {
						for attached_item in &item.attached_items {
							if attached_item.i_type == i_node.subject {
								found_item = Some(attached_item.clone());
								break;
							}
						}
					}
					found_item
				},
			},
		}
	}

	pub fn remove_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.items.iter().position(|ref n| n.i_type == i_node.subject) {
			Some(i) => Some(self.items.remove(i)),
			None => None,
		}
	}

	pub fn display_items(&self) -> String {
		let mut display = String::new();
		let mut something = false;
		let n = self.items.len();
		let mut i = 1;
		for item in &self.scenery_items {
			let n = item.attached_items.len();
			i = 1;
			if n > 0 {
				something = true;
				display.push_str("\nAttached to the ");
				display.push_str(&item.name);
				display.push_str(" is ");
				for attached_item in &item.attached_items {
					if i == n && i != 1 {
						display.push_str("and ");
					}
					display.push_str(&attached_item.name);
					if i == n {
						display.push_str(".");
					} else if n != 2 {
						display.push_str(", ");
					} else {
						display.push_str(" ");
					}
					i += 1;
				}
			}
		}
		if n > 0 {
			something = true;
			display.push_str("\nThere is ");
			for item in &self.items {
				if i == n && i != 1 {
					display.push_str("and ");
				}
				display.push_str(&item.name);
				if i == n {
					display.push_str(" here.");
				} else if n != 2 {
					display.push_str(", ");
				} else {
					display.push_str(" ");
				}
				i += 1;
			}
		}
		for item in &self.items {
			let n = item.attached_items.len();
			i = 1;
			if n > 0 {
				something = true;
				display.push_str("\nAttached to the ");
				display.push_str(&item.name);
				display.push_str(" is ");
				for attached_item in &item.attached_items {
					if i == n && i != 1 {
						display.push_str("and ");
					}
					display.push_str(&attached_item.name);
					if i == n {
						display.push_str(".");
					} else if n != 2 {
						display.push_str(", ");
					} else {
						display.push_str(" ");
					}
					i += 1;
				}
			}
		}
		if !something {
			display.push_str("\nThere is nothing else of note that you can see here.");
		}
		display
	}

	pub fn find_item_by_id(&self, id: Uuid) -> Option<Item> {
		let mut found_item: Option<Item> = None;
		for item in &self.items {
			match item.find_item_by_id(id) {
				Some(i) => {
					found_item = Some(i);
					break;
				},
				None => (),
			}
		};
		for item in &self.scenery_items {
			match item.find_item_by_id(id) {
				Some(i) => {
					found_item = Some(i);
					break;
				},
				None => (),
			}
		};
		found_item
	}

	pub fn remove_first_item_by_damage_type(&mut self, damage_type: DamageType) -> Option<Item> {
		match self.items.iter().position(|ref n| match n.damaged_by.clone() {
			Some(d) => d == damage_type,
			None => false,
		}) {
			Some(i) => Some(self.items.remove(i)),
			None => None,
		}
	}

	pub fn remove_first_attachable_item(&mut self) -> Option<Item> {
		match self.items.iter().position(|ref n| n.can_attach) {
			Some(i) => Some(self.items.remove(i)),
			None => {
				let mut found_item: Option<Item> = None;
				for item in &mut self.scenery_items {
					match item.attached_items.iter().position(|ref n| n.can_attach) {
						Some(i) => {
							found_item = Some(item.attached_items.remove(i));
							break;
						},
						None => (),
					}
				};
				found_item
			},
		}
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:\n{}\n{}{}", self.name, self.desc, self.display_exits(), self.display_items())
	}
}