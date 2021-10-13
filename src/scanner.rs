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

#[derive(Clone, Debug)]
pub struct Scanner {
	text: String
}

impl Scanner {
	pub fn new(text: String) -> Self {
		Self { text }
	}

	pub fn scan(&mut self) -> Token {
		// TODO
		Token::Boolean(true)
	}
}
