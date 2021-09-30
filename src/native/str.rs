use std::borrow::Borrow;
use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Number, String as StringVal};

pub fn add_str_procs(env: &mut Environment) {
	env.add_proc("str-cat".to_string(), str_cat);
	env.add_proc("str-contains?".to_string(), str_contains);
	env.add_proc("str-len".to_string(), str_len);
	env.add_proc("str-trunc".to_string(), str_trunc);
}

fn str_cat(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("str-cat", 2, rands);

	let mut ret = String::from("");

	for val in rands {
		ret.push_str(val.to_string().as_str())
	}

	Ok(StringVal(ret))
}

fn str_contains(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-contains?", 2, rands);

	match rands[0].borrow() {
		StringVal(s) =>
			match rands[1].borrow() {
				StringVal(sub) => Ok(Boolean(s.contains(sub))),
				_ => fail_on_bad_type!("str-contains?", "string", rands)
			}
		_ => fail_on_bad_type!("str-contains?", "string", rands)
	}
}

fn str_len(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-len", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(Number(s.len() as f64)),
		_ => fail_on_bad_type!("str-len", "string", rands)
	}
}

fn str_trunc(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-trunc", 2, rands);

	match rands[0].borrow() {
		StringVal(s) =>
			match rands[1].borrow() {
				Number(n) => {
					let mut ret = s.clone();
					ret.truncate(*n as usize);
					Ok(StringVal(ret))
				}
				_ => fail_on_bad_type!("str-trunc", "number", rands)
			}
		_ => fail_on_bad_type!("str-trunc", "string", rands)
	}
}
