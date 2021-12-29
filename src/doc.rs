use std::fmt::{Debug, Display, Formatter, Result as ResultFmt};

use std::collections::{BTreeMap, HashMap};

#[derive(Clone)]
pub struct Documentation {
	entries: HashMap<String, Entry>
}

impl Documentation {
	pub fn new() -> Self {
		Documentation { entries: HashMap::new() }
	}

	pub fn add_entry(&mut self, proc: String, entry: Entry) {
		self.entries.insert(proc, entry);
	}

	pub fn get_entry(&mut self, proc: &str) -> Option<&Entry> {
		self.entries.get(proc)
	}
}

#[derive(Clone)]
pub struct Entry {
	proc: String,
	aliases: Vec<String>,
	description: String,
	params: BTreeMap<String, String>
}

impl Entry {
	pub fn new(proc: String) -> Self {
		Self { proc, aliases: Vec::new(), description: String::new(), params: BTreeMap::new() }
	}

	pub fn new_full(proc: String, aliases: Vec<String>, description: String, params: BTreeMap<String, String>) -> Self {
		Self { proc, aliases, description, params }
	}
}

impl Display for Entry {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		writeln!(f, "{}", self.proc)?;
		writeln!(f)?;

		writeln!(f, "{}", self.description)?;
		writeln!(f)?;

		writeln!(f, "Parameters:")?;

		if self.params.is_empty() {
			writeln!(f, "- None")?;
		} else {
			for (name, description) in &self.params {
				writeln!(f, "- {}: {}", name, description)?;
			}
		}

		writeln!(f)
	}
}
