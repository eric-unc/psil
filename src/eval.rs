use std::collections::HashMap;

use crate::ast::*;

pub enum Value {
	Number(f64),
	Boolean(bool),
	String(String),
	Void,
	Procedure(Procedure),
	Error(String)
}

use Value::{Number, Boolean, String, Void, Procedure, Error};

pub type ValueList = Vec<Value>;

pub enum Procedure {
	Native(NativeProcedure),
	Pure(Atom::Lambda)
}

pub type NativeProcedure = fn(ValueList) -> Value;

impl PartialEq for Value {
	fn eq(&self, other: &Self) -> bool {
		match self {
			Number(n) =>
				match other {
					Number(o) => *n == *o,
					_ => false
				}
			Boolean(b) =>
				match other {
					Boolean(o) => *b == *o,
					_ => false
				}
			String(s) =>
				match other {
					String(o) => s.eq(o),
					_ => false
				}
			Procedure(p) =>
				match other {
					Procedure(o) => p == o,
					_ => false
				}
			_ => false
		}
	}

	fn ne(&self, other: &Self) -> bool {
		!self.eq(other)
	}
}

pub type Scope = HashMap<String, LimpValue>;
pub type Bindings = Vec<Scope>;

pub struct Environment {
	bindings: Bindings
}

impl Environment {
	pub fn new() -> Self {
		Self {
			bindings: vec![Scope::new()]
		}
	}

	pub fn add_scope(&mut self){
		self.bindings.push(Scope::new());
	}

	pub fn close_scope(&mut self){
		self.bindings.pop();
	}

	pub fn add_binding(&mut self, name: String, val: Value){
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, val);
	}

	pub fn get_binding(&mut self, name: String) -> Value {
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

pub fn eval(program: Program) {
	eval_with_env(program, &mut Environment::new());
}

pub fn eval_with_env(program: Program, env: &mut Environment) {
	eval_program(program, env);
}

// program ::= expr+
fn eval_program(program: Program, env: &mut Environment) {
	eval_expr_list(expr_list, env);
}

// expr_list ::= expr+
fn eval_expr_list(expr_list: ExpressionList, env: &mut Environment) -> Vec<Value> {
	// TODO
}



