

use obj::DamageType;
use obj::item::ItemBuilder;
use obj::location::{Location,ExitBuilder};
use parsing::DirectionType;

pub fn build_fixed_world() -> Vec<Location> {
	let mut locations = build_temple_locations();
	let seal = ItemBuilder::new(
		"seal",
		"a stone seal",
		"The door is blocked with a seal of stone bricks. They appear to be worn with age, and might not stand for long."
		).set_fixed(true)
		.set_health(3)
		.set_damaged_by(DamageType::SMASHING)
		.set_light_damage_desc("The stones of the seal have begun to crack, and rock chips litter the ground.")
		.set_heavy_damage_desc("The seal sports a hole about the size of a baseball, and large gouges and cracks cross the stones.")
		.set_destroyed_desc("The open door lies before you, the stone seal in pieces.")
		.finalize();
	let cobwebs = ItemBuilder::new(
		"cobwebs",
		"thick cobwebs",
		"Cobwebs fill the tunnel, making it difficult to move any further."
		).set_fixed(true)
		.set_health(1)
		.set_damaged_by(DamageType::CUTTING)
		.set_destroyed_desc("The cobwebs hang limp against the wall, slashed through.")
		.finalize();
	let root = ItemBuilder::new(
		"root",
		"a root",
		"The root is sturdy and positioned conveniently over the pit trap."
		).set_fixed(true)
		.finalize();
	locations[0].add_exit(
		DirectionType::EAST,
		ExitBuilder::new()
			.set_dest(1)
			.set_desc("The entrance, a door outlined in carved stone, lies")
			.set_travel_desc("After hesitating a bare moment, you duck in the temple's entrance.")
			.finalize());
	locations[1].add_exit(
		DirectionType::WEST,
		ExitBuilder::new()
			.set_dest(0)
			.set_desc("Light beckons from the entrance")
			.set_travel_desc("You hurriedly walk back out into the light.")
			.finalize());
	locations[1].add_exit(
		DirectionType::EAST,
		ExitBuilder::new()
			.set_dest(2)
			.set_desc("The temple continues")
			.set_travel_desc("Steeling your nerves, you walk down the dim hall.")
			.finalize());
	locations[2].add_exit(
		DirectionType::WEST,
		ExitBuilder::new()
			.set_dest(1)
			.finalize());
	locations[2].add_exit(
		DirectionType::EAST,
		ExitBuilder::new()
			.set_dest(3)
			.finalize());
	locations[2].add_exit(
		DirectionType::NORTH,
		ExitBuilder::new()
			.set_dest(4)
			.finalize());
	locations[3].add_exit(
		DirectionType::WEST,
		ExitBuilder::new()
			.set_dest(2)
			.finalize());
	locations[3].add_exit(
		DirectionType::NORTH,
		ExitBuilder::new()
			.set_dest(7)
			.set_desc("The hall continues")
			.set_blocked_by_item(cobwebs.get_id())
			.set_blocked_desc("The cobwebs are too thick to push through.")
			.finalize());
	locations[4].add_exit(
		DirectionType::SOUTH,
		ExitBuilder::new()
			.set_dest(2)
			.finalize());
	locations[4].add_exit(
		DirectionType::NORTH,
		ExitBuilder::new()
			.set_dest(5)
			.set_desc("The tunnel continues across the pit trap")
			.set_travel_desc("You swing deftly across the pit and, smiling to yourself, head further into the temple.")
			.set_blocked_by_item(root.get_id())
			.set_blocked_desc("There's no way you can jump across the pit.")
			.finalize());
	locations[5].add_exit(
		DirectionType::SOUTH,
		ExitBuilder::new()
			.set_dest(4)
			.set_desc("The pit trap is")
			.set_travel_desc("You leave the tomb, eventually coming to the pit trap. You swing across the gap.")
			.finalize());
	locations[6].add_exit(
		DirectionType::WEST,
		ExitBuilder::new()
			.set_dest(7)
			.set_desc("The door out is")
			.finalize());
	locations[7].add_exit(
		DirectionType::SOUTH,
		ExitBuilder::new()
			.set_dest(3)
			.set_desc("The hallway leads back")
			.finalize());
	locations[7].add_exit(
		DirectionType::EAST,
		ExitBuilder::new()
			.set_dest(6)
			.set_desc("Behind the seal is another room")
			.set_travel_desc("You leave the small room behind and enter the chamber.")
			.set_blocked_by_item(seal.get_id())
			.set_blocked_desc("The door is sealed.")
			.finalize());
	let ceiling = ItemBuilder::new(
		"ceiling",
		"the ceiling",
		"It is made of stone."
		).set_scenery(true)
		.finalize();
	ceiling.put_item_on_this(root);
	locations[3].add_item(cobwebs);
	locations[4].add_item(ceiling);
	locations[7].add_item(seal);
	locations
}

fn build_temple_locations() -> Vec<Location> {
	let mut outside = Location::new(
		"Outside the Temple",
		"You stand on the slope of a hill, at the entrance to an ancient temple. Trees surround you and obscure your view, while the sounds of the jungle fill your ears.",
		"You shouldn't go into the jungle without a guide."
		);
	outside.add_item(ItemBuilder::new(
		"trees",
		"trees of all sizes",
		"You see jungle trees in all directions, with green leaves and green moss on their trunks. They loom ominously."
		).set_scenery(true)
		.finalize());
	let mut entrance = Location::new(
		"Entrance",
		"You stand just inside the entrance to an ancient temple.",
		"You can't go that direction."
		);
	entrance.add_item(ItemBuilder::new(
		"machete",
		"a machete",
		"The machete is sharp. It's perfect for hacking through vegetation."
		).set_damage_type(DamageType::CUTTING)
		.finalize());
	let fork = Location::new(
		"Forked Passage",
		"You are at an intersection in the halls of the temple.",
		"You can't go that direction."
		);
	let mut hall = Location::new(
		"Narrow Hallway",
		"You are in a rather cramped passage.",
		"You can't go that direction."
		);
	hall.add_item(ItemBuilder::new(
		"rope",
		"a rope",
		"The rope is about twenty feet long, and is of fine quality."
		).set_can_attach(true)
		.finalize());
	let mut pit_trap = Location::new(
		"Pit Trap",
		"You are at the edge of a pit trap, too wide to jump across.",
		"You can't go that direction."
		);
	pit_trap.add_item(ItemBuilder::new(
		"pit",
		"a deep pit trap",
		"The pit trap is too wide to jump across, and you can't see its bottom."
		).set_scenery(true)
		.finalize());
	let mut tomb = Location::new(
		"Tomb",
		"You stand in a small room, lit by cracks in the stone walls. At the center of the room is a stone sarcophagus.",
		"You can't go that direction."
		);
	tomb.add_item(ItemBuilder::new(
		"pickaxe",
		"a pickaxe",
		"It's a large, unweildy pickaxe, with a head built for busting through rock, rather than dirt."
		).set_damage_type(DamageType::SMASHING)
		.finalize());
	let mut chamber = Location::new(
		"Chamber",
		"You are in a large chamber. A small shaft of light from a skylight illuminates room.",
		"You can't go that direction."
		);
	chamber.add_item(ItemBuilder::new(
		"idol",
		"a small idol",
		"The idol is in the shape of a man and appears to be made entirely of gold."
		).finalize());
	let seal = Location::new(
		"Seal Room",
		"You stand in a small stone room.",
		"You can't go in that direction."
		);
	vec![outside, entrance, fork, hall, pit_trap, tomb, chamber, seal]
}