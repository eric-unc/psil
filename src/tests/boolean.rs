use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Boolean, Number, String};

use crate::tests::{eval, parse};

#[test]
fn boolean_literals() {
	evals_and_eq!("true", Boolean(true));
	evals_and_eq!("false", Boolean(false));
}

#[test]
fn and() {
	// TODO: could be expanded
	evals_and_eq!("(and true true)", Boolean(true));
	evals_and_eq!("(and true false)", Boolean(false));
	evals_and_eq!("(and true true true)", Boolean(true));
	evals_and_eq!("(and false (/ 1 0))", Boolean(false));
}

#[test]
fn or() {
	// TODO: could be expanded
	evals_and_eq!("(or false true)", Boolean(true));
	evals_and_eq!("(or false false)", Boolean(false));
	evals_and_eq!("(or false false true)", Boolean(true));
	evals_and_eq!("(or true (/ 1 0))", Boolean(true));
}

#[test]
fn not() {
	evals_and_eq!("(not false)", Boolean(true));
	evals_and_eq!("(not true)", Boolean(false));

	fails_eval!("(not)");
	fails_eval!("(not true true)");
}

#[test]
fn xor() {
	evals_and_eq!("(xor false false)", Boolean(false));
	evals_and_eq!("(xor true false)", Boolean(true));
	evals_and_eq!("(xor false true)", Boolean(true));
	evals_and_eq!("(xor true true)", Boolean(false));
	evals_and_eq!("(xor true true true)", Boolean(true));

	fails_eval!("(xor)");
	fails_eval!("(xor true)");
}

#[test]
fn eq() {
	// For more equality tests, see tests/vals.rs
	evals_and_eq!("(== 5 5)", Boolean(true));
	evals_and_eq!("(== 7 5)", Boolean(false));
	evals_and_eq!("(== 5 7)", Boolean(false));
	evals_and_eq!("(== 7 7)", Boolean(true));
	evals_and_eq!("(== 7 7 7)", Boolean(true));

	fails_eval!("(==)");
	fails_eval!("(== true)");
}

#[test]
fn neq() {
	// For more equality tests, see tests/vals.rs
	evals_and_eq!("(!= 5 5)", Boolean(false));
	evals_and_eq!("(!= 7 5)", Boolean(true));
	evals_and_eq!("(!= 5 7)", Boolean(true));
	evals_and_eq!("(!= 7 7)", Boolean(false));
	evals_and_eq!("(!= 5 6 6)", Boolean(false));

	fails_eval!("(!=)");
	fails_eval!("(!= true)");
}

#[test]
fn gt() {
	evals_and_eq!("(> 9 5)", Boolean(true));
	evals_and_eq!("(> 9 5 4)", Boolean(true));
	evals_and_eq!("(> 3 5)", Boolean(false));
	evals_and_eq!("(> 3 3)", Boolean(false));

	fails_eval!("(>)");
	fails_eval!("(> 9)");
	fails_eval!("(> 9 true)");
}

#[test]
fn gte() {
	evals_and_eq!("(>= 9 5)", Boolean(true));
	evals_and_eq!("(>= 9 9)", Boolean(true));
	evals_and_eq!("(>= 9 5 4)", Boolean(true));
	evals_and_eq!("(>= 3 5)", Boolean(false));

	fails_eval!("(>=)");
	fails_eval!("(>= 9)");
	fails_eval!("(>= 9 true)");
}

#[test]
fn lt() {
	evals_and_eq!("(< 5 9)", Boolean(true));
	evals_and_eq!("(< 3 5 4 9)", Boolean(true));
	evals_and_eq!("(< 5 3)", Boolean(false));
	evals_and_eq!("(< 3 3)", Boolean(false));

	fails_eval!("(<)");
	fails_eval!("(< 9)");
	fails_eval!("(< 9 true)");
}

#[test]
fn lte() {
	evals_and_eq!("(<= 5 9)", Boolean(true));
	evals_and_eq!("(<= 9 9)", Boolean(true));
	evals_and_eq!("(<= 3 9 5 4)", Boolean(true));
	evals_and_eq!("(<= 5 3)", Boolean(false));

	fails_eval!("(<=)");
	fails_eval!("(<= 9)");
	fails_eval!("(<= 9 true)");
}

#[test]
fn is_bool() {
	evals_and_eq!("(is-bool? true)", Boolean(true));
	evals_and_eq!("(is-bool? false)", Boolean(true));
	evals_and_eq!("(is-bool? 9)", Boolean(false));

	fails_eval!("(is-bool?)");
	fails_eval!("(is-bool? 9 9)");
}
