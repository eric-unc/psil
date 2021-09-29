use crate::{check_arity_at_least, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::Number;

pub fn add_math_procs(env: &mut Environment) {
	env.add_proc("+".to_string(), add);
	env.add_proc("-".to_string(), subtract);
	env.add_proc("*".to_string(), multiply);
	env.add_proc("/".to_string(), divide);
	env.add_proc("%".to_string(), remainder);
}

fn add(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("+", 2, rands);

	let mut ret = 0.0;

	for val in rands {
		match val {
			Number(n) => ret += n,
			_ => fail_on_bad_type!("+", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn subtract(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("-", 2, rands);

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret -= n
				}
			_ => fail_on_bad_type!("-", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn multiply(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("*", 2, rands);

	let mut ret = 1.0;

	for val in rands {
		match val {
			Number(n) => ret *= n,
			_ => fail_on_bad_type!("*", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn divide(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("/", 2, rands);

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret /= n
				}
			_ => fail_on_bad_type!("/", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn remainder(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("%", 2, rands);

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret %= n
				}
			_ => fail_on_bad_type!("%", "number", rands)
		}
	}

	Ok(Number(ret))
}
