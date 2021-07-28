use crate::eval::{Val, ValList, ProcedureType, Environment};

pub fn add_native_library(mut env: Environment) {
	// Math
	env.add_proc(String::from("+"), add);
	env.add_proc(String::from("-"), sub);

	// System
	env.add_proc(String::from("exit"), exit);
	env.add_proc(String::from("exit"), put);
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
			Val::Number(n) => { ret += n; }
			_ => { return Val::Error(String::from("Bad type")); }
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
			_ => { return Val::Error(String::from("Bad type")); }
		}
	}

	Val::Number(ret)
}

///// System

fn exit(args: ValList) -> Val {
	match args.len() {
		0 => std::process::exit(0),
		1 => {
			match args[0] {
				Val::Number(n) => std::process::exit(n as i32),
				_ => Val::Error(format!("Bad type of {:?} for exit!", args[0]))
			}
		}
		_ => Val::Void
	}
}

fn put(args: ValList) -> Val {
	// expect_arity_at_least!(1, args.len());

	for arg in args {
		match arg {
			Val::Number(n) => { println!("{}", n) }
			Val::Boolean(b) => { println!("{}", b) }
			Val::String(s) => { println!("{}", s) }
			Val::Void => { println!("void") }
			Val::Procedure(_) => { println!("TODO (lol)") } // TODO
			Val::Error(e) => { println!("{}", e) }
		}
	}

	Val::Void
}
