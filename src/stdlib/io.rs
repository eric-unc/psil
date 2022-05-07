use std::io;

use crate::{check_arity_at_least, check_arity_is, load_into};
use crate::environment::Environment;
use crate::val::{Val, ValList, void};
use crate::val::Val::StringV;

pub fn add_native(env: &mut Environment) {
	env.add_proc("print", print);
	env.add_proc("put", put);
	env.add_proc("put-each", put_each);
	env.add_proc("input", input);
}

pub fn add_pure(env: &mut Environment) {
	load_into("src/stdlib/io.lisp", env).expect("Failure to load io.lisp!");
}

fn print(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("print", 1, rands);

	for arg in rands {
		print!("{}", arg);
	}

	Ok(void())
}

fn put(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	for arg in rands {
		print!("{}", arg);
	}

	println!();

	Ok(void())
}

fn put_each(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("put-each", 1, rands);

	for arg in rands {
		println!("{}", arg);
	}

	Ok(void())
}

fn input(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("input", 0, rands);

	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line = line.trim().to_string();

	Ok(StringV(line))
}
