use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Number, List};
use crate::tests::{eval, parse};

#[test]
fn list() {
	evals_and_eq!("(list)", List(vec![]));
	evals_and_eq!("(list 1)", List(vec![Number(1.0)]));
}

#[test]
fn range() {
	fails_eval!("(range)");
	fails_eval!("(range 1)");
	fails_eval!("(range 1 1 1 1)");
}
