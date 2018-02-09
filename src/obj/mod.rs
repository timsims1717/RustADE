pub mod item;
pub mod location;
pub mod player;

use uuid::Uuid;

#[derive(Clone, PartialEq)]
pub enum ObjId {
	ITEMID(Uuid),
	// LOCATIONID(Uuid),
	// PLAYER,
}

#[derive(Clone, PartialEq)]
pub enum DamageType {
	SMASHING,
	CUTTING,
}

#[derive(Clone, PartialEq)]
pub enum DamageLevel {
	NODAMAGE,
	LIGHT,
	HEAVY,
	DESTROYED,
}

// #[derive(Clone, PartialEq)]
// pub enum Solution {
// 	PREPOSITION(PrepositionSolution),
// }

// #[derive(Clone, PartialEq)]
// pub struct PrepositionSolution {
// 	pub item: ItemNode,
// 	pub preposition: PrepositionNode,
// 	pub boolean: bool,
// }