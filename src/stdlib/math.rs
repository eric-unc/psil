use crate::{check_arity_is, check_arity_at_least, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Number};

pub fn add_native(env: &mut Environment) {
	env.add_proc("+", add);
	env.add_proc("-", subtract);
	env.add_proc("*", multiply);
	env.add_proc("/", divide);
	env.add_proc("%", remainder);
	env.add_proc("is-num?", is_num);
}

fn add(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
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

fn subtract(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
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

fn multiply(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
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

fn divide(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
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
					if n == 0.0 {
						return Err("Divide by zero!".to_string());
					}

					ret /= n;
				}
			_ => fail_on_bad_type!("/", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn remainder(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
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
					if n == 0.0 {
						return Err("Divide/remainder by zero!".to_string());
					}

					ret %= n;
				}
			_ => fail_on_bad_type!("%", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn is_num(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-num?", 1, rands);

	Ok(Boolean(matches!(rands[0], Number(_))))
}
