use parser::token;

const MACHINE_STATE_SIZE: usize = 5;
const CHAR_TYPE_SIZE: usize = 4;

#[derive(Clone, Copy, PartialEq)]
pub enum MachineState {
	// Control States
	START, CANTMOVE, NEWLINE, BAD,
	// Token State
	TOKEN,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CharType {
	LETTER, WHITESPACE, BAD, NEWLINE,
}

pub struct StateMachine {
	current_state: MachineState,
	legal_moves: [[MachineState; CHAR_TYPE_SIZE]; MACHINE_STATE_SIZE],
	corresponding_token_types: [token::TokenType; MACHINE_STATE_SIZE],
}

impl StateMachine {
	pub fn new() -> StateMachine {
		let mut sm = StateMachine {
			current_state: MachineState::START,
			legal_moves: [[MachineState::CANTMOVE; CHAR_TYPE_SIZE]; MACHINE_STATE_SIZE],
			corresponding_token_types: [token::TokenType::BAD; MACHINE_STATE_SIZE],
		};
		sm.fill_arrays();
		sm
	}

	fn fill_arrays(&mut self) {
		// LEGAL MOVES MATRIX
		// Token State
		self.legal_moves[MachineState::START as usize][CharType::LETTER as usize] = MachineState::TOKEN;
		self.legal_moves[MachineState::TOKEN as usize][CharType::LETTER as usize] = MachineState::TOKEN;
		// New Line State
		self.legal_moves[MachineState::START as usize][CharType::NEWLINE as usize] = MachineState::NEWLINE;
		// Start State
		self.legal_moves[MachineState::START as usize][CharType::WHITESPACE as usize] = MachineState::START;

		// CORRESPONDING TOKEN ARRAY
		self.corresponding_token_types[MachineState::TOKEN as usize] = token::TokenType::WORD;
		self.corresponding_token_types[MachineState::NEWLINE as usize] = token::TokenType::NEWLINE;
	}

	pub fn update_state(&mut self, c: &char, tt: &mut token::TokenType) -> MachineState {
		// convert the input character into an input character type
		*tt = self.corresponding_token_types[self.current_state as usize];
		let ct = get_corresponding_char(c);
		self.current_state = self.legal_moves[self.current_state as usize][ct as usize];
		self.current_state
	}
}

pub fn get_corresponding_char(c: &char) -> CharType {
	if c.is_alphabetic() {
		return CharType::LETTER;
	}
	if c.eq(&'\n') {
		return CharType::NEWLINE;
	}
	if c.is_whitespace() {
		return CharType::WHITESPACE;
	}
	CharType::BAD
}