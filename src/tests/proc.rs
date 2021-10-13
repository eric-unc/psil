use crate::{evals_and_eq, fails_eval};
use crate::val::Val::Boolean;

use crate::tests::{eval, parse};

// TODO: could test invocations and stuff

#[test]
fn proc_literals() {
	assert!(eval("{|x| x}").is_ok());
}

#[test]
fn is_proc() {
	evals_and_eq!("(is-proc? {|x| x})", Boolean(true));
	evals_and_eq!("(is-proc? is-proc?)", Boolean(true));
	evals_and_eq!("(is-proc? 5)", Boolean(false));

	fails_eval!("(is-proc?)");
	fails_eval!("(is-proc? {|x| x} {|x| x})");
}
