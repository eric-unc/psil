use std::borrow::Borrow;
use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Number, String as StringVal};

pub fn add_str_procs(env: &mut Environment) {
	env.add_proc("str-cat".to_string(), str_cat);
	env.add_proc("str-len".to_string(), str_len);
}

fn str_cat(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("str-cat", 2, rands);

	let mut ret = String::from("");

	for val in rands {
		ret.push_str(val.to_string().as_str())
	}

	Ok(StringVal(ret))
}

fn str_len(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-len", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(Number(s.len() as f64)),
		_ => fail_on_bad_type!("str-len", "string", rands)
	}
}
