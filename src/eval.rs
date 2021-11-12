use std::string::String;

use crate::ast::*;
use crate::environment::Environment;
use crate::val::{ProcedureType, Val, void};
use crate::val::Val::{Boolean, Number, Procedure, String as StringVal, Symbol};

pub fn eval(program: ProgramAst) -> Result<Vec<Val>, String> {
	eval_program(program, &mut Environment::new())
}

// program ::= expr_list?
fn eval_program(program: ProgramAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	eval_expr_list(program.expr_list, env)
}

// expr_list ::= expr+
fn eval_expr_list(expr_list: ExprListAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	let mut ret = Vec::new(); // TODO: could maybe convert to functional-style

	for expr in expr_list.into_iter() {
		let val = eval_expr(expr, env)?;
		ret.push(val);
	}

	Ok(ret)
}

// expr ::= atom | special_form | invocation
pub fn eval_expr(expr: ExprAst, env: &mut Environment) -> Result<Val, String> {
	match expr {
		ExprAst::Atom(a) => eval_atom(*a, env),
		ExprAst::SpecialForm(s) => eval_special_form(*s, env),
		ExprAst::Invocation(i) => eval_invocation(i, env)
	}
}

// atom ::= number | boolean | string | symbol | lambda | name
fn eval_atom(atom: AtomAst, env: &mut Environment) -> Result<Val, String> {
	match atom {
		AtomAst::Number(n) => Ok(Number(n)),
		AtomAst::Boolean(b) => Ok(Boolean(b)),
		AtomAst::String(s) => Ok(StringVal(s)),
		AtomAst::Symbol(s) => Ok(Symbol(s)),
		AtomAst::Lambda(l) => Ok(Procedure(ProcedureType::Pure(l))),
		AtomAst::Name(n) => env.get_binding(n)
	}
}

// special_form ::= if | cond | define | do | and | or
fn eval_special_form(special_form: SpecialFormAst, env: &mut Environment) -> Result<Val, String> {
	match special_form {
		SpecialFormAst::If(i) => eval_if(i, env),
		SpecialFormAst::Cond(c) => eval_cond(c, env),
		SpecialFormAst::Define(d) => eval_define(d, env),
		SpecialFormAst::Do(d) => eval_do(d, env),
		SpecialFormAst::And(a) => eval_and(a, env),
		SpecialFormAst::Or(a) => eval_or(a, env)
	}
}

// invocation ::= ( name expr_list? )
fn eval_invocation(invocation: InvocationAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: some weirdness with Rust borrowing here, should try to see if there's a better way to do this.
	let name = invocation.proc;
	let proc_fetch = env.get_binding(name.clone());

	match proc_fetch {
		Ok(Procedure(p)) => {
			// TODO: could maybe convert to functional-style
			let mut rands = Vec::new();

			for expr in invocation.expr_list.into_iter() {
				let val = eval_expr(expr, env)?;
				rands.push(val);
			}

			match p {
				ProcedureType::Native(n) => { n(rands) }
				ProcedureType::Pure(p) => {
					if p.params.names.len() != rands.len() {
						return match p.params.names.len() {
							0 => Err(format!("Proc '{}' expected no rands! Given {}.", name.clone(), rands.len())),
							1 => Err(format!("Proc '{}' expected 1 rand! Given {}.", name.clone(), rands.len())),
							_ => Err(format!("Proc '{}' expected {} rands! Given {}.", name.clone(), p.params.names.len(), rands.len()))
						}
					}

					env.add_scope();

					p.params.names.into_iter()
						.zip(rands.into_iter())
						.for_each(|(name, value)| env.add_binding(name, value));

					let ret = eval_expr(p.expr, env);

					env.close_scope();

					ret
				}
			}
		}
		Ok(_) => Err(format!("Binding {} is not a procedure!", name.clone())),
		Err(e) => Err(e)
	}
}

// if ::= ( if expr expr expr )
fn eval_if(if_form: IfAst, env: &mut Environment) -> Result<Val, String> {
	match eval_expr(if_form.cond, env) {
		Ok(Boolean(b)) => eval_expr(if b { if_form.if_true } else { if_form.if_false }, env),
		Ok(_)=> Err("Special form 'if' expected a boolean!".to_string()),
		Err(e) => Err(e)
	}
}

// cond ::= ( cond (expr expr)+ )
fn eval_cond(cond_form: CondAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: convert to functional
	for (cond, expr) in cond_form.conds.into_iter().zip(cond_form.expr_list.into_iter()) {
		match eval_expr(cond, env) {
			Ok(Boolean(false)) => continue,
			Ok(Boolean(true)) => return eval_expr(expr, env),
			Ok(_) => return Err("Special form 'cond' expected a boolean!".to_string()),
			Err(e) => return Err(e)
		}
	}

	Ok(void())
}

// define ::= ( define name expr )
fn eval_define(define: DefineAst, env: &mut Environment) -> Result<Val, String> {
	let val = eval_expr(define.value, env)?;
	env.add_binding(define.name, val);
	Ok(void())
}

// do ::= ( do expr_list? )
fn eval_do(do_ast: DoAst, env: &mut Environment) -> Result<Val, String> {
	for expr in do_ast.expr_list {
		eval_expr(expr, env)?;
	}

	Ok(void())
}

// and ::= ( and expr_list )
fn eval_and(and_ast: AndAst, env: &mut Environment) -> Result<Val, String> {
	if and_ast.expr_list.len() < 2 {
		return Err(format!("Special form 'and' expected at least 2 rands! Given {}.", and_ast.expr_list.len()))
	}

	for expr in and_ast.expr_list {
		match eval_expr(expr, env) {
			Ok(Boolean(false)) => return Ok(Boolean(false)),
			Ok(Boolean(true)) => continue,
			Ok(_) => return Err("Special form 'and' expected a boolean!".to_string()),
			Err(e) => return Err(e)
		}
	}

	Ok(Boolean(true))
}

// or ::= ( or expr_list )
fn eval_or(or_ast: OrAst, env: &mut Environment) -> Result<Val, String> {
	if or_ast.expr_list.len() < 2 {
		return Err(format!("Special form 'or' expected at least 2 rands! Given {}.", or_ast.expr_list.len()))
	}

	for expr in or_ast.expr_list {
		match eval_expr(expr, env) {
			Ok(Boolean(false)) => continue,
			Ok(Boolean(true)) => return Ok(Boolean(true)),
			Ok(_) => return Err("Special form 'or' expected a boolean!".to_string()),
			Err(e) => return Err(e)
		}
	}

	Ok(Boolean(false))
}
