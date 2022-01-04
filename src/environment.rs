use std::collections::HashMap;

use crate::ast::Name;
use crate::doc::{Documentation, Entry};
use crate::stdlib::add_standard_library;
use crate::val::{NativeProcedure, Procedure, Val};
use crate::val::Val::ProcedureV;

pub type Scope = HashMap<String, Val>;
pub type Bindings = Vec<Scope>;

#[derive(Clone)]
pub struct Environment {
	bindings: Bindings,
	doc: Documentation,
	curr_module: String
}

impl Default for Environment {
	fn default() -> Self {
		Self::new()
	}
}

impl Environment {
	pub fn new() -> Self {
		let mut ret = Self { bindings: vec![Scope::new()], doc: Documentation::new(), curr_module: String::from("<None>") };

		add_standard_library(&mut ret);

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

	pub fn add_proc(&mut self, name: &str, val: NativeProcedure) {
		let len = self.bindings.len();

		self.bindings[len - 1].insert(name.to_string(), ProcedureV(Procedure::Native(val)));
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

	pub fn get_doc(&self) -> &Documentation {
		&self.doc
	}

	pub fn add_entry(&mut self, proc: String, entry: Entry) {
		self.doc.add_entry(proc, entry);
	}

	pub fn get_entry(&mut self, proc: &str) -> Option<&Entry> {
		self.doc.get_entry(proc)
	}

	pub fn get_curr_module(&self) -> &str {
		self.curr_module.as_str()
	}

	pub fn set_curr_module(&mut self, module: &str) {
		self.curr_module = String::from(module);
	}
}
