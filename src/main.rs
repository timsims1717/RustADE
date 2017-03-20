
use std::io;

mod parser;
use parser::{token, state_machine, scanner};

fn main() {
	let mut test = token::Token { t_type: token::TokenType::WORD, lexeme: "go".to_string() };

	let mut input = String::new();
	io::stdin().read_line(&mut input)
			.expect("Failed to read line");

	input.push('\n');
	let mut buf = input.into_bytes();
	let mut i = 0;
	loop {
		if i == buf.len() || test.t_type == token::TokenType::NEWLINE {
			break;
		}
		i = scanner::get_next_token(&buf, &mut test);
		println!("{}", test);
		buf = buf[i..buf.len()].to_vec();
	}
}