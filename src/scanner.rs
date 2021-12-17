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
	StringT(String),
	Symbol(String),

	// Special forms/keywords
	If,
	Cond,
	Define,
	Do,
	And,
	Or
}
use Token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ScannerError {
	IncompleteString,
	UnknownEscapeChar(char)
}
use ScannerError::*;

pub struct Scanner<'a> {
	peek_token: Option<Result<Token, ScannerError>>,
	iter: Peekable<Chars<'a>>
}

impl<'a> Scanner<'a> {
	pub fn new(text: &'a str) -> Self {
		let iter = text.chars().peekable();

		Self {
			peek_token: None,
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
				'"' => return Ok(StringT(str)),
				'\\' => match self.iter.next() {
					None => return Err(IncompleteString),
					Some('\\') => str.push('\\'),
					Some('n') => str.push('\n'),
					Some('r') => str.push('\r'),
					Some('t') => str.push('\t'),
					Some('"') => str.push('"'),
					Some(c) => return Err(UnknownEscapeChar(c))
				}
				c => str.push(c) // and continue;
			}
		}

		Err(IncompleteString)
	}

	fn read_word(&mut self) -> Result<Token, ScannerError> {
		let mut ret = String::from("");

		while self.iter.peek().is_some() && is_iden_char(*self.iter.peek().unwrap()) {
			ret.push(self.iter.next().unwrap());
		}

		Ok(match ret.chars().nth(0).unwrap() {
			'-' | '.' | '0'..='9' =>
				match ret.parse::<f64>() { // TODO: this will be improved in the big number update
					Ok(n) => Number(n),
					Err(_) => Identifier(ret)
				}
			'#' => {
				let ret = &ret[1..];
				Symbol(ret.to_string())
			}
			_ => match ret.as_str() {
				"true" => Boolean(true),
				"false" => Boolean(false),
				"if" => If,
				"cond" => Cond,
				"define" => Define,
				"do" => Do,
				"and" => And,
				"or" => Or,
				_ => Identifier(ret)
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
			None => Ok(End),
			Some(';') => {
				self.remove_sl_comment();
				self.scan()
			}
			Some('(') => {
				self.iter.next();
				Ok(LeftParen)
			}
			Some(')') => {
				self.iter.next();
				Ok(RightParen)
			}
			Some('{') => {
				self.iter.next();
				Ok(LeftBracket)
			}
			Some('}') => {
				self.iter.next();
				Ok(RightBracket)
			}
			Some('|') => {
				self.iter.next();
				Ok(Bar)
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
			ret
		}
	}
}

fn is_iden_char(c: char) -> bool {
	!(c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == '|')
}
