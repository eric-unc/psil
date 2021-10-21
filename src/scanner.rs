#[derive(Clone, Debug, PartialEq)]
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
	text: String,
	pos: usize
}

impl Scanner {
	pub fn new(text: String) -> Self {
		Self { text, pos: 0 }
	}

	fn remove_whitespace(&mut self) {
		let mut chars = self.text.chars();

		let mut possible_char = chars.nth(self.pos);
		while possible_char.is_some() && possible_char.unwrap().is_whitespace() {
			self.pos += 1;
			possible_char = chars.nth(self.pos);
		}
	}

	fn remove_sl_comment(&mut self) {
		let mut chars = self.text.chars();

		let mut possible_char = chars.nth(self.pos);
		while possible_char.is_some() && possible_char.unwrap() != '\n' {
			self.pos += 1;
			possible_char = chars.nth(self.pos);
		}
	}

	fn read_string(&mut self) -> Token {
		let mut chars = self.text.chars();

		let mut possible_char = chars.nth(self.pos);
		if possible_char.is_none() || possible_char.unwrap() != '"' {
			panic!();
		}
		self.pos += 1;

		let mut str = String::from("");

		possible_char = chars.nth(self.pos);
		while possible_char.is_some() {
			match possible_char.unwrap() {
				'"' => {
					self.pos += 1;
					return Token::String(str);
				},
				'\\' => {
					self.pos += 1;

					match chars.nth(self.pos) {
						None => panic!("Incomplete string!"),
						Some('\\') => {
							str.push('\\');
							self.pos += 1;
						}
						Some('n') => {
							str.push('\n');
							self.pos += 1;
						}
						Some('r') => {
							str.push('\r');
							self.pos += 1;
						}
						Some('t') => {
							str.push('\t');
							self.pos += 1;
						}
						Some('"') => {
							str.push('"');
							self.pos += 1;
						}
						Some(_) => panic!("Unexpected escape character!")
					}
				}
				c => {
					str.push(c);
					self.pos += 1;
				} // and continue;
			}
		}

		panic!("Incomplete string!")
	}

	fn read_word(&mut self) -> Token {
		let mut chars = self.text.chars();
		let mut ret = String::from("");

		let mut possible_char = chars.nth(self.pos);
		while possible_char.is_some() && !possible_char.unwrap().is_whitespace() {
			ret.push(possible_char.unwrap());
			self.pos += 1;
			possible_char = chars.nth(self.pos);
		}

		match ret.chars().nth(0).unwrap() {
			'-' | '.' | '0'..='9' => {
				match ret.parse::<f64>() {
					Ok(n) => Token::Number(n),
					Err(_) => Token::Identifier(ret)
				}
			}
			'#' => {
				let ret = &ret[1..];
				Token::Symbol(ret.to_string())
			}
			_ => {
				match ret.as_str() {
					"true" => Token::Boolean(true),
					"false" => Token::Boolean(true),
					_ => Token::Identifier(ret)
				}
			}
		}
	}

	pub fn scan(&mut self) -> Token {
		self.remove_whitespace();

		match self.text.chars().nth(self.pos) {
			None => Token::End,
			Some(';') => {
				self.remove_sl_comment();
				self.scan()
			}
			Some('(') => {
				self.pos += 1;
				Token::LeftParen
			}
			Some(')') => {
				self.pos += 1;
				Token::RightParen
			}
			Some('{') => {
				self.pos += 1;
				Token::LeftBracket
			}
			Some('}') => {
				self.pos += 1;
				Token::RightBracket
			}
			Some('|') => {
				self.pos += 1;
				Token::Bar
			}
			Some('"') => self.read_string(),
			Some(_) => self.read_word()
		}
	}
}
