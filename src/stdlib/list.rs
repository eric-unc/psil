use crate::{check_arity_at_least, check_arity_between, check_arity_is, fail_on_bad_type, get_integer, get_list, get_natural_number, get_proc, load_into};
use crate::environment::Environment;
use crate::eval::eval_proc_with_rands;
use crate::val::{Val, ValList, void};
use crate::val::Val::{Boolean, Number, List, ProcedureV};

pub fn add_native(env: &mut Environment) {
	env.add_proc("is-list?", is_list);
	env.add_proc("list", list);
	env.add_proc("list-append", list_append);
	env.add_proc("list-each", list_each);
	env.add_proc("list-empty?", list_empty);
	env.add_proc("list-filter", list_filter);
	env.add_proc("list-get", list_get);
	env.add_proc("list-join", list_join);
	env.add_proc("list-len", list_len);
	env.add_proc("list-map", list_map);
	env.add_proc("list-range", list_range);
	env.add_proc("list-remove", list_remove);
	env.add_proc("list-reverse", list_reverse);
	env.add_proc("list-swap", list_swap);
}

pub fn add_pure(env: &mut Environment) {
	load_into("src/native/list.lisp", env).expect("Failure to load list.lisp!");
}

macro_rules! check_bounds {
	( $index:expr, $list:expr ) => {
		if $index > $list.len() - 1 {
			return Err(format!("Attempted list index {} on list of {} length!", $index, $list.len()));
		}
	}
}

fn is_list(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-list?", 1, rands);

	Ok(Boolean(matches!(rands[0], List(_))))
}

fn list(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	Ok(List(rands))
}

fn list_append(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("list-append", 2, rands);

	let list = get_list!("list-append", rands, 0);

	let mut new_list = list.clone();

	for i in 1..rands.len() {
		new_list.push(rands[i].clone());
	}


	Ok(List(new_list))
}

fn list_each(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-each", 2, rands);

	let list = get_list!("list-each", rands, 0);
	let proc = get_proc!("list-each", rands, 1);

	for item in list {
		eval_proc_with_rands(proc.clone(), vec![item.clone()], "anonymous".to_string(), env)?;
	}

	Ok(void())
}

fn list_empty(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-empty?", 1, rands);

	let list = get_list!("list-empty?", rands, 0);

	Ok(Boolean(list.is_empty()))
}

fn list_filter(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-filter", 2, rands);

	let list = get_list!("list-filter", rands, 0);
	let proc = get_proc!("list-filter", rands, 1);

	let mut new_list = vec![];

	for item in list {
		let bool = eval_proc_with_rands(proc.clone(), vec![item.clone()], "anonymous".to_string(), env)?;

		match bool {
			Boolean(b) => if b { new_list.push(item.clone()); }
			_ => return Err("Procedure in list_filter returned non-boolean!".to_string())
		}
	}

	Ok(List(new_list))
}

fn list_get(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-get", 2, rands);

	let list = get_list!("list-get", rands, 0);
	let index = get_natural_number!("list-get", rands, 1) as usize;

	check_bounds!(index, list);

	Ok(list.get(index).unwrap().clone())
}

fn list_join(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("list-join", 2, rands);

	let list = get_list!("list-join", rands, 0);

	let mut new_list = list.clone();

	for i in 1..rands.len() {
		let list = get_list!("list-join", rands, i);

		for i in 0..list.len() {
			new_list.push(list[i].clone());
		}
	}


	Ok(List(new_list))
}

fn list_len(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-len", 1, rands);

	let list = get_list!("list-len", rands, 0);
	Ok(Number(list.len() as f64))
}

fn list_map(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-map", 2, rands);

	let list = get_list!("list-map", rands, 0);
	let proc = get_proc!("list-map", rands, 1);

	let mut new_list = vec![];

	for item in list {
		new_list.push(eval_proc_with_rands(proc.clone(), vec![item.clone()], "anonymous".to_string(), env)?);
	}

	Ok(List(new_list))
}

fn list_range(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_between!("list-range", 2, 3, rands);

	let n1 = get_integer!("list-range", rands, 0);
	let n2 = get_integer!("list-range", rands, 1);

	if rands.len() == 2 {
		if n2 < n1 {
			return Err("First number is bigger than second!".to_string());
		}

		let mut ret = ValList::with_capacity((n2 - n1) as usize);
		for i in n1..=n2 {
			ret.push(Number(i as f64));
		}

		Ok(List(ret))
	} else { // 3
		let step = get_integer!("list-range", rands, 2);

		let mut ret = ValList::with_capacity(((n2 - n1) / step) as usize);
		for i in (n1..=n2).step_by(step as usize) {
			ret.push(Number(i as f64));
		}

		Ok(List(ret))
	}
}

fn list_remove(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-remove", 2, rands);

	let list = get_list!("list-remove", rands, 0);
	let index = get_natural_number!("list-remove", rands, 1) as usize;

	check_bounds!(index, list);

	let mut new_list = list.clone();
	new_list.remove(index);
	Ok(List(new_list))
}

fn list_reverse(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-reverse", 1, rands);

	let list = get_list!("list-reverse", rands, 0);

	let mut new_list = list.clone();
	new_list.reverse();
	Ok(List(new_list))
}

fn list_swap(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-swap", 3, rands);

	let list = get_list!("list-swap", rands, 0);
	let index1 = get_natural_number!("list-swap", rands, 1) as usize;
	let index2 = get_natural_number!("list-swap", rands, 2) as usize;

	check_bounds!(index1, list);
	check_bounds!(index2, list);

	let mut new_list = list.clone();
	new_list.swap(index1, index2);
	Ok(List(new_list))
}
