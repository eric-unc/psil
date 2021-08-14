use std::collections::HashMap;
use std::string::String;

use crate::ast::*;

#[derive(Clone, Debug)]
pub enum Val {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Procedure(ProcedureType),
	Error(String),
}

use Val::{Boolean, Error, Number, Procedure as ProcedureVal, String as StringVal, Void};

pub type ValList = Vec<Val>;

#[derive(Clone, Debug)]
pub enum ProcedureType {
	Native(NativeProcedure),
	Pure(LambdaAst),
}

pub type NativeProcedure = fn(ValList) -> Val;

impl PartialEq for Val {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Number(n) => match other {
				Number(o) => *n == *o,
				_ => false,
			},
			Boolean(b) => match other {
				Boolean(o) => *b == *o,
				_ => false,
			},
			StringVal(s) => match other {
				StringVal(o) => s.eq(o),
				_ => false,
			},
			_ => false,
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
		Self {
			bindings: vec![Scope::new()],
		}
	}

	pub fn add_scope(&mut self) {
		self.bindings.push(Scope::new());
	}

	pub fn close_scope(&mut self) {
		self.bindings.pop();
	}

	pub fn add_binding(&mut self, name: String, val: Val) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, val);
	}

	pub fn add_proc(&mut self, name: String, val: NativeProcedure) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, Val::Procedure(ProcedureType::Native(val)));
	}

	pub fn get_binding(&mut self, name: String) -> Val {
		let len = self.bindings.len();

		for i in len..0 {
			if self.bindings[i].contains_key(&name) {
				let value = self.bindings[i].get(&name).unwrap();
				return value.clone();
			}
		}

		panic!("Binding does not exist!")
	}
}

pub fn eval(program: ProgramAst) {
	eval_with_env(program, &mut Environment::new());
}

pub fn eval_with_env(program: ProgramAst, env: &mut Environment) {
	eval_program(program, env);
}

// program ::= expr+
fn eval_program(program: ProgramAst, env: &mut Environment) {
	eval_expr_list(program.expr_list, env);
}

// expr_list ::= expr+
fn eval_expr_list(expr_list: ExprListAst, env: &mut Environment) -> Vec<Val> {
	vec![]
}
