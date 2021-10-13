use crate::evals_and_eq;
use crate::val::Val;

use crate::tests::{eval, parse};

#[test]
fn simple_num() {
	evals_and_eq!("5", Val::Number(5.0));
}

#[test]
fn negative_num() {
	evals_and_eq!("-5", Val::Number(-5.0));
}

#[test]
fn decimal_num() {
	evals_and_eq!("5.5", Val::Number(5.5));
}
