use std::io;

use crate::eval::{Environment, Val, ValList};

use Val::{Boolean, Number, Procedure, String as StringVal, Void};

pub fn add_native_library(env: &mut Environment) {
	// Math
	env.add_proc("+".to_string(), add);
	env.add_proc("-".to_string(), subtract);
	env.add_proc("*".to_string(), multiply);
	env.add_proc("/".to_string(), divide);
	env.add_proc("%".to_string(), remainder);

	// Boolean
	env.add_proc("not".to_string(), not);
	env.add_proc("xor".to_string(), xor);
	env.add_proc("==".to_string(), equal);
	env.add_proc("!=".to_string(), no_eq);
	env.add_proc(">".to_string(), gt);
	env.add_proc(">=".to_string(), gte);
	env.add_proc("<".to_string(), lt);
	env.add_proc("<=".to_string(), lte);

	// System
	env.add_proc("exit".to_string(), exit);
	env.add_proc("print".to_string(), print);
	env.add_proc("put".to_string(), put);
	env.add_proc("input".to_string(), input);
}

// Macros/constants
macro_rules! check_arity_at_least {
	( $proc_name:literal, $arity:literal, $rands:expr ) => {
		if $rands.len() < $arity {
			return Err(format!("Native proc {} expected at least {} rands!", $proc_name, $arity))
		}
	}
}

////////// Native (Rust) methods

///// Math

fn add(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("+", 2, rands);

	let mut ret = 0.0;

	for val in rands {
		match val {
			Number(n) => ret += n,
			_ => return Err("Bad type".to_string())
		}
	}

	Ok(Number(ret))
}

fn subtract(rands: ValList) -> Result<Val, String> {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret -= n
				}
			_ => return Err("Bad type".to_string())
		}
	}

	Ok(Number(ret))
}

fn multiply(rands: ValList) -> Result<Val, String> {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 1.0;

	for val in rands {
		match val {
			Number(n) => ret *= n,
			_ => return Err("Bad type".to_string())
		}
	}

	Ok(Number(ret))
}

fn divide(rands: ValList) -> Result<Val, String> {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret /= n
				}
			_ => return Err("Bad type".to_string())
		}
	}

	Ok(Number(ret))
}

fn remainder(rands: ValList) -> Result<Val, String> {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in rands {
		match val {
			Number(n) =>
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret %= n
				}
			_ => return Err("Bad type".to_string())
		}
	}

	Ok(Number(ret))
}

///// Boolean
fn not(rands: ValList) -> Result<Val, String> {
	// TODO: error checkings

	match rands[0] {
		Boolean(b) => Ok(Boolean(!b)),
		_ => Err("Bad type for not!".to_string())
	}
}

fn xor(rands: ValList) -> Result<Val, String> {
	// Wikipedia: "[xor] may be considered to be an n-ary operator which is true if and only if an odd number of arguments are true"
	let mut trues: usize = 0;

	for rand in rands {
		match rand {
			Boolean(true) => trues += 1,
			Boolean(false) => continue,
			_ => return Err("Bad type for xor!".to_string())
		}
	}


	Ok(Boolean(trues % 2 == 1))
}

fn equal(rands: ValList) -> Result<Val, String> {
	for i in 1..rands.len() {
		if rands[0].ne(&rands[i]) {
			return Ok(Boolean(false))
		}
	}

	Ok(Boolean(true))
}

fn no_eq(rands: ValList) -> Result<Val, String> {
	for i in 0..rands.len() {
		for j in i + 1..rands.len() {
			if rands[i].eq(&rands[j]) {
				return Ok(Boolean(false))
			}
		}
	}

	Ok(Boolean(true))
}

fn gt(rands: ValList) -> Result<Val, String> {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n >= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => return Err("Bad type for >!".to_string())
		}
	}

	Ok(Boolean(true))
}

fn gte(rands: ValList) -> Result<Val, String> {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n > f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => return Err("Bad type for >=!".to_string())
		}
	}

	Ok(Boolean(true))
}

fn lt(rands: ValList) -> Result<Val, String> {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n <= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => return Err("Bad type for <!".to_string())
		}
	}

	Ok(Boolean(true))
}

fn lte(rands: ValList) -> Result<Val, String> {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n < f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => return Err("Bad type for <=!".to_string())
		}
	}

	Ok(Boolean(true))
}

///// System

fn exit(rands: ValList) -> Result<Val, String> {
	match rands.len() {
		0 => std::process::exit(0),
		1 => match rands[0] {
			Number(n) => std::process::exit(n as i32),
			_ => Err(format!("Bad type of {:?} for exit!", rands[0])),
		}
		_ => Err("Bad arity for exit!".to_string())
	}
}

fn print(rands: ValList) -> Result<Val, String> {
	// expect_arity_at_least!(1, rands.len());

	for arg in rands {
		print!("{}", arg);
	}

	Ok(Void)
}

fn put(rands: ValList) -> Result<Val, String> {
	if rands.len() == 0 {
		println!()
	} else {
		for arg in rands {
			println!("{}", arg);
		}
	}

	Ok(Void)
}

fn input(_rands: ValList) -> Result<Val, String> {
	// TODO: check for no rands
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line = line.trim().to_string();

	Ok(StringVal(line))
}
