use parser::state_machine;
use parser::token;

/*
Change:
This function will receive a String and will return a Token.
Easy peasy. Remove the State Machine altogether.
*/

pub fn get_next_token(buf: &Vec<u8>, token: &mut token::Token) -> usize {
	let mut dfa = state_machine::StateMachine::new();
	let mut token_type = token::TokenType::BAD;
	let mut c: char;
	let mut state: state_machine::MachineState;
	let mut lexeme = String::new();
	let mut i = 0;
	loop {
		if i == buf.len() {
			break;
		} else {
			c = buf[i] as char;
			state = dfa.update_state(&c, &mut token_type);
		}
		if state == state_machine::MachineState::CANTMOVE {
			break;
		}
		if state != state_machine::MachineState::START {
			lexeme.push(c);
		}
		i += 1;
	}
	*token = token::Token { t_type: token_type, lexeme: lexeme };
	token.check_reserved();
	return i;
}