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
pub struct Scanner<'a> {
	text: Chars<'a>,
	position: usize
}

impl Scanner {
	pub fn new(text: String) -> Self {
		Self { text: text.chars(), position: 0 }
	}

	pub fn scan(&mut self) -> Token {
		let curr_char = self.text;
		let x = curr_char.nth(0).unwrap();
	}
}
