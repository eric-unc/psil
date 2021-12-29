use std::collections::HashMap;
use crate::{check_arity_at_least, check_arity_between, check_arity_even, check_arity_is, fail_on_bad_type, get_integer, get_list, get_natural_number, get_proc, load_into};
use crate::environment::Environment;
use crate::eval::eval_proc_with_rands;
use crate::val::{Val, ValList, void};
use crate::val::Val::{Boolean, Number, List, ProcedureV, Table};

pub fn add_native(env: &mut Environment) {
	env.add_proc("is-table?", is_table);
	env.add_proc("table", table);
}

fn is_table(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-table?", 1, rands);

	Ok(Boolean(matches!(rands[0], Table(_))))
}

fn table(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_even!("table", rands);

	let mut table = HashMap::new();

	for i in 0..(rands.len()/2) {
		let k = rands[2 * i].clone();
		let v = rands[2 * i + 1].clone();
		table.insert(k.to_string(), v);
	}

	Ok(Table(table))
}
