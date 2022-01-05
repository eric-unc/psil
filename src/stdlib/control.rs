use std::collections::BTreeMap;
use crate::{check_arity_between, check_arity_is, fail_on_bad_type, get_integer, get_list, get_string, get_table, load_into};
use crate::doc::Entry;
use crate::environment::Environment;
use crate::val::{Val, ValList, void};
use crate::val::Val::{Number, StringV, Symbol, List, Table};

pub fn add_native(env: &mut Environment) {
	env.add_proc("doc", doc);
	env.add_proc("exit", exit);
	env.add_proc("fail", fail);
	env.add_proc("help", help);
	env.add_proc("load", load);
	env.add_proc("type", _type);
}

pub fn add_pure(env: &mut Environment) {
	load_into("src/stdlib/control.lisp", env).expect("Failure to load control.lisp!");
}

fn doc(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("doc", 4, rands);

	let proc = get_string!("doc", rands, 0);
	let aliases = get_list!("doc", rands, 1);
	let description = get_string!("doc", rands, 2);
	let params = get_table!("doc", rands, 3);

	let mut aliases2 = Vec::new();

	for alias in aliases {
		match alias {
			StringV(s) => aliases2.push(s.clone()),
			_ => fail_on_bad_type!("doc", "string", rands)
		}
	}

	let mut params2 = BTreeMap::new();

	for (k, v) in params {
		match v {
			StringV(s) => {params2.insert(k.clone(), s.clone());},
			_ => fail_on_bad_type!("doc", "string", rands)
		}
	}

	env.add_entry(proc.clone(), Entry::new(proc.clone(), aliases2, description.clone(), params2, env.get_curr_module()));

	Ok(void())
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

fn help(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("help", 1, rands);

	let proc = get_string!("help", rands, 0);

	match env.get_entry(proc) {
		None => eprintln!("Missing doc for {}!", proc),
		Some(entry) => println!("{}", entry)
	}

	Ok(void())
}

fn load(rands: ValList, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("load", 1, rands);

	let path = get_string!("load", rands, 0);

	load_into(path, env)?;
	Ok(void())
}

fn _type(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("type", 1, rands);

	Ok(Symbol(rands[0].get_type_name().to_string()))
}
