use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Boolean, Number};

use crate::tests::{eval, parse};

#[test]
fn if_test() {
	evals_and_eq!("(if true true false)", Boolean(true));
	evals_and_eq!("(if true 5 (/ 10 0))", Number(5.0));
	fails_eval!("(if false 5 (/ 10 0))");
}

#[test]
fn cond() {
	evals_and_eq!("(cond false 1.0 false 2.0 true 3.0)", Number(3.0));
	evals_and_eq!("(cond false 1.0 false 2.0 true 3.0 true (/ 4.0 0))", Number(3.0));
}
