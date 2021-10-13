use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Boolean, Number};

use crate::tests::{eval, parse};

#[test]
fn add() {
	evals_and_eq!("(+ 5 5)", Number(10.0));
	evals_and_eq!("(+ 1 2 3 4)", Number(10.0));
	fails_eval!("(+)");
	fails_eval!("(+ 1)");
	fails_eval!("(+ 5 true)");
}

#[test]
fn sub() {
	evals_and_eq!("(- 5 5)", Number(0.0));
	evals_and_eq!("(- 50 10 30)", Number(10.0));
	fails_eval!("(-)");
	fails_eval!("(- 1)");
	fails_eval!("(- 1 -)");
}

#[test]
fn multiply() {
	evals_and_eq!("(* 5 5)", Number(25.0));
	evals_and_eq!("(* 1 2 3 4)", Number(24.0));
	fails_eval!("(*)");
	fails_eval!("(* 1)");
	fails_eval!("(* 5 true)");
}

#[test]
fn divide() {
	evals_and_eq!("(/ 25 5)", Number(5.0));
	evals_and_eq!("(/ 25 1 5 5)", Number(1.0));
	fails_eval!("(/)");
	fails_eval!("(/ 1)");
	fails_eval!("(/ 5 true)");
	fails_eval!("(/ 5 0)");
}

#[test]
fn remainder() {
	evals_and_eq!("(% 25 5)", Number(0.0));
	evals_and_eq!("(% 26 5)", Number(1.0));
	evals_and_eq!("(% 5 5 1)", Number(0.0));
	fails_eval!("(%)");
	fails_eval!("(% 1)");
	fails_eval!("(% 5 true)");
	fails_eval!("(% 5 0)");
}

#[test]
fn is_num() {
	evals_and_eq!("(is-num? 5)", Boolean(true));
	evals_and_eq!("(is-num? true)", Boolean(false));
	fails_eval!("(is-num?)");
	fails_eval!("(is-num? 5 9)");
}
