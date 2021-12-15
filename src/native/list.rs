use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::List;

pub fn add_list_procs(env: &mut Environment) {
	env.add_proc("list".to_string(), list);
}

fn list(rands: ValList) -> Result<Val, String> {
	Ok(List(rands))
}
