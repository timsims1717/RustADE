

use std::fmt;

use obj::item::Item;
use parsing::token::DirectionType;
use parsing::grammar::ItemNode;

pub struct Exit {
	dest: usize,
	desc: String,
	// pub blocked_desc: String,
	travel_desc: String,
	// pub is_blocked: bool,
	// Command that solves the blockage?
}

impl Exit {
	/*
	Creates a new Exit from a destination index, destination desc, etc.
	*/
	pub fn new(dest: usize, desc: &str, travel_desc: &str) -> Exit {
		Exit {
			dest: dest,
			desc: desc.to_string(),
			travel_desc: travel_desc.to_string(),
		}
	}
}

impl fmt::Display for Exit {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.desc)
	}
}

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
	fixed_items: Vec<Item>,
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
			fixed_items: Vec::new(),
		}
	}

	pub fn create_exit(&mut self, d: DirectionType, dest: usize, desc: &str, travel_desc: &str) {
		let new_exit = ExitExists::YES(Exit::new(dest, desc, travel_desc));
		match d {
			DirectionType::NORTH => self.exits.n = new_exit,
			DirectionType::SOUTH => self.exits.s = new_exit,
			DirectionType::EAST => self.exits.e = new_exit,
			DirectionType::WEST => self.exits.w = new_exit,
		};
	}

	pub fn display_exits(&self) -> String {
		format!("{}{}{}{}", match self.exits.n {
				ExitExists::YES(ref e) => format!("{}\n", e),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.s {
				ExitExists::YES(ref e) => format!("{}\n", e),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.e {
				ExitExists::YES(ref e) => format!("{}\n", e),
				ExitExists::NO(_) => "".to_string(),
			}, match self.exits.w {
				ExitExists::YES(ref e) => format!("{}\n", e),
				ExitExists::NO(_) => "".to_string(),
			})
	}

	pub fn find_exit(&self, d: DirectionType) -> Option<usize> {
		match d {
			DirectionType::NORTH => match self.exits.n {
				ExitExists::YES(ref e) => Some(e.dest),
				ExitExists::NO(_) => None,
			},
			DirectionType::SOUTH => match self.exits.s {
				ExitExists::YES(ref e) => Some(e.dest),
				ExitExists::NO(_) => None,
			},
			DirectionType::EAST => match self.exits.e {
				ExitExists::YES(ref e) => Some(e.dest),
				ExitExists::NO(_) => None,
			},
			DirectionType::WEST => match self.exits.w {
				ExitExists::YES(ref e) => Some(e.dest),
				ExitExists::NO(_) => None,
			},
		}
	}

	pub fn display_exiting(&self, d: DirectionType) -> String {
		format!("{}\n", match d {
			DirectionType::NORTH => match self.exits.n {
				ExitExists::YES(ref e) => e.travel_desc.clone(),
				ExitExists::NO(s) => s.to_string(),
			},
			DirectionType::SOUTH => match self.exits.s {
				ExitExists::YES(ref e) => e.travel_desc.clone(),
				ExitExists::NO(s) => s.to_string(),
			},
			DirectionType::EAST => match self.exits.e {
				ExitExists::YES(ref e) => e.travel_desc.clone(),
				ExitExists::NO(s) => s.to_string(),
			},
			DirectionType::WEST => match self.exits.w {
				ExitExists::YES(ref e) => e.travel_desc.clone(),
				ExitExists::NO(s) => s.to_string(),
			},
		})
	}

	pub fn add_item(&mut self, i: Item) {
		if i.is_fixed {
			self.fixed_items.push(i);
		} else {
			self.items.push(i);
		}
	}

	/*pub fn find_item(&mut self, i: &ItemNode) -> Option<Item> {

	}*/

	pub fn get_item(&mut self, i_node: &ItemNode) -> Option<Item> {
		match self.items.iter().position(|ref n| n.i_type == i_node.subject) {
			Some(i) => Some(self.items.remove(i)),
			None => match self.fixed_items.iter().position(|ref n| n.i_type == i_node.subject) {
				Some(i) => Some(self.fixed_items[i].clone()),
				None => None,
			},
		}
	}

	pub fn display_items(&self) -> String {
		let n = self.items.len();
		if n == 0 {
			"There is nothing else of note that you can see here.\n".to_string()
		} else {
			let mut i = 1;
			let mut display = String::new();
			display.push_str("There is ");
			for item in &self.items {
				if i == n && i != 1 {
					display.push_str("and ");
				}
				display.push_str(&item.name);
				if i == n {
					display.push_str(" here.\n");
				} else {
					display.push_str(", ");
				}
				i += 1;
			}
			display
		}
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}:\n{}\n{}{}", self.name, self.desc, self.display_exits(), self.display_items())
	}
}