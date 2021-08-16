use crate::eval::{Environment, Val, ValList};

use Val::{Boolean, Error, Number, Procedure as ProcedureVal, String as StringVal, Void};

pub fn add_native_library(env: &mut Environment) {
	// Math
	env.add_proc("+".to_string(), add);
	env.add_proc("-".to_string(), subtract);
	env.add_proc("*".to_string(), multiply);
	env.add_proc("/".to_string(), divide);

	// System
	env.add_proc("exit".to_string(), exit);
	env.add_proc("print".to_string(), print);
	env.add_proc("put".to_string(), put);
}

// Macros
// TODO

////////// Native (Rust) methods

///// Math

fn add(args: ValList) -> Val {
	//expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;

	for val in args {
		match val {
			Number(n) => {
				ret += n;
			}
			_ => {
				return Error(String::from("Bad type"));
			}
		}
	}

	Val::Number(ret)
}

fn subtract(args: ValList) -> Val {
	//expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in args {
		match val {
			Val::Number(n) => {
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret -= n;
				}
			}
			_ => {
				return Error(String::from("Bad type"));
			}
		}
	}

	Number(ret)
}

fn multiply(args: ValList) -> Val {
	//expect_arity_at_least!(2, args.len());

	let mut ret = 1.0;

	for val in args {
		match val {
			Number(n) => {
				ret *= n;
			}
			_ => {
				return Error(String::from("Bad type"));
			}
		}
	}

	Val::Number(ret)
}

fn divide(args: ValList) -> Val {
	//expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in args {
		match val {
			Val::Number(n) => {
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret /= n;
				}
			}
			_ => {
				return Error(String::from("Bad type"));
			}
		}
	}

	Number(ret)
}

///// System

fn exit(args: ValList) -> Val {
	match args.len() {
		0 => std::process::exit(0),
		1 => match args[0] {
			Number(n) => std::process::exit(n as i32),
			_ => Error(format!("Bad type of {:?} for exit!", args[0])),
		},
		_ => Error("Bad arity for exit!".to_string()),
	}
}

fn print(args: ValList) -> Val {
	// expect_arity_at_least!(1, args.len());

	for arg in args {
		print!("{}", arg);
	}

	Void
}

fn put(args: ValList) -> Val {
	if args.len() == 0 {
		println!();
	} else {
		for arg in args {
			println!("{}", arg);
		}
	}

	Void
}
