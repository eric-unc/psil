use std::collections::HashMap;

use crate::ast::Name;
use crate::native::add_native_library;
use crate::val::{NativeProcedure, ProcedureType, Val};
use crate::val::Val::Procedure;

pub type Scope = HashMap<String, Val>;
pub type Bindings = Vec<Scope>;

#[derive(Clone)]
pub struct Environment {
	bindings: Bindings
}

impl Default for Environment {
	fn default() -> Self {
		Self::new()
	}
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

	pub fn add_binding(&mut self, name: Name, val: Val) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, val);
	}

	pub fn add_proc(&mut self, name: Name, val: NativeProcedure) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name, Procedure(ProcedureType::Native(val)));
	}

	pub fn get_binding(&self, name: Name) -> Result<Val, String> {
		for bindings in self.bindings.iter().rev() {
			if bindings.contains_key(&name) {
				let value = bindings.get(&name).unwrap();
				return Ok(value.clone());
			}
		}

		Err(format!("Binding {} does not exist!", name))
	}
}
