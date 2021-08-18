use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
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
	fn fmt(&self, f: &mut Formatter) -> Result {
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

	pub fn get_binding(&self, name: NameAst) -> Val {
		for bindings in self.bindings.iter().rev() {
			if bindings.contains_key(&name) {
				let value = bindings.get(&name).unwrap();
				return value.clone();
			}
		}

		Error(format!("Binding {} does not exist!", name))
	}
}

pub fn eval(program: ProgramAst) {
	//eval_with_env(program, &mut Environment::new());
	eval_program(program, &mut Environment::new());
}

/*fn eval_with_env(program: ProgramAst, env: &mut Environment) {
	eval_program(program, env);
}*/

pub fn eval_expr_with_env(expr: ExprAst, env: &mut Environment) {
	eval_expr_with_env(expr, env);
}

// program ::= expr_list?
fn eval_program(program: ProgramAst, env: &mut Environment) {
	eval_expr_list(program.expr_list, env);
}

// expr_list ::= expr+
fn eval_expr_list(expr_list: ExprListAst, env: &mut Environment) -> Vec<Val> {
	expr_list.into_iter()
		.map(|expr| eval_expr(expr, env))
		.collect()
}

// expr ::= atom | special_form | invocation
fn eval_expr(expr: ExprAst, env: &mut Environment) -> Val {
	match expr {
		ExprAst::Atom(a) => eval_atom(*a, env),
		ExprAst::SpecialForm(s) => eval_special_form(*s, env),
		ExprAst::Invocation(i) => eval_invocation(i, env)
	}
}

// atom ::= number | boolean | string | void | lambda | name
fn eval_atom(atom: AtomAst, env: &mut Environment) -> Val {
	match atom {
		AtomAst::Number(n) => Number(n),
		AtomAst::Boolean(b) => Boolean(b),
		AtomAst::String(s) => StringVal(s),
		AtomAst::Void => Void,
		AtomAst::Lambda(l) => Procedure(ProcedureType::Pure(l)),
		AtomAst::Name(n) => env.get_binding(n)
	}
}

// special_form ::= if | cond | define | do | and | or
fn eval_special_form(special_form: SpecialFormAst, env: &mut Environment) -> Val {
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
fn eval_invocation(invocation: InvocationAst, env: &mut Environment) -> Val {
	// TODO: some weirdness with Rust borrowing here, should try to see if there's a better way to do this.
	let name = invocation.proc;
	let proc = env.get_binding(name.clone());

	match proc {
		Procedure(p) => {
			let rands = invocation.expr_list.into_iter()
				.map(|expr| eval_expr(expr, env))
				.collect();

			match p {
				ProcedureType::Native(n) => { n(rands) }
				ProcedureType::Pure(p) => {
					if p.params.names.len() > rands.len() {
						return Error(format!("Procedure {} called with {} missing parameters!", name.clone(), p.params.names.len() - rands.len()))
					} else if p.params.names.len() < rands.len() {
						return Error(format!("Procedure {} called with {} extra parameters!", name.clone(), rands.len() - p.params.names.len()))
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
		_ => Error(format!("Binding {} is not a procedure!", name.clone()))
	}
}

// if ::= ( if expr expr expr )
fn eval_if(if_form: IfAst, env: &mut Environment) -> Val {
	let cond = eval_expr(if_form.cond, env);

	match cond {
		Boolean(b) => eval_expr(if b { if_form.if_true } else { if_form.if_false }, env),
		Error(e) => Error(e),
		_ => Error("Expected boolean as condition!".to_string())
	}
}

// cond ::= ( cond (expr expr)+ )
fn eval_cond(cond_form: CondAst, env: &mut Environment) -> Val {
	// TODO: convert to functional
	for (cond, expr) in cond_form.conds.into_iter().zip(cond_form.expr_list.into_iter()) {
		match eval_expr(cond, env) {
			Boolean(b) =>
				if b {
					return eval_expr(expr, env)
				}
			Error(e) => return Error(e),
			_ => return Error("Expected boolean as condition!".to_string()),
		}
	}

	Void
}

// define ::= ( define name expr )
fn eval_define(define: DefineAst, env: &mut Environment) -> Val {
	let val = eval_expr(define.value, env);

	env.add_binding(define.name, val);

	Void
}

// do ::= ( do expr_list? )
fn eval_do(do_ast: DoAst, env: &mut Environment) -> Val {
	for expr in do_ast.expr_list {
		eval_expr(expr, env);
	}

	// TODO: For some reason, this doesn't work.
	/*do_ast.expr_list.into_iter()
		.for_each(|expr| eval_expr(expr, env));*/

	Void
}

// and ::= ( and expr_list )
fn eval_and(and_ast: AndAst, env: &mut Environment) -> Val {
	// TODO: check for rands
	for expr in and_ast.expr_list {
		match eval_expr(expr, env) {
			Boolean(b) =>
				if !b {
					return Boolean(false)
				}
			_ => return Error("Expected boolean as condition!".to_string())
		}
	}

	Boolean(true)
}

// or ::= ( or expr_list )
fn eval_or(or_ast: OrAst, env: &mut Environment) -> Val {
	// TODO: check for rands
	for expr in or_ast.expr_list {
		match eval_expr(expr, env) {
			Boolean(b) =>
				if b {
					return Boolean(true)
				}
			_ => return Error("Expected boolean as condition!".to_string())
		}
	}

	Boolean(false)
}

