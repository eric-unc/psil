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

type Scanner<'a> = Peekable<Chars<'a>>;

fn remove_whitespace(iter: &mut Scanner) {
	while iter.peek().is_some() && iter.peek().unwrap().is_whitespace() {
		iter.next();
	}
}

fn remove_sl_comment(iter: &mut Scanner) {
	while iter.peek().is_some() && iter.next().unwrap() != '\n' {
		// nothing
	}
}

fn read_string(iter: &mut Scanner) -> Result<Token, ScannerError> {
	iter.next().unwrap(); // get rid of "

	let mut str = String::from("");
	while iter.peek().is_some() {
		match iter.next().unwrap() {
			'"' => return Ok(Token::String(str)),
			'\\' => match iter.next() {
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

fn is_iden_char(c: char) -> bool {
	!(c.is_whitespace() || c == '(' || c == ')' || c == '{' || c == '}' || c == '|')
}

fn read_word(iter: &mut Scanner) -> Result<Token, ScannerError> {
	let mut ret = String::from("");

	while iter.peek().is_some() && is_iden_char(*iter.peek().unwrap()) {
		ret.push(iter.next().unwrap());
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

pub fn scan(iter: &mut Scanner) -> Result<Token, ScannerError> {
	remove_whitespace(iter);
	match iter.peek() {
		None => Ok(Token::End),
		Some(';') => {
			remove_sl_comment(iter);
			scan(iter)
		}
		Some('(') => {
			iter.next();
			Ok(Token::LeftParen)
		}
		Some(')') => {
			iter.next();
			Ok(Token::RightParen)
		}
		Some('{') => {
			iter.next();
			Ok(Token::LeftBracket)
		}
		Some('}') => {
			iter.next();
			Ok(Token::RightBracket)
		}
		Some('|') => {
			iter.next();
			Ok(Token::Bar)
		}
		Some('"') => read_string(iter),
		Some(_) => read_word(iter)
	}
}
