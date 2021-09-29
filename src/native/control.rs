use crate::{check_arity_between, fail_on_bad_type};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Number, String as StringVal};

pub fn add_control_procs(env: &mut Environment) {
	env.add_proc("exit".to_string(), exit);
	env.add_proc("fail".to_string(), fail);
}

fn exit(rands: ValList) -> Result<Val, String> {
	check_arity_between!("exit", 0, 1, rands);

	match rands.len() {
		0 => std::process::exit(0),
		1 => match rands[0] {
			Number(n) => std::process::exit(n as i32),
			_ => fail_on_bad_type!("exit", "number", rands)
		}
		_ => unreachable!()
	}
}

fn fail(rands: ValList) -> Result<Val, String> {
	check_arity_between!("fail", 0, 1, rands);

	match rands.len() {
		0 => Err("'fail' called from within Psil!".to_string()),
		1 => match &rands[0] {
			StringVal(s) => Err(s.clone()),
			_ => fail_on_bad_type!("fail", "string", rands)
		}
		_ => unreachable!()
	}
}
