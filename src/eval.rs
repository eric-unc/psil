use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as ResultFmt};
use std::string::String;

use crate::ast::*;
use crate::native::add_native_library;

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Procedure(ProcedureType),
	Error(String),
}

use Val::{Boolean, Error, Number, Procedure, String as StringVal, Void};

impl Display for Val {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		write!(f, "{}", match self {
			Number(n) => n.to_string(),
			Boolean(b) => b.to_string(),
			StringVal(s) => s.to_string(),
			Void => "void".to_string(),
			Procedure(_) => "<procedure>".to_string(), // TODO: some day this will be much more advanced
			Error(e) => e.to_string(), // TODO: this too
		})
	}
}

pub type ValList = Vec<Val>;

#[derive(Clone, Debug)]
pub enum ProcedureType {
	Native(NativeProcedure),
	Pure(LambdaAst),
}

pub type NativeProcedure = fn(ValList) -> Val;

impl PartialEq for Val {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Number(n), Number(o)) => *n == *o,
			(Boolean(b), Boolean(o)) => *b == *o,
			(StringVal(s), StringVal(o)) => s.eq(o),
			_ => false
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

pub type Scope = HashMap<String, Val>;
pub type Bindings = Vec<Scope>;

pub struct Environment {
	bindings: Bindings,
}

impl Environment {
	pub fn new() -> Self {
		let mut ret = Self { bindings: vec![Scope::new()], };

		add_native_library(&mut ret);

		ret
	}

	pub fn add_scope(&mut self) {
		self.bindings.push(Scope::new());
	}

	pub fn close_scope(&mut self) {
		self.bindings.pop();
	}

	pub fn add_binding(&mut self, name: NameAst, val: Val) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, val);
	}

	pub fn add_proc(&mut self, name: NameAst, val: NativeProcedure) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, Procedure(ProcedureType::Native(val)));
	}

	pub fn get_binding(&self, name: NameAst) -> Result<Val, String> {
		for bindings in self.bindings.iter().rev() {
			if bindings.contains_key(&name) {
				let value = bindings.get(&name).unwrap();
				return Ok(value.clone());
			}
		}

		Err(format!("Binding {} does not exist!", name))
	}
}

pub fn eval(program: ProgramAst) -> Result<Vec<Val>, String> {
	eval_program(program, &mut Environment::new())
}


// program ::= expr_list?
fn eval_program(program: ProgramAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	eval_expr_list(program.expr_list, env)
}

// expr_list ::= expr+
fn eval_expr_list(expr_list: ExprListAst, env: &mut Environment) -> Result<Vec<Val>, String> {
	/*Ok(expr_list.into_iter()
		.map(|expr| eval_expr(expr, env))
		.collect())*/

	let mut ret = Vec::new();

	for expr in expr_list.into_iter() {
		let val = eval_expr(expr, env);

		match val {
			Ok(v) => ret.push(v),
			Err(e) => return Err(e)
		}
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

// atom ::= number | boolean | string | void | lambda | name
fn eval_atom(atom: AtomAst, env: &mut Environment) -> Result<Val, String> {
	match atom {
		AtomAst::Number(n) => Ok(Number(n)),
		AtomAst::Boolean(b) => Ok(Boolean(b)),
		AtomAst::String(s) => Ok(StringVal(s)),
		AtomAst::Void => Ok(Void),
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
		Ok(proc) => {
			match proc {
				Procedure(p) => {
					/*let rands = invocation.expr_list.into_iter()
						.map(|expr| eval_expr(expr, env))
						.collect();*/

					let mut rands = Vec::new();

					for expr in invocation.expr_list.into_iter() {
						match eval_expr(expr, env) {
							Ok(val) => rands.push(val),
							Err(e) => return Err(e)
						}
					}

					match p {
						ProcedureType::Native(n) => { Ok(n(rands)) } // TODO
						ProcedureType::Pure(p) => {
							if p.params.names.len() > rands.len() {
								return Err(format!("Procedure {} called with {} missing parameters!", name.clone(), p.params.names.len() - rands.len()))
							} else if p.params.names.len() < rands.len() {
								return Err(format!("Procedure {} called with {} extra parameters!", name.clone(), rands.len() - p.params.names.len()))
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
				_ => Err(format!("Binding {} is not a procedure!", name.clone()))
			}
		}
		Err(e) => Err(e)
	}
}

// if ::= ( if expr expr expr )
fn eval_if(if_form: IfAst, env: &mut Environment) -> Result<Val, String> {
	match eval_expr(if_form.cond, env) {
		Ok(cond) => match cond {
			Boolean(b) => eval_expr(if b { if_form.if_true } else { if_form.if_false }, env),
			_ => Err("Expected boolean as condition!".to_string())
		}
		Err(e) => Err(e)
	}
}

// cond ::= ( cond (expr expr)+ )
fn eval_cond(cond_form: CondAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: convert to functional
	for (cond, expr) in cond_form.conds.into_iter().zip(cond_form.expr_list.into_iter()) {
		/*match eval_expr(cond, env) {
			Boolean(b) =>
				if b {
					return eval_expr(expr, env)
				}
			Error(e) => return Error(e),
			_ => return Error("Expected boolean as condition!".to_string())
		}*/
		match eval_expr(cond, env) {
			Ok(val) => match val {
				Boolean(b) =>
					if b {
						return eval_expr(expr, env)
					}
				_ => return Err("Expected boolean as condition!".to_string())
			}
			Err(e) => return Err(e)
		}
	}

	Ok(Void)
}

// define ::= ( define name expr )
fn eval_define(define: DefineAst, env: &mut Environment) -> Result<Val, String> {
	let val = eval_expr(define.value, env);

	match val {
		Ok(val) => {
			env.add_binding(define.name, val);
			Ok(Void)
		}
		Err(e) => Err(e)
	}
}

// do ::= ( do expr_list? )
fn eval_do(do_ast: DoAst, env: &mut Environment) -> Result<Val, String> {
	for expr in do_ast.expr_list {
		match eval_expr(expr, env) {
			Ok(_) => {},
			Err(e) => return Err(e)
		}
	}

	// TODO: For some reason, this doesn't work.
	/*do_ast.expr_list.into_iter()
		.for_each(|expr| eval_expr(expr, env));*/

	Ok(Void)
}

// and ::= ( and expr_list )
fn eval_and(and_ast: AndAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: check for rands
	for expr in and_ast.expr_list {
		match eval_expr(expr, env) {
			Ok(v) => match v {
				Boolean(b) =>
					if !b {
						return Ok(Boolean(false))
					}
				_ => return Err("Expected boolean as condition!".to_string())
			}
			Err(e) => return Err(e)
		}
	}

	Ok(Boolean(true))
}

// or ::= ( or expr_list )
fn eval_or(or_ast: OrAst, env: &mut Environment) -> Result<Val, String> {
	// TODO: check for rands
	for expr in or_ast.expr_list {
		match eval_expr(expr, env) {
			Ok(val) => match val {
				Boolean(b) =>
					if b {
						return Ok(Boolean(true))
					}
				_ => return Err("Expected boolean as condition!".to_string())
			}
			Err(e) => return Err(e)
		}

	}

	Ok(Boolean(false))
}

