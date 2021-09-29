use crate::eval::{Environment, Val, ValList};
use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type};

use Val::{Boolean, Number};

pub fn add_boolean_procs(env: &mut Environment) {
	env.add_proc("not".to_string(), not);
	env.add_proc("xor".to_string(), xor);
	env.add_proc("==".to_string(), equal);
	env.add_proc("!=".to_string(), no_eq);
	env.add_proc(">".to_string(), gt);
	env.add_proc(">=".to_string(), gte);
	env.add_proc("<".to_string(), lt);
	env.add_proc("<=".to_string(), lte);
}

fn not(rands: ValList) -> Result<Val, String> {
	check_arity_is!("not", 1, rands);

	match rands[0] {
		Boolean(b) => Ok(Boolean(!b)),
		_ => fail_on_bad_type!("not", "boolean", rands)
	}
}

fn xor(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("xor", 1, rands);

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

fn equal(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("==", 2, rands);

	for i in 1..rands.len() {
		if rands[0].ne(&rands[i]) {
			return Ok(Boolean(false))
		}
	}

	Ok(Boolean(true))
}

fn no_eq(rands: ValList) -> Result<Val, String> {
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

fn gt(rands: ValList) -> Result<Val, String> {
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

fn gte(rands: ValList) -> Result<Val, String> {
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

fn lt(rands: ValList) -> Result<Val, String> {
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

fn lte(rands: ValList) -> Result<Val, String> {
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
