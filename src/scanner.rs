use std::iter::Peekable;
use std::str::Chars;

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
	Symbol(String),

	// Special forms/keywords
	If,
	Cond,
	Define,
	Do,
	And,
	Or
}

#[derive(Clone, Debug, PartialEq)]
pub enum ScannerError {
	IncompleteString,
	UnknownEscapeChar(char)
}

pub struct Scanner<'a> {
	peek_token: Option<Result<Token, ScannerError>>,
	text: &'a str,
	iter: Peekable<Chars<'a>>
}

impl<'a> Scanner<'a> {
	pub fn new_scanner(text: &'a str) -> Self {
		let iter = text.chars().peekable();

		Self {
			peek_token: None,
			text,
			iter
		}
	}

	fn remove_whitespace(&mut self) {
		while self.iter.peek().is_some() && self.iter.peek().unwrap().is_whitespace() {
			self.iter.next();
		}
	}

	fn remove_sl_comment(&mut self) {
		while self.iter.peek().is_some() && self.iter.next().unwrap() != '\n' {
			// nothing
		}
	}

	fn read_string(&mut self) -> Result<Token, ScannerError> {
		self.iter.next().unwrap(); // get rid of "

		let mut str = String::from("");
		while self.iter.peek().is_some() {
			match self.iter.next().unwrap() {
				'"' => return Ok(Token::String(str)),
				'\\' => match self.iter.next() {
					None => return Err(ScannerError::IncompleteString),
					Some('\\') => str.push('\\'),
					Some('n') => str.push('\n'),
					Some('r') => str.push('\r'),
					Some('t') => str.push('\t'),
					Some('"') => str.push('"'),
					Some(c) => return Err(ScannerError::UnknownEscapeChar(c))
				}
				c => str.push(c) // and continue;
			}
		}

		Err(ScannerError::IncompleteString)
	}

	fn read_word(&mut self) -> Result<Token, ScannerError> {
		let mut ret = String::from("");

		while self.iter.peek().is_some() && is_iden_char(*self.iter.peek().unwrap()) {
			ret.push(self.iter.next().unwrap());
		}

		Ok(match ret.chars().nth(0).unwrap() {
			'-' | '.' | '0'..='9' =>
				match ret.parse::<f64>() { // TODO: this will be improved in the big number update
					Ok(n) => Token::Number(n),
					Err(_) => Token::Identifier(ret)
				}
			'#' => {
				let ret = &ret[1..];
				Token::Symbol(ret.to_string())
			}
			_ => match ret.as_str() {
				"true" => Token::Boolean(true),
				"false" => Token::Boolean(false),
				"if" => Token::If,
				"cond" => Token::Cond,
				"define" => Token::Define,
				"do" => Token::Do,
				"and" => Token::And,
				"or" => Token::Or,
				_ => Token::Identifier(ret)
			}
		})
	}

	pub fn scan(&mut self) -> Result<Token, ScannerError> {
		if self.peek_token.is_some() {
			let ret = self.peek_token.as_ref().unwrap().clone();
			self.peek_token = None;
			return ret;
		}

		self.remove_whitespace();
		let token = match self.iter.peek() {
			None => Ok(Token::End),
			Some(';') => {
				self.remove_sl_comment();
				self.scan()
			}
			Some('(') => {
				self.iter.next();
				Ok(Token::LeftParen)
			}
			Some(')') => {
				self.iter.next();
				Ok(Token::RightParen)
			}
			Some('{') => {
				self.iter.next();
				Ok(Token::LeftBracket)
			}
			Some('}') => {
				self.iter.next();
				Ok(Token::RightBracket)
			}
			Some('|') => {
				self.iter.next();
				Ok(Token::Bar)
			}
			Some('"') => self.read_string(),
			Some(_) => self.read_word()
		};

		token
	}

	pub fn peek(&mut self) -> Result<Token, ScannerError> {
		if self.peek_token.is_some() {
			self.peek_token.as_ref().unwrap().clone()
		} else {
			let ret = self.scan();
			self.peek_token = Some(ret.clone());
			ret.clone()
		}
	}
}

fn is_iden_char(c: char) -> bool {
	!(c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == '|')
}
