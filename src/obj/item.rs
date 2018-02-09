/*
item.rs
|_Item											struct
	|_i_type									String
	|_name										String
	|_desc										String
	|_is_fixed									bool
	|_is_plural									bool

Created by Tim Sims on 22/3/2017
Edited by Tim Sims on 24/3/2017
Version 0.1.4
*/

use std::fmt;
use uuid::Uuid;

use obj::{DamageType,DamageLevel};

#[derive(Clone, PartialEq)]
pub struct Item {
	id: Uuid,
	pub i_type: String,
	pub name: String,
	// Descriptions
	pub desc: String,
	pub is_on_desc: String,
	pub light_damage_desc: String,
	pub heavy_damage_desc: String,
	pub destroyed_desc: String,
	// Attributes
	pub is_fixed: bool,
	pub is_scenery: bool,
	pub has_surface: bool,
	pub has_inside: bool,
	pub has_behind: bool,
	pub has_under: bool,
	can_turn_on: bool,
	is_on: bool,
	// Health and Damage
	pub max_health: i32,
	pub current_health: i32,
	pub to_dmg: i32,
	pub damaged_by: Option<DamageType>,
	pub damage_type: Option<DamageType>,
	// Owned Items
	pub on_items: Vec<Item>,
	pub in_items: Vec<Item>,
	pub behind_items: Vec<Item>,
	pub under_items: Vec<Item>,
}

impl Item {

	pub fn get_id(&self) -> Uuid {
		self.id
	}

	pub fn is_blocking(&self) -> bool {
		(self.max_health > 0 && self.current_health > 0) || (self.can_attach && self.attached_items.len() == 0) || self.is_on
	}

	pub fn toggle_on(&mut self) -> Option<bool> {
		if self.can_turn_on {
			self.is_on = !self.is_on;
			Some(self.is_on)
		} else {
			None
		}
	}

	pub fn damage(&mut self, amount: i32) {
		self.to_dmg += amount;
	}

// println!("HP for {}:\nmax: {}\ncurrent: {}\ndamage: {}\n", self.name, self.max_health, self.current_health, self.to_dmg);

	pub fn update(&mut self, mut display: &mut String) {
		if !self.is_scenery {
			let dmg_lvl = self.damage_level();
			self.current_health -= self.to_dmg;
			if self.current_health < 0 {
				self.current_health = 0;
			}
			if dmg_lvl != self.damage_level() {
				display.push_str(format!("{}\n", self).as_str());
			}
			for item in &mut self.on_items {
				item.update(&mut display);
			}
			for item in &mut self.in_items {
				item.update(&mut display);
			}
			for item in &mut self.behind_items {
				item.update(&mut display);
			}
			for item in &mut self.under_items {
				item.update(&mut display);
			}
			self.to_dmg = 0;
		}
	}

	pub fn damage_level(&self) -> DamageLevel {
		let mut result = DamageLevel::NODAMAGE;
		if self.max_health == self.current_health || self.max_health < 0 {
			result = DamageLevel::NODAMAGE;
		} else if self.current_health < self.max_health && self.current_health >= (self.max_health + 1)/2 {
			result = DamageLevel::LIGHT;
		} else if self.current_health <= self.max_health/2 && self.current_health > 0 {
			result = DamageLevel::HEAVY;
		} else if self.current_health == 0 && self.max_health != 0 {
			result = DamageLevel::DESTROYED;
		}
		result
	}

	pub fn put_item_on_this(&mut self, item: Item) {
		self.on_items.push(item);
	}

	pub fn find_item_by_id(&self, id: Uuid) -> Option<Item> {
		if self.id == id {
			Some(self.clone())
		} else {
			let mut found_item: Option<Item> = None;
			for item in &self.attached_items {
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
	}

	pub fn display_item_in_inventory(&self, mut display: &mut String) {
		display.push_str(format!("")
	}
}

impl fmt::Display for Item {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", match self.damage_level() {
			DamageLevel::NODAMAGE => self.desc.clone(),
			DamageLevel::LIGHT => self.light_damage_desc.clone(),
			DamageLevel::HEAVY => self.heavy_damage_desc.clone(),
			DamageLevel::DESTROYED => self.destroyed_desc.clone(),
		}, if self.is_on {
			format!(" {}", self.is_on_desc.clone())
		} else {
			"".to_string()
		})
	}
}

pub struct ItemBuilder {
	i_type: String,
	name: String,
	// Descriptions
	desc: String,
	is_on_desc: String,
	light_damage_desc: String,
	heavy_damage_desc: String,
	destroyed_desc: String,
	// Attributes
	is_fixed: bool,
	is_scenery: bool,
	has_surface: bool,
	has_inside: bool,
	has_behind: bool,
	has_under: bool,
	can_turn_on: bool,
	is_on: bool,
	// Health and Damage
	max_health: i32,
	current_health: i32,
	damaged_by: Option<DamageType>,
	damage_type: Option<DamageType>,
}

impl ItemBuilder {

	pub fn new(it: &str, n: &str, d: &str) -> ItemBuilder {
		match it {
			"UNKNOWN" => panic!("Cannot create an item with type UNKNOWN."),
			_ => ItemBuilder {
				i_type: it.to_string(),
				name: n.to_string(),
				desc: d.to_string(),
				is_on_desc: d.to_string(),
				light_damage_desc: d.to_string(),
				heavy_damage_desc: d.to_string(),
				destroyed_desc: d.to_string(),
				is_fixed: false,
				is_scenery: false,
				has_surface: false,
				has_inside: false,
				has_behind: false,
				has_under: false,
				can_turn_on: false,
				is_on: false,
				max_health: -1,
				current_health: -1,
				damaged_by: None,
				damage_type: None,
			},
		}
	}

	pub fn set_is_on_desc(&mut self, s: &str) -> &mut ItemBuilder {
		self.is_on_desc = s.to_string();
		self
	}

	pub fn set_light_damage_desc(&mut self, s: &str) -> &mut ItemBuilder {
		self.light_damage_desc = s.to_string();
		self
	}

	pub fn set_heavy_damage_desc(&mut self, s: &str) -> &mut ItemBuilder {
		self.heavy_damage_desc = s.to_string();
		self
	}

	pub fn set_destroyed_desc(&mut self, s: &str) -> &mut ItemBuilder {
		self.destroyed_desc = s.to_string();
		self
	}

	pub fn set_fixed(&mut self, b: bool) -> &mut ItemBuilder {
		self.is_fixed = b;
		self
	}

	pub fn set_scenery(&mut self, b: bool) -> &mut ItemBuilder {
		self.is_scenery = b;
		self
	}

	pub fn set_has_surface(&mut self, b: bool) -> &mut ItemBuilder {
		self.has_surface = b;
		self
	}

	pub fn set_has_inside(&mut self, b: bool) -> &mut ItemBuilder {
		self.has_inside = b;
		self
	}

	pub fn set_has_behind(&mut self, b: bool) -> &mut ItemBuilder {
		self.has_behind = b;
		self
	}

	pub fn set_has_under(&mut self, b: bool) -> &mut ItemBuilder {
		self.has_under = b;
		self
	}

	pub fn set_can_turn_on(&mut self, b: bool) -> &mut ItemBuilder {
		self.can_turn_on = b;
		self
	}

	pub fn set_is_on(&mut self, b: bool) -> &mut ItemBuilder {
		self.is_on = b;
		self
	}

	pub fn set_max_health(&mut self, hp: i32) -> &mut ItemBuilder {
		self.max_health = hp;
		self
	}

	pub fn set_current_health(&mut self, hp: i32) -> &mut ItemBuilder {
		self.current_health = hp;
		self
	}

	pub fn set_health(&mut self, hp: i32) -> &mut ItemBuilder {
		self.max_health = hp;
		self.current_health = hp;
		self
	}

	pub fn set_damaged_by(&mut self, d: DamageType) -> &mut ItemBuilder {
		self.damaged_by = Some(d);
		self
	}

	pub fn set_damage_type(&mut self, d: DamageType) -> &mut ItemBuilder {
		self.damage_type = Some(d);
		self
	}

	pub fn finalize(&self) -> Item {
		Item {
			id: Uuid::new_v4(),
			i_type: self.i_type.clone(),
			name: self.name.clone(),
			desc: self.desc.clone(),
			is_on_desc: self.is_on_desc.clone(),
			light_damage_desc: self.light_damage_desc.clone(),
			heavy_damage_desc: self.heavy_damage_desc.clone(),
			destroyed_desc: self.destroyed_desc.clone(),
			is_fixed: self.is_fixed,
			is_scenery: self.is_scenery,
			has_surface: self.has_surface,
			has_inside: self.has_inside,
			has_behind: self.has_behind,
			has_under: self.has_under,
			can_turn_on: self.can_turn_on,
			is_on: self.is_on,
			max_health: self.max_health,
			current_health: self.current_health,
			to_dmg: 0,
			damaged_by: self.damaged_by.clone(),
			damage_type: self.damage_type.clone(),
			on_items: Vec::new(),
			in_items: Vec::new(),
			behind_items: Vec::new(),
			under_items: Vec::new(),
		}
	}
}