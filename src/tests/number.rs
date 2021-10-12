use crate::val::Val;

use crate::tests::{eval, parse};

#[test]
fn simple_num() {
	let num = "5";
	assert!(parse(num).is_ok());
	assert!(eval(num).is_ok());
	assert_eq!(eval(num).unwrap(), Val::Number(5.0));
}

#[test]
fn negative_num() {
	let num = "-5";
	assert!(parse(num).is_ok());
	assert!(eval(num).is_ok());
	assert_eq!(eval(num).unwrap(), Val::Number(-5.0));
}

#[test]
fn decimal_num() {
	let num = "5.5";
	assert!(parse(num).is_ok());
	assert!(eval(num).is_ok());
	assert_eq!(eval(num).unwrap(), Val::Number(5.5));
}
