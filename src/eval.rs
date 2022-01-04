use std::string::String;

use crate::ast::*;
use crate::{check_arity_at_least, check_arity_is};
use crate::environment::Environment;
use crate::val::{Procedure, Val, ValList, void};
use crate::val::Val::{Boolean, Number, ProcedureV, StringV, Symbol};

pub fn eval(program: ProgramAst) -> Result<Vec<Val>, String> {
	eval_program(program, &mut Environment::new())
}

pub fn eval_program(program: ProgramAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	eval_expr_list(program.expr_list, env)
}

fn eval_expr_list(expr_list: ExprListAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	let mut ret = Vec::new();

	for expr in expr_list.into_iter() {
		let val = eval_expr(expr, env)?;
		ret.push(val);
	}

	Ok(ret)
}

pub fn eval_expr(expr: ExprAst, env: &mut Environment) -> Result<Val, String> {
	match expr {
		ExprAst::Atom(a) => eval_atom(*a, env),
		ExprAst::Invocation(i) => eval_invocation(i, env)
	}
}

// atom ::= number | boolean | string | symbol | lambda | name
fn eval_atom(atom: AtomAst, env: &mut Environment) -> Result<Val, String> {
	match atom {
		AtomAst::Number(n) => Ok(Number(n)),
		AtomAst::Boolean(b) => Ok(Boolean(b)),
		AtomAst::String(s) => Ok(StringV(s)),
		AtomAst::Symbol(s) => Ok(Symbol(s)),
		AtomAst::Lambda(l) => Ok(ProcedureV(Procedure::Pure(l))),
		AtomAst::Identifier(n) => env.get_binding(n),
		AtomAst::SpecialForm(s) => Ok(ProcedureV(Procedure::SpecialForm(s)))
	}
}

fn eval_invocation(invocation: InvocationAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: I don't think all of this is necessary, exactly...
	// There's definitely a way to simplify the AST, but that's a problem for later.
	match invocation.proc {
		PossibleProc::Name(name) => {
			let proc_fetch = env.get_binding(name.clone())?;

			match proc_fetch {
				ProcedureV(p) => eval_proc(p, invocation.expr_list, name, env),
				_ => Err(format!("Binding {} is not a procedure!", name))
			}
		}
		PossibleProc::SpecialForm(s) => {
			match s {
				SpecialForms::If => eval_if(invocation.expr_list, env),
				SpecialForms::Cond => eval_cond(invocation.expr_list, env),
				SpecialForms::Define => eval_define(invocation.expr_list, env),
				SpecialForms::Do => eval_do(invocation.expr_list, env),
				SpecialForms::And => eval_and(invocation.expr_list, env),
				SpecialForms::Or => eval_or(invocation.expr_list, env)
			}
		}
	}
}

pub fn eval_proc(p: Procedure, expr_list: ExprListAst, name: Name, env: &mut Environment) -> Result<Val, String> {
	match p {
		Procedure::Native(n) => {
			let mut rands = Vec::new();

			for expr in expr_list.into_iter() {
				let val = eval_expr(expr, env)?;
				rands.push(val);
			}

			n(rands, env)
		},
		Procedure::Pure(p) => {
			let mut rands = Vec::new();

			for expr in expr_list.into_iter() {
				let val = eval_expr(expr, env)?;
				rands.push(val);
			}

			if p.params.names.len() != rands.len() {
				return match p.params.names.len() {
					0 => Err(format!("Proc '{}' expected no rands! Given {}.", name, rands.len())),
					1 => Err(format!("Proc '{}' expected 1 rand! Given {}.", name, rands.len())),
					_ => Err(format!("Proc '{}' expected {} rands! Given {}.", name, p.params.names.len(), rands.len()))
				}
			}

			env.add_scope();

			for (name, rand_val) in p.params.names.into_iter().zip(rands) {
				env.add_binding(name, rand_val);
			}

			let ret = eval_expr(p.expr, env);

			env.close_scope();

			ret
		},
		Procedure::SpecialForm(s) => {
			match s {
				SpecialForms::If => eval_if(expr_list, env),
				SpecialForms::Cond => eval_cond(expr_list, env),
				SpecialForms::Define => eval_define(expr_list, env),
				SpecialForms::Do => eval_do(expr_list, env),
				SpecialForms::And => eval_and(expr_list, env),
				SpecialForms::Or => eval_or(expr_list, env)
			}
		}
	}
}

pub fn eval_proc_with_rands(p: Procedure, rands: ValList, name: Name, env: &mut Environment) -> Result<Val, String> {
	match p {
		Procedure::Native(n) => n(rands, env),
		Procedure::Pure(p) => {
			if p.params.names.len() != rands.len() {
				return match p.params.names.len() {
					0 => Err(format!("Proc '{}' expected no rands! Given {}.", name, rands.len())),
					1 => Err(format!("Proc '{}' expected 1 rand! Given {}.", name, rands.len())),
					_ => Err(format!("Proc '{}' expected {} rands! Given {}.", name, p.params.names.len(), rands.len()))
				}
			}

			env.add_scope();

			for (name, rand_val) in p.params.names.into_iter().zip(rands) {
				env.add_binding(name, rand_val);
			}

			let ret = eval_expr(p.expr, env);

			env.close_scope();

			ret
		},
		Procedure::SpecialForm(_) => Err("Cannot execute special forms with rands already evaluated!".to_string())
	}
}

fn eval_if(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("if", 3, expr_list);

	let cond = expr_list.get(0).unwrap().clone();

	match eval_expr(cond, env)? {
		Boolean(b) => eval_expr(if b { expr_list.get(1).unwrap().clone() } else { expr_list.get(2).unwrap().clone() }, env),
		_ => Err("Special form 'if' expected a boolean!".to_string())
	}
}

fn eval_cond(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("cond", 2, expr_list);
	if expr_list.len() % 2 != 0 {
		return Err("Special form 'cond' expected an even amount of args!".to_string());
	}

	for i in 0..(expr_list.len()/2) {
		let cond = 2 * i;
		let possible_v = 2 * i + 1;

		match eval_expr(expr_list[cond].clone(), env)? {
			Boolean(false) => continue,
			Boolean(true) => return eval_expr(expr_list[possible_v].clone(), env),
			_ => return Err("Special form 'cond' expected a boolean!".to_string())
		}
	}

	Ok(void())
}

fn eval_define(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	check_arity_is!("define", 2, expr_list);
	let binding = match expr_list[0].clone() {
		ExprAst::Atom(a) => {
			match *a {
				AtomAst::Identifier(n) => n,
				_ => return Err("Unexpected binding!".to_string())
			}
		}
		_ => return Err("Unexpected binding!".to_string())
	};

	let val = eval_expr(expr_list[1].clone(), env)?;

	env.add_binding(binding, val);
	Ok(void())
}

fn eval_do(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	for expr in expr_list {
		eval_expr(expr, env)?;
	}

	Ok(void())
}

fn eval_and(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("and", 2, expr_list);

	for expr in expr_list {
		match eval_expr(expr, env)? {
			Boolean(false) => return Ok(Boolean(false)),
			Boolean(true) => continue,
			_ => return Err("Special form 'and' expected a boolean!".to_string())
		}
	}

	Ok(Boolean(true))
}

fn eval_or(expr_list: ExprListAst, env: &mut Environment) -> Result<Val, String> {
	check_arity_at_least!("and", 2, expr_list);

	for expr in expr_list {
		match eval_expr(expr, env)? {
			Boolean(false) => continue,
			Boolean(true) => return Ok(Boolean(true)),
			_ => return Err("Special form 'or' expected a boolean!".to_string())
		}
	}

	Ok(Boolean(false))
}
