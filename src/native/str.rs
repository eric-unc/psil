use std::borrow::Borrow;
use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Number, String as StringVal};

pub fn add_str_procs(env: &mut Environment) {
	env.add_proc("2str".to_string(), to_str);
	env.add_proc("is-str?".to_string(), is_str);
	env.add_proc("str-cat".to_string(), str_cat);
	env.add_proc("str-contains?".to_string(), str_contains);
	env.add_proc("str-empty?".to_string(), str_empty);
	env.add_proc("str-insert".to_string(), str_insert);
	env.add_proc("str-len".to_string(), str_len);
	env.add_proc("str-low".to_string(), str_low);
	env.add_proc("str-repeat".to_string(), str_repeat);
	env.add_proc("str-replace".to_string(), str_replace);
	env.add_proc("str-starts-with?".to_string(), str_starts_with);
	env.add_proc("str-strip".to_string(), str_strip);
	env.add_proc("str-trunc".to_string(), str_trunc);
	env.add_proc("str-up".to_string(), str_up);
}

fn to_str(rands: ValList) -> Result<Val, String> {
	check_arity_is!("2str", 1, rands);

	Ok(StringVal(rands[0].to_string()))
}

fn is_str(rands: ValList) -> Result<Val, String> {
	check_arity_is!("is-str?", 1, rands);

	match rands[0] {
		StringVal(_) => Ok(Boolean(true)),
		_ => Ok(Boolean(false))
	}
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

fn str_empty(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-empty?", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(Boolean(s.is_empty())),
		_ => fail_on_bad_type!("str-empty?", "string", rands)
	}
}

fn str_insert(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-insert", 3, rands);

	match rands[0].borrow() {
		StringVal(s1) =>
			match rands[1] {
				Number(i) =>
					match rands[2].borrow() {
						StringVal(s2) => {
							let mut ret = s1.clone();
							ret.insert_str(i as usize, s2.clone().as_str());
							Ok(StringVal(ret))
						}
						_ => fail_on_bad_type!("str-insert", "string", rands)
					}
				_ => fail_on_bad_type!("str-insert", "number", rands)
			}
		_ => fail_on_bad_type!("str-insert", "string", rands)
	}
}

fn str_len(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-len", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(Number(s.len() as f64)),
		_ => fail_on_bad_type!("str-len", "string", rands)
	}
}

fn str_low(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-low", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(StringVal(s.to_lowercase())),
		_ => fail_on_bad_type!("str-low", "string", rands)
	}
}

fn str_repeat(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-repeat", 2, rands);

	match rands[0].borrow() {
		StringVal(s) =>
			match rands[1] {
				Number(n) =>
					if n < 0.0 || n % 1.0 != 0.0 {
						Err("str-repeat expects integer value!".to_string())
					} else {
						Ok(StringVal(s.repeat(n as usize)))
					}
				_ => fail_on_bad_type!("str-repeat", "number", rands)
			}
		_ => fail_on_bad_type!("str-repeat", "string", rands)
	}
}

fn str_replace(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-replace", 3, rands);

	match rands[0].borrow() {
		StringVal(s) =>
			match rands[1].borrow() {
				StringVal(old) =>
					match rands[2].borrow() {
						StringVal(new) => Ok(StringVal(s.replace(old, new))),
						_ => fail_on_bad_type!("str-replace", "string", rands)
					}
				_ => fail_on_bad_type!("str-replace", "string", rands)
			}
		_ => fail_on_bad_type!("str-replace", "string", rands)
	}
}

fn str_starts_with(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-starts-with?", 2, rands);

	match rands[0].borrow() {
		StringVal(s1) =>
			match rands[1].borrow() {
				StringVal(s2) => Ok(Boolean(s1.starts_with(s2))),
				_ => fail_on_bad_type!("str-starts-with?", "string", rands)
			}
		_ => fail_on_bad_type!("str-starts-with?", "string", rands)
	}
}

fn str_strip(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-strip", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(StringVal(s.trim().to_string())),
		_ => fail_on_bad_type!("str-strip", "string", rands)
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

fn str_up(rands: ValList) -> Result<Val, String> {
	check_arity_is!("str-up", 1, rands);

	match rands[0].borrow() {
		StringVal(s) => Ok(StringVal(s.to_uppercase())),
		_ => fail_on_bad_type!("str-up", "string", rands)
	}
}
