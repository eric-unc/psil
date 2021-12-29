use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type, get_boolean};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Number};

pub fn add_native(env: &mut Environment) {
	env.add_proc("not", not);
	env.add_proc("xor", xor);
	env.add_proc("==", equal);
	env.add_proc("!=", no_eq);
	env.add_proc(">", gt);
	env.add_proc(">=", gte);
	env.add_proc("<", lt);
	env.add_proc("<=", lte);
	env.add_proc("is-bool?", is_bool);
}

fn not(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("not", 1, rands);

	let bool = get_boolean!("not", rands, 0);
	Ok(Boolean(!bool))
}

fn xor(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("xor", 2, rands);

	// Wikipedia: "[xor] may be considered to be an n-ary operator which is true if and only if an odd number of arguments are true"
	let mut trues: usize = 0;

	for rand in rands {
		match rand {
			Boolean(true) => trues += 1,
			Boolean(false) => continue,
			_ => fail_on_bad_type!("xor", "boolean", rands)
		}
	}


	Ok(Boolean(trues % 2 == 1))
}

fn equal(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("==", 2, rands);

	for i in 1..rands.len() {
		if rands[0].ne(&rands[i]) {
			return Ok(Boolean(false))
		}
	}

	Ok(Boolean(true))
}

fn no_eq(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("!=", 2, rands);

	for i in 0..rands.len() {
		for j in i + 1..rands.len() {
			if rands[i].eq(&rands[j]) {
				return Ok(Boolean(false))
			}
		}
	}

	Ok(Boolean(true))
}

fn gt(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!(">", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n >= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!(">", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn gte(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!(">=", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n > f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!(">=", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn lt(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("<", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n <= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!("<", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn lte(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("<=", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n < f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!("<=", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn is_bool(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-bool?", 1, rands);

	Ok(Boolean(matches!(rands[0], Boolean(_))))
}
