use crate::{check_arity_at_least, check_arity_is, fail_on_bad_type, get_natural_number, get_string};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, Number, StringV};

pub fn add_native(env: &mut Environment) {
	env.add_proc("2str", to_str);
	env.add_proc("is-str?", is_str);
	env.add_proc("str-cat", str_cat);
	env.add_proc("str-contains?", str_contains);
	env.add_proc("str-empty?", str_empty);
	env.add_proc("str-insert", str_insert);
	env.add_proc("str-len", str_len);
	env.add_proc("str-low", str_low);
	env.add_proc("str-repeat", str_repeat);
	env.add_proc("str-replace", str_replace);
	env.add_proc("str-starts-with?", str_starts_with);
	env.add_proc("str-strip", str_strip);
	env.add_proc("str-trunc", str_trunc);
	env.add_proc("str-up", str_up);
}

fn to_str(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("2str", 1, rands);

	Ok(StringV(rands[0].to_string()))
}

fn is_str(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-str?", 1, rands);
	
	Ok(Boolean(matches!(rands[0], StringV(_))))
}

fn str_cat(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("str-cat", 2, rands);

	let mut ret = String::from("");

	for val in rands {
		ret.push_str(val.to_string().as_str())
	}

	Ok(StringV(ret))
}

fn str_contains(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-contains?", 2, rands);

	let str = get_string!("str-contains?", rands, 0);
	let possible_sub = get_string!("str-contains?", rands, 1);

	Ok(Boolean(str.contains(possible_sub)))
}

fn str_empty(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-empty?", 1, rands);

	let s = get_string!("str-empty?", rands, 0);
	Ok(Boolean(s.is_empty()))
}

fn str_insert(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-insert", 3, rands);

	let s1 = get_string!("str-insert", rands, 0);
	let index = get_natural_number!("str-insert", rands, 1) as usize;
	let s2 = get_string!("str-insert", rands, 2);

	let mut ret = s1.clone();
	ret.insert_str(index, s2.clone().as_str());
	Ok(StringV(ret))
}

fn str_len(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-len", 1, rands);

	let s = get_string!("str-len", rands, 0);
	Ok(Number(s.len() as f64))
}

fn str_low(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-low", 1, rands);

	let s = get_string!("str-len", rands, 0);
	Ok(StringV(s.to_lowercase()))
}

fn str_repeat(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-repeat", 2, rands);

	let s = get_string!("str-repeat", rands, 0);
	let n = get_natural_number!("str-repeat", rands, 1);

	Ok(StringV(s.repeat(n as usize)))
}

fn str_replace(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-replace", 3, rands);

	let s = get_string!("str-replace", rands, 0);
	let old = get_string!("str-replace", rands, 1);
	let new = get_string!("str-replace", rands, 2);

	Ok(StringV(s.replace(old, new)))
}

fn str_starts_with(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-starts-with?", 2, rands);

	let s1 = get_string!("str-starts-with?", rands, 0);
	let s2 = get_string!("str-starts-with?", rands, 1);

	Ok(Boolean(s1.starts_with(s2)))
}

fn str_strip(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-strip", 1, rands);

	Ok(StringV(get_string!("str-strip", rands, 0).trim().to_string()))
}

fn str_trunc(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-trunc", 2, rands);

	let s = get_string!("str-trunc", rands, 0);
	let n = get_natural_number!("str-trunc", rands, 1);

	let mut ret = s.clone();
	ret.truncate(n as usize);
	Ok(StringV(ret))
}

fn str_up(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str-up", 1, rands);

	Ok(StringV(get_string!("str-trunc", rands, 0).to_uppercase()))
}
