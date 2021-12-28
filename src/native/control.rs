use std::fs;
use crate::{check_arity_between, check_arity_is, fail_on_bad_type, get_integer, get_string};
use crate::environment::Environment;
use crate::eval::eval_program;
use crate::parser::parse;
use crate::val::{Val, ValList, void};
use crate::val::Val::{List, Number, StringV, Symbol};

pub fn add_control_procs(env: &mut Environment) {
	env.add_proc("exit".to_string(), exit);
	env.add_proc("fail".to_string(), fail);
	env.add_proc("load".to_string(), load);
	env.add_proc("type".to_string(), _type);
}

fn exit(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_between!("exit", 0, 1, rands);

	match rands.len() {
		0 => std::process::exit(0),
		1 => std::process::exit(get_integer!("exit", rands, 0) as i32),
		_ => unreachable!()
	}
}

fn fail(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_between!("fail", 0, 1, rands);

	match rands.len() {
		0 => Err("'fail' called from within Psil!".to_string()),
		1 => Err(get_string!("fail", rands, 0).clone()),
		_ => unreachable!()
	}
}

fn load(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("load", 1, rands);

	let file = get_string!("load", rands, 0);

	match fs::read_to_string(file) {
		Ok(s) => {
			let parse_tree = parse(s).unwrap();
			match eval_program(parse_tree, env) {
				Ok(e) => Ok(List(e)),
				Err(e) => Err(format!("{}", e))
			}
		}
		Err(e) => Err(format!("{:?}", e))
	}
}

fn _type(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("type", 1, rands);

	Ok(Symbol(rands[0].get_type_name().to_string()))
}
