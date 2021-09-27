use std::io;

use crate::eval::{Environment, Val, ValList};

use Val::{Boolean, Number, Procedure, String as StringVal, Void};

////////// Actually extends environment with natively-defined functions
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

////////// Macros/constants
macro_rules! check_arity_at_least {
	( $proc_name:literal, $arity:literal, $rands:expr ) => {
		if $rands.len() < $arity {
			if $arity == 1 {
				return Err(format!("Native proc '{}' expected at least {} rand! Given {}.", $proc_name, $arity, $rands.len()))
			} else {
				return Err(format!("Native proc '{}' expected at least {} rands! Given {}.", $proc_name, $arity, $rands.len()))
			}
		}
	}
}

macro_rules! check_arity_is {
	( $proc_name:literal, $arity:literal, $rands:expr ) => {
		if $rands.len() != $arity {
			return match $arity {
				0 => Err(format!("Native proc '{}' expected no rands! Given {}.", $proc_name, $rands.len())),
				1 => Err(format!("Native proc '{}' expected 1 rand! Given {}.", $proc_name, $rands.len())),
				_ => Err(format!("Native proc '{}' expected {} rands! Given {}.", $proc_name, $arity, $rands.len()))
			}
		}
	}
}

/// inclusively between
macro_rules! check_arity_between {
	( $proc_name:literal, $low_arity:literal, $high_arity:literal, $rands:expr ) => {
		if $rands.len() < $low_arity || $rands.len() > $high_arity {
			return Err(format!("Native proc '{}' expected {} to {} rands! Given {}.", $proc_name, $low_arity, $high_arity, $rands.len()))
		}
	}
}

// used in eval for and, or, etc
#[macro_export]
macro_rules! fail_on_bad_type {
	( $proc_name:literal, $expected_type:literal, $rands:expr ) => {
		return Err(format!("Native proc '{}' expected a {}!", $proc_name, $expected_type))
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
			_ => fail_on_bad_type!("+", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn subtract(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("-", 2, rands);

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
			_ => fail_on_bad_type!("-", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn multiply(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("*", 2, rands);

	let mut ret = 1.0;

	for val in rands {
		match val {
			Number(n) => ret *= n,
			_ => fail_on_bad_type!("*", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn divide(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("/", 2, rands);

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
			_ => fail_on_bad_type!("/", "number", rands)
		}
	}

	Ok(Number(ret))
}

fn remainder(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("%", 2, rands);

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
			_ => fail_on_bad_type!("%", "number", rands)
		}
	}

	Ok(Number(ret))
}

///// Boolean
fn not(rands: ValList) -> Result<Val, String> {
	check_arity_is!("not", 1, rands);

	match rands[0] {
		Boolean(b) => Ok(Boolean(!b)),
		_ => fail_on_bad_type!("not", "boolean", rands)
	}
}

fn xor(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("xor", 1, rands);

	// Wikipedia: "[xor] may be considered to be an n-ary operator which is true if and only if an odd number of arguments are true"
	let mut trues: usize = 0;

	for rand in rands {
		match rand {
			Boolean(true) => trues += 1,
			Boolean(false) => continue,
			_ => fail_on_bad_type!("xor", "boolean", rands)
		}
	}


	Ok(Boolean(trues % 2 == 1))
}

fn equal(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("==", 2, rands);

	for i in 1..rands.len() {
		if rands[0].ne(&rands[i]) {
			return Ok(Boolean(false))
		}
	}

	Ok(Boolean(true))
}

fn no_eq(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("!=", 2, rands);

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
	check_arity_at_least!(">", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n >= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!(">", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn gte(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!(">=", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n > f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!(">=", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn lt(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("<", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n <= f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!("<", "number", rands)
		}
	}

	Ok(Boolean(true))
}

fn lte(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("<=", 2, rands);

	let mut first = None;

	for rand in rands {
		match (rand, first) {
			(Number(n), Some(f)) =>
				if n < f {
					return Ok(Boolean(false))
				}
			(Number(n), None) => first = Some(n),
			_ => fail_on_bad_type!("<=", "number", rands)
		}
	}

	Ok(Boolean(true))
}

///// System

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

fn print(rands: ValList) -> Result<Val, String> {
	check_arity_at_least!("print", 1, rands);

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

fn input(rands: ValList) -> Result<Val, String> {
	check_arity_is!("input", 0, rands);

	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line = line.trim().to_string();

	Ok(StringVal(line))
}
