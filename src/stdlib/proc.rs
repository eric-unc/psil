use crate::check_arity_is;
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, ProcedureV};

pub fn add_native(env: &mut Environment) {
	env.add_proc("is-proc?", is_proc);
}

fn is_proc(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-proc?", 1, rands);

	Ok(Boolean(matches!(rands[0], ProcedureV(_))))
}
