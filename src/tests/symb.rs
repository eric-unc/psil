use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Boolean, String, Symbol};

use crate::tests::{eval, parse};

#[test]
fn symb_literals() {
	evals_and_eq!("#left", Symbol("left".to_string()));
	evals_and_eq!("#right", Symbol("right".to_string()));
}

#[test]
fn str_to_symb() {
	evals_and_eq!("(str2symb \"left\")", Symbol("left".to_string()));

	fails_eval!("(str2symb)");
	fails_eval!("(str2symb \"left\" \"left\")");
	fails_eval!("(str2symb 5)");
}

#[test]
fn symb_to_str() {
	evals_and_eq!("(symb2str #left)", String("left".to_string()));

	fails_eval!("(symb2str)");
	fails_eval!("(symb2str #left #left)");
	fails_eval!("(symb2str 5)");
}

#[test]
fn is_symb() {
	evals_and_eq!("(is-symb? #symb)", Boolean(true));
	evals_and_eq!("(is-symb? 5)", Boolean(false));

	fails_eval!("(is-symb?)");
	fails_eval!("(is-symb? #symb #symb)");
}

#[test]
fn is_void() {
	evals_and_eq!("(is-void? #void)", Boolean(true));
	evals_and_eq!("(is-void? #symb)", Boolean(false));
	evals_and_eq!("(is-void? 5)", Boolean(false));

	fails_eval!("(is-void?)");
	fails_eval!("(is-void? #void #void)");
}