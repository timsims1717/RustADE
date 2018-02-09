#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate uuid;

mod game;
mod obj;
mod parsing;
mod world;

use game::gamestate::GameState;
use game::gamecontroller;
// use parsing::grammar::CommandNode;
// use parsing::token::GameStateType;
use world::builder;

fn main() {
	let game_state = GameState::new(0, builder::build_fixed_world());

	// println!("Completed on a{} command.", match gamecontroller::take_control(game_state) {
	// 	Some(c) => match c {
	// 		CommandNode::GAMESTATE(s) => match s {
	// 			GameStateType::QUIT => " QUIT".to_string(),
	// 		},
	// 		_ => " valid".to_string(),
	// 	},
	// 	None => "n invalid".to_string(),
	// })
	gamecontroller::take_control(game_state);
}