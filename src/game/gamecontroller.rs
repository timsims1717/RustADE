

use game::gamestate::GameState;
use obj::DamageType;
use parsing::{GameStateType,DirectionType,OtherType,CommandNode};
use parsing::grammar::{ItemNode,PrepositionNode};
use parsing::parser;

pub fn take_control(mut game_state: GameState) -> Option<CommandNode> {
	println!("\n{}", look(&mut game_state));
	println!("{}", game_state.update());
	let mut command: Option<CommandNode>;
	'control: loop {
		command = parser::get_next_command();
		println!("\n{}", match command.clone() {
			Some(c) => match c {
				CommandNode::GAMESTATE(s) => gamestate(s, &mut game_state),
				CommandNode::INVENTORY => inventory(&mut game_state),
				CommandNode::LOOK(prep) => match prep {
					Some(p_node) => {
						item_look(p_node, &mut game_state)
					},
					None => look(&mut game_state),
				},
				CommandNode::GO(d) => go(d, &mut game_state),
				CommandNode::TAKE(i_node) => get(i_node, &mut game_state),
				CommandNode::DROP(i_node) => drop(i_node, &mut game_state),
				CommandNode::EXAMINE(i_node) => examine(i_node, &game_state),
				CommandNode::USE(i_node) => use_item(i_node, &mut game_state),
				_ => "Not implemented.".to_string(),
			},
			None => "I don't know how to do that.".to_string(),
		});
		println!("{}", game_state.update());
		if game_state.update_player() {
			println!("You win!\n\n");
			break 'control;
		}
		/*words = input.split_whitespace().collect::<Vec<&str>>();
		words.push("end");
		for word in &words {
			test = token::Token::new(word);
			println!("{}", test);
			if test.t_type == token::TokenType::QUIT {
				break 'control;
			}
		}*/
		if game_state.break_control {
			break 'control;
		}
	}
	command
}

fn gamestate(s: GameStateType, game_state: &mut GameState) -> String {
	match s {
		GameStateType::QUIT => match quit() {
			Some(yes) => {
				if yes {
					game_state.break_control = true;
					"Goodbye".to_string()
				} else {
					"Excellent.".to_string()
				}
			},
			None => "I'll take that as a no.".to_string(),
		},
	}
}

fn quit() -> Option<bool> {
	println!("{}", "\nAre you sure you want to quit? Your game will not be saved. (yes/[no])\n");
	match parser::get_next_command() {
		Some(c) => match c {
			CommandNode::GAMESTATE(s) => match s {
				GameStateType::QUIT => Some(true),
			},
			CommandNode::OTHER(o) => match o {
				OtherType::YES => Some(true),
				OtherType::NO => Some(false),
			},
			_ => None,
		},
		None => None,
	}
}

fn inventory(game_state: &mut GameState) -> String {
	game_state.player().display_inventory()
}

fn look(game_state: &mut GameState) -> String {
	format!("{}", game_state.current_location())
}

fn item_look(p_node: PrepositionNode, game_state: &mut GameState) -> String {
	"Looking for item...".to_string()
}

fn go(d: DirectionType, game_state: &mut GameState) -> String {
	format!("{}{}", game_state.current_location().display_exiting(d),
		match game_state.move_player(d) {
			Some(_) => {
				format!("\n{}", game_state.current_location())
			},
			None => "".to_string()
		}
	)
}

fn get(i_node: ItemNode, game_state: &mut GameState) -> String {
	match game_state.get_item(&i_node) {
		Some(i) => {
			if !i.is_fixed {
				format!("You pick up the {}.", i_node.subject_lexeme)
			} else {
				format!("You can't pick up the {}.", i_node.subject_lexeme)
			}
		},
		None => format!("You don't see a {} here.", i_node.subject_lexeme),
	}
}

fn drop(i_node: ItemNode, game_state: &mut GameState) -> String {
	match game_state.drop_item(&i_node) {
		Some(_) => "Dropped.".to_string(),
		None => format!("You aren't carrying a {}.", i_node.subject_lexeme),
	}
}

fn examine(i_node: ItemNode, game_state: &GameState) -> String {
	match game_state.has_item(&i_node) {
		Some(i) => format!("{}", i),
		None => format!("You don't see a {} here.", i_node.subject_lexeme),
	}
}

fn use_item(i_node: ItemNode, game_state: &mut GameState) -> String {
	match game_state.has_item(&i_node) {
		Some(sub) => match game_state.damage_first_item(sub.damage_type.clone()) {
			Some(obj) => format!("You {} the {} with the {}.", match sub.damage_type.clone().unwrap() {
				DamageType::SMASHING => "smash".to_string(),
				DamageType::CUTTING => "cut".to_string(),
			}, obj.i_type, sub.i_type),
			None => match game_state.attach_first_item(&i_node, sub.clone()) {
				Some(obj) => format!("You attach the {} to the {}.", sub.i_type, obj.i_type),
				None => match game_state.turn_on_item(&i_node) {
					Some(b) if b => format!("You turn on the {}.", sub.i_type),
					Some(_) => format!("You turn off the {}.", sub.i_type),
					None => format!("You don't see anything you can use the {} on.", sub.i_type),
				}
			},
		},
		None => format!("You aren't carrying a {}.", i_node.subject_lexeme),
	}
}