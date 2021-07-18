use crate::eval::{Value, ValueList, Procedure, Environment};

pub fn add_native_library(mut env: Environment) {
	env.add_binding("+".parse().unwrap(), Value(Procedure::Native(add)));
	env.add_binding("-".parse().unwrap(), Value(Procedure::Native(sub)));
}

pub fn add(args: ValueList) -> Value {
	expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;

	for val in args {
		match val {
			Value::Number(n) => { ret += n; }
			_ => Value::Error("Bad type".parse().unwrap())
		}
	}

	Value::Number(ret)
}

pub fn sub(args: ValueList) -> Value {
	expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;
	let mut ret_init = false;

	for val in args {
		match val {
			Value::Number(n) => {
				if !ret_init {
					ret = n;
					ret_init = true;
				} else {
					ret -= n;
				}
			}
			_ => Value::Error("Bad type".parse().unwrap())
		}
	}

	Value::Number(ret)
}
