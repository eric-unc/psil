use crate::{evals_and_eq, fails_eval};
use crate::val::Val::{Number};

#[test]
fn list() {
	evals_and_eq!("(list)", vec![]);
	evals_and_eq!("(list 1)", vec![Number(1)]);
}

#[test]
fn range() {
	fails_eval!("(range)");
	fails_eval!("(range 1)");
	fails_eval!("(range 1 1 1 1)");
}
