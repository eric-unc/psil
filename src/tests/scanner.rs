use crate::scanner::*;

#[test]
fn end() {
	let mut scanner = "".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::End);
}


#[test]
fn word() {
	let mut scanner = "word".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Identifier("word".to_string()));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn letter() {
	let mut scanner = "l".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Identifier("l".to_string()));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn simple_num() {
	let mut scanner = "5".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Number(5.0));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn long_num() {
	let mut scanner = "6000".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Number(6000.0));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn negative_num() {
	let mut scanner = "-7000".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Number(-7000.0));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn decimal_num() {
	let mut scanner = "0.56".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Number(0.56));
	assert_eq!(scan(&mut scanner), Token::End);
}

#[test]
fn negative_decimal_num() {
	let mut scanner = "-0.399".chars().peekable();

	assert_eq!(scan(&mut scanner), Token::Number(-0.399));
	assert_eq!(scan(&mut scanner), Token::End);
}