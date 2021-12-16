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
	evals_and_eq!("(range 1 2)", List(vec![Number(1.0), Number(2.0)]));
	evals_and_eq!("(range 2 5)", List(vec![Number(2.0), Number(3.0), Number(4.0), Number(5.0)]));
	evals_and_eq!("(range 2 8 2)", List(vec![Number(2.0), Number(4.0), Number(6.0), Number(8.0)]));
	fails_eval!("(range)");
	fails_eval!("(range 1)");
	fails_eval!("(range 1 1 1 1)");
}
