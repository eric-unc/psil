use std::str::Chars;

#[derive(Clone, Debug)]
pub enum Token {
	// main
	Identifier(String),
	End,

	// symbols
	LeftParen,
	RightParen,
	LeftBracket,
	RightBracket,
	Bar,

	// literals
	Number(f64),
	Boolean(bool),
	String(String),
	Symbol(String)
}

// TODO

#[derive(Clone, Debug)]
pub struct Scanner {
	text: String,
	position: usize
}

impl Scanner {
	pub fn new(text: String) -> Self {
		Self { text, position: 0 }
	}

	pub fn scan(&mut self) -> Token {
		let mut curr_char = self.text.chars().nth(0).unwrap();
	}
}
