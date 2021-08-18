use std::io;

use crate::eval::{Environment, Val, ValList};

use Val::{Boolean, Error, Number, Procedure, String as StringVal, Void};

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

// Macros
// TODO

////////// Native (Rust) methods

///// Math

fn add(rands: ValList) -> Val {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 0.0;

	for val in rands {
		match val {
			Number(n) => ret += n,
			_ => return Error(String::from("Bad type"))
		}
	}

	Number(ret)
}

fn subtract(rands: ValList) -> Val {
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
			_ => return Error("Bad type".to_string())
		}
	}

	Number(ret)
}

fn multiply(rands: ValList) -> Val {
	//expect_arity_at_least!(2, rands.len());

	let mut ret = 1.0;

	for val in rands {
		match val {
			Number(n) => ret *= n,
			_ => return Error(String::from("Bad type"))
		}
	}

	Val::Number(ret)
}

fn divide(rands: ValList) -> Val {
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
					ret /= n;
				}
			_ => return Error(String::from("Bad type"))
		}
	}

	Number(ret)
}

fn remainder(rands: ValList) -> Val {
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
					ret %= n;
				}
			_ => return Error(String::from("Bad type"))
		}
	}

	Number(ret)
}

///// Boolean
fn not(rands: ValList) -> Val {
	// TODO: error checkings

	match rands[0] {
		Boolean(b) => Boolean(!b),
		_ => Error("Bad type for not!".to_string())
	}
}

fn xor(rands: ValList) -> Val {
	// Wikipedia: "[xor] may be considered to be an n-ary operator which is true if and only if an odd number of arguments are true"
	let mut trues: usize = 0;

	for rand in rands {
		match rand {
			Boolean(b) =>
				if b {
					trues += 1
				}
			_ => return Error("Bad type for xor!".to_string())
		}
	}


	Boolean(trues % 2 == 1)
}

fn equal(rands: ValList) -> Val {
	for i in 1..rands.len() {
		if rands[0].ne(&rands[i]) {
			return Boolean(false)
		}
	}

	Boolean(true)
}

fn no_eq(rands: ValList) -> Val {
	for i in 0..rands.len() {
		for j in i + 1..rands.len() {
			if rands[i].eq(&rands[j]) {
				return Boolean(false)
			}
		}
	}

	Boolean(true)
}

fn gt(rands: ValList) -> Val {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match rand {
			Number(n) => {
				match first {
					Some(f) =>
						if n >= f {
							return Boolean(false)
						}
					None => first = Some(n)
				}
			}
			Error(e) => return Error(e),
			_ => return Error("Bad type for >!".to_string())
		}
	}

	Boolean(true)
}

fn gte(rands: ValList) -> Val {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match rand {
			Number(n) => {
				match first {
					Some(f) =>
						if n > f {
							return Boolean(false)
						}
					None => first = Some(n)
				}
			}
			Error(e) => return Error(e),
			_ => return Error("Bad type for >=!".to_string())
		}
	}

	Boolean(true)
}

fn lt(rands: ValList) -> Val {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match rand {
			Number(n) => {
				match first {
					Some(f) =>
						if n <= f {
							return Boolean(false)
						}
					None => first = Some(n)
				}
			}
			Error(e) => return Error(e),
			_ => return Error("Bad type for <!".to_string())
		}
	}

	Boolean(true)
}

fn lte(rands: ValList) -> Val {
	// TODO: check rands
	let mut first = None;

	for rand in rands {
		match rand {
			Number(n) => {
				match first {
					Some(f) =>
						if n < f {
							return Boolean(false)
						}
					None => first = Some(n)
				}
			}
			Error(e) => return Error(e),
			_ => return Error("Bad type for <=!".to_string())
		}
	}

	Boolean(true)
}

///// System

fn exit(rands: ValList) -> Val {
	match rands.len() {
		0 => std::process::exit(0),
		1 => match rands[0] {
			Number(n) => std::process::exit(n as i32),
			_ => Error(format!("Bad type of {:?} for exit!", rands[0])),
		},
		_ => Error("Bad arity for exit!".to_string()),
	}
}

fn print(rands: ValList) -> Val {
	// expect_arity_at_least!(1, rands.len());

	for arg in rands {
		print!("{}", arg);
	}

	Void
}

fn put(rands: ValList) -> Val {
	if rands.len() == 0 {
		println!();
	} else {
		for arg in rands {
			println!("{}", arg);
		}
	}

	Void
}

fn input(_rands: ValList) -> Val {
	// TODO: check for no rands
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	line = line.trim().to_string();

	StringVal(line)
}
