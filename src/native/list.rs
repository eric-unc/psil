use crate::{check_arity_between, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Number, List};

pub fn add_list_procs(env: &mut Environment) {
	env.add_proc("list".to_string(), list);
	env.add_proc("range".to_string(), range);
}

fn list(rands: ValList) -> Result<Val, String> {
	Ok(List(rands))
}

fn range(rands: ValList) -> Result<Val, String> {
	check_arity_between!("range", 2, 3, rands);

	let n1 = match rands[0] {
		Number(n) => n,
		_ => fail_on_bad_type!("exit", "number", rands)
	};

	let n2 = match rands[1] {
		Number(n) => n,
		_ => fail_on_bad_type!("exit", "number", rands)
	};

	if rands.len() == 2 {
		if n2 < n1 {
			return Err("First number is bigger than second!".to_string());
		}

		/*if n1 % 1 != 0 || n2 % 1 != 0 {
			return Err("Arg is not an integer!".to_string());
		}*/

		let (n1, n2) = (n1 as u64, n2 as u64);

		let mut ret = ValList::with_capacity((n2 - n1) as usize);
		for i in n1..=n2 {
			ret.push(Number(i as f64));
		}

		return Ok(List(ret));
	} else { // 3
		let step = match rands[2] {
			Number(n) => n,
			_ => fail_on_bad_type!("exit", "number", rands)
		};
	}

	Err("Arg is not an integer!".to_string())
}
