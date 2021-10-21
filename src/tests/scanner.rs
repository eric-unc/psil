use crate::scanner::*;

#[test]
fn end() {
	let mut scanner = Scanner::new("".to_string());

	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn word() {
	let mut scanner = Scanner::new("word".to_string());

	assert_eq!(scanner.scan(), Token::Identifier("word".to_string()));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn letter() {
	let mut scanner = Scanner::new("l".to_string());

	assert_eq!(scanner.scan(), Token::Identifier("l".to_string()));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn simple_num() {
	let mut scanner = Scanner::new("5".to_string());

	assert_eq!(scanner.scan(), Token::Number(5.0));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn long_num() {
	let mut scanner = Scanner::new("6000".to_string());

	assert_eq!(scanner.scan(), Token::Number(6000.0));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn negative_num() {
	let mut scanner = Scanner::new("-7000".to_string());

	assert_eq!(scanner.scan(), Token::Number(-7000.0));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn decimal_num() {
	let mut scanner = Scanner::new("0.56".to_string());

	assert_eq!(scanner.scan(), Token::Number(0.56));
	assert_eq!(scanner.scan(), Token::End);
}

#[test]
fn negative_decimal_num() {
	let mut scanner = Scanner::new("-0.399".to_string());

	assert_eq!(scanner.scan(), Token::Number(-0.399));
	assert_eq!(scanner.scan(), Token::End);
}
