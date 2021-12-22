use crate::{check_arity_between, check_arity_is, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Number, List};

pub fn add_list_procs(env: &mut Environment) {
	env.add_proc("list".to_string(), list);
	env.add_proc("list-get".to_string(), list_get);
	env.add_proc("list-len".to_string(), list_len);
	env.add_proc("list-range".to_string(), list_range);
	env.add_proc("list-reverse".to_string(), list_reverse);
}

fn list(rands: ValList) -> Result<Val, String> {
	Ok(List(rands))
}

fn list_get(rands: ValList) -> Result<Val, String> {
	check_arity_is!("list-get", 2, rands);

	let list = match &rands[0] {
		List(l) => l,
		_ => fail_on_bad_type!("list-get", "number", rands)
	};

	let index = match &rands[1] {
		Number(n) => *n as usize,
		_ => fail_on_bad_type!("list-get", "number", rands)
	};

	match list.get(index) {
		Some(v) => Ok(v.clone()),
		None => Err(format!("Attempted to access list index {} which does not exist!", index))
	}
}

fn list_len(rands: ValList) -> Result<Val, String> {
	check_arity_is!("list-len", 1, rands);

	match &rands[0] {
		List(l) => Ok(Number(l.len() as f64)),
		_ => fail_on_bad_type!("list_len", "number", rands)
	}
}

fn list_range(rands: ValList) -> Result<Val, String> {
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

fn list_reverse(rands: ValList) -> Result<Val, String> {
	check_arity_is!("list-reverse", 1, rands);

	match &rands[0] {
		List(l) => Ok({
			let mut new_list = l.clone();
			new_list.reverse();
			List(new_list)
		}),
		_ => fail_on_bad_type!("list-reverse", "number", rands)
	}
}
