

use obj::item::Item;
use obj::location::Location;
use parsing::token::DirectionType;
use parsing::token::ItemType;

pub fn build_world() -> Vec<Location> {
	let entrance = Location::new("Entrance", "You stand outside in the jungle at the entrance to an underground stone temple. Trees fill your view everywhere you look.", "You shouldn't go into the forest without a guide.");
	let tunnel = Location::new("Bend in the Tunnel", "You are at a bend in a stone-lined tunnel, just inside the temple. Despite how close you are to the outside, the light has a difficult time penetrating this deep.", "You can't go that direction.");
	let chamber = Location::new("Chamber", "You are in a large antechamber. A small shaft of light from a skylight illuminates a stone altar in the exact center.", "You can't go in that direction.");
	let mut locations = vec![entrance, tunnel, chamber];
	locations[0].create_exit(DirectionType::EAST, 1, "The entrance, a tunnel outlined in carved stone, lies to the east.", "After hesitating a bare moment, you duck into the temple's entrance.");
	locations[1].create_exit(DirectionType::WEST, 0, "To the west, light beckons from the entrance.", "You hurriedly walk back out into the light.");
	locations[1].create_exit(DirectionType::SOUTH, 2, "To the south, the tunnel continues further underground.", "Steeling your nerves, you continue down the dark tunnel. Very soon, you are navigating using only your hand on the wall, but eventually you see light ahead. You exit the tunnel into a large room.");
	locations[2].create_exit(DirectionType::NORTH, 1, "A tunnel leads to the north.", "You leave the chamber through the dark tunnel, eventually coming to a bend.");
	locations[0].add_item(Item::new(ItemType::KEY, "a small key", "It's a key with funny little markings on the side. Oh, it's just the manufacturer.", false, false));
	locations[0].add_item(Item::new(ItemType::TREE, "trees of all sizes", "You see jungle trees in all directions, with green leaves and green moss on their trunks. They loom ominously.", true, true));
	locations[2].add_item(Item::new(ItemType::ALTAR, "a stone altar", "The altar is a cylinder made of stones and mortar.", true, false));
	locations[2].add_item(Item::new(ItemType::IDOL, "a small idol", "The idol is in the shape of a man and appears to be made entirely of gold.", false, false));
	locations
}