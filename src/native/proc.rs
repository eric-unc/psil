use std::borrow::Borrow;
use crate::{check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Procedure};

pub fn add_procedure_procs(env: &mut Environment) {
	env.add_proc("is-proc?".to_string(), is_proc);
}

fn is_proc(rands: ValList) -> Result<Val, String> {
	check_arity_is!("is-proc?", 1, rands);

	match rands[0] {
		Procedure(_) => Ok(Boolean(true)),
		_ => Ok(Boolean(false))
	}
}
