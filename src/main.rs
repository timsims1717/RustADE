#[macro_use]
extern crate lazy_static;

mod game;
mod obj;
mod parsing;
mod world;

use game::gamestate::GameState;
use parsing::grammar::CommandNode;
use parsing::parser;
use world::builder;

fn main() {
	let mut game_state = GameState::new(0, builder::build_world());

	'outer: loop {
		println!("\n{}", match parser::get_next_command() {
			Some(c) => match c {
				CommandNode::INVENTORY => game_state.player().display_inventory(),
				CommandNode::LOOK => format!("{}", game_state.current_location()),
				CommandNode::GO(d) => {
					format!("{}{}",
						game_state.current_location()
							.display_exiting(d),
						match game_state.move_player(d) {
							Some(_) => {
								format!("\n{}", game_state.current_location())
							},
							None => "".to_string()
						}
					)
				}
				CommandNode::GET(i_node) => {
					match game_state.get_item(&i_node) {
						Some(i) => {
							if !i.is_fixed {
								format!("You pick up the {}.\n", i_node.subject_lexeme)
							} else {
								format!("You can't pick up the {}.\n", i_node.subject_lexeme)
							}
						},
						None => format!("You don't see a {} here.\n", i_node.subject_lexeme),
					}
				}
				_ => "Not implemented.".to_string(),
			},
			None => "Invalid command.".to_string(),
		});
		/*words = input.split_whitespace().collect::<Vec<&str>>();
		words.push("end");
		for word in &words {
			test = token::Token::new(word);
			println!("{}", test);
			if test.t_type == token::TokenType::QUIT {
				break 'outer;
			}
		}*/
	}
}