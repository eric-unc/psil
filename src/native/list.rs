use crate::{check_arity_at_least, check_arity_between, check_arity_is, fail_on_bad_type, get_list};
use crate::environment::Environment;
use crate::eval::eval_proc_with_rands;
use crate::val::{Val, ValList, void};
use crate::val::Val::{Boolean, Number, List, ProcedureV};

pub fn add_list_procs(env: &mut Environment) {
	env.add_proc("is-list?".to_string(), is_list);
	env.add_proc("list".to_string(), list);
	env.add_proc("list-append".to_string(), list_append);
	env.add_proc("list-each".to_string(), list_each);
	env.add_proc("list-empty?".to_string(), list_empty);
	env.add_proc("list-filter".to_string(), list_filter);
	env.add_proc("list-get".to_string(), list_get);
	env.add_proc("list-join".to_string(), list_join);
	env.add_proc("list-len".to_string(), list_len);
	env.add_proc("list-map".to_string(), list_map);
	env.add_proc("list-range".to_string(), list_range);
	env.add_proc("list-remove".to_string(), list_remove);
	env.add_proc("list-reverse".to_string(), list_reverse);
	env.add_proc("list-swap".to_string(), list_swap);
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

	Ok(match rands[0] {
		List(_) => Boolean(true),
		_ => Boolean(false)
	})
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

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-each", "list", rands)
	};

	let proc = match &rands[1] {
		ProcedureV(p) => p,
		_ => fail_on_bad_type!("list-map", "proc", rands)
	};

	for item in list {
		eval_proc_with_rands(proc.clone(), vec![item.clone()], "anonymous".to_string(), env)?;
	}

	Ok(void())
}

fn list_empty(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-empty?", 1, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-empty?", "list", rands)
	};

	Ok(Boolean(list.is_empty()))
}

fn list_filter(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-filter", 2, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-filter", "list", rands)
	};

	let proc = match &rands[1] {
		ProcedureV(p) => p,
		_ => fail_on_bad_type!("list-filter", "proc", rands)
	};

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

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-get", "list", rands)
	};

	let index = match &rands[1] {
		Number(n) => *n as usize,
		_ => fail_on_bad_type!("list-get", "number", rands)
	};

	check_bounds!(index, list);

	Ok(list.get(index).unwrap().clone())
}

fn list_join(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("list-join", 2, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-join", "list", rands)
	};

	let mut new_list = list.clone();

	for i in 1..rands.len() {
		let list = match &rands[i] {
			List(l) => l,
			_ => fail_on_bad_type!("list-join", "list", rands)
		};

		for i in 0..list.len() {
			new_list.push(list[i].clone());
		}
	}


	Ok(List(new_list))
}

fn list_len(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-len", 1, rands);

	match &rands[0] {
		List(l) => Ok(Number(l.len() as f64)),
		_ => fail_on_bad_type!("list_len", "number", rands)
	}
}

fn list_map(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-map", 2, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-map", "list", rands)
	};

	let proc = match &rands[1] {
		ProcedureV(p) => p,
		_ => fail_on_bad_type!("list-map", "proc", rands)
	};

	let mut new_list = vec![];

	for item in list {
		new_list.push(eval_proc_with_rands(proc.clone(), vec![item.clone()], "anonymous".to_string(), env)?);
	}

	Ok(List(new_list))
}

fn list_range(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_between!("list-range", 2, 3, rands);

	let n1 = match rands[0] {
		Number(n) => n,
		_ => fail_on_bad_type!("list-range", "number", rands)
	} as u64;

	let n2 = match rands[1] {
		Number(n) => n,
		_ => fail_on_bad_type!("list-range", "number", rands)
	} as u64;

	// TODO: some kind of integer checks

	if rands.len() == 2 {
		if n2 < n1 {
			return Err("First number is bigger than second!".to_string());
		}


		/*if n1 % 1 != 0 || n2 % 1 != 0 {
			return Err("Arg is not an integer!".to_string());
		}*/

		//let (n1, n2) = (n1 as u64, n2 as u64);

		let mut ret = ValList::with_capacity((n2 - n1) as usize);
		for i in n1..=n2 {
			ret.push(Number(i as f64));
		}

		Ok(List(ret))
	} else { // 3
		let step = match rands[2] {
			Number(n) => n,
			_ => fail_on_bad_type!("list-range", "number", rands)
		} as u64;

		let mut ret = ValList::with_capacity(((n2 - n1) / step) as usize);
		for i in (n1..=n2).step_by(step as usize) {
			ret.push(Number(i as f64));
		}

		Ok(List(ret))
	}
}

fn list_remove(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-remove", 2, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-remove", "list", rands)
	};

	let index = match &rands[1] {
		Number(n) => *n as usize,
		_ => fail_on_bad_type!("list-remove", "number", rands)
	};

	check_bounds!(index, list);

	let mut new_list = list.clone();
	new_list.remove(index);
	Ok(List(new_list))
}

fn list_reverse(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-reverse", 1, rands);

	match &rands[0] {
		List(l) => Ok({
			let mut new_list = l.clone();
			new_list.reverse();
			List(new_list)
		}),
		_ => fail_on_bad_type!("list-reverse", "list", rands)
	}
}

fn list_swap(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("list-swap", 3, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-swap", "list", rands)
	};

	let index1 = match &rands[1] {
		Number(n) => *n as usize,
		_ => fail_on_bad_type!("list-swap", "number", rands)
	};

	let index2 = match &rands[2] {
		Number(n) => *n as usize,
		_ => fail_on_bad_type!("list-swap", "number", rands)
	};

	check_bounds!(index1, list);
	check_bounds!(index2, list);

	let mut new_list = list.clone();
	new_list.swap(index1, index2);
	Ok(List(new_list))
}
