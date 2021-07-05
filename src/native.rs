pub fn add_native_library(env: Environment) {
	env.add_binding("+", Procedure::Native(&add));
	env.add_binding("-", Procedure::Native(&add));
}

pub fn add(args: ValueList) -> Value {
	expect_arity_at_least!(2, args.len());

	let mut ret = 0.0;

	for val in args {
		match val {
			Value::Number(n) => { ret += n; }
			_ => Value::Error("Bad type")
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
					ret = val;
					ret_init = true;
				} else {
					ret -= val;
				}
			}
			_ => Value::Error("Bad type")
		}
	}

	Value::Number(ret)
}
