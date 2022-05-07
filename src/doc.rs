use std::fmt::{Display, Formatter, Result as ResultFmt};

use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Documentation {
	// XXX: We shouldn't care about order, but the documentation generating code is just kind of a mess right now so we have to, to get ABC order. Eventually, we'll probably have this all ported to Psil directly, and then it won't necessarily be an issue.
	entries: BTreeMap<String, Entry>
}

impl Documentation {
	pub fn new() -> Self {
		Documentation { entries: BTreeMap::new() }
	}

	pub fn add_entry(&mut self, proc: String, entry: Entry) {
		self.entries.insert(proc, entry);
	}

	pub fn get_entry(&mut self, proc: &str) -> Option<&Entry> {
		self.entries.get(proc)
	}

	pub fn get_entries(&self) -> &BTreeMap<String, Entry> {
		&self.entries
	}
}

#[derive(Clone)]
pub struct Entry {
	pub proc: String,
	pub aliases: Vec<String>,
	pub description: String,
	pub params: BTreeMap<String, String>,
	pub module: String
}

impl Entry {
	pub fn new(proc: String, aliases: Vec<String>, description: String, params: BTreeMap<String, String>, module: &str) -> Self {
		Self { proc, aliases, description, params, module: String::from(module) }
	}
}

impl Display for Entry {
	fn fmt(&self, f: &mut Formatter) -> ResultFmt {
		writeln!(f, "## {}", self.proc)?;

		writeln!(f, "{}", self.description)?;
		writeln!(f)?;

		writeln!(f, "Parameters:")?;

		if self.params.is_empty() {
			writeln!(f, "* None")?;
		} else {
			for (name, description) in &self.params {
				writeln!(f, "* `{}`: {}", name, description)?;
			}
		}

		writeln!(f)
	}
}
