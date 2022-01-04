use std::collections::HashMap;
use crate::{check_arity_even, check_arity_is};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Table};

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
