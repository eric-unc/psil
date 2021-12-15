use crate::{evals_and_eq, fails_eval};
use crate::val::void;

use crate::tests::{eval, parse};

// TODO: kind of a pain to test these properly

#[test]
fn put() {
	evals_and_eq!("(put)", void());
	evals_and_eq!("(put \"Hello world!\")", void());
}

#[test]
fn put_each() {
	fails_eval!("(put-each)");
	evals_and_eq!("(put-each \"Hello\" \"world!\")", void());
}

#[test]
fn print() {
	fails_eval!("(print)");
	evals_and_eq!("(print \"Hello\" \"world!\")", void());
}

#[test]
fn input() {
	fails_eval!("(input 1)");
}
