use crate::{check_arity_is, fail_on_bad_type, get_string, get_symbol};
use crate::environment::Environment;
use crate::val::{Val, ValList};
use crate::val::Val::{Boolean, StringV, Symbol};

pub fn add_symbol_procs(env: &mut Environment) {
	env.add_proc("str2symb".to_string(), str_to_symb);
	env.add_proc("symb2str".to_string(), symb_to_str);
	env.add_proc("is-symb?".to_string(), is_symb);
	env.add_proc("is-void?".to_string(), is_void);
}

fn str_to_symb(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("str2symb", 1, rands);

	Ok(Symbol(get_string!("str2symb", rands, 0).to_string()))
}

fn symb_to_str(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("symb2str", 1, rands);

	Ok(StringV(get_symbol!("symb2str", rands, 0).to_string()))
}

fn is_symb(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-symb?", 1, rands);

	Ok(Boolean(matches!(rands[0], Symbol(_))))
}

fn is_void(rands: ValList, _env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("is-void?", 1, rands);

	match &rands[0] {
		Symbol(s) => Ok(Boolean(s.eq("void"))),
		_ => Ok(Boolean(false))
	}
}
