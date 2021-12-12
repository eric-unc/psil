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
	Symbol(String)
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

fn read_string(iter: &mut Scanner) -> Token {
	let c = iter.next();
	if c.is_none() || c.unwrap() != '"' {
		panic!("missing \"");
	}

	let mut str = String::from("");
	while iter.peek().is_some() {
		match iter.next().unwrap() {
			'"' => return Token::String(str),
			'\\' => {
				match iter.next() {
					None => panic!("Incomplete string!"),
					Some('\\') => str.push('\\'),
					Some('n') => str.push('\n'),
					Some('r') => str.push('\r'),
					Some('t') => str.push('\t'),
					Some('"') => str.push('"'),
					Some(_) => panic!("Unexpected escape character!")
				}
			}
			c => str.push(c) // and continue;
		}
	}

	panic!("Incomplete string!")
}

fn read_word(iter: &mut Scanner) -> Token {
	let mut ret = String::from("");

	while iter.peek().is_some() && !iter.peek().unwrap().is_whitespace() {
		ret.push(iter.next().unwrap());
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

pub fn scan(iter: &mut Scanner) -> Token {
	remove_whitespace(iter);
	match iter.peek() {
		None => Token::End,
		Some(';') => {
			remove_sl_comment(iter);
			scan(iter)
		}
		Some('(') => {
			iter.next();
			Token::LeftParen
		}
		Some(')') => {
			iter.next();
			Token::RightParen
		}
		Some('{') => {
			iter.next();
			Token::LeftBracket
		}
		Some('}') => {
			iter.next();
			Token::RightBracket
		}
		Some('|') => {
			iter.next();
			Token::Bar
		}
		Some('"') => read_string(iter),
		Some(_) => read_word(iter)
	}
}
