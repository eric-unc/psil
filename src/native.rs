use crate::eval::{Environment, ProcedureType, Val, ValList};

pub fn add_native_library(env: &mut Environment) {
	// Math
	env.add_proc("+".to_string(), add);
	env.add_proc("-".to_string(), sub);

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
			Val::Number(n) => {
				ret += n;
			}
			_ => {
				return Val::Error(String::from("Bad type"));
			}
		}
	}

	Val::Number(ret)
}

fn sub(args: ValList) -> Val {
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
				return Val::Error(String::from("Bad type"));
			}
		}
	}

	Val::Number(ret)
}

///// System

fn exit(args: ValList) -> Val {
	match args.len() {
		0 => std::process::exit(0),
		1 => match args[0] {
			Val::Number(n) => std::process::exit(n as i32),
			_ => Val::Error(format!("Bad type of {:?} for exit!", args[0])),
		},
		_ => Val::Void,
	}
}

fn print(args: ValList) -> Val {
	// expect_arity_at_least!(1, args.len());

	for arg in args {
		match arg {
			Val::Number(n) => {
				print!("{}", n)
			}
			Val::Boolean(b) => {
				print!("{}", b)
			}
			Val::String(s) => {
				print!("{}", s)
			}
			Val::Void => {
				print!("void")
			}
			Val::Procedure(_) => {
				print!("<procedure>") // TODO: some day this will be much more advanced
			}
			Val::Error(e) => {
				print!("{}", e)
			}
		}
	}

	Val::Void
}

fn put(args: ValList) -> Val {
	print(args);
	println!();

	Val::Void
}
